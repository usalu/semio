//! 🧩️ The PDF 1.7 standard's composer registry: the base subset's composer and every conformance subset's
//! (`a`/`x`/`e`/`ua`/`vt`/`h`), owned rows in one table. It composes all subsets of the standard, so it lives at
//! the standard level above them.

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1_7::subsets::a::io::PdfAComposer;
    use crate::standards::v1_7::subsets::base::PdfComposer as PdfRawAnyComposer;
    use crate::standards::v1_7::subsets::e::io::PdfEComposer;
    use crate::standards::v1_7::subsets::h::io::PdfHComposer;
    use crate::standards::v1_7::subsets::ua::io::PdfUaComposer;
    use crate::standards::v1_7::subsets::vt::io::PdfVtComposer;
    use crate::standards::v1_7::subsets::x::io::PdfXComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<PdfRawAnyComposer>(),
                    composer_entry_of::<PdfAComposer>(),
                    composer_entry_of::<PdfXComposer>(),
                    composer_entry_of::<PdfEComposer>(),
                    composer_entry_of::<PdfUaComposer>(),
                    composer_entry_of::<PdfVtComposer>(),
                    composer_entry_of::<PdfHComposer>(),
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
