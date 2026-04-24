pub enum Okus {
    Cokolada,
    Vanilija,
    Jagoda,
    Drugo(String),
}

pub enum Preliv {
    Cokoladni,
    Sadni,
    Karamelni,
}

pub enum Topping {
    Svecka,
    Cesnja,
    Sadje,
}

//----------------------------------------------------------------
pub struct Nadstropje {
    okus: Okus,
    preliv: Preliv,
    topping: Option<Topping>,
}

pub struct Torta {
    spodnje: Nadstropje,
    ostala: Vec<Nadstropje>,
}

//-----------------------------------------------------------------
impl Nadstropje {
    pub fn novo(okus: Okus, preliv: Preliv) -> Self {
        Self {
            okus,
            preliv,
            topping: None,
        }
    }
}

impl Torta {

}