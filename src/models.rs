use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pay {
    /// Zoznam jednej alebo viacerých platieb v prípade hromadného príkazu. Hlavná (preferovaná) platba sa uvádza ako prvá.
    pub payments: Payments
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payments {
    /// 1+, order = 2
    pub payment: Vec<Payment>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    /// Možnosti platby sa dajú kombinovať.
    /// Oddeľujú sa medzerou a treba uviesť vždy aspoň jednu z možností.
    /// paymentorder - platobný príkaz
    /// standingorder - trvalý príkaz, údaje sa vyplnia do StandingOrderExt
    /// directdebit - inkaso, údaje sa vyplnia do DirectDebitExt
    /// req, order = 1, priority = 999
    pub payment_options: String,

    /// Čiastka platby. Povolené sú len kladné hodnoty. Desatinná čast je oddelená bodkou.
    /// Môže ostať nevyplnené, napríklad pre dobrovoľný príspevok (donations).
    /// Príklad:
    /// Tisíc sa uvádza ako "1000".
    /// Jedna celá deväťdesiatdeväť sa uvádza ako "1.99".
    /// Desať celých peťdesiat sa uvádza ako "10.5".
    /// Nula celá nula osem sa uvádza ako "0.08".
    /// opt, order = 2, priority = 999
    pub amount: Option<f32>,

    /// Mena platby v ISO 4217 formáte (3 písmená skratka). Príklad: "EUR".
    /// req, order = 3, priority = 999, 3 pismenka ISO, pattern [A-Z]{3}
    pub currency_code: String,

    /// Dátum splatnosti vo formáte ISO 8601 "RRRR-MM-DD". Nepovinný údaj.
    /// V prípade trvalého príkazu označuje dátum prvej platby.
    /// opt, order = 4, priority = 999, datum format
    pub payment_due_date: Option<String>,

    /// Variabilný symbol je maximálne 10 miestne číslo. Nepovinný údaj.
    /// opt, order = 5, priority = 7, max len 10, pattern: [0-9]{0,10}
    pub variable_symbol: Option<String>,

    /// Konštantný symbol je 4 miestne identifikačné číslo. Nepovinný údaj.
    /// opt, order = 6, priority = 5, max len 4, pattern: [0-9]{0,4}
    pub constant_symbol: Option<String>,

    /// Špecifický symbol je maximálne 10 miestne číslo. Nepovinný údaj.
    /// opt, order = 7, priority = 6, max len 10, pattern: [0-9]{0,10}
    pub specific_symbol: Option<String>,

    /// Referenčná informácia prijímateľa podľa SEPA.
    /// opt, order = 8, priority = 12, max len 35
    pub originators_reference_information: Option<String>,

    /// Správa pre prijímateľa.
    /// Údaje o platbe, na základe ktorých príjemca bude môcť platbu identifikovať.
    /// Odporúča sa maximálne 140 Unicode znakov.
    /// opt, order = 9, priority = 1, max len 140, unicode
    pub payment_note: Option<String>,

    /// Zoznam bankových účtov.
    /// req 1+, order = 10
    pub bank_accounts: BankAccounts,

    /// TODO: StandingOrderExt
    /// TODO: DirectDebitExt

    /// Rozšírenie o meno príjemcu
    /// opt, order = 13, priority 999, max length 140
    pub beneficiary_name: Option<String>,

    /// Rozšírenie o adresu príjemcu
    /// opt, order = 14, priority 999, max length 70
    pub beneficiary_address_line_1: Option<String>,

    /// Rozšírenie o adresu príjemcu (druhý riadok)
    /// opt, order = 14, priority 999, max length 70
    pub beneficiary_address_line_2: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
// Údaje bankového účtu prijímateľa platby.
pub struct BankAccount {
    /// Medzinárodné číslo bankového účtu vo formáte IBAN.
    /// Príklad: "SK8209000000000011424060".
    /// Viac na http://www.sbaonline.sk/sk/projekty/financne-vzdelavanie/slovnik-bankovych-pojmov/iii/.
    /// req, order = 1, priority = 999, pattern: [A-Z]{2}[0-9]{2}[A-Z0-9]{0,30}, max length 34
    #[serde(rename = "IBAN")]
    pub iban: String,

    /// Medzinárodný bankový identifikačný kód (z ang. Bank Identification Code).
    /// Viac na http://www.sbaonline.sk/sk/projekty/financne-vzdelavanie/slovnik-bankovych-pojmov/bbb/bic.html.
    /// opt, order = 2, priority = 999
    #[serde(rename = "BIC")]
    pub bic: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BankAccounts {
    pub bank_account: Vec<BankAccount>
}
