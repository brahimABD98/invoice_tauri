// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Invoiceline } from "./invoiceline";

export interface Invoice { client: string, date: string, htva: number, tva: number, timbre: number, number: number, taux: number, ttc: number, invoicelinelist: Array<Invoiceline>, }