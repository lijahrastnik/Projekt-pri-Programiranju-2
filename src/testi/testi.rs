mod torta;

use crate::torta::{Torta, Nadstropje, Okus, Preliv, Topping};
use crate::generiranje_narocil::*;
use crate::logika::*;

#[test_1]
fn test_torti_enakost() {
    let prvo = Nadstropje::novo(Okus::Cokolada, Preliv::Karamelni, Some(Topping::Cesnja));
    let drugo = Nadstropje::novo(Okus::Vanilija, Preliv::Sadni, None);

    let mut t1 = Torta::nova(prvo);
    t1.dodaj_nadstropje(drugo);

    let prvo2 = Nadstropje::novo(Okus::Cokolada, Preliv::Karamelni, Some(Topping::Cesnja));
    let drugo2 = Nadstropje::novo(Okus::Vanilija, Preliv::Sadni, None);

    let mut t2 = Torta::nova(prvo2);
    t2.dodaj_nadstropje(drugo2);

    assert!(preveri_enakost(&t1, &t2));

    let prvo3 = Nadstropje::novo(Okus::Jagoda, Preliv::Karamelni, Some(Topping::Cesnja));
    let drugo3 = Nadstropje::novo(Okus::Vanilija, Preliv::Sadni, None);

    let mut t3 = Torta::nova(prvo3);
    t3.dodaj_nadstropje(drugo3);

    assert!(!preveri_enakost(&t1, &t3));
}

#[test_2]
fn random_narocilo_samo_s_seboj() {
    let torta = zgeneriraj_narocilo();

    assert!(preveri_enakost(&torta, &torta));
}