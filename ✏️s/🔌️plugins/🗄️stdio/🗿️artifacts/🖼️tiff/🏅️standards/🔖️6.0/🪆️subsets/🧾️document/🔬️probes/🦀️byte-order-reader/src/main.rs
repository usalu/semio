//! 🔡️ Projects a TIFF's declared BYTE ORDER, alongside enough decoded content to prove the pair
//! differs in nothing else.
//!
//! `tiff` 0.11's `Decoder::byte_order()` is public and documented as "only relevant to interpreting
//! raw bytes read from tags", because "the image decoding methods will correct to the host byte order
//! automatically" — which is exactly why a pixel-level projection cannot see this mutation and this
//! one can.
//!
//! The pixel CHECKSUM is projected deliberately: a `change-byte-order` pair must differ in the order
//! and in nothing else, and a fixture that also changed the image would pass a bare order comparison.
//!
//! usage: reader project <file.tif> | reader compare <expected.tif> <actual.tif>

use std::process::exit;

fn project(path: &str) -> Result<String, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut decoder = tiff::decoder::Decoder::new(file).map_err(|e| e.to_string())?;
    let order = format!("{:?}", decoder.byte_order());
    let (width, height) = decoder.dimensions().map_err(|e| e.to_string())?;
    let colortype = format!("{:?}", decoder.colortype().map_err(|e| e.to_string())?);
    let image = decoder.read_image().map_err(|e| e.to_string())?;
    let (count, checksum) = match image {
        tiff::decoder::DecodingResult::U8(v) => (v.len(), v.iter().map(|b| u64::from(*b)).sum::<u64>()),
        tiff::decoder::DecodingResult::U16(v) => (v.len(), v.iter().map(|b| u64::from(*b)).sum::<u64>()),
        other => (0, format!("{other:?}").len() as u64),
    };
    Ok(format!(
        "{{\"byteOrder\":\"{order}\",\"width\":{width},\"height\":{height},\"colorType\":\"{colortype}\",\"sampleCount\":{count},\"pixelChecksum\":{checksum}}}"
    ))
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

fn report(probe: &str, status: &str, measurements: &str, diagnostic: Option<&str>) -> String {
    let diagnostics = match diagnostic {
        Some(message) => format!(",\"diagnostics\":[{{\"severity\":\"error\",\"message\":\"{}\"}}]", escape(message)),
        None => String::new(),
    };
    format!(
        "{{\"schema\":\"semio.repository-test.probe-report/v2\",\"probe\":\"{probe}\",\"probeVersion\":\"tiff@0.11.3\",\"engine\":{{\"family\":\"tiff\",\"implementation\":\"tiff 0.11 Decoder::byte_order\",\"version\":\"0.11.3\"}},\"status\":\"{status}\",\"durationMs\":0,\"measurements\":{measurements}{diagnostics}}}"
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("project") => match project(args.get(2).expect("usage: reader project <file.tif>")) {
            Ok(json) => println!("{}", report("tiff-byte-order-project", "ok", &json, None)),
            Err(error) => {
                println!("{}", report("tiff-byte-order-project", "failed", "{}", Some(&error)));
                exit(1);
            }
        },
        Some("compare") => {
            let expected = args.get(2).expect("usage: reader compare <expected> <actual>");
            let actual = args.get(3).expect("usage: reader compare <expected> <actual>");
            match (project(expected), project(actual)) {
                (Ok(left), Ok(right)) => {
                    let equal = left == right;
                    println!("{}", report("tiff-byte-order-compare", "ok", &format!("{{\"equal\":{equal},\"expected\":{left},\"actual\":{right}}}"), None));
                }
                (Err(error), _) | (_, Err(error)) => {
                    println!("{}", report("tiff-byte-order-compare", "failed", "{}", Some(&error)));
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: reader project <file.tif> | reader compare <expected.tif> <actual.tif>");
            exit(2);
        }
    }
}
