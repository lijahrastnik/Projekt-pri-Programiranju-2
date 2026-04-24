use rand::Rng;
use crate::{torta::{Okus, Preliv, Topping, Torta, Nadstropje}};

pub fn zgeneriraj_torto() -> Torta {
    let mut rng = rand::thread_rng();

    // Zgeneriramo število nadstropij (med 1 in 5)
    let st_nadstropij = rng.gen_range(1..6);

    // Seznam za nadstropja
    let mut vsa_nadstropja = Vec::new();

    for _ in 0..st_nadstropij {
        let okus = match rng.gen_range(0..3) {
            0 => Okus::Cokolada,
            1 => Okus::Vanilija,
            _ => Okus::Jagoda,
        };

        let preliv = match rng.gen_range(0..3) {
            0 => Preliv::Cokoladni,
            1 => Preliv::Sadni,
            _ => Preliv::Karamelni,
        };

        let topping = match rng.gen_range(0..4) {
            0 => Some(Topping::Svecka),
            1 => Some(Topping::Cesnja),
            2 => Some(Topping::Sadje),
            _ => None,
        };

        vsa_nadstropja.push(Nadstropje { okus, preliv, topping });
    }

    // Sestavimo strukturo Torta
    Torta {
        spodnje: vsa_nadstropja.remove(0), 
        ostala: vsa_nadstropja,
    }
}