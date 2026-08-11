//! ⚙️ Tsv (iana) engine — REAL and complete for the scaffold scope: IANA text/tab-separated-
//! values has no quoting/escaping, so a full split-on-tab/newline parser is genuinely correct
//! today (not partial like the other 6 formats' engines).

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TsvParsed {
    pub header: Vec<String>,
    pub records: Vec<Vec<String>>,
    pub trailing_newline: bool,
}

/// 📐️ TSV has no reserved magic; a real structural check instead: at least one line, and no CR
/// bytes (LF-only, per the IANA TSV convention this fixture follows — see its NOTES.md sidecar).
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    !bytes.is_empty() && !bytes.contains(&b'\r')
}

pub fn parse(text: &str) -> TsvParsed {
    let trailing_newline = text.ends_with('\n');
    let body = text.strip_suffix('\n').unwrap_or(text);
    let mut lines = body.split('\n');
    let header = lines.next().map(|l| l.split('\t').map(|s| s.to_string()).collect()).unwrap_or_default();
    let records = lines.map(|l| l.split('\t').map(|s| s.to_string()).collect()).collect();
    TsvParsed { header, records, trailing_newline }
}

pub fn print_tsv(parsed: &TsvParsed) -> String {
    let mut lines: Vec<String> = vec![parsed.header.join("\t")];
    for r in &parsed.records {
        lines.push(r.join("\t"));
    }
    let mut out = lines.join("\n");
    if parsed.trailing_newline { out.push('\n'); }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_real_shaped_tsv_body() {
        let text = "name\tage\nAda\t30\nGrace\t85\n";
        assert!(sniff_real_bytes(text.as_bytes()));
        let parsed = parse(text);
        assert_eq!(parsed.header, vec!["name", "age"]);
        assert_eq!(parsed.records.len(), 2);
        assert!(parsed.trailing_newline);
        assert_eq!(print_tsv(&parsed), text);
    }

    #[test]
    fn sniff_rejects_crlf_bytes() {
        assert!(!sniff_real_bytes(b"a\tb\r\n"));
    }
}

//#region 🔖️Register
/// 📌️ Registers this standard's single (✳️any) subset. Real magic-byte `sniff_real_bytes`/
/// `parse_minimal` above are used by the subset's analyzer; schema descriptor + document codec
/// registration is the subset composer's own job (see that module).
pub fn register() {
    crate::artifacts::tsv::standards::iana::subsets::any::composer::register();
}
//#endregion 🔖️Register
