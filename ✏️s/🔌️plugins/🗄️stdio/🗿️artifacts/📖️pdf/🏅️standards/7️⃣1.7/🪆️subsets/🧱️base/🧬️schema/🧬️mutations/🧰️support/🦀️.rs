//! 🧰️ `PdfMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn refused(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn position(segment: &str, length: usize, insert: bool) -> Result<usize, Fault> {
    if insert && segment == "-" {
        return Ok(length);
    }
    match segment.parse::<usize>() {
        Ok(index) if index < length || (insert && index == length) => Ok(index),
        _ => Err(refused("pdf-edit.invalid-index", format!("'{segment}' addresses no position of a list of {length}"))),
    }
}
