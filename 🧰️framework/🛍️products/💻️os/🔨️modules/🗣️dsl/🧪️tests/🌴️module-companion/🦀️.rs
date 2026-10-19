//! 🌴️ Companion tests of os-dsl: the unit tests of the family kits, the grammar, the registry and the receiving cursors, each hosted by a module that re-exports the module under test.
#![cfg(test)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;

use semio_framework_diagnostic::TextSpan;
use semio_framework_dsl::*;
use semio_framework_dsl_record::*;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};

pub use semio_framework_os_dsl::*;

//#region 🧪️ModuleTests
#[path = "."]
mod family_catalog_tests {
    pub use semio_framework_os_dsl::family::catalog::*;
    use semio_framework_diagnostic::Limits;
    use semio_framework_diagnostic::TextError;
    use semio_framework_diagnostic::TextSpan;
    use semio_framework_dsl::TokenKind;
    use semio_framework_dsl::lex;
    use semio_framework_value::ValueRefusalKind;
    #[path = "../../👪️family/🗂️catalog/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod family_embed_tests {
    pub use semio_framework_os_dsl::family::embed::*;
    #[path = "../../👪️family/📎️embed/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod family_geo_tests {
    pub use semio_framework_os_dsl::family::geo::*;
    use semio_framework_diagnostic::Limits;
    use semio_framework_diagnostic::TextError;
    use semio_framework_dsl::TokenKind;
    use semio_framework_dsl::lex;
    use semio_framework_value::ValueRefusalKind;
    #[path = "../../👪️family/🌍️geo/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod family_graph_tests {
    pub use semio_framework_os_dsl::family::graph::*;
    use semio_framework_diagnostic::Limits;
    use semio_framework_diagnostic::TextError;
    use semio_framework_dsl::TokenKind;
    use semio_framework_dsl::lex;
    use semio_framework_dsl_record::notation::{EdgeLabel, EdgeLink, EdgeNode, EdgeValue};
    use semio_framework_value::ValueRefusalKind;
    #[path = "../../👪️family/🕸️graph/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod family_recipe_tests {
    pub use semio_framework_os_dsl::family::recipe::*;
    use semio_framework_diagnostic::Limits;
    use semio_framework_diagnostic::TextError;
    use semio_framework_diagnostic::TextSpan;
    use semio_framework_dsl::TokenKind;
    use semio_framework_dsl::lex;
    use semio_framework_value::ValueRefusalKind;
    #[path = "../../👪️family/🧑‍🍳recipe/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod family_scene_tests {
    pub use semio_framework_os_dsl::family::scene::*;
    use semio_framework_diagnostic::Limits;
    use semio_framework_diagnostic::TextError;
    use semio_framework_dsl::TokenKind;
    use semio_framework_dsl::lex;
    use semio_framework_value::ValueRefusalKind;
    #[path = "../../👪️family/🎬️scene/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod family_sheet_tests {
    pub use semio_framework_os_dsl::family::sheet::*;
    use semio_framework_diagnostic::Limits;
    use semio_framework_diagnostic::TextError;
    use semio_framework_diagnostic::TextSpan;
    use semio_framework_dsl::TokenKind;
    use semio_framework_dsl::lex;
    use semio_framework_dsl_record::ExprOp;
    use semio_framework_dsl_record::ExprValue;
    use semio_framework_dsl_record::parse_expr_text;
    use semio_framework_dsl_record::print_expr;
    use semio_framework_value::ValueRefusalKind;
    use std::collections::HashMap;
    #[path = "../../👪️family/📊️sheet/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod grammar_tests {
    pub use semio_framework_os_dsl::grammar::*;
    use semio_framework_diagnostic::TextError;
    use semio_framework_dsl::LexOptions;
    use semio_framework_dsl::grammar::*;
    #[path = "../../📖️grammar/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
    #[path = "../../📖️grammar/🧪️tests/🧬️originals/🦀️.rs"]
    mod original_shipped_grammar_tests;
}

#[cfg(not(target_arch = "wasm32"))]
#[path = "."]
mod registry_tests {
    pub use semio_framework_os_dsl::registry::*;
    use semio_framework_os_pack_tools::SchemaResolver;
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    #[path = "../../📇️registry/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod receiving_tests {
    pub use semio_framework_os_dsl::original_receiving::*;
    use semio_framework_dsl_record::{FieldValue,receiving::{DslOwnedReceivingVariants,OwnedFieldReceiving,OwnedSlot}};
    use semio_framework_os_pack::record::RetainedRecordReceiving;
    use semio_framework_value::{RetainedCloneStep, RetirementDemand, ValueRefusalKind, retirement::{RetireOwned, RetirementCursor, RetirementStep}};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use std::mem::{ManuallyDrop,size_of};
    #[path = "../../🫴️receiving/🧪️tests/🦀️.rs"]
    mod tests;
}
//#endregion 🧪️ModuleTests
