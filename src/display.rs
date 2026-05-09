//??????? mogoce delava kje drugje (Bevy, bracket - lib, macroquad)

//vse je treba prepisat v macroquad strukturo



use macroquad::prelude::*;

use crate::torta::{Torta, Okus, Preliv, Topping};

pub fn narisi_torta(torta: &Torta, x: f32, y_tla: f32) {
    let vsa_nadstropja = torta.get_ostala();
    let spodnje = torta.get_spodnje();
    

    let mut vse_plasti = vec![spodnje];
    for n in vsa_nadstropja {
        vse_plasti.push(n);
    }

    let mut trenutni_y = y_tla;

    
    for n in vse_plasti.iter() {
        let barva_biskvita = okus_v_barvo(n.get_okus());
        let barva_preliva = preliv_v_barvo(n.get_preliv());

        
        draw_rectangle(x - 100.0, trenutni_y - 40.0, 200.0, 40.0, barva_biskvita);
        
        draw_rectangle_lines(x - 100.0, trenutni_y - 40.0, 200.0, 40.0, 2.0, BLACK);
        
        
        draw_rectangle(x - 100.0, trenutni_y - 50.0, 200.0, 10.0, barva_preliva);
        
       
        trenutni_y -= 55.0; 
    }

    
    if let Some(zadnje) = vse_plasti.last() {
        if let Some(t) = zadnje.get_topping() {
            let emoji = match t {
                Topping::Svecka => "🕯️",
                Topping::Cesnja => "🍒",
                Topping::Sadje => "🍓",
            };
            
            draw_text(emoji, x - 15.0, trenutni_y + 5.0, 40.0, WHITE);
        }
    }
}



fn okus_v_barvo(okus: &Okus) -> Color {
    match okus {
        Okus::Cokolada => BROWN,
        Okus::Vanilija => BEIGE,
        Okus::Jagoda => PINK,
        Okus::Drugo(_) => WHITE,
    }
}

fn preliv_v_barvo(preliv: &Preliv) -> Color {
    match preliv {
        Preliv::Cokoladni => DARKBROWN,
        Preliv::Sadni => RED,
        Preliv::Karamelni => ORANGE,
    }
}















/* 
use std::fmt;

use crate::{Nadstropje, Okus, Preliv, Topping, Torta};

impl fmt::Display for Okus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Okus::Cokolada => write!(f, "Čokolada"),
            Okus::Vanilija => write!(f, "Vanilija"),
            Okus::Jagoda => write!(f, "Jagoda"),
            Okus::Drugo(s) => write!(f, "{s}"),
        }
    }
}

impl Preliv {
    fn narisi(&self, sirina: usize) -> String {
        let znak = match self {
            Preliv::Cokoladni => "~",
            Preliv::Sadni => "^",
            Preliv::Karamelni => "=",
        };

        znak.repeat(sirina)
    }
}

impl Topping {
    fn narisi(&self) -> &str {
        match self {
            Topping::Svecka => "🕯",
            Topping::Cesnja => "🍒",
            Topping::Sadje => "🍓",
        }
    }
}
*/













/* 
impl fmt::Display for Nadstropje {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let okus = self.okus();
        let notranja_sirina = okus.len().max(10);

        let topping = match self.topping() {
            Some(t) => t.narisi(),
            None => " ",
        };

        writeln!(
            f,
            "  {:^width$}",
            topping,
            width = notranja_sirina
        )?;

        writeln!(
            f,
            "  {}",
            self.preliv().narisi(notranja_sirina + 4)
        )?;

        writeln!(
            f,
            " | {:^width$} |",
            okus,
            width = notranja_sirina
        )?;

        writeln!(
            f,
            " |{}|",
            "_".repeat(notranja_sirina + 2)
        )
    }
}

impl fmt::Display for Torta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for n in self.ostala().iter().rev() {
            writeln!(f, "{n}")?;
        }

        writeln!(f, "{}", self.spodnje())
    }
} */