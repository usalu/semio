//! 🎞️ Puzzle3d catalog mutation scenes — each committed before/mutation/diff/after quartet of the domain and kind-catalog leaves, applied through the dispatch union.

#![allow(clippy::result_large_err)]



extern crate semio_framework_os_kernel as protocol;

#[path = ".."]
mod change_domain {
    #[path = "🌐change-domain/🧪️tests/⚙️architecture/🦀️.rs"]
    mod tests_architecture_to_engineering;
}

#[path = ".."]
mod connect_kind_compatibility {
    #[path = "🤝connect-kind-compatibility/🧪️tests/🤝️adds/🦀️.rs"]
    mod tests_adds_vortex_kind_pair;
}

#[path = ".."]
mod disconnect_kind_compatibility {
    #[path = "💔disconnect-kind-compatibility/🧪️tests/🚫️removes/🦀️.rs"]
    mod tests_removes_vortex_kind_pair;
    #[path = "💔disconnect-kind-compatibility/🧪️tests/💔️withdraws-middle-row/🦀️.rs"]
    mod tests_withdraws_the_middle_row;
}

#[path = ".."]
mod replace_kind_catalogs {
    #[path = "📚replace-kind-catalogs/🧪️tests/📇️installs/🦀️.rs"]
    mod tests_installs_vortex_kind_catalog;
}
