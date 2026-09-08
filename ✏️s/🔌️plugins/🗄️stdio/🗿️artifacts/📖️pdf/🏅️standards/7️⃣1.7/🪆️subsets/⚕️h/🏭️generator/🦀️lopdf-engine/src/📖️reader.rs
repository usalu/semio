//! 🔍️ The READER. Projects a committed PDF through `lopdf`'s own public API and compares two of them.
//! Nothing here applies a mutation or predicts what one should produce.
//!
//! usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>

use std::env;
use std::fs;
use std::process::exit;

use pdf_h_lopdf::project;

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

fn report(probe: &str, status: &str, measurements: &str, diagnostic: Option<&str>) -> String {
    let diagnostics = match diagnostic {
        Some(message) => format!(",\"diagnostics\":[{{\"severity\":\"error\",\"message\":\"{}\"}}]", escape(message)),
        None => String::new(),
    };
    format!(
        "{{\"schema\":\"semio.repository-test.probe-report/v2\",\"probe\":\"{probe}\",\"probeVersion\":\"lopdf@0.44\",\"engine\":{{\"family\":\"lopdf\",\"implementation\":\"lopdf 0.44 COS object graph\",\"version\":\"0.44\"}},\"status\":\"{status}\",\"durationMs\":0,\"measurements\":{measurements}{diagnostics}}}"
    )
}

fn read(path: &str) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    project(&bytes)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("project") => match read(args.get(2).expect("usage: reader project <file.pdf>")) {
            Ok(json) => println!("{}", report("pdf-h-project", "ok", &json, None)),
            Err(error) => {
                println!("{}", report("pdf-h-project", "failed", "{}", Some(&error)));
                exit(1);
            }
        },
        Some("compare") => {
            let expected = args.get(2).expect("usage: reader compare <expected> <actual>");
            let actual = args.get(3).expect("usage: reader compare <expected> <actual>");
            match (read(expected), read(actual)) {
                (Ok(left), Ok(right)) => {
                    let equal = left == right;
                    println!("{}", report("pdf-h-compare", "ok", &format!("{{\"equal\":{equal},\"expected\":{left},\"actual\":{right}}}"), None));
                }
                (Err(error), _) | (_, Err(error)) => {
                    println!("{}", report("pdf-h-compare", "failed", "{}", Some(&error)));
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");
            exit(2);
        }
    }
}
