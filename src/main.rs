use macroquad::prelude::*;
mod torta;
mod logika;
mod generiranje_narocil;
mod display;

use crate::torta::{Torta, Nadstropje, Okus, Preliv, Topping};

#[macroquad::main("Integral place")]
async fn main() {
    let mut narocilo = generiranje_narocil::zgeneriraj_narocilo();
    let mut igralec_torta: Option<Torta> = None;
    
    let mut torta_je_pravilna = false;

    // Spremenljivka za točke
    let mut tocke = 0;

    let svetlo_vijolicna = Color::new(0.92, 0.85, 0.98, 1.0);

    loop {
        clear_background(svetlo_vijolicna);

        if !torta_je_pravilna && (is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Key3)) {
            let okus = if is_key_pressed(KeyCode::Key1) { Okus::Cokolada }
                       else if is_key_pressed(KeyCode::Key2) { Okus::Vanilija }
                       else { Okus::Jagoda };
            
            if let Some(ref mut t) = igralec_torta {
                t.nastavi_okus_zadnjemu(okus);
            } else {
                let n = Nadstropje::novo(okus, Preliv::Cokoladni, None); 
                igralec_torta = Some(Torta::nova(n));
            }
        }

        if !torta_je_pravilna && is_key_pressed(KeyCode::Space) {
            if let Some(ref mut t) = igralec_torta {
                if t.get_ostala().len() < 4 { 
                    let n = Nadstropje::novo(Okus::Cokolada, Preliv::Cokoladni, None);
                    t.dodaj_nadstropje(n);
                }
            }
        }
        
        if !torta_je_pravilna {
            if let Some(ref mut t) = igralec_torta {
                if is_key_pressed(KeyCode::Key4) { t.nastavi_preliv_zadnjemu(Preliv::Cokoladni); }
                if is_key_pressed(KeyCode::Key5) { t.nastavi_preliv_zadnjemu(Preliv::Sadni); }
                if is_key_pressed(KeyCode::Key6) { t.nastavi_preliv_zadnjemu(Preliv::Karamelni); }
            }
        }

        if !torta_je_pravilna {
            if let Some(ref mut t) = igralec_torta {
                if is_key_pressed(KeyCode::T) { t.dodaj_topping_zadnjemu(Topping::Svecka); }
                if is_key_pressed(KeyCode::C) { t.dodaj_topping_zadnjemu(Topping::Cesnja); }
                if is_key_pressed(KeyCode::S) { t.dodaj_topping_zadnjemu(Topping::Sadje); }
            }
        }

        if is_key_pressed(KeyCode::R) { 
            igralec_torta = None; 
            torta_je_pravilna = false;
        }
        if is_key_pressed(KeyCode::N) { 
            narocilo = generiranje_narocil::zgeneriraj_narocilo(); 
            igralec_torta = None;
            torta_je_pravilna = false;
        }

        if is_key_pressed(KeyCode::Enter) {
            if let Some(ref t) = igralec_torta {
                if logika::preveri_enakost(&narocilo, t) {
                    println!("PRAVILNO");
                    
                    if !torta_je_pravilna {
                        tocke += 1;
                    }
                    torta_je_pravilna = true;
                }
            }
        }

        draw_text("NAROCILO (N za novo)", 50.0, 30.0, 20.0, DARKGRAY);
        display::narisi_torta(&narocilo, 200.0, 550.0);

        draw_text("TVOJA TORTA (1-3 Okus biskvita, SPACE Novo nadstropje, 4-6 Preliv, T/C/S Okras, R Reset, ENTER Preveri)", 400.0, 30.0, 16.0, BLACK);
        if let Some(ref t) = igralec_torta {
            display::narisi_torta(t, 600.0, 550.0);
        }

       
        let besedilo_tocke = format!("TOČKE: {}", tocke);
        draw_text(&besedilo_tocke, 50.0, 70.0, 25.0, BLUE);

        if torta_je_pravilna {
            draw_text("TORTA JE PRAVILNA!", 400.0, 100.0, 30.0, GREEN);
        }

        next_frame().await
    }
}