//! 📚️ Puzzle3d catalog mutations — the document domain, the kind catalogs and the kind compatibility rows.

#![allow(clippy::result_large_err)]


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "."]
pub mod change_domain {
    #[path = "🌐change-domain/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌐change-domain/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🌐change-domain/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod connect_kind_compatibility {
    #[path = "🤝connect-kind-compatibility/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🤝connect-kind-compatibility/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🤝connect-kind-compatibility/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod disconnect_kind_compatibility {
    #[path = "💔disconnect-kind-compatibility/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💔disconnect-kind-compatibility/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "💔disconnect-kind-compatibility/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod replace_kind_catalogs {
    #[path = "📚replace-kind-catalogs/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📚replace-kind-catalogs/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "📚replace-kind-catalogs/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🌳️Modules
