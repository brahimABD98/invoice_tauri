use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize,Deserialize,TS, Debug)]
#[ts(export,export_to="../src/bindings/invoiceline.ts")]
pub struct Invoiceline {
    pub produit: String,
    pub qte: u16,
    pub puht: f32,
    pub tva: f32,
    pub ttc: f32,
    taux: f32,
}
impl Default for Invoiceline {
    fn default() -> Self {
        Self{produit: "".to_owned(),
        qte: 1,
        puht: 0.0,
        tva: 0.0,
        taux: 20.0,
        ttc: 0.0,} 
    }
    
}
impl Invoiceline {
    pub fn new() -> Self {
        Self {
            produit: "".to_owned(),
            qte: 1,
            puht: 0.0,
            tva: 0.0,
            taux: 20.0,
            ttc: 0.0,
        }
    }

    fn get_tva(&mut self) -> f32 {
        if self.tva == 0.0 {
            if self.puht > 0.0 && self.ttc == 0.0 {
                return (self.puht * self.qte as f32 * self.taux) / 100.0;
            }

            if self.puht == 0.0 && self.ttc > 0.0 {
                return (self.ttc * 100.0 * self.taux) / ((100.0 + self.taux) * 100.0);
            }
        }
        return self.tva;
    }

    fn get_puht(&mut self) -> f32 {
        if self.puht == 0.0 {
            self.tva = self.get_tva();
            return (self.ttc / self.qte as f32) - self.tva;
        }
        return self.puht;
    }
    fn get_ttc(&mut self) -> f32 {
        if self.ttc == 0.0 {
            self.tva = self.get_tva();
            self.puht = self.get_puht();
            return (self.puht * self.qte as f32) + self.tva;
        }
        return self.ttc;
    }
    pub fn get_taux(self) -> f32 {
        self.taux
    }
    pub fn resolve(&mut self) {
        self.puht = self.get_puht();
        self.tva = self.get_tva();
        self.ttc = self.get_ttc();
    }
}
