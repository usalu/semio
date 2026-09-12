//! 🏷️ Projects a GIF's EXTENSION BLOCKS through `gif`'s own low-level decoder and compares two files.
//!
//! usage: reader project <file.gif> | reader compare <expected.gif> <actual.gif>

use std::fmt::Write as _;
use std::process::exit;

use gif::streaming_decoder::{Decoded, OutputBuffer, StreamingDecoder};

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// 🏷️ Every extension block, by LABEL and payload, in the order the file declares them.
///
/// The payload is accumulated across sub-blocks: `last_ext()` reports the extension's data so far and
/// whether this was the final sub-block, so an extension longer than 255 bytes is reported whole
/// rather than as fragments.
fn project(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let mut decoder = StreamingDecoder::new();
    let mut sink = OutputBuffer::None;
    let mut offset = 0usize;
    let mut extensions: Vec<(u8, Vec<u8>)> = Vec::new();

    while offset < bytes.len() {
        let (consumed, decoded) = decoder.update(&bytes[offset..], &mut sink).map_err(|e| e.to_string())?;
        if consumed == 0 {
            break;
        }
        offset += consumed;
        if let Decoded::SubBlockFinished(label) | Decoded::BlockFinished(label) = decoded {
            let (id, data, _end) = decoder.last_ext();
            let _ = label;
            match extensions.last_mut() {
                Some((existing, payload)) if *existing == id.0 => {
                    payload.clear();
                    payload.extend_from_slice(data);
                }
                _ => extensions.push((id.0, data.to_vec())),
            }
        }
    }

    let mut out = String::from("{\"extensions\":[");
    for (index, (label, payload)) in extensions.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let text = String::from_utf8_lossy(payload).to_string();
        let printable = text.chars().all(|c| !c.is_control() || c == '\n');
        let _ = write!(
            out,
            "{{\"label\":\"0x{label:02x}\",\"bytes\":{},\"payloadHex\":\"{}\"{}}}",
            payload.len(),
            hex(payload),
            if printable { format!(",\"text\":\"{}\"", escape(&text)) } else { String::new() }
        );
    }
    let _ = write!(out, "],\"extensionCount\":{}}}", extensions.len());
    Ok(out)
}

fn report(probe: &str, status: &str, measurements: &str, diagnostic: Option<&str>) -> String {
    let diagnostics = match diagnostic {
        Some(message) => format!(",\"diagnostics\":[{{\"severity\":\"error\",\"message\":\"{}\"}}]", escape(message)),
        None => String::new(),
    };
    format!(
        "{{\"schema\":\"semio.repository-test.probe-report/v2\",\"probe\":\"{probe}\",\"probeVersion\":\"gif@0.13.3\",\"engine\":{{\"family\":\"gif\",\"implementation\":\"gif 0.13 StreamingDecoder (extension blocks)\",\"version\":\"0.13.3\"}},\"status\":\"{status}\",\"durationMs\":0,\"measurements\":{measurements}{diagnostics}}}"
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("project") => match project(args.get(2).expect("usage: reader project <file.gif>")) {
            Ok(json) => println!("{}", report("gif-extension-project", "ok", &json, None)),
            Err(error) => {
                println!("{}", report("gif-extension-project", "failed", "{}", Some(&error)));
                exit(1);
            }
        },
        Some("compare") => {
            let expected = args.get(2).expect("usage: reader compare <expected> <actual>");
            let actual = args.get(3).expect("usage: reader compare <expected> <actual>");
            match (project(expected), project(actual)) {
                (Ok(left), Ok(right)) => {
                    let equal = left == right;
                    println!("{}", report("gif-extension-compare", "ok", &format!("{{\"equal\":{equal},\"expected\":{left},\"actual\":{right}}}"), None));
                }
                (Err(error), _) | (_, Err(error)) => {
                    println!("{}", report("gif-extension-compare", "failed", "{}", Some(&error)));
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: reader project <file.gif> | reader compare <expected.gif> <actual.gif>");
            exit(2);
        }
    }
}
