use crate::torta::{Okus, Preliv, Topping, Torta};

pub fn preveri_enakost(zgenerirana_torta: &Torta, igralec_torta: &Torta) -> bool {
    zgenerirana_torta == igralec_torta
}