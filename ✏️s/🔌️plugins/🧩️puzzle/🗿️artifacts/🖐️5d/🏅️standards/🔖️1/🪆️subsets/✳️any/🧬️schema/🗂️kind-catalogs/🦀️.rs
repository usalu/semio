//! 🗂️ Kind catalog composition of the puzzle 5d artifact: the puzzle-owned extra halves of the part, grip, fastener and rope kind rows and the converters that split and join them with the shared kit types.

use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dAttribute, Puzzle5dAuthor, Puzzle5dCatalogFastenerKind, Puzzle5dCatalogGripKind, Puzzle5dCatalogPartKind, Puzzle5dCatalogRopeKind, Puzzle5dGripTemplate, Puzzle5dKindCatalogs, Puzzle5dRepresentation};
use semio_s_artifact_stdio_semio_kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};

//#region 🔖️KindCatalogComposition
/// 🧩️ Puzzle5d-owned overflow for one part-kind catalog row — everything `SemioKitType` cannot
/// represent. Id-joined 1:1 to a `SemioKitType` (`category = "part"`) in the composed
/// `Puzzle5dSnapshot::kind_catalogs` child.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogPartKindExtra {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub image: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub unit: String,
    #[cfg_attr(test, serde(default, rename = "abstract"))]
    #[value(default, rename = "abstract")]
    pub is_abstract: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub base_kinds: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub representations: Vec<Puzzle5dRepresentation>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub grips: Vec<Puzzle5dGripTemplate>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attributes: Vec<Puzzle5dAttribute>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub authors: Vec<Puzzle5dAuthor>,
}

/// 🧩️ Puzzle5d-owned overflow for one grip-kind catalog row (`category = "grip"`). `SemioKitType` has
/// no `name` slot this row ever populated (`Puzzle5dCatalogGripKind` never carried one either) — the
/// composed `SemioKitType.name` is a display-only derivation (`label` else `code`), never round-
/// tripped back into `code`/`label` themselves, both of which live here unchanged.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripKindExtra {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub compatible_with: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub color: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "rope_kind")]
    pub default_rope_kind: String,
}

/// 🧩️ Puzzle5d-owned overflow for one fastener-kind catalog row (`category = "fastener"`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogFastenerKindExtra {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🧩️ Puzzle5d-owned overflow for one rope-kind catalog row (`category = "rope"`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogRopeKindExtra {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "fastener_kind")]
    pub default_fastener_kind: String,
}

/// 🗂️ The puzzle5d-owned overflow half of `Puzzle5dKindCatalogs`, sibling to the composed
/// `kind_catalogs` child — see the region doc for the split/join contract.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCatalogsExtra {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub parts: Vec<Puzzle5dCatalogPartKindExtra>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub grips: Vec<Puzzle5dCatalogGripKindExtra>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub fasteners: Vec<Puzzle5dCatalogFastenerKindExtra>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub ropes: Vec<Puzzle5dCatalogRopeKindExtra>,
}

//#region 🔖️RowConverters
pub fn kit_type_from_part_kind(k: &Puzzle5dCatalogPartKind) -> SemioKitType {
    SemioKitType { id: k.id.clone(), name: k.name.clone(), category: "part".into() }
}

pub fn part_kind_extra_from_part_kind(k: &Puzzle5dCatalogPartKind) -> Puzzle5dCatalogPartKindExtra {
    Puzzle5dCatalogPartKindExtra {
        id: k.id.clone(),
        name: k.name.clone(),
        label: k.label.clone(),
        description: k.description.clone(),
        icon: k.icon.clone(),
        image: k.image.clone(),
        unit: k.unit.clone(),
        is_abstract: k.is_abstract,
        base_kinds: k.base_kinds.clone(),
        representations: k.representations.clone(),
        grips: k.grips.clone(),
        attributes: k.attributes.clone(),
        authors: k.authors.clone(),
    }
}

