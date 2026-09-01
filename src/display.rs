use macroquad::prelude::*;
use crate::torta::{Torta, Okus, Preliv, Topping};

//DEFINIRANE BARVE ZA PRELIVE IN NADSTROPJA
const BARVA_COKOLADA: Color = Color::new(0.36, 0.21, 0.14, 1.0);  
const BARVA_VANILIJA: Color = Color::new(0.96, 0.90, 0.71, 1.0);  
const BARVA_JAGODA: Color = Color::new(0.94, 0.53, 0.61, 1.0);    

const PRELIV_COKOLADA: Color = Color::new(0.24, 0.12, 0.07, 1.0); 
const PRELIV_SADNI: Color = Color::new(0.85, 0.11, 0.25, 1.0);    
const PRELIV_KARAMELA: Color = Color::new(0.82, 0.51, 0.22, 1.0); 

pub fn narisi_torta(torta: &Torta, x: f32, y_base: f32) {
    // SESTAVIMO SEZNAM VSEH NADSTROPIJ OD SPODAJ NAVZGOR (spodnje + ostala)
    let mut vsa_nadstropja = vec![torta.get_spodnje()];
    for n in torta.get_ostala().iter() {
        vsa_nadstropja.push(n);
    }

    let mut trenutni_y = y_base;
    
    let mut sirina = 180.0; //ZAČETNA ŠIRINA NAJNIŽJEGA NADSTROPJA
    let visina_biskvita = 40.0;
    let visina_preliva = 10.0;

    for nadstropje in vsa_nadstropja {
        trenutni_y -= visina_biskvita;
        let trenutni_x = x - (sirina / 2.0);

        // DOLOČITEV BARVE BISKVITA
        let barva_biskvita = match nadstropje.get_okus() {
            Okus::Cokolada => BARVA_COKOLADA,
            Okus::Vanilija => BARVA_VANILIJA,
            Okus::Jagoda => BARVA_JAGODA,
            Okus::Drugo(_) => GRAY,

        };

        
        draw_rectangle(trenutni_x, trenutni_y, sirina, visina_biskvita, barva_biskvita);

        // DOLOČITEV BARVE PRELIVA
        let barva_preliva = match nadstropje.get_preliv() {
            Preliv::Cokoladni => PRELIV_COKOLADA,
            Preliv::Sadni => PRELIV_SADNI,
            Preliv::Karamelni => PRELIV_KARAMELA,
        };

        
        draw_rectangle(trenutni_x + 4.0, trenutni_y, sirina - 8.0, visina_preliva, barva_preliva);

        // IZRIS TOPPINGA
        if let Some(topping) = nadstropje.get_topping() {
            match topping {

                // ČEŠNJA
                Topping::Cesnja => {
                    // cesnja
                    draw_circle(x - 6.0, trenutni_y - 8.0, 6.0, RED);
                    draw_circle(x + 6.0, trenutni_y - 8.0, 6.0, RED);
                
                    // peclja
                    draw_line(
                        x - 6.0,
                        trenutni_y - 14.0,
                        x,
                        trenutni_y - 24.0,
                        2.0,
                        DARKGREEN,
                    );
                
                    draw_line(
                        x + 6.0,
                        trenutni_y - 14.0,
                        x,
                        trenutni_y - 24.0,
                        2.0,
                        DARKGREEN,
                    );
                }
            
                // SADJE
                Topping::Sadje => {
                    draw_circle(x - 10.0, trenutni_y - 8.0, 5.0, ORANGE);
                    draw_circle(x,       trenutni_y - 12.0, 5.0, RED);
                    draw_circle(x + 10.0, trenutni_y - 8.0, 5.0, PINK);
                }
            
                // SVEČKA
                Topping::Svecka => {
                    // telo svečke
                    draw_rectangle(
                        x - 2.0,
                        trenutni_y - 20.0,
                        4.0,
                        14.0,
                        YELLOW,
                    );
                
                    // plamen
                    draw_circle(
                        x,
                        trenutni_y - 24.0,
                        4.0,
                        ORANGE,
                    );
                }
            };
        }
        sirina -= 25.0; 
    }
}












