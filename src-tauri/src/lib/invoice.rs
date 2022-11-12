use chrono::{Date, Utc};
use crate::lib::invoiceline::InvoiceLine;

pub struct Invoice {
    client: String,
    date: String,
    ttc: f32,
    htva: f32,
    tva: f32,
    timbre: f32,
    invoiceline:InvoiceLine
}

impl Invoice {
    fn new() -> Invoice {
        Invoice {
            client: String::from(""),
            date: chrono::offset::Local::now().to_string(),
            ttc: 0.0,
            htva: 0.0,
            tva: 0.0,
            timbre: 0.0,
            invoiceline:InvoiceLine::new()
        }
    }

}
