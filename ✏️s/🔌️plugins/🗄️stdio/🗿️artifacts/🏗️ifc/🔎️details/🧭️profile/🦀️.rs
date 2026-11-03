//! 🧭️ Profile authority: declaration roles, type metadata and local-rule candidates every IFC details layer shares.

use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, RetainedWorkOutcome, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};
#[path="type_metadata.rs"]pub mod type_metadata;
pub use type_metadata::{IfcDetailsTypeMetadata,IfcTypeProjection};
#[cfg(feature="testkit")]#[path="testkit.rs"]pub mod testkit;
#[cfg(feature="testkit")]pub use testkit::DetailsTestProfile;
#[path="retained.rs"]pub mod retained;
pub use retained::{own_with,begun,costed,cursor_step,idle_step,owned_step,refused};
#[path="candidate_rules.rs"]pub mod candidate_rules;
pub use candidate_rules::{IfcCandidateCheck,IfcCandidateRules,IfcCandidateRuleStep,IfcTypeCandidate};

/// 🪪️ Borrowed declaration roles distinguish derived slots from directly owned identity slots.
#[derive(Clone,Copy)]
pub struct IfcNormalizationAttribute{pub derived:bool,pub global_id:bool,pub optional:bool,pub name:&'static str,pub type_expr:&'static dyn IfcDetailsTypeMetadata}
pub trait IfcDetailsProfile {
    type CandidateRules: IfcCandidateRules;
    const MAXIMUM_ATTRIBUTES: usize;
    fn catalog() -> &'static IfcCatalog;
    /// 🧬️ Borrow one defined alias expression from the same profile metadata authority.
    fn defined_type_expression(name:&str)->Option<&'static dyn IfcDetailsTypeMetadata>{Some(Self::catalog().types.get(name)?.type_expr.as_ref()? as &dyn IfcDetailsTypeMetadata)}
    /// 🧭️ Borrow one direct or inherited attribute's normalization roles without copying its declaration.
    fn normalization_attribute(name:&str,index:usize,direct:bool)->Result<Option<IfcNormalizationAttribute>,()>{let declaration=Self::catalog().entities.get(name).ok_or(())?;let attributes=if direct{&declaration.attributes}else{&declaration.all_attributes};Ok(attributes.get(index).map(|attribute|IfcNormalizationAttribute{derived:attribute.derived,global_id:attribute.name=="GlobalId",optional:attribute.optional,name:&attribute.name,type_expr:&attribute.type_expr}))}
    /// 📏️ Borrow the direct attribute count for a canonical released entity name.
    fn direct_attribute_count(name:&str)->Option<usize>{Some(Self::catalog().entities.get(name)?.attributes.len())}
    /// 🌱️ A rootless declaration proves canonical full and direct tuple positions coincide.
    fn rootless_attribute_count(name:&str)->Option<usize>{let declaration=Self::catalog().entities.get(name)?;declaration.supertype.is_none().then_some(declaration.all_attributes.len())}
    /// 🔤️ Borrow each released enumeration member through the same immutable profile authority.
    fn enumeration_count(name:&str)->Option<usize>{let declaration=Self::catalog().types.get(name)?;(declaration.kind=="enum").then_some(declaration.members.len())}
    fn enumeration_member(name:&str,index:usize)->Option<&'static str>{let declaration=Self::catalog().types.get(name)?;if declaration.kind!="enum"{return None;}Some(declaration.members.get(index)?.as_str())}
    /// 🔀️ Borrow the complete SELECT roster from the same immutable declaration authority.
    fn select_count(name:&str)->Option<usize>{let declaration=Self::catalog().types.get(name)?;(declaration.kind=="select").then_some(declaration.members.len())}
    fn select_member(name:&str,index:usize)->Option<&'static str>{let declaration=Self::catalog().types.get(name)?;if declaration.kind!="select"{return None;}Some(declaration.members.get(index)?.as_str())}
    fn detail_variants(_snapshot: &IfcSnapshot, _path: &[P], _locale: Locale) -> Option<Vec<SnapshotDetailVariant>> {
        None
    }
    fn entity_presentation(_entity: &IfcEntity, _locale: Locale) -> Option<SnapshotDetailPresentation> {
        None
    }
}


pub fn key(segment: &P, expected: &str) -> bool {
    matches!(segment, P::Key(value) if value == expected)
}
