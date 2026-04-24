mod torta;

use crate::torta::{Torta, Nadstropje, Okus, Preliv, Topping};

 #[test]
    fn it_works() {
        let result = true; // tukej poklici tvojo funkcijo in ji kaj za parameter to torto spodej in navodilo za to isto torto
        assert_eq!(result, true);
}

/* torta ki  jo preveri

let prvo = Nadstropje::novo(Okus::Cokolada, Preliv::Karamelni);

    let drugo = Nadstropje::novo(Okus::Vanilija, Preliv::Sadni);

    let mut torta = Torta::nova(prvo);

    torta.dodaj_nadstropje(drugo);

    torta.dodaj_topping_zadnjemu(Topping::Cesnja);

*/