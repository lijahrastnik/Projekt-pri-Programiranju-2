use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

use crate::torta::{Torta, Nadstropje, Okus, Preliv, Topping};
use crate::generiranje_narocil;
use crate::logika;
use crate::display;

// STRUKTURI ZA LESTVICO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rezultat {
    pub ime: String,
    pub tocke: i32,
}

pub struct Lestvica {
    pub seznam: Vec<Rezultat>,
}

impl Lestvica {
    //NALOŽI OBSTOJEČE REZULTATE IZ DATOTEKE "rezultati.json"
    pub fn nalozi() -> Self {
        if let Ok(vsebina) = fs::read_to_string("rezultati.json") {
            if let Ok(seznam) = serde_json::from_str(&vsebina) {
                return Self { seznam };
            }
        }
        Self { seznam: Vec::new() } //ČE DATOTEKA ŠE NE OBSTAJA, VRNEMO PRAZEN SEZNAM
    }

    pub fn dodaj_rezultat(&mut self, ime: String, tocke: i32) {
        let končno_ime = if ime.trim().is_empty() {
            "Neznanec".to_string()
        } else {
            ime
        };

        self.seznam.push(Rezultat { ime: končno_ime, tocke });
        
        // RAZVRSTIMO PO TOČKAH PADAJOČE
        self.seznam.sort_by(|a, b| b.tocke.cmp(&a.tocke));
        
        // OBDRŽIMO LE NAJBOLJŠIH 5
        self.seznam.truncate(5);

        // SHRANIMO POSODOBLJEN SEZNAM V JSON DATOTEKO
        if let Ok(vsebina) = serde_json::to_string_pretty(&self.seznam) {
            let _ = fs::write("rezultati.json", vsebina);
        }
    }
}



// --- LOGIKA IGRE ---
enum StanjeIgre {
    VnosImena,
    Igra,
    KonecIgre,
}

