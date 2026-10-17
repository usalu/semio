//! 🧩️ The PDF 1.4 standard's composer registry: the base subset's composer and the `a` and `x` subsets' composers in one
//! table. It composes all subsets of the standard, so it lives at the standard level above them.

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1_4::subsets::a::io::PdfAComposer;
    use crate::standards::v1_4::subsets::base::io::PdfComposer as PdfRawAnyComposer;
    use crate::standards::v1_4::subsets::x::io::PdfXComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PdfRawAnyComposer>(), composer_entry_of::<PdfAComposer>(), composer_entry_of::<PdfXComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
