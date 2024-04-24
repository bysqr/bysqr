use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use crate::AttributeValue::{Complex, Simple};

#[derive(Debug)]
enum AttributeValue {
    Simple(String),
    Complex(Vec<Attribute>),
}

// Tu možem dať informaciu o max occurs a podobne
#[derive(Debug)]
struct Attribute {
    depth: u32,
    name: String,
    path: String,
    value: AttributeValue,
}

fn main() {
    let file = File::open("/Users/peter/code/bysqr/payment.xml").unwrap();
    let file = BufReader::new(file);

    // Element može byť taky:
    // => Simple = obsahuje iba textovu hodnotu
    // => Complex = obsahuje elementy, alebo array elementov

    // Steps:
    // - začnem streamovať elementy a spracovavať ich.
    // - budem prechadzať element od start eventu po end event a budem
    //   manualne parsovať XML a validovať hodnoty.
    // - zaroveň urobim post-processing hodnot
    // - vysledok parsovania jedneho elementu od start po end bude 1 string

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

    let parser = EventReader::new(file);

    // - zatiaľ podporujem len Pay element

    let mut path: Vec<String> = Vec::new();

    let mut depth = 0;

    let mut attrs: Vec<Attribute> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                depth += 1;

                path.push(format!("{}", name.local_name));

                let el_path = path.join(".");

                attrs.push(Attribute {
                    depth,
                    name: name.local_name.clone(),
                    path: el_path,
                    value: Complex(Vec::new()),
                });
            }
            Ok(XmlEvent::Characters(data)) => {
                // Nastavim poslednemu elementu hodnotu.
                if let Some(mut last) = attrs.pop() {
                    last.value = Simple(data.clone());
                    attrs.push(last);
                }
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;

                if let Some(last) = attrs.pop() {
                    if let Some(mut parent) = attrs.pop() {
                        match parent.value {
                            Complex(mut items) => {
                                items.push(last);
                                parent.value = Complex(items);
                                attrs.push(parent);
                            },
                            Simple(_) => {
                                panic!("Simple element tu nema čo robiť.");
                            }
                        }
                    } else {
                        // Ak už nemam parenta tak toto je posledny element.
                        attrs.push(last);
                    }
                }

                path.pop();
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }

    dbg!("{:?}", attrs);
}
