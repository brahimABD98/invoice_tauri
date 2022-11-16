use crate::lib::invoiceline::Invoiceline;
#[derive(Debug, Default)]
pub struct Invoice {
    pub client: String,
    pub date: String,

    pub htva: f32,
    pub tva: f32,
    pub timbre: f32,
    pub number: u16,
    taux: f32,
    pub ttc: f32,
    pub invoicelinelist: Vec<Invoiceline>,
}
impl Invoice {
    fn make() -> Self {
        Self {
            client: "".to_owned(),
            date: "".to_owned(),
            ttc: 0.0,
            htva: 0.0,
            tva: 0.0,
            timbre: 0.6,
            number: 1,
            taux: 0.0,
            invoicelinelist: Default::default(),
        }
    }
    fn new(
        _client: String,
        _date: String,
        _ttc: f32,
        _htva: f32,
        _tva: f32,
        _timbre: f32,
        _number: u16,
        _taux: f32,
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
    fn get_tva(&mut self) -> f32 {
        if self.tva == 0.0 {
            if self.invoicelinelist.len() > 0 {
                return self
                    .invoicelinelist
                    .iter()
                    .fold(0.0, |acc, currentvalue| currentvalue.tva + acc);
            }
            if self.ttc > 0.0 {
                return ((self.ttc - self.timbre) * 100.0 * self.taux)
                    / ((100.0 + self.taux) * 100.0);
            }
        }
        return self.tva;
    }
    fn get_ttc(&mut self) -> f32 {
        if self.ttc == 0.0 {
            if self.htva > 0.0 && self.tva > 0.0 {
                return self.htva + self.tva + self.timbre;
            } else {
                return self
                    .invoicelinelist
                    .iter()
                    .fold(0.0, |acc, currentvalue| acc + currentvalue.ttc)
                    + self.timbre;
            }
        }
        return self.ttc;
    }
    fn get_htva(&mut self) -> f32 {
        if self.htva > 0.0 {
            return self.htva;
        }

        if self.invoicelinelist.len() > 0 {
            return self.invoicelinelist.iter().fold(0.0, |acc, currentvalue| {
                (currentvalue.puht * currentvalue.qte as f32) + acc
            });
        }
        self.tva = self.get_tva();
        self.ttc = self.get_ttc();
        if self.tva > 0.0 && self.ttc > 0.0 {
            return (self.ttc - self.timbre) - self.tva;
        }
        // }
        return self.htva;
    }
    pub fn resolve(&mut self) {
        self.ttc = self.get_ttc();
        self.tva = self.get_tva();
        self.htva = self.get_htva();
    }
    pub fn get_taux(&self) -> f32 {
        self.taux
    }
}