pub fn part_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogPartKindExtra) -> Puzzle5dCatalogPartKind {
    Puzzle5dCatalogPartKind {
        id: kit_type.id.clone(),
        name: kit_type.name.clone(),
        label: extra.label.clone(),
        description: extra.description.clone(),
        icon: extra.icon.clone(),
        image: extra.image.clone(),
        unit: extra.unit.clone(),
        is_abstract: extra.is_abstract,
        base_kinds: extra.base_kinds.clone(),
        representations: extra.representations.clone(),
        grips: extra.grips.clone(),
        attributes: extra.attributes.clone(),
        authors: extra.authors.clone(),
    }
}

pub fn kit_type_from_grip_kind(k: &Puzzle5dCatalogGripKind) -> SemioKitType {
    let name = k.label.clone().or_else(|| k.code.clone()).unwrap_or_default();
    SemioKitType { id: k.id.clone(), name, category: "grip".into() }
}

pub fn grip_kind_extra_from_grip_kind(k: &Puzzle5dCatalogGripKind) -> Puzzle5dCatalogGripKindExtra {
    Puzzle5dCatalogGripKindExtra {
        id: k.id.clone(),
        code: k.code.clone(),
        label: k.label.clone(),
        order: k.order,
        compatible_with: k.compatible_with.clone(),
        description: k.description.clone(),
        icon: k.icon.clone(),
        color: k.color.clone(),
        default_rope_kind: k.default_rope_kind.clone(),
    }
}

pub fn grip_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogGripKindExtra) -> Puzzle5dCatalogGripKind {
    let _ = kit_type;
    Puzzle5dCatalogGripKind {
        id: extra.id.clone(),
        code: extra.code.clone(),
        label: extra.label.clone(),
        order: extra.order,
        compatible_with: extra.compatible_with.clone(),
        description: extra.description.clone(),
        icon: extra.icon.clone(),
        color: extra.color.clone(),
        default_rope_kind: extra.default_rope_kind.clone(),
    }
}

pub fn kit_type_from_fastener_kind(k: &Puzzle5dCatalogFastenerKind) -> SemioKitType {
    SemioKitType { id: k.id.clone(), name: k.name.clone(), category: "fastener".into() }
}

pub fn fastener_kind_extra_from_fastener_kind(k: &Puzzle5dCatalogFastenerKind) -> Puzzle5dCatalogFastenerKindExtra {
    Puzzle5dCatalogFastenerKindExtra { id: k.id.clone(), name: k.name.clone(), label: k.label.clone() }
}

pub fn fastener_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogFastenerKindExtra) -> Puzzle5dCatalogFastenerKind {
    Puzzle5dCatalogFastenerKind { id: kit_type.id.clone(), name: kit_type.name.clone(), label: extra.label.clone() }
}

pub fn kit_type_from_rope_kind(k: &Puzzle5dCatalogRopeKind) -> SemioKitType {
    SemioKitType { id: k.id.clone(), name: k.name.clone(), category: "rope".into() }
}

pub fn rope_kind_extra_from_rope_kind(k: &Puzzle5dCatalogRopeKind) -> Puzzle5dCatalogRopeKindExtra {
    Puzzle5dCatalogRopeKindExtra { id: k.id.clone(), name: k.name.clone(), label: k.label.clone(), default_fastener_kind: k.default_fastener_kind.clone() }
}

pub fn rope_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogRopeKindExtra) -> Puzzle5dCatalogRopeKind {
    Puzzle5dCatalogRopeKind { id: kit_type.id.clone(), name: kit_type.name.clone(), label: extra.label.clone(), default_fastener_kind: extra.default_fastener_kind.clone() }
}
//#endregion 🔖️RowConverters

//#region 🔖️WholeListConverters
/// 🔀️ `Puzzle5dKindCatalogs` → the shared `SemioKitType` half of the composed catalog child (all
/// four kind lists flattened into one `category`-tagged list, matching `SemioKitSnapshot.types`'s
/// own id-keyed, category-differentiated shape).
pub fn kind_catalogs_kit_types(catalogs: &Puzzle5dKindCatalogs) -> Vec<SemioKitType> {
    catalogs.parts.iter().map(kit_type_from_part_kind).chain(catalogs.grips.iter().map(kit_type_from_grip_kind)).chain(catalogs.fasteners.iter().map(kit_type_from_fastener_kind)).chain(catalogs.ropes.iter().map(kit_type_from_rope_kind)).collect()
}

/// 🔀️ `Puzzle5dKindCatalogs` → the puzzle5d-owned overflow half. Lossless together with
/// `kind_catalogs_kit_types`: every field of every row lands in exactly one of the two halves.
pub fn kind_catalogs_extra_from_kind_catalogs(catalogs: &Puzzle5dKindCatalogs) -> Puzzle5dKindCatalogsExtra {
    Puzzle5dKindCatalogsExtra {
        parts: catalogs.parts.iter().map(part_kind_extra_from_part_kind).collect(),
        grips: catalogs.grips.iter().map(grip_kind_extra_from_grip_kind).collect(),
        fasteners: catalogs.fasteners.iter().map(fastener_kind_extra_from_fastener_kind).collect(),
        ropes: catalogs.ropes.iter().map(rope_kind_extra_from_rope_kind).collect(),
    }
}

/// 🔀️ Inverse of the split above — reassembles a full `Puzzle5dKindCatalogs` from its composed-child
/// half (a flat, category-tagged `SemioKitType` list) and its puzzle5d-owned overflow half, id-joined
/// per category. A `SemioKitType` with no matching `*Extra` row (composed-child content the working-
/// scene cache hasn't seen yet — see `kind_catalogs_of`'s doc comment) is silently dropped rather than
/// fabricated with placeholder fields.
pub fn kind_catalogs_from_kit_types_and_extra(types: &[SemioKitType], extra: &Puzzle5dKindCatalogsExtra) -> Puzzle5dKindCatalogs {
    let by_category = |category: &str| -> std::collections::HashMap<&str, &SemioKitType> { types.iter().filter(|t| t.category == category).map(|t| (t.id.as_str(), t)).collect() };
    let part_types = by_category("part");
    let grip_types = by_category("grip");
    let fastener_types = by_category("fastener");
    let rope_types = by_category("rope");
    Puzzle5dKindCatalogs {
        parts: extra.parts.iter().filter_map(|e| part_types.get(e.id.as_str()).map(|t| part_kind_from_parts(t, e))).collect(),
        grips: extra.grips.iter().filter_map(|e| grip_types.get(e.id.as_str()).map(|t| grip_kind_from_parts(t, e))).collect(),
        fasteners: extra.fasteners.iter().filter_map(|e| fastener_types.get(e.id.as_str()).map(|t| fastener_kind_from_parts(t, e))).collect(),
        ropes: extra.ropes.iter().filter_map(|e| rope_types.get(e.id.as_str()).map(|t| rope_kind_from_parts(t, e))).collect(),
    }
}

/// 🔀️ The full kind-catalogs' shared half, as a fresh (design-less, link-less, object/model/
/// properties-less) `SemioKitSnapshot` — content-addressed by `kind_catalogs_child_handle` below,
/// never embedded inline in `Puzzle5dSnapshot`.
pub fn kind_catalogs_kit_snapshot(catalogs: &Puzzle5dKindCatalogs) -> SemioKitSnapshot {
    SemioKitSnapshot { types: kind_catalogs_kit_types(catalogs), ..SemioKitSnapshot::default() }
}

/// 🪪️ Content-addressed child handle for a kind-catalogs bundle — hashes the deterministic JSON of
/// the derived `SemioKitType` list so peers replaying the same catalogs converge on the same
/// `child_id` (never a random/incrementing id), mirroring `sourcing`'s `catalog_child_handle`.
pub fn kind_catalogs_child_handle(catalogs: &Puzzle5dKindCatalogs) -> store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot> {
    let types = kind_catalogs_kit_types(catalogs);
    let canonical = semio_framework_pack_json::to_json_string(&types);
    let child_id = store::content_id("kind-catalogs", canonical.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️WholeListConverters

/// 🌱 Seeds the working-scene cache for `catalogs`' deterministic `kind_catalogs_child_handle`,
/// without building a whole `Puzzle5dSnapshot` — for fixture loaders that parse the persisted
/// snapshot from DSL text (which never embeds child content) but still need the SAME content-
/// addressed handle's catalogs resolvable immediately after loading.
pub fn validate_kind_catalogs_payload(catalogs: &Puzzle5dKindCatalogs) {
    let _ = kind_catalogs_child_handle(catalogs);
}

/// 🏗️ Mints a fresh content-addressed handle for `catalogs`, splits it into its composed-child half
/// and puzzle5d-owned overflow half, and seeds the working-scene cache so this SAME call's render/
/// export/inference/mutation paths can resolve the handle immediately. Returns the handle plus the
/// overflow half — the two fields a `Puzzle5dSnapshot`/`Puzzle5dArtifact`/`Puzzle5dDiff` now carry in
/// place of the old inline `Puzzle5dKindCatalogs` field.
pub fn split_and_seed_kind_catalogs(catalogs: Option<Puzzle5dKindCatalogs>) -> (Option<store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot>>, Option<Puzzle5dKindCatalogsExtra>) {
    match catalogs {
        None => (None, None),
        Some(catalogs) => {
            let handle = kind_catalogs_child_handle(&catalogs);
            (Some(handle), Some(kind_catalogs_extra_from_kind_catalogs(&catalogs)))
        }
    }
}

/// 👁️ The one accessor every render/export/inference/mutation call site funnels through to read the
/// full reassembled kind-catalogs bundle back in its original `Puzzle5dKindCatalogs` shape.
pub fn kind_catalogs_of(handle: &Option<store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot>>, extra: &Option<Puzzle5dKindCatalogsExtra>) -> Option<Puzzle5dKindCatalogs> {
    let _ = handle.as_ref()?;
    let extra = extra.clone().unwrap_or_default();
    Some(Puzzle5dKindCatalogs {
        parts: extra
            .parts
            .iter()
            .map(|row| Puzzle5dCatalogPartKind {
                id: row.id.clone(),
                name: row.name.clone(),
                label: row.label.clone(),
                description: row.description.clone(),
                icon: row.icon.clone(),
                image: row.image.clone(),
                unit: row.unit.clone(),
                is_abstract: row.is_abstract,
                base_kinds: row.base_kinds.clone(),
                representations: row.representations.clone(),
                grips: row.grips.clone(),
                attributes: row.attributes.clone(),
                authors: row.authors.clone(),
            })
            .collect(),
        grips: extra
            .grips
            .iter()
            .map(|row| Puzzle5dCatalogGripKind {
                id: row.id.clone(),
                code: row.code.clone(),
                label: row.label.clone(),
                order: row.order,
                compatible_with: row.compatible_with.clone(),
                description: row.description.clone(),
                icon: row.icon.clone(),
                color: row.color.clone(),
                default_rope_kind: row.default_rope_kind.clone(),
            })
            .collect(),
        fasteners: extra.fasteners.iter().map(|row| Puzzle5dCatalogFastenerKind { id: row.id.clone(), name: row.name.clone(), label: row.label.clone() }).collect(),
        ropes: extra.ropes.iter().map(|row| Puzzle5dCatalogRopeKind { id: row.id.clone(), name: row.name.clone(), label: row.label.clone(), default_fastener_kind: row.default_fastener_kind.clone() }).collect(),
    })
}
//#endregion 🔖️KindCatalogComposition
