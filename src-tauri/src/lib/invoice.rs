use crate::lib::invoiceline::Invoiceline;
use round::{round, round_up};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

struct Date;
impl Date {
    pub fn today() -> String {
        let d = chrono::Local::now().date_naive();
        let d = format!("{}", d.format("%d-%m-%y"));
        return d;
    }
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/invoice.ts")]
#[derive(Default)]
pub struct Invoice {
    pub client: String,
    pub date: String,
    pub htva: f64,
    pub tva: f64,
    pub timbre: f64,
    pub number: u16,
    taux: f64,
    pub ttc: f64,
    pub invoicelinelist: Vec<Invoiceline>,
}
impl Invoice {
    pub fn make() -> Self {
        Self {
            client: "".to_owned(),
            date: Date::today(),
            ttc: 0.0,
            htva: 0.0,
            tva: 0.0,
            timbre: 0.6,
            number: 1,
            taux: 20.0,
            invoicelinelist: vec![Invoiceline::default()],
        }
    }
    pub fn new(
        _client: String,
        _date: String,
        _ttc: f64,
        _htva: f64,
        _tva: f64,
        _timbre: f64,
        _number: u16,
        _taux: f64,
        _invoiceline: Vec<Invoiceline>,
    ) -> Self {
        Self {
            client: _client,
            date: _date,
            ttc: _ttc,
            htva: _htva,
            tva: _tva,
            timbre: _timbre,
            number: _number,
            taux: _taux,
            invoicelinelist: _invoiceline,
        }
    }
    fn get_tva(&mut self) -> f64 {
        if self.tva > 0.0 {
            return self.tva;
        }
        if self.tva < 0.0 {
            return 0.0;
        }

        if self.htva > 0.0 {
            return self.htva * self.taux / 100.0;
        }
        if self.ttc > 0.0 {
            return ((self.ttc - self.timbre) * 100.0 * self.taux) / ((100.0 + self.taux) * 100.0);
        }
        if self.invoicelinelist.len() > 0 {
            return self
                .invoicelinelist
                .iter()
                .fold(0.0, |acc, currentvalue| currentvalue.tva + acc);
        }

        return self.tva;
    }

    fn get_ttc(&mut self) -> f64 {
        if self.ttc > 0.0 {
            return self.ttc;
        }
        if self.ttc < 0.0 {
            return 0.0;
        }

        if self.htva > 0.0 {
            return self.htva + self.htva * self.taux / 100.0;
        }
        if self.tva > 0.0 {
            return self.tva / (1.0 - (1.0 / (1.0 + (self.taux as f64 / 100.0))));
        }
        if self.invoicelinelist.len() > 0 {
            return self
                .invoicelinelist
                .iter()
                .fold(0.0, |acc, currentvalue| acc + currentvalue.ttc)
                + self.timbre;
        }
        return self.ttc;
    }

    fn get_htva(&mut self) -> f64 {
        if self.invoicelinelist.len() > 0 {
            let result = self.invoicelinelist.iter().fold(0.0, |acc, currentvalue| {
                (currentvalue.puht * currentvalue.qte as f64) + acc
            });
            if result > self.htva {
                return result;
            }
        }
        if self.htva < 0.0 {
            return 0.0;
        }
        if self.ttc > 0.0 && self.tva > 0.0 {
            return (self.ttc - self.timbre) - self.tva;
        }
        if self.ttc > 0.0 {
            return self.ttc + (1.0 + (self.taux / 100.0));
        }
        if self.tva > 0.0 {
            return (self.tva * 100.0) / self.taux;
        }

        return self.htva;
    }
    pub fn resolve(&mut self) {
        self.invoicelinelist
            .iter_mut()
            .for_each(|invoiceline| invoiceline.resolve());

        self.ttc = round(self.get_ttc(), 3);

        self.tva = round(self.get_tva(), 3);

        self.htva = round_up(self.get_htva(), 3);
    }
    pub fn get_taux(&self) -> f64 {
        self.taux
    }
}
