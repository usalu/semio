//! ⚡️ Block2d artifact — OpText/OpBinary codecs + grammar for `Block2dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::{inverse_block2d_mutation, Block2dMutation};


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for Block2dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}


//#endregion 🔖️HandcraftedOpCodecs

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::Block2dSnapshot;
use protocol::Mutation;
use crate::standards::v1::subsets::any::schema::mutations::add_attribute::{add_attribute, AddAttribute};
use crate::standards::v1::subsets::any::schema::mutations::add_author::{add_author, AddAuthor};
use crate::standards::v1::subsets::any::schema::mutations::add_compatibility_rule::{add_compatibility_rule, AddCompatibilityRule};
use crate::standards::v1::subsets::any::schema::mutations::change_handle_handle_kind::{change_handle_handle_kind, ChangeHandleHandleKind};
use crate::standards::v1::subsets::any::schema::mutations::change_handle_kind_color::{change_handle_kind_color, ChangeHandleKindColor};
use crate::standards::v1::subsets::any::schema::mutations::change_handle_kind_default_wire_kind::{change_handle_kind_default_wire_kind, ChangeHandleKindDefaultWireKind};
use crate::standards::v1::subsets::any::schema::mutations::change_handle_kind_label::{change_handle_kind_label, ChangeHandleKindLabel};
use crate::standards::v1::subsets::any::schema::mutations::change_meta_description::{change_meta_description, ChangeMetaDescription};
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind_description::{change_node_kind_description, ChangeNodeKindDescription};
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind_icon::{change_node_kind_icon, ChangeNodeKindIcon};
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind_label::{change_node_kind_label, ChangeNodeKindLabel};
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind_unit::{change_node_kind_unit, ChangeNodeKindUnit};
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind_variant::{change_node_kind_variant, ChangeNodeKindVariant};
use crate::standards::v1::subsets::any::schema::mutations::create_handle::{create_handle, CreateHandle};
use crate::standards::v1::subsets::any::schema::mutations::create_handle_kind::{create_handle_kind, CreateHandleKind};
use crate::standards::v1::subsets::any::schema::mutations::delete_handle::{delete_handle, DeleteHandle};
use crate::standards::v1::subsets::any::schema::mutations::delete_handle_kind::{delete_handle_kind, DeleteHandleKind};
use crate::standards::v1::subsets::any::schema::mutations::move_camera2d::{move_camera2d, MoveCamera2d};
use crate::standards::v1::subsets::any::schema::mutations::move_handle::{move_handle, MoveHandle};
use crate::standards::v1::subsets::any::schema::mutations::remove_attribute::{remove_attribute, RemoveAttribute};
use crate::standards::v1::subsets::any::schema::mutations::remove_author::{remove_author, RemoveAuthor};
use crate::standards::v1::subsets::any::schema::mutations::remove_compatibility_rule::{remove_compatibility_rule, RemoveCompatibilityRule};
use crate::standards::v1::subsets::any::schema::mutations::rename_handle_kind::{rename_handle_kind, RenameHandleKind};
use crate::standards::v1::subsets::any::schema::mutations::rename_node_kind::{rename_node_kind, RenameNodeKind};
use crate::standards::v1::subsets::any::schema::mutations::scale_camera2d::{scale_camera2d, ScaleCamera2d};
use crate::standards::v1::subsets::any::schema::mutations::update_presentation::{update_presentation, UpdatePresentation};

/// 🌉️ One report for a `(base, mutation, after)` triple, in the framework's own JSON, so a test host
/// can exercise this subset's codec without linking `serde_json` itself. Mirrors the bridge every
/// other converted subset ships (`🗺️gismap`, `🏗️fem`); this subset had none, so its adapter could
/// only read committed vectors and never ran the implementation at all.
///
/// `base` is the decoded input, `snapshot` the applied document, `expectedSnapshot` the decoded
/// `after_json`, `diff` the produced delta, `messages` the diagnostics it raised, `inverseSteps` the
/// computed inverse and `inverseSnapshot` the document those steps land on.
pub fn block2d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let base: Block2dSnapshot = semio_framework_pack_json::from_json_str(base_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let expected: Block2dSnapshot = semio_framework_pack_json::from_json_str(after_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mutation: Block2dMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let forward = <Block2dMutation as Mutation<Block2dSnapshot>>::diff(&mutation, &base);
    let applied = protocol::apply_diff(forward.diff(), &base).map_err(|error| format!("{error:?}"))?;
    let inverse = <Block2dMutation as Mutation<Block2dSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Block2dMutation as Mutation<Block2dSnapshot>>::diff(step, &undone);
        undone = protocol::apply_diff(outcome.diff(), &undone).map_err(|error| format!("{error:?}"))?;
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = semio_framework_value::DslValue::object([
        ("base".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&base))),
        ("expectedSnapshot".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&expected))),
        ("snapshot".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&applied))),
        ("diff".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(forward.diff()))),
        ("messages".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&forward.messages().to_vec()))),
        ("inverseSteps".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&inverse))),
        ("inverseSnapshot".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&undone))),
        ("inverseMessages".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(semio_framework_pack_json::to_json_string(&report))
}
}
pub use mutations_codec::*;
