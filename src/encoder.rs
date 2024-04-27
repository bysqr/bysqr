use std::io::Write;
use chrono::NaiveDate;
use crc32fast::Hasher;
use regex::Regex;
// use xz2::write::XzEncoder;
use xz2::write::XzEncoder;
use xz2::stream::{Check, Filters, LzmaOptions, Stream};

use crate::models::{BankAccount, Pay, Payment};

fn as_pattern_str(value: &Option<String>, pattern: &str, description: &str) -> String {
    if let Some(val) = value {
        if Regex::new(pattern).unwrap().is_match(val) {
            val.clone()
        } else {
            panic!("Encoding error: The {} does not match pattern {}", description, pattern);
        }
    } else {
        String::new()
    }
}

fn as_valid_date(value: &String) -> String {
    if Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap().is_match(value) {
        if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            date.format("%Y%m%d").to_string()
        } else {
            panic!("Encoding error: The date is not valid {}", value)
        }
    } else if Regex::new(r"\d{4}\d{2}\d{2}").unwrap().is_match(value) {
        if let Ok(_) = NaiveDate::parse_from_str(value, "%Y%m%d") {
            value.clone()
        } else {
            panic!("Encoding error: The date is not valid {}", value)
        }
    } else {
        panic!("Encoding error: Invalid date format {}", value)
    }
}

fn as_decimal_str(decimal: f32) -> String {
    let as_str = format!("{:.2}", decimal);

    if let Some(_) = as_str.find('.') {
        let mut truncated = as_str;

        while truncated.ends_with('0') {
            truncated.pop();
        }

        if truncated.ends_with('.') {
            truncated.pop();
        }

        truncated
    } else {
        as_str
    }
}

fn bank_account_to_seq(bank_account: &BankAccount) -> Vec<String> {
    let mut seq: Vec<String> = Vec::new();

    // IBAN = order 1
    if Regex::new(r"^[A-Z]{2}\d{2}[A-Z\d]{0,30}$").unwrap().is_match(&bank_account.iban) {
        seq.push(bank_account.iban.clone())
    } else {
        panic!("Encoding error: IBAN does not have valid format")
    }

    // BIC = order 2
    seq.push(as_pattern_str(&bank_account.bic, r"^[A-Z]{4}[A-Z]{2}[A-Z\d]{2}([A-Z\d]{3})?$", "BIC"));

    seq
}

/// Convert Payment to sequence.
fn payment_to_seq(payment: &Payment) -> Vec<String> {
    let mut seq: Vec<String> = Vec::new();

    // PaymentOptions = order 1
    if payment.payment_options == "paymentorder" {
        seq.push(String::from("1"));
    } else if payment.payment_options == "standingorder" {
        seq.push(String::from("2"));
    } else if payment.payment_options == "directdebit" {
        seq.push(String::from("4"));
    } else {
        panic!("Encoding error: Unkown PaymentOptions value {}", payment.payment_options);
    }

    // Amount = order 2
    match payment.amount {
        None => {
            seq.push(String::new());
        }
        Some(value) => {
            if ! value.is_sign_positive() {
                panic!("Encoding error: The amount must be a positive number")
            }

            seq.push(as_decimal_str(value));
        }
    }

    // Currency = order 3
    if Regex::new("[A-Z]{3}").unwrap().is_match(&payment.currency_code) {
        seq.push(payment.currency_code.clone());
    } else {
        panic!("Encoding error: The currency code is not in ISO 4217 format");
    }

    // Payment due date = order 4
    if let Some(due_date) = &payment.payment_due_date {
        seq.push(as_valid_date(due_date));
    } else {
        seq.push(String::new());
    }

    // Variable Symbol = order 5
    seq.push(as_pattern_str(&payment.variable_symbol, r"^\d{0,10}$", "variable symbol"));

    // Constant Symbol = order 6
    seq.push(as_pattern_str(&payment.constant_symbol, r"^\d{0,4}$", "constant symbol"));

    // Specific Symbol = order 7
    seq.push(as_pattern_str(&payment.specific_symbol, r"^\d{0,10}$", "specific symbol"));

    // Originators Reference Information = order 8
    seq.push(as_pattern_str(&payment.originators_reference_information, r"^.{0,35}$", "originators reference information"));

    // Payment Note = order 9
    seq.push(as_pattern_str(&payment.payment_note, r"^[\p{L}\p{N}\p{P}\p{Z}\p{M}]{1,140}$", "payment note"));

    // Bank Accounts = order 10
    seq.push(format!("{}", payment.bank_accounts.bank_account.len()));
    for bank_account in &payment.bank_accounts.bank_account {
        seq.append(&mut bank_account_to_seq(bank_account));
    }

    // TODO: 11 StandingOrderExt
    seq.push(String::from("0"));

    // TODO: 12 DirectDebitExt
    seq.push(String::from("0"));

    // Beneficiary Name = order 13
    // seq.push(as_pattern_str(&payment.beneficiary_name, r"^.{0,140}$", "beneficiary name"));

    // Beneficiary Address Line 1 = order 14
    // seq.push(as_pattern_str(&payment.beneficiary_address_line_1, r"^.{0,70}$", "beneficiary address line 1"));

    // Beneficiary Address Line 2 = order 15
    // seq.push(as_pattern_str(&payment.beneficiary_address_line_2, r"^.{0,70}$", "beneficiary address line 2"));

    seq
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let hex_string: String = bytes.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    hex_string
}

