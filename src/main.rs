use std::fs;
use std::io::Read;

use serde::Deserialize;

mod models;
mod encoder;

use crate::models::Pay;
// Value processing:
// - datove typy sa musia byt konvertovane
//      => date = YYYYMMDD (bez čiarok)
//      => decimal number (musi byť pužite bodka ako desatinne čislo)
//      => currency code = 3 znaky
//      => country code = 3 znaky
//      => bic = ISO 9362
//      => iban = ISO 13616
// - ak obsahuje hodnota \t tak musi to byť vymenene za space char
// - ak optional atribut chyba, tak je to vnimane ako prazdny field (0 length)
// - ak je atribut "maxoccurs = unbounded" tak sa musi najprv uviesť dlžka

fn main() {
    let xml = fs::read_to_string("/Users/peter/code/bysqr/test.xml").unwrap();

    let pay: Pay = quick_xml::de::from_str(&xml).unwrap();

    let encoded = encoder::encode(&pay);

    println!("{}", encoded);
}
