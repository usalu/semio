//! 🧭️ IFC4x3 profile for shared catalog-aware details controls.
use crate::catalog::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_ui_locale::Locale;
use semio_s_artifact_stdio_contract::editing::{SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailVariant};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcSnapshot, IfcValue};
#[path = "../../../../../🔎️details/🦀️.rs"]
mod shared;
pub type Ifc4x3DetailsProvider<'a> = shared::IfcDetailsProvider<'a, Ifc4x3DetailsProfile>;
pub struct Ifc4x3DetailsProfile;
impl shared::IfcDetailsTypeMetadata for crate::validation::metadata::StaticTypeExpression {
    fn projection(&'static self)->shared::IfcTypeProjection { shared::IfcTypeProjection{kind:self.kind,name:self.name,width:self.width,fixed:self.fixed,aggregate:self.aggregate,lower:self.lower,upper:self.upper,lower_expression:self.lower_expression,upper_expression:self.upper_expression,unique:self.unique,optional:self.optional,item:self.item.map(|item|item as &dyn shared::IfcDetailsTypeMetadata)} }
}
pub use shared::{IfcPublication, SnapshotSource};
pub use shared::IfcDetailsRenderWork;
pub use shared::{IfcTypeInputSourceWork,IfcPreparedTypeInputSource};
pub use shared::{IfcValueReceiveMode, IfcValueReceiver};
pub use shared::{IfcCandidateRules,IfcCandidateRuleStep,IfcTypeCandidate};
impl shared::IfcDetailsProfile for Ifc4x3DetailsProfile {
    type CandidateRules = crate::validation::local::NativeCandidateRules;
    const MAXIMUM_ATTRIBUTES: usize = crate::catalog::IFC4X3_MAXIMUM_ATTRIBUTES;
    fn catalog() -> &'static IfcCatalog {
        crate::catalog::ifc4x3_catalog()
    }
    fn normalization_attribute(name:&str,index:usize,direct:bool)->Result<Option<shared::IfcNormalizationAttribute>,()>{let declaration=crate::validation::metadata::entity(name).ok_or(())?;let attributes=if direct{declaration.attributes}else{declaration.all_attributes};Ok(attributes.get(index).map(|attribute|shared::IfcNormalizationAttribute{derived:attribute.derived,global_id:attribute.name=="GlobalId",optional:attribute.optional,name:attribute.name,type_expr:&attribute.type_expr}))}
    fn direct_attribute_count(name:&str)->Option<usize>{Some(crate::validation::metadata::entity(name)?.attributes.len())}
    fn rootless_attribute_count(name:&str)->Option<usize>{let declaration=crate::validation::metadata::entity(name)?;declaration.supertype.is_none().then_some(declaration.all_attributes.len())}
    fn enumeration_count(name:&str)->Option<usize>{let declaration=crate::validation::metadata::defined_type(name)?;(declaration.kind=="enum").then_some(declaration.members.len())}
    fn enumeration_member(name:&str,index:usize)->Option<&'static str>{let declaration=crate::validation::metadata::defined_type(name)?;if declaration.kind!="enum"{return None;}declaration.members.get(index).copied()}
    fn select_count(name:&str)->Option<usize>{let declaration=crate::validation::metadata::defined_type(name)?;(declaration.kind=="select").then_some(declaration.members.len())}
    fn select_member(name:&str,index:usize)->Option<&'static str>{let declaration=crate::validation::metadata::defined_type(name)?;if declaration.kind!="select"{return None;}declaration.members.get(index).copied()}
    fn defined_type_expression(name:&str)->Option<&'static dyn shared::IfcDetailsTypeMetadata>{Some(crate::validation::metadata::defined_type(name)?.type_expr? as &dyn shared::IfcDetailsTypeMetadata)}
    fn detail_variants(snapshot: &IfcSnapshot, path: &[P], locale: Locale) -> Option<Vec<SnapshotDetailVariant>> {
        let [P::Key(header), P::Key(field), P::Index(0), P::Key(value), P::Index(index)] = path else { return None };
        if header != "header" || field != "fileDescription" || value != "value" {
            return None;
        }
        let IfcValue::Aggregate(items) = snapshot.header.file_description.first()? else { return None };
        let IfcValue::String(text) = items.get(*index)? else { return None };
        if !text.is_empty() && !text.starts_with("ViewDefinition[") && !text.starts_with("ViewDefinition [") {
            return None;
        }
        use crate::mvd::{IfcMvdLocale, IfcMvdView, ifc4x3_mvd_header_text, ifc4x3_mvd_label};
        let language = match locale {
            Locale::En => IfcMvdLocale::En,
            Locale::De => IfcMvdLocale::De,
        };
        let suffix = match locale {
            Locale::En => "declared view",
            Locale::De => "deklarierte Ansicht",
        };
        Some(
            [IfcMvdView::Reference, IfcMvdView::Alignment]
                .into_iter()
                .map(|view| SnapshotDetailVariant { label: format!("{} ({suffix})", ifc4x3_mvd_label(view, language)), value: semio_framework_value::ToValue::to_value(&IfcValue::String(ifc4x3_mvd_header_text(view).into())) })
                .collect(),
        )
    }
    fn entity_presentation(entity: &IfcEntity, locale: Locale) -> Option<SnapshotDetailPresentation> {
        use crate::mvd::{IfcMvdLocale, IfcMvdScopeState, IfcMvdView, ifc4x3_mvd_entity_scope, ifc4x3_mvd_label};
        let language = match locale {
            Locale::En => IfcMvdLocale::En,
            Locale::De => IfcMvdLocale::De,
        };
        let scopes = [IfcMvdView::Reference, IfcMvdView::Alignment].map(|view| {
            let state = ifc4x3_mvd_entity_scope(&entity.name, view).state;
            let label = match (locale, state) {
                (Locale::En, IfcMvdScopeState::Universal) => "included as product or type",
                (Locale::De, IfcMvdScopeState::Universal) => "als Produkt oder Typ enthalten",
                (Locale::En, IfcMvdScopeState::Included) => "included",
                (Locale::De, IfcMvdScopeState::Included) => "enthalten",
                (Locale::En, IfcMvdScopeState::Unlisted) => "unlisted",
                (Locale::De, IfcMvdScopeState::Unlisted) => "nicht aufgeführt",
                (Locale::En, IfcMvdScopeState::Unknown) => "unknown entity type",
                (Locale::De, IfcMvdScopeState::Unknown) => "unbekannter Entitätstyp",
            };
            format!("{}: {label}", ifc4x3_mvd_label(view, language))
        });
        let status = match locale {
            Locale::En => "Published scope. Conformance: not evaluated.",
            Locale::De => "Veröffentlichter Umfang. Konformität: nicht geprüft.",
        };
        return Some(SnapshotDetailPresentation { label: format!("#{} {}", entity.id, entity.name), description: Some(format!("{} · {}. {status}", scopes[0], scopes[1])) });
    }
}

pub fn details_edit_is_admitted(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot) -> bool {
    shared::details_edit_is_admitted::<Ifc4x3DetailsProfile>(event, snapshot)
}
pub fn details_edit_special(
    event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,
    snapshot: &semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot,
) -> Result<Option<Vec<semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation>>, semio_framework_plugin::Fault> {
    shared::details_edit_special::<Ifc4x3DetailsProfile>(event, snapshot)
}
#[cfg(test)]
pub(crate) use shared::{encode_global_id, fresh_global_id, next_entity_id};
#[cfg(test)]
pub(crate) use shared::IfcDetailsWork;

pub fn details_work_capacity() -> semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity {
    shared::details_work_capacity::<Ifc4x3DetailsProfile>()
}
pub fn details_edit_retained_path_is_admitted(event:&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,snapshot:&IfcSnapshot)->bool {shared::details_edit_retained_path_is_admitted::<Ifc4x3DetailsProfile>(event,snapshot)}
pub fn details_edit_work<E>(tool: &'static str) -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<E>>>
where
    E: semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor<
            Snapshot = semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot,
            Mutation = semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation,
        >,
{
    Box::new(shared::IfcDetailsWork::<E, Ifc4x3DetailsProfile>::new(tool))
}
pub fn details_edit_special_controlled(
    event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,
    snapshot: &semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot,
    tick: &mut dyn FnMut() -> Result<(), String>,
) -> Result<Option<Vec<semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation>>, semio_framework_plugin::Fault> {
    shared::details_edit_special_controlled::<Ifc4x3DetailsProfile>(event, snapshot, tick)
}

#[cfg(test)]
type DetailsTestEditor = crate::editor::Ifc4x3AnyEditor;
#[cfg(test)]
type DetailsTestProfile = Ifc4x3DetailsProfile;
#[cfg(test)]
fn details_test_command(event: semio_s_artifact_stdio_contract::editing::SnapshotEditEvent) -> crate::editor::Ifc4x3EditCommand {
    crate::editor::Ifc4x3EditCommand::EditSnapshot { event }
}
