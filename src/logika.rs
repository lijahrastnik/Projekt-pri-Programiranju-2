use crate::torta::{Okus, Preliv, Topping, Torta};

pub fn preveri_enakost(zgenerirana_torta: &Torta, igralec_torta: &Torta) -> bool {
    // Preveri enakost števila nadstropij 
    if zgenerirana_torta.get_ostala().len() != igralec_torta.get_ostala().len() {
        println!("Napačno število nadstropij!");
        return false; 
    }

    // Sestavimo sezname referenc na nadstropja
    let mut vsa_zgenerirana = vec![zgenerirana_torta.get_spodnje()];
    for n in zgenerirana_torta.get_ostala().iter() {
        vsa_zgenerirana.push(n);
    }

    let mut vsa_igralec = vec![igralec_torta.get_spodnje()];
    for n in igralec_torta.get_ostala().iter() {
        vsa_igralec.push(n);
    }

    // Preverimo vse lastnosti za vsako nadstropje v eni zanki
    for i in 0..vsa_zgenerirana.len() {
        // Preveri okuse
        if !enakost_okusov(vsa_zgenerirana[i].get_okus(), vsa_igralec[i].get_okus()) {
            println!("Napačen okus v {}. nadstropju!", i + 1);
            return false;
        }

        // Preveri prelive 
        if !enakost_prelivov(vsa_zgenerirana[i].get_preliv(), vsa_igralec[i].get_preliv()) {
            println!("Napačen preliv v {}. nadstropju!", i + 1);
            return false;
        }

        // Preveri toppinge
        if !enakost_toppingov(vsa_zgenerirana[i].get_topping(), vsa_igralec[i].get_topping()) {
            println!("Napačen topping v {}. nadstropju!", i + 1);
            return false;
        }
    }
  
    true
}

fn enakost_okusov(o1: &Okus, o2: &Okus) -> bool {
    match (o1, o2) {
        (Okus::Cokolada, Okus::Cokolada) => true,
        (Okus::Vanilija, Okus::Vanilija) => true,
        (Okus::Jagoda, Okus::Jagoda) => true,
        (Okus::Drugo(s1), Okus::Drugo(s2)) => s1 == s2,
        _ => false,
    }
}

fn enakost_prelivov(p1: &Preliv, p2: &Preliv) -> bool {
    match (p1, p2) {
        (Preliv::Cokoladni, Preliv::Cokoladni) => true,
        (Preliv::Sadni, Preliv::Sadni) => true,
        (Preliv::Karamelni, Preliv::Karamelni) => true,
        _ => false,
    }
}

fn enakost_toppingov(t1: &Option<Topping>, t2: &Option<Topping>) -> bool {
    match (t1, t2) {
        (Some(Topping::Svecka), Some(Topping::Svecka)) => true,
        (Some(Topping::Cesnja), Some(Topping::Cesnja)) => true,
        (Some(Topping::Sadje), Some(Topping::Sadje)) => true,
        (None, None) => true,
        _ => false,
    }
}