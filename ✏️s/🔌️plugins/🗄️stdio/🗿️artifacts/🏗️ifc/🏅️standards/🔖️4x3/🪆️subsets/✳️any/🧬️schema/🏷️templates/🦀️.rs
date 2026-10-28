//! 🏷️ Immutable released IFC property and quantity metadata.
#[path = "✍️apply/🦀️.rs"]
pub mod application;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcTemplateEntryKind {
    SingleValue,
    ReferenceValue,
    EnumeratedValue,
    ListValue,
    BoundedValue,
    TableValue,
    Weight,
    Length,
    Area,
    Volume,
    Count,
    Time,
}
impl IfcTemplateEntryKind {
    pub const fn source_key(self) -> &'static str {
        match self {
            Self::SingleValue => "P_SINGLEVALUE",
            Self::ReferenceValue => "P_REFERENCEVALUE",
            Self::EnumeratedValue => "P_ENUMERATEDVALUE",
            Self::ListValue => "P_LISTVALUE",
            Self::BoundedValue => "P_BOUNDEDVALUE",
            Self::TableValue => "P_TABLEVALUE",
            Self::Weight => "Q_WEIGHT",
            Self::Length => "Q_LENGTH",
            Self::Area => "Q_AREA",
            Self::Volume => "Q_VOLUME",
            Self::Count => "Q_COUNT",
            Self::Time => "Q_TIME",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateEnumeration {
    pub name: &'static str,
    pub literals: &'static [&'static str],
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateEntry {
    pub name: &'static str,
    pub definition: &'static str,
    pub kind: IfcTemplateEntryKind,
    pub data_type: &'static str,
    pub enumeration: Option<IfcTemplateEnumeration>,
    pub secondary_type: Option<&'static str>,
    pub expression: Option<&'static str>,
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplate {
    pub name: &'static str,
    pub definition: &'static str,
    pub kind: &'static str,
    pub applicable: &'static str,
    pub classes: &'static [&'static str],
    pub entries: &'static [IfcTemplateEntry],
    pub source_file: &'static str,
    pub source_sha256: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateSource {
    pub url: &'static str,
    pub commit: &'static str,
    pub sha256: &'static str,
    pub language: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct IfcTemplateCatalog {
    pub schema: &'static str,
    pub source: IfcTemplateSource,
    pub templates: &'static [IfcTemplate],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcTemplateApplicability {
    Applicable,
    Inapplicable,
    RequiresPredefinedType,
    UnknownEntity,
}
impl IfcTemplateApplicability {
    pub const fn source_key(self) -> &'static str {
        match self {
            Self::Applicable => "applicable",
            Self::Inapplicable => "inapplicable",
            Self::RequiresPredefinedType => "requiresPredefinedType",
            Self::UnknownEntity => "unknownEntity",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum IfcTemplateLocale {
    En,
    De,
}
include!("data.rs");

/// 📖️ Borrow the full immutable release without runtime parsing or allocation.
pub fn ifc4x3_template_catalog() -> &'static IfcTemplateCatalog {
    &CATALOG
}
/// 🔎️ Resolve a named template through the case-independent sorted release index.
pub fn ifc4x3_template(name: &str) -> Option<&'static IfcTemplate> {
    CATALOG.templates.binary_search_by(|template| template.name.bytes().map(|byte| byte.to_ascii_uppercase()).cmp(name.bytes().map(|byte| byte.to_ascii_uppercase()))).ok().map(|index| &CATALOG.templates[index])
}
/// 🎯️ Preserve entity inheritance and explicit predefined-type applicability.
pub fn ifc4x3_template_applicability(template: &IfcTemplate, entity: &str, predefined_type: Option<&str>) -> IfcTemplateApplicability {
    if crate::validation::metadata::entity(entity).is_none() {
        return IfcTemplateApplicability::UnknownEntity;
    }
    let mut pending = false;
    for selector in template.classes {
        let (target, predefined) = selector.split_once('/').map_or((*selector, None), |(entity, predefined)| (entity, Some(predefined)));
        if !crate::validation::metadata::is_subtype(entity, target) {
            continue;
        }
        match (predefined, predefined_type) {
            (None, _) => return IfcTemplateApplicability::Applicable,
            (Some(_), None) => pending = true,
            (Some(expected), Some(actual)) if expected.eq_ignore_ascii_case(actual) => return IfcTemplateApplicability::Applicable,
            _ => {}
        }
    }
    if pending { IfcTemplateApplicability::RequiresPredefinedType } else { IfcTemplateApplicability::Inapplicable }
}
/// 🌐️ Select a field-form label with an explicit supported locale.
pub fn ifc4x3_template_kind_label(kind: IfcTemplateEntryKind, locale: IfcTemplateLocale) -> &'static str {
    let labels = match kind {
        IfcTemplateEntryKind::SingleValue => ("Single Value", "Einzelwert"),
        IfcTemplateEntryKind::ReferenceValue => ("Reference Value", "Referenzwert"),
        IfcTemplateEntryKind::EnumeratedValue => ("Enumerated Value", "Aufzählungswert"),
        IfcTemplateEntryKind::ListValue => ("List Value", "Listenwert"),
        IfcTemplateEntryKind::BoundedValue => ("Bounded Value", "Wertebereich"),
        IfcTemplateEntryKind::TableValue => ("Table Value", "Tabellenwert"),
        IfcTemplateEntryKind::Weight => ("Weight", "Gewicht"),
        IfcTemplateEntryKind::Length => ("Length", "Länge"),
        IfcTemplateEntryKind::Area => ("Area", "Fläche"),
        IfcTemplateEntryKind::Volume => ("Volume", "Volumen"),
        IfcTemplateEntryKind::Count => ("Count", "Anzahl"),
        IfcTemplateEntryKind::Time => ("Time", "Zeit"),
    };
    match locale {
        IfcTemplateLocale::En => labels.0,
        IfcTemplateLocale::De => labels.1,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_release_matches_every_canonical_field_and_neutral_case() {
        let canonical: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let cases: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
        assert_eq!(CATALOG.templates.len(), 760);
        assert_eq!(CATALOG.templates.iter().map(|template| template.entries.len()).sum::<usize>(), 3988);
        for (actual, expected) in CATALOG.templates.iter().zip(canonical["templates"].as_array().unwrap()) {
            assert_eq!(actual.name, expected["name"].as_str().unwrap());
            assert_eq!(actual.definition, expected["definition"].as_str().unwrap());
            assert_eq!(actual.kind, expected["kind"].as_str().unwrap());
            assert_eq!(actual.applicable, expected["applicable"].as_str().unwrap());
            assert_eq!(actual.source_file, expected["sourceFile"].as_str().unwrap());
            assert_eq!(actual.source_sha256, expected["sourceSha256"].as_str().unwrap());
            assert_eq!(actual.classes, expected["classes"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>());
            assert_eq!(actual.entries.len(), expected["entries"].as_array().unwrap().len());
            for (entry, source) in actual.entries.iter().zip(expected["entries"].as_array().unwrap()) {
                assert_eq!(entry.name, source["name"].as_str().unwrap());
                assert_eq!(entry.definition, source["definition"].as_str().unwrap());
                assert_eq!(entry.kind.source_key(), source["kind"].as_str().unwrap());
                assert_eq!(entry.data_type, source["dataType"].as_str().unwrap());
                assert_eq!(entry.secondary_type, source["secondaryType"].as_str());
                assert_eq!(entry.expression, source["expression"].as_str());
                if let Some(enumeration) = entry.enumeration {
                    assert_eq!(enumeration.name, source["enumeration"]["name"].as_str().unwrap());
                    assert_eq!(enumeration.literals, source["enumeration"]["literals"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>());
                } else {
                    assert!(source["enumeration"].is_null());
                }
            }
        }
        for case in cases["applicability"].as_array().unwrap() {
            assert_eq!(ifc4x3_template_applicability(ifc4x3_template(case["template"].as_str().unwrap()).unwrap(), case["entity"].as_str().unwrap(), case["predefinedType"].as_str()).source_key(), case["state"].as_str().unwrap());
        }
        assert_eq!(ifc4x3_template_kind_label(IfcTemplateEntryKind::SingleValue, IfcTemplateLocale::De), "Einzelwert");
    }
}
