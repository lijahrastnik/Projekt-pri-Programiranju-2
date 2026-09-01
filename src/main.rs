    

    mod torta;
    mod logika;
    mod generiranje_narocil;
    mod display;
    mod igra;
    
    #[macroquad::main("Integral place")]
    async fn main() {
        igra::poganjaj_igro().await;
    }