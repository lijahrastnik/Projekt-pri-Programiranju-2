use rand::Rng;
use crate::torta::{Torta, Nadstropje, Okus, Preliv, Topping};

pub fn zgeneriraj_narocilo() -> Torta {
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

        
        vsa_nadstropja.push(Nadstropje::novo(okus, preliv, None));
    }

    // Prvo nadstropje vzamemo kot spodnje
    let mut torta = Torta::nova(vsa_nadstropja.remove(0));

    // Ostala nadstropja dodamo na torto
    for n in vsa_nadstropja {
        torta.dodaj_nadstropje(n);
    }

    // Na koncu, ko je torta zgrajena, naključno določimo okrasek
    let topping = match rng.gen_range(0..4) {
        0 => Some(Topping::Svecka),
        1 => Some(Topping::Cesnja),
        2 => Some(Topping::Sadje),
        _ => None, 
    };

    if let Some(t) = topping {
        torta.dodaj_topping_zadnjemu(t);
    }

    torta
}