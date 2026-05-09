use macroquad::prelude::*;
mod torta;
mod logika;
mod generiranje_narocil;
mod display;

use crate::torta::{Torta, Nadstropje, Okus, Preliv, Topping};



#[macroquad::main("Purple Place Cake Factory")]
async fn main() {
    let mut narocilo = generiranje_narocil::zgeneriraj_narocilo();
    let mut igralec_torta: Option<Torta> = None;

    loop {
        clear_background(WHITE);

        
        if is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Key3) {
            let okus = if is_key_pressed(KeyCode::Key1) { Okus::Cokolada }
                       else if is_key_pressed(KeyCode::Key2) { Okus::Vanilija }
                       else { Okus::Jagoda };
            
            let n = Nadstropje::novo(okus, Preliv::Cokoladni, None); 
            if let Some(ref mut t) = igralec_torta {
                if t.get_ostala().len() < 4 { 
                    t.dodaj_nadstropje(n);
                }
            } else {
                igralec_torta = Some(Torta::nova(n));
            }
        }

       
        if let Some(ref mut t) = igralec_torta {
            if is_key_pressed(KeyCode::Key4) { t.nastavi_preliv_zadnjemu(Preliv::Cokoladni); }
            if is_key_pressed(KeyCode::Key5) { t.nastavi_preliv_zadnjemu(Preliv::Sadni); }
            if is_key_pressed(KeyCode::Key6) { t.nastavi_preliv_zadnjemu(Preliv::Karamelni); }
        }

        
        if let Some(ref mut t) = igralec_torta {
            if is_key_pressed(KeyCode::T) { t.dodaj_topping_zadnjemu(Topping::Svecka); }
            if is_key_pressed(KeyCode::C) { t.dodaj_topping_zadnjemu(Topping::Cesnja); }
            if is_key_pressed(KeyCode::S) { t.dodaj_topping_zadnjemu(Topping::Sadje); }
        }

        
        if is_key_pressed(KeyCode::R) { igralec_torta = None; }
        if is_key_pressed(KeyCode::N) { narocilo = generiranje_narocil::zgeneriraj_narocilo(); }

        
        if is_key_pressed(KeyCode::Enter) {
            if let Some(ref t) = igralec_torta {
                if logika::preveri_enakost(&narocilo, t) {
                    
                    println!("TORTA JE PRAVILNA!"); 
                }
            }
        }

        //RISANJE
        draw_text("NAROCILO (N za novo)", 50.0, 30.0, 20.0, DARKGRAY);
        display::narisi_torta(&narocilo, 200.0, 550.0);

        draw_text("TVOJA TORTA (1-3 Biskvit, 4-6 Preliv, T/C/S Okras, R Reset, ENTER Preveri)", 400.0, 30.0, 20.0, BLACK);
        if let Some(ref t) = igralec_torta {
            display::narisi_torta(t, 600.0, 550.0);
        }

        next_frame().await
    }
}