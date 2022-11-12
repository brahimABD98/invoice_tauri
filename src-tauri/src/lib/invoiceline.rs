pub struct InvoiceLine {
    produit: String,
    qte: u16,
    puht: f32,
    taux: u8,
    tva: f32,

}

impl InvoiceLine {
   pub fn new()->InvoiceLine{
        InvoiceLine{
            produit: "".to_owned(),
            qte: 0,
            puht: 0.0,
            taux: 0,
            tva: 0.0
        }
    }
}