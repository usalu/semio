//! ⚙️ Html (5, WHATWG) engine — 🚧 scaffolded by W1b: real `<!DOCTYPE html>` detection (case-
//! insensitive, leading-whitespace-tolerant — genuinely inspects the bytes, not a fixed offset
//! check). The full tokenizer/node tree (Element/Text/Comment/RawText, void-element set) lands
//! in W3.

pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes);
    text.trim_start().to_ascii_lowercase().starts_with("<!doctype html")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniffs_a_real_doctype_case_insensitively() {
        assert!(sniff_real_bytes(b"<!DOCTYPE html>\n<html></html>"));
        assert!(sniff_real_bytes(b"  \n<!doctype HTML>"));
    }

    #[test]
    fn rejects_non_html() {
        assert!(!sniff_real_bytes(b"just some text"));
    }
}

//#region 🔖️Register
/// 📌️ Registers this standard's single (✳️any) subset. Real magic-byte `sniff_real_bytes`/
/// `parse_minimal` above are used by the subset's analyzer; schema descriptor + document codec
/// registration is the subset composer's own job (see that module).
pub fn register() {
    crate::artifacts::html::standards::v5::subsets::any::io::register();
}
//#endregion 🔖️Register
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::html::standards::v5::subsets::any::schema::HtmlComposer as HtmlRawAnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<HtmlRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