pub async fn poganjaj_igro() {
    let mut narocilo = generiranje_narocil::zgeneriraj_narocilo();
    let mut igralec_torta: Option<Torta> = None;
    
    let mut torta_je_pravilna = false;
    let mut tocke = 0;

    let mut stanje = StanjeIgre::VnosImena;
    let mut zacetni_cas = 0.0;
    let cas_za_igro = 45.0;

    // LESTVICA IN VNOS IMENA
    let mut lestvica = Lestvica::nalozi();
    let mut ime_igralca = String::new();

    let svetlo_vijolicna = Color::new(0.92, 0.85, 0.98, 1.0);

    loop {
        clear_background(svetlo_vijolicna);

        match stanje {
            StanjeIgre::VnosImena => {
                draw_text("VNESI SVOJE IME:", 280.0, 200.0, 30.0, DARKGRAY);
                
                // ZAJEMANJE TIPKANJA S TIPKOVNICE
                while let Some(c) = get_char_pressed() {
                    if c.is_alphanumeric() || c == ' ' {
                        if ime_igralca.len() < 12 { // OMEJITEV DOLŽINE IMENA
                            ime_igralca.push(c);
                        }
                    }
                }
                
                // BRISANJE Z BACKSPACE
                if is_key_pressed(KeyCode::Backspace) {
                    ime_igralca.pop();
                }

                // OKVIR IN PRIKAZ NATIPKANEGA IMENA
                draw_rectangle(290.0, 230.0, 220.0, 45.0, WHITE);
                draw_rectangle_lines(290.0, 230.0, 220.0, 45.0, 2.0, DARKGRAY);
                draw_text(&ime_igralca, 300.0, 262.0, 25.0, BLACK);

                draw_text("Pritisni ENTER za zacetek!", 260.0, 320.0, 20.0, DARKGRAY);

                if is_key_pressed(KeyCode::Enter) {
                    stanje = StanjeIgre::Igra;
                    zacetni_cas = get_time();
                    tocke = 0;
                    narocilo = generiranje_narocil::zgeneriraj_narocilo();
                    igralec_torta = None;
                    torta_je_pravilna = false;
                }
            }

            StanjeIgre::Igra => {
                let pretecen_cas = get_time() - zacetni_cas;
                let preostali_cas = cas_za_igro - pretecen_cas;

                if preostali_cas <= 0.0 {
                    // SHRANIMO REZULTAT V LESTVICO KO POTEČE ČAS
                    lestvica.dodaj_rezultat(ime_igralca.clone(), tocke);
                    stanje = StanjeIgre::KonecIgre;
                }

                // TIPKE 1 DO 3
                if !torta_je_pravilna && (is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Key3)) {
                    let okus = if is_key_pressed(KeyCode::Key1) { Okus::Cokolada }
                               else if is_key_pressed(KeyCode::Key2) { Okus::Vanilija }
                               else { Okus::Jagoda };
                    
                    if let Some(ref mut t) = igralec_torta {
                        t.nastavi_okus_zadnjemu(okus);
                    } else {
                        let n = Nadstropje::novo(okus, Preliv::Cokoladni); 
                        igralec_torta = Some(Torta::nova(n));
                    }
                }
                // TIPKA SPACE
                if !torta_je_pravilna && is_key_pressed(KeyCode::Space) {
                    if let Some(ref mut t) = igralec_torta {
                        if t.get_nadstropja().len() < 5 { 
                            let n = Nadstropje::novo(Okus::Cokolada, Preliv::Cokoladni);
                            t.dodaj_nadstropje(n);
                        }
                    }
                }
                //TIPKE 4-6
                if !torta_je_pravilna {
                    if let Some(ref mut t) = igralec_torta {
                        if is_key_pressed(KeyCode::Key4) { t.nastavi_preliv_zadnjemu(Preliv::Cokoladni); }
                        if is_key_pressed(KeyCode::Key5) { t.nastavi_preliv_zadnjemu(Preliv::Sadni); }
                        if is_key_pressed(KeyCode::Key6) { t.nastavi_preliv_zadnjemu(Preliv::Karamelni); }
                    }
                }
                //TIPKE TCS
                if !torta_je_pravilna {
                    if let Some(ref mut t) = igralec_torta {
                        if is_key_pressed(KeyCode::T) { t.dodaj_topping(Topping::Svecka); }
                        if is_key_pressed(KeyCode::C) { t.dodaj_topping(Topping::Cesnja); }
                        if is_key_pressed(KeyCode::S) { t.dodaj_topping(Topping::Sadje); }
                    }
                }

                //TIPKA R
                if is_key_pressed(KeyCode::R) { 
                    igralec_torta = None; 
                    torta_je_pravilna = false;
                }

                //ENTER
                if is_key_pressed(KeyCode::Enter) {
                    if let Some(ref t) = igralec_torta {
                        if logika::preveri_enakost(&narocilo, t) {
                            tocke += 1;
                            narocilo = generiranje_narocil::zgeneriraj_narocilo();
                            igralec_torta = None;
                            torta_je_pravilna = false;
                        }
                    }
                }

                // IZRIS GRAFIČNIH ELEMENTOV
                draw_text("NAROCILO", 50.0, 30.0, 20.0, DARKGRAY);
                display::narisi_torta(&narocilo, 200.0, 550.0);

                
    draw_rectangle(440.0, 20.0, 270.0, 75.0, Color::new(0.9, 0.9, 0.9, 0.85));
    draw_rectangle_lines(440.0, 20.0, 270.0, 75.0, 1.5, DARKGRAY);

    
    draw_text("1-3: Okus  •  SPACE: Nadstropje", 450.0, 40.0, 13.0, BLACK);
    draw_text("4-6: Preliv  •  T/C/S: Okras", 450.0, 58.0, 13.0, BLACK);
    draw_text("R: Reset  •  ENTER: Preveri", 450.0, 76.0, 13.0, DARKGRAY);

    if let Some(ref t) = igralec_torta {
    display::narisi_torta(t, 600.0, 550.0);
    }

                let besedilo_tocke = format!("TOCKE: {}", tocke);
                draw_text(&besedilo_tocke, 50.0, 70.0, 25.0, BLUE);

                let besedilo_cas = format!("CAS: {:.1} s", preostali_cas.max(0.0));
                let barva_casa = if preostali_cas < 5.0 { RED } else { DARKGRAY };
                draw_text(&besedilo_cas, 50.0, 100.0, 25.0, barva_casa);
            }

            StanjeIgre::KonecIgre => {
                draw_text("CAS JE POTEKEL!", 260.0, 80.0, 40.0, Color::new(1.0, 0.41, 0.71, 1.0));
                
                let rezultat_tekst = format!("{} - Tvoj rezultat: {}", ime_igralca, tocke);
                draw_text(&rezultat_tekst, 250.0, 130.0, 25.0, BLACK);

                // PRIKAZ LESTVICE NAJBOLJŠIH 5
                draw_text("--- NAJBOLJSIH 5 ---", 240.0, 190.0, 25.0, DARKGRAY);
                let mut y_offset = 230.0;
                for (i, r) in lestvica.seznam.iter().enumerate() {
                    let vrstica = format!("{}. {} : {}", i + 1, r.ime, r.tocke);
                    draw_text(&vrstica, 290.0, y_offset, 22.0, BLUE);
                    y_offset += 30.0;
                }

                draw_text("Pritisni SPACE za ponovno igro", 240.0, 450.0, 20.0, DARKGRAY);

                // PONOVNI ZAGON IGRE
                if is_key_pressed(KeyCode::Space) {
                    ime_igralca.clear();
                    tocke = 0;
                    igralec_torta = None;
                    torta_je_pravilna = false;

                    while get_char_pressed().is_some() {}

                    stanje = StanjeIgre::VnosImena;
                }
            }
        }

        next_frame().await;
    }
}