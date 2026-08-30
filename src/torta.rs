
#[derive(PartialEq)]
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
    topping: Option<Topping>,
}

#[derive(PartialEq)]
pub struct Torta {
    spodnje: Nadstropje,
    ostala: Vec<Nadstropje>,
}

//-----------------------------------------------------------------

impl Nadstropje {
    //USTVARI NOVO NADSTROPJE Z DANIMI LASTNOSTMI
    pub fn novo(okus: Okus, preliv: Preliv, topping: Option<Topping>) -> Self {
        Self {
            okus,
            preliv,
            topping,
        }
    }

    pub fn get_okus(&self) -> &Okus {
        &self.okus
    }

    pub fn get_preliv(&self) -> &Preliv {
        &self.preliv
    }

    pub fn get_topping(&self) -> &Option<Topping>{
        &self.topping
    }
   
}

impl Torta {
    //USTVARI NOVO TORTO, KI IMA ZAČETNO SPODNJE NADSTROPJE
    pub fn nova(prvo: Nadstropje) -> Self {
        Self {
            spodnje: prvo,
            ostala: Vec::new(),
        }
    }

    pub fn get_ostala (&self) -> &Vec<Nadstropje> {
        &self.ostala
    }

    pub fn get_spodnje (&self) -> &Nadstropje{
        &self.spodnje
    }

    pub fn dodaj_nadstropje(&mut self, novo_nadstropje: Nadstropje) {
        //OB DODAJANJU NOVEGA NADSTROPJA SE OKRAS NA PREJ NAJVIŠJEM NADSTROPJU ODSTRANI
        if let Some(last) = self.ostala.last_mut() {
            last.topping = None;
        } else {
            self.spodnje.topping = None;
        }

        self.ostala.push(novo_nadstropje);
    }


    pub fn dodaj_topping_zadnjemu(&mut self, topping: Topping) {
        if let Some(last) = self.ostala.last_mut() {
            last.topping = Some(topping);
        } else {
            self.spodnje.topping = Some(topping);
        }
    }

    pub fn nastavi_preliv_zadnjemu(&mut self, novi_preliv: Preliv) {
        if let Some(last) = self.ostala.last_mut() {
            last.preliv = novi_preliv; 
        } else {
            self.spodnje.preliv = novi_preliv;
        }
    }

    pub fn nastavi_okus_zadnjemu(&mut self, novi_okus: Okus) {
        if let Some(last) = self.ostala.last_mut() {
            last.okus = novi_okus; 
        } else {
            self.spodnje.okus = novi_okus;
        }
    }


}