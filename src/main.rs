mod torta;
mod logika;
mod generiranje_narocil;
mod display;

use crate::torta::{Torta, Nadstropje, Okus, Preliv, Topping};

fn main() {
    let prvo = Nadstropje::novo(Okus::Cokolada, Preliv::Karamelni);

    let drugo = Nadstropje::novo(Okus::Vanilija, Preliv::Sadni);

    let mut torta = Torta::nova(prvo);

    torta.dodaj_nadstropje(drugo);

    torta.dodaj_topping_zadnjemu(Topping::Cesnja);
}
