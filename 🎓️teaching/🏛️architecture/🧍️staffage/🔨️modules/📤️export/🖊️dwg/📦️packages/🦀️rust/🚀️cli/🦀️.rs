//! 🚀️ `staffage-dwg <in.json> <out.dwg>`: ModelScene JSON in, genuine AC1024 out.

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [input, output] = arguments.as_slice() else {
        eprintln!("usage: staffage-dwg <in.json> <out.dwg>");
        std::process::exit(2);
    };
    let result = std::fs::read(input).map_err(|error| format!("{input}: {error}")).and_then(|json| staffage_dwg::encode_json(&json)).and_then(|bytes| std::fs::write(output, bytes).map_err(|error| format!("{output}: {error}")));
    if let Err(error) = result {
        eprintln!("staffage-dwg: {error}");
        std::process::exit(1);
    }
}
