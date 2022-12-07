use round::{round, round_up};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/invoiceline.ts")]
pub struct Invoiceline {
    pub produit: String,
    pub qte: u16,
    pub puht: f64,
    pub tva: f64,
    pub ttc: f64,
    taux: f64,
}
impl Default for Invoiceline {
    fn default() -> Self {
        Self {
            produit: "".to_owned(),
            qte: 1,
            puht: 0.0,
            tva: 0.0,
            taux: 20.0,
            ttc: 0.0,
        }
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

    fn get_tva(&self) -> f64 {
        if self.tva > 0.0 {
            return self.tva;
        }
        if self.tva < 0.0 {
            return 0.0;
        }

        if self.puht > 0.0 {
            return ((self.puht * self.qte as f64) * self.taux) / 100.0;
        }
        if self.ttc > 0.0 {
            return (self.ttc * 100.0 * self.taux) / ((100.0 + self.taux) * 100.0);
        }

        return self.tva;
    }

    fn get_puht(&self) -> f64 {
        if self.puht > 0.0 {
            return self.puht;
        }
        if self.puht < 0.0 {
            return 0.0;
        }

        if self.ttc > 0.0 {
            return (self.ttc / self.qte as f64) + (1.0 + (self.taux / 100.0));
        }
        if self.tva > 0.0 {
            return ((self.tva * 100.0) / self.taux) / self.qte as f64;
        }

        return self.puht;
    }

    //get tout taxe compris
    fn get_ttc(&self) -> f64 {
        if self.ttc > 0.0 {
            return self.ttc;
        }
        if self.ttc < 0.0 {
            return 0.0;
        }

        if self.puht > 0.0 {
            return self.puht * self.qte as f64 + self.puht * self.qte as f64 * self.taux / 100.0;
        }
        if self.tva > 0.0 {
            return self.tva / (1.0 - (1.0 / (1.0 + self.taux as f64)));
        }

        return self.ttc;
    }
    pub fn get_taux(&self) -> f64 {
        self.taux
    }
    pub fn resolve(&mut self) {
        self.puht = round_up(self.get_puht(), 3);
        self.tva = round(self.get_tva(), 3);
        self.ttc = round(self.get_ttc(), 3);
    }
}
