
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Okus {
    Cokolada,
    Vanilija,
    Jagoda,
    Drugo(String),
    
}

#[derive(PartialEq)]
pub enum Preliv {
    Cokoladni,
    Sadni,
    Karamelni,
}

#[derive(PartialEq)]
pub enum Topping {
    Svecka,
    Cesnja,
    Sadje,
}

//----------------------------------------------------------------
#[derive(PartialEq)]
pub struct Nadstropje {
    okus: Okus,
    preliv: Preliv,
}

#[derive(PartialEq)]
pub struct Torta {
    nadstropja: Vec<Nadstropje>,
    topping: Option<Topping>,
}

//-----------------------------------------------------------------

impl Nadstropje {
    //USTVARI NOVO NADSTROPJE Z DANIMI LASTNOSTMI
    pub fn novo(okus: Okus, preliv: Preliv) -> Self {
        Self {
            okus,
            preliv
        }
    }

    pub fn get_okus(&self) -> &Okus {
        &self.okus
    }

    pub fn get_preliv(&self) -> &Preliv {
        &self.preliv
    }
    }

impl Torta {
    //USTVARI NOVO TORTO, KI IMA ZAČETNO SPODNJE NADSTROPJE
    pub fn nova(prvo: Nadstropje) -> Self {
        Self {
            nadstropja: vec![prvo],
            topping: None,
        }
    }

    pub fn get_nadstropja (&self) -> &Vec<Nadstropje> {
        &self.nadstropja
    }

    pub fn get_topping(&self) -> Option<&Topping> {
        self.topping.as_ref()
    }

    pub fn dodaj_nadstropje(&mut self, novo_nadstropje: Nadstropje) {
        self.nadstropja.push(novo_nadstropje);
    }


    pub fn dodaj_topping(&mut self, topping: Topping) {
        self.topping = Some(topping);
    }

    pub fn nastavi_preliv_zadnjemu(&mut self, novi_preliv: Preliv) {
        if let Some(last) = self.nadstropja.last_mut() {
            last.preliv = novi_preliv;
        }
    }

    pub fn nastavi_okus_zadnjemu(&mut self, novi_okus: Okus) {
        if let Some(last) = self.nadstropja.last_mut() {
            last.okus = novi_okus;
        }
    }
    }