//! 🔍️ Projects a GIF87a through `gif` 0.13's own public API and compares two of them.
//!
//! Projects the VERSION first and foremost. This subset's previous fixture declared `GIF87a` while
//! carrying a Graphic Control Extension — an 89a-only block — because its generator patched the
//! signature onto `gif::Encoder` output that emits a GCE unconditionally. A reader that does not report
//! the version cannot catch that, so this one does, and the gate compares it like any other field.
//!
//! Nothing here applies a mutation or predicts what one should produce.
//!
//! usage: reader project <file.gif> | reader compare <expected.gif> <actual.gif>

use std::fmt::Write as _;
use std::process::exit;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// 📖️ Asks `gif`'s own header parser which version the file declares.
fn version_of(path: &str) -> Result<&'static str, String> {
    use gif::streaming_decoder::StreamingDecoder;
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let mut decoder = StreamingDecoder::new();
    let mut sink = gif::streaming_decoder::OutputBuffer::None;
    let mut offset = 0usize;
    // 🔁️The header is the first thing the state machine consumes; a handful of updates settles it, and
    // the loop is bounded so a malformed file cannot spin here.
    while offset < bytes.len().min(64) {
        let (consumed, _) = decoder.update(&bytes[offset..], &mut sink).map_err(|e| e.to_string())?;
        if consumed == 0 {
            break;
        }
        offset += consumed;
    }
    Ok(match decoder.version() {
        gif::Version::V87a => "GIF87a",
        gif::Version::V89a => "GIF89a",
    })
}

fn project(path: &str) -> Result<String, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = options.read_info(file).map_err(|e| e.to_string())?;

    // 📖️The VERSION comes from `gif`'s own header parser, not from us reading six bytes. The high-level
    // `Decoder` does not expose it, so this uses the crate's documented low-level entry point
    // (`gif::streaming_decoder::StreamingDecoder`, re-exported for exactly this kind of question) and
    // drives it far enough to have parsed the header. Still the library's reading, not ours.
    let version = version_of(path)?;
    let width = decoder.width();
    let height = decoder.height();
    let background = decoder.bg_color().map(|b| b.to_string()).unwrap_or_else(|| "null".to_string());
    let palette = decoder.global_palette().map(hex).unwrap_or_default();

    let mut frames = String::new();
    let mut count = 0usize;
    loop {
        let info = match decoder.next_frame_info().map_err(|e| e.to_string())? {
            Some(info) => info.clone(),
            None => break,
        };
        let mut buffer = vec![0u8; decoder.buffer_size()];
        decoder.read_into_buffer(&mut buffer).map_err(|e| e.to_string())?;
        if count > 0 {
            frames.push(',');
        }
        let _ = write!(
            frames,
            "{{\"left\":{},\"top\":{},\"width\":{},\"height\":{},\"interlaced\":{},\"localPalette\":{},\"indicesHex\":\"{}\"}}",
            info.left,
            info.top,
            info.width,
            info.height,
            info.interlaced,
            info.palette.is_some(),
            hex(&buffer)
        );
        count += 1;
    }

    Ok(format!(
        "{{\"version\":\"{version}\",\"width\":{width},\"height\":{height},\"backgroundColorIndex\":{background},\"globalPaletteHex\":\"{palette}\",\"frameCount\":{count},\"frames\":[{frames}]}}"
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
        "{{\"schema\":\"semio.repository-test.probe-report/v2\",\"probe\":\"{probe}\",\"probeVersion\":\"gif@0.13.3\",\"engine\":{{\"family\":\"gif\",\"implementation\":\"gif 0.13 Decoder (public API only)\",\"version\":\"0.13.3\"}},\"status\":\"{status}\",\"durationMs\":0,\"measurements\":{measurements}{diagnostics}}}"
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("project") => match project(args.get(2).expect("usage: reader project <file.gif>")) {
            Ok(json) => println!("{}", report("gif-87a-project", "ok", &json, None)),
            Err(error) => {
                println!("{}", report("gif-87a-project", "failed", "{}", Some(&error)));
                exit(1);
            }
        },
        Some("compare") => {
            let expected = args.get(2).expect("usage: reader compare <expected> <actual>");
            let actual = args.get(3).expect("usage: reader compare <expected> <actual>");
            match (project(expected), project(actual)) {
                (Ok(left), Ok(right)) => {
                    let equal = left == right;
                    println!("{}", report("gif-87a-compare", "ok", &format!("{{\"equal\":{equal},\"expected\":{left},\"actual\":{right}}}"), None));
                }
                (Err(error), _) | (_, Err(error)) => {
                    println!("{}", report("gif-87a-compare", "failed", "{}", Some(&error)));
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