pub fn encode(pay: &Pay) -> String {
    let mut buf: Vec<String> = Vec::new();

    buf.push(format!("{}", pay.payments.payment.len()));

    for payment in &pay.payments.payment {
        let mut encoded = payment_to_seq(&payment);

        buf.append(&mut encoded);
    }

    let seq = format!("\t{}", buf.join("\t"));

    // println!("{:?}", &seq);

    let mut hasher = Hasher::new();
    hasher.update(seq.as_bytes());
    let crc = hasher.finalize();

    let mut to_compress: Vec<u8> = Vec::from(crc.to_le_bytes());
    to_compress.extend_from_slice(seq.as_bytes());

    let mut options = LzmaOptions::new_preset(6).unwrap();
    options.literal_context_bits(3);
    options.literal_position_bits(0);
    options.position_bits(2);
    options.dict_size(131072);

    // let stream = Stream::new_easy_encoder(6, Check::None).unwrap();
    // let mut filter = Filters::new();
    // filter.lzma1(&options);
    // let stream = Stream::new_stream_encoder(&filter, Check::None).unwrap();
    let stream = Stream::new_lzma_encoder(&options).unwrap();

    // let compressed: Vec<u8> = Vec::new();
    let mut compressor = XzEncoder::new_stream(Vec::new(), stream);
    compressor.write_all(&to_compress).unwrap();

    let compressed = &compressor.finish().unwrap()[13..];

    println!("{:?}", bytes_to_hex_string(&compressed).to_lowercase());

    // OUT
    // - 5d00000200ffffffffffffffff
    // 002f8f4eefe04b128d1f0fd9cc382236ee3d498816c57392aa1137aae2bb6888ac0ed7b7e319da1a0f72552b73136af1281802c0c2bed81d1b21e43957ae34865de949cce2fd5e4a0d008df0599da8e9dc1b7380fd316bffde67e000
    // 002f8f4eefe04b128d1f0fd9cc382236ee3d498816c57392aa1137aae2bb6888ac0ed7b7e319da1a0f72552b73136af1281802c0c2bed81d1b21e43957ae34865de949cce2fd5e4a0d008df0599da8e9dc1b7380fd316bffde67e000

    // println!("{:?}", bytes_to_hex_string(&compressed).to_lowercase());

    // 5f3d97fe09310931093132332e34350945555209323032343036303109313233343536373839093033303809313131310909706f7a6e616d6b61093109534b383831313030303030303030323934353130323334370954415452534b4258093009305d00000200ffffffffffffffff0083fffbffffc0000000
    // 00006200002f8f4eefe04b128d1f0fd9cc382236ee3d498816c57392aa1137aae2bb6888ac0ed7b7e319da1a0f72552b73136af1281802c0c2bed81d1b21e43957ae34865de949cce2fd5e4a0d008df0599da8e9dc1b7380fd316bffde67e000

    // compressor.write_all()

    // println!("{:?}", seq.len());

    // Header: BySquareType * Version * DocumentType * Reserved

    // Tipek
    // \t1\t1\t123.45\tEUR\t20240601\t123456789\t0308\t1111\t\tpoznamka\t1\tSK8811000000002945102347\tTATRSKBX\t0\t0
    // \t1\t1\t123.45\tEUR\t20240601\t123456789\t0308\t0308\t\tpoznamka\t1\tSK8811000000002945102347\tTATRSKBX\t0\t0

    String::from("test")
}
