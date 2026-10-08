//! 🧭️ The glTF editor's edit rules: which snapshot pointer raises which ONE concrete glTF mutation kind. [`RULES`] is the declarative table the stdio contract resolves (names, weights, material flags, topology mode, row insert/delete); [`resolve_special`] answers the gestures whose payload is computed from the addressed entry (binds and unbinds, extras and extensions, node transforms, reorders, moves, removals by value). An edit that no kind expresses is refused, never approximated and never replayed through a whole-document diff.

use super::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector, SnapshotEditEvent};
use super::{kinds, Fault, GltfMutation, GltfSnapshot};
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_framework_value::{DslValue, FromValue};

type Rows = Result<Vec<GltfMutation>, Fault>;

struct Ctx<'a> {
    base: &'a GltfSnapshot,
    ix: Vec<usize>,
    key: Option<String>,
    value: Option<&'a DslValue>,
}

struct FieldRule {
    path: &'static str,
    resolve: fn(&Ctx) -> Rows,
}

struct ListRule {
    path: &'static str,
    insert: Option<fn(&GltfSnapshot, &[usize], usize, &DslValue) -> Rows>,
    remove: Option<fn(&GltfSnapshot, &[usize], usize) -> Rows>,
    relocate: fn(&GltfSnapshot, &[usize], usize, usize) -> Rows,
}

macro_rules! one {
    ($module:ident, $payload:ident { $($field:tt)* }) => {
        Ok(vec![kinds::$module::mutation(kinds::$module::$payload { $($field)* })])
    };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refuse(what: impl std::fmt::Display) -> Fault {
    Fault::from(format!("snapshot-edit.unsupported: {what}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode<T: FromValue>(value: &DslValue) -> Result<T, Fault> {
    T::from_value(value.clone()).map_err(|error| Fault::from(format!("snapshot-edit.schema-invalid: {error}")))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn optional<T: FromValue>(ctx: &Ctx) -> Result<Option<T>, Fault> {
    match ctx.value {
        None | Some(DslValue::Null) => Ok(None),
        Some(value) => decode(value).map(Some),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn required<T: FromValue>(ctx: &Ctx) -> Result<T, Fault> {
    optional(ctx)?.ok_or_else(|| refuse("a required glTF field cannot be removed"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row<T: Clone>(items: &[T], index: usize) -> Result<T, Fault> {
    items.get(index).cloned().ok_or_else(|| refuse(format!("row {index} does not exist")))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reordered<T: Ord + Clone>(current: &[T], ctx: &Ctx) -> Result<Option<Vec<T>>, Fault>
where
    Vec<T>: FromValue,
{
    let order: Vec<T> = required(ctx)?;
    let (mut left, mut right) = (current.to_vec(), order.clone());
    left.sort();
    right.sort();
    if left != right {
        return Err(refuse("a reference list changes its members; use the bind/unbind pointer of one entry"));
    }
    Ok((current != order.as_slice()).then_some(order))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn metadata(ctx: &Ctx, field: &str) -> Rows {
    let asset = &ctx.base.document.asset;
    let (mut generator, mut copyright, mut min_version) = (asset.generator.clone(), asset.copyright.clone(), asset.min_version.clone());
    let value = optional::<String>(ctx)?;
    match field {
        "generator" => generator = value,
        "copyright" => copyright = value,
        _ => min_version = value,
    }
    one!(change_asset_descriptive_metadata, GltfChangeAssetDescriptiveMetadataPayload { generator, copyright, min_version })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform(ctx: &Ctx, field: &str) -> Rows {
    let node = row(&ctx.base.document.nodes, ctx.ix[0])?;
    let (mut translation, mut rotation, mut scale) = (node.translation, node.rotation, node.scale);
    let transform = match field {
        "matrix" => match optional::<[f64; 16]>(ctx)? {
            Some(matrix) if translation.is_some() || rotation.is_some() || scale.is_some() => return Err(refuse(format!("a node holding both a matrix {matrix:?} and translation/rotation/scale"))),
            Some(matrix) => kinds::change_node_transform::GltfNodeTransform::Matrix { matrix },
            None => kinds::change_node_transform::GltfNodeTransform::Trs { translation, rotation, scale },
        },
        other => {
            if node.matrix.is_some() {
                return Err(refuse("a node holding both a matrix and translation/rotation/scale"));
            }
            match other {
                "translation" => translation = optional(ctx)?,
                "rotation" => rotation = optional(ctx)?,
                _ => scale = optional(ctx)?,
            }
            kinds::change_node_transform::GltfNodeTransform::Trs { translation, rotation, scale }
        }
    };
    one!(change_node_transform, GltfTransformNodePayload { node: ctx.ix[0], transform })
}

static FIELDS: &[FieldRule] = &[
    FieldRule { path: "document/asset/version", resolve: |c| one!(change_asset_version, GltfChangeAssetVersionPayload { version: required(c)? }) },
    FieldRule { path: "document/asset/generator", resolve: |c| metadata(c, "generator") },
    FieldRule { path: "document/asset/copyright", resolve: |c| metadata(c, "copyright") },
    FieldRule { path: "document/asset/minVersion", resolve: |c| metadata(c, "minVersion") },
    FieldRule { path: "document/asset/extensions", resolve: |c| one!(change_asset_extension_data, GltfChangeAssetExtensionDataPayload { data: optional(c)? }) },
    FieldRule { path: "document/asset/extras", resolve: |c| one!(change_asset_extra_data, GltfChangeAssetExtraDataPayload { data: optional(c)? }) },
    FieldRule { path: "document/extensions", resolve: |c| one!(change_document_extension_data, GltfChangeDocumentExtensionDataPayload { data: optional(c)? }) },
    FieldRule { path: "document/extras", resolve: |c| one!(change_document_extra_data, GltfChangeDocumentExtraDataPayload { data: optional(c)? }) },
    FieldRule {
        path: "document/scene",
        resolve: |c| match optional::<usize>(c)? {
            Some(scene) => one!(bind_default_scene, GltfBindDefaultScenePayload { scene }),
            None => one!(unbind_default_scene, GltfUnbindDefaultScenePayload {}),
        },
    },
    FieldRule {
        path: "document/extensionsUsed",
        resolve: |c| Ok(reordered(&c.base.document.extensions_used, c)?.map(|order| kinds::reorder_used_extensions::mutation(kinds::reorder_used_extensions::GltfReorderUsedExtensionsPayload { order })).into_iter().collect()),
    },
    FieldRule {
        path: "document/extensionsRequired",
        resolve: |c| Ok(reordered(&c.base.document.extensions_required, c)?.map(|order| kinds::reorder_required_extensions::mutation(kinds::reorder_required_extensions::GltfReorderRequiredExtensionsPayload { order })).into_iter().collect()),
    },
    FieldRule { path: "document/scenes/*/name", resolve: |c| one!(change_scene_name, GltfChangeSceneNamePayload { scene: c.ix[0], value: optional(c)? }) },
    FieldRule { path: "document/scenes/*/extras", resolve: |c| one!(change_scene_extra_data, GltfChangeSceneExtraDataPayload { scene: c.ix[0], data: kinds::change_scene_extra_data::presence(&optional(c)?) }) },
    FieldRule { path: "document/scenes/*/extensions", resolve: |c| one!(change_scene_extension_data, GltfChangeSceneExtensionDataPayload { scene: c.ix[0], data: kinds::change_scene_extension_data::presence(&optional(c)?) }) },
    FieldRule {
        path: "document/scenes/*/nodes",
        resolve: |c| Ok(reordered(&row(&c.base.document.scenes, c.ix[0])?.nodes, c)?.map(|order| kinds::reorder_scene_root_nodes::mutation(kinds::reorder_scene_root_nodes::GltfReorderSceneRootNodesPayload { scene: c.ix[0], order })).into_iter().collect()),
    },
    FieldRule {
        path: "document/nodes/*/name",
        resolve: |c| one!(change_node_name, GltfChangeNodeNamePayload { node: u32::try_from(c.ix[0]).map_err(|_| refuse("a node index past u32"))?, value: optional(c)? }),
    },
    FieldRule { path: "document/nodes/*/extras", resolve: |c| one!(change_node_extra_data, GltfChangeNodeExtraDataPayload { node: c.ix[0], data: kinds::change_node_extra_data::presence(&optional(c)?) }) },
    FieldRule { path: "document/nodes/*/extensions", resolve: |c| one!(change_node_extension_data, GltfChangeNodeExtensionDataPayload { node: c.ix[0], data: kinds::change_node_extension_data::presence(&optional(c)?) }) },
    FieldRule { path: "document/nodes/*/weights", resolve: |c| one!(change_node_morph_weights, GltfChangeNodeMorphWeightsPayload { node: c.ix[0], weights: optional(c)?.unwrap_or_default() }) },
    FieldRule {
        path: "document/nodes/*/mesh",
        resolve: |c| match optional::<usize>(c)? {
            Some(mesh) => one!(bind_node_mesh, GltfBindNodeMeshPayload { node: c.ix[0], mesh }),
            None => one!(unbind_node_mesh, GltfUnbindNodeMeshPayload { node: c.ix[0] }),
        },
    },
    FieldRule {
        path: "document/nodes/*/camera",
        resolve: |c| match optional::<usize>(c)? {
            Some(camera) => one!(bind_node_camera, GltfBindNodeCameraPayload { node: c.ix[0], camera }),
            None => one!(unbind_node_camera, GltfUnbindNodeCameraPayload { node: c.ix[0] }),
        },
    },
    FieldRule {
        path: "document/nodes/*/skin",
        resolve: |c| match optional::<usize>(c)? {
            Some(skin) => one!(bind_node_skin, GltfBindNodeSkinPayload { node: c.ix[0], skin }),
            None => one!(unbind_node_skin, GltfUnbindNodeSkinPayload { node: c.ix[0] }),
        },
    },
    FieldRule {
        path: "document/nodes/*/children",
        resolve: |c| Ok(reordered(&row(&c.base.document.nodes, c.ix[0])?.children, c)?.map(|order| kinds::reorder_node_children::mutation(kinds::reorder_node_children::GltfReorderNodeChildrenPayload { parent: c.ix[0], order })).into_iter().collect()),
    },
    FieldRule { path: "document/nodes/*/matrix", resolve: |c| transform(c, "matrix") },
    FieldRule { path: "document/nodes/*/translation", resolve: |c| transform(c, "translation") },
    FieldRule { path: "document/nodes/*/rotation", resolve: |c| transform(c, "rotation") },
    FieldRule { path: "document/nodes/*/scale", resolve: |c| transform(c, "scale") },
    FieldRule { path: "document/meshes/*/name", resolve: |c| one!(change_mesh_name, GltfChangeMeshNamePayload { mesh: c.ix[0], value: optional(c)? }) },
    FieldRule { path: "document/meshes/*/extras", resolve: |c| one!(change_mesh_extra_data, GltfChangeMeshExtraDataPayload { mesh: c.ix[0], data: kinds::change_mesh_extra_data::presence(&optional(c)?) }) },
    FieldRule { path: "document/meshes/*/extensions", resolve: |c| one!(change_mesh_extension_data, GltfChangeMeshExtensionDataPayload { mesh: c.ix[0], data: kinds::change_mesh_extension_data::presence(&optional(c)?) }) },
    FieldRule { path: "document/meshes/*/weights", resolve: |c| one!(change_mesh_morph_weights, GltfChangeMeshMorphWeightsPayload { mesh: c.ix[0], weights: optional(c)?.unwrap_or_default() }) },
    FieldRule { path: "document/meshes/*/primitives/*/mode", resolve: |c| one!(change_primitive_topology_mode, GltfChangePrimitiveTopologyModePayload { mesh: c.ix[0], primitive: c.ix[1], mode: optional(c)? }) },
    FieldRule {
        path: "document/meshes/*/primitives/*/material",
        resolve: |c| match optional::<usize>(c)? {
            Some(material) => one!(bind_primitive_material, GltfBindPrimitiveMaterialPayload { mesh: c.ix[0], primitive: c.ix[1], material }),
            None => one!(unbind_primitive_material, GltfUnbindPrimitiveMaterialPayload { mesh: c.ix[0], primitive: c.ix[1] }),
        },
    },
    FieldRule {
        path: "document/meshes/*/primitives/*/indices",
        resolve: |c| match optional::<usize>(c)? {
            Some(accessor) => one!(bind_primitive_indices, GltfBindPrimitiveIndicesPayload { mesh: c.ix[0], primitive: c.ix[1], accessor }),
            None => one!(unbind_primitive_indices, GltfUnbindPrimitiveIndicesPayload { mesh: c.ix[0], primitive: c.ix[1] }),
        },
    },
    FieldRule { path: "document/meshes/*/primitives/*/extras", resolve: |c| one!(change_primitive_extra_data, GltfChangePrimitiveExtraDataPayload { mesh: c.ix[0], primitive: c.ix[1], data: kinds::change_primitive_extra_data::presence(&optional(c)?) }) },
    FieldRule { path: "document/meshes/*/primitives/*/extensions", resolve: |c| one!(change_primitive_extension_data, GltfChangePrimitiveExtensionDataPayload { mesh: c.ix[0], primitive: c.ix[1], data: kinds::change_primitive_extension_data::presence(&optional(c)?) }) },
    FieldRule {
        path: "document/meshes/*/primitives/*/attributes/$",
        resolve: |c| {
            let semantic = c.key.clone().unwrap_or_default();
            match optional::<usize>(c)? {
                Some(accessor) => one!(bind_primitive_attribute, GltfBindPrimitiveAttributePayload { mesh: c.ix[0], primitive: c.ix[1], semantic, accessor }),
                None => one!(unbind_primitive_attribute, GltfUnbindPrimitiveAttributePayload { mesh: c.ix[0], primitive: c.ix[1], semantic }),
            }
        },
    },
    FieldRule {
        path: "document/meshes/*/primitives/*/targets/*/$",
        resolve: |c| {
            let semantic = c.key.clone().unwrap_or_default();
            match optional::<usize>(c)? {
                Some(accessor) => one!(bind_morph_target_attribute, GltfBindMorphTargetAttributePayload { mesh: c.ix[0], primitive: c.ix[1], target: c.ix[2], semantic, accessor }),
                None => one!(unbind_morph_target_attribute, GltfUnbindMorphTargetAttributePayload { mesh: c.ix[0], primitive: c.ix[1], target: c.ix[2], semantic }),
            }
        },
    },
    FieldRule { path: "document/materials/*/alphaMode", resolve: |c| one!(change_material_alpha_mode, GltfChangeMaterialAlphaModePayload { material: c.ix[0], alpha_mode: required(c)? }) },
    FieldRule { path: "document/materials/*/doubleSided", resolve: |c| one!(change_material_double_sided, GltfChangeMaterialDoubleSidedPayload { material: c.ix[0], double_sided: required(c)? }) },
];

macro_rules! relocation {
    ($path:literal, $relocate:ident $relocate_payload:ident) => {
        ListRule { path: $path, insert: None, remove: None, relocate: |_, _, from, to| one!($relocate, $relocate_payload { index: from, position: to }) }
    };
}

static LISTS: &[ListRule] = &[
    relocation!("document/scenes", move_scene GltfMoveScenePayload),
    relocation!("document/nodes", move_node GltfMoveNodePayload),
    relocation!("document/meshes", move_mesh GltfMoveMeshPayload),
    relocation!("document/materials", move_material GltfMoveMaterialPayload),
    relocation!("document/textures", move_texture GltfMoveTexturePayload),
    relocation!("document/images", move_image GltfMoveImagePayload),
    relocation!("document/samplers", move_sampler GltfMoveSamplerPayload),
    relocation!("document/skins", move_skin GltfMoveSkinPayload),
    relocation!("document/animations", move_animation GltfMoveAnimationPayload),
    ListRule {
        path: "document/cameras",
        insert: Some(|_, _, at, value| {
            let record: GltfCamera = decode(value)?;
            one!(create_camera, GltfCreateCameraPayload { position: at, projection: record.projection.clone(), camera: Some(Box::new(record)) })
        }),
        remove: None,
        relocate: |_, _, from, to| one!(move_camera, GltfMoveCameraPayload { index: from, position: to }),
    },
    ListRule {
        path: "document/accessors",
        insert: Some(|_, _, at, value| {
            let record: GltfAccessor = decode(value)?;
            one!(create_accessor, GltfCreateAccessorPayload { position: at, component_type: record.component_type, count: record.count, kind: record.kind, accessor: Some(Box::new(record)) })
        }),
        remove: None,
        relocate: |_, _, from, to| one!(move_accessor, GltfMoveAccessorPayload { index: from, position: to }),
    },
    ListRule {
        path: "document/bufferViews",
        insert: Some(|_, _, at, value| {
            let record: GltfBufferView = decode(value)?;
            one!(create_buffer_view, GltfCreateBufferViewPayload { position: at, buffer: record.buffer, byte_offset: record.byte_offset, byte_length: record.byte_length, buffer_view: Some(Box::new(record)) })
        }),
        remove: None,
        relocate: |_, _, from, to| one!(move_buffer_view, GltfMoveBufferViewPayload { index: from, position: to }),
    },
    ListRule {
        path: "document/buffers",
        insert: Some(|_, _, _, _| Err(refuse("a buffer's bytes live in the container, so a buffer entry cannot be inserted through its JSON record"))),
        remove: None,
        relocate: |_, _, from, to| one!(move_buffer, GltfMoveBufferPayload { index: from, position: to }),
    },
    ListRule {
        path: "document/extensionsUsed",
        insert: None,
        remove: Some(|base, _, at| one!(remove_used_extension, GltfWithdrawUsedExtensionPayload { extension: row(&base.document.extensions_used, at)? })),
        relocate: |base, _, from, to| one!(move_used_extension, GltfMoveUsedExtensionPayload { extension: row(&base.document.extensions_used, from)?, position: to }),
    },
    ListRule {
        path: "document/extensionsRequired",
        insert: None,
        remove: Some(|base, _, at| one!(remove_required_extension, GltfUnrequireExtensionPayload { extension: row(&base.document.extensions_required, at)? })),
        relocate: |base, _, from, to| one!(move_required_extension, GltfMoveRequiredExtensionPayload { extension: row(&base.document.extensions_required, from)?, position: to }),
    },
    ListRule {
        path: "document/scenes/*/nodes",
        insert: None,
        remove: Some(|base, ix, at| one!(unbind_scene_root_node, GltfUnbindSceneRootNodePayload { scene: ix[0], node: row(&row(&base.document.scenes, ix[0])?.nodes, at)? })),
        relocate: |base, ix, from, to| one!(move_scene_root_node, GltfMoveSceneRootNodePayload { scene: ix[0], node: row(&row(&base.document.scenes, ix[0])?.nodes, from)?, position: to }),
    },
    ListRule {
        path: "document/nodes/*/children",
        insert: None,
        remove: Some(|base, ix, at| one!(unbind_node_child, GltfUnbindNodeChildPayload { parent: ix[0], child: row(&row(&base.document.nodes, ix[0])?.children, at)? })),
        relocate: |base, ix, from, to| one!(move_node_child, GltfMoveNodeChildPayload { parent: ix[0], child: row(&row(&base.document.nodes, ix[0])?.children, from)?, position: to }),
    },
    ListRule {
        path: "document/meshes/*/primitives",
        insert: None,
        remove: None,
        relocate: |_, ix, from, to| one!(move_primitive, GltfMovePrimitivePayload { mesh: ix[0], primitive: from, position: to }),
    },
    ListRule {
        path: "document/meshes/*/primitives/*/targets",
        insert: None,
        remove: None,
        relocate: |_, ix, from, to| one!(move_morph_target, GltfMoveMorphTargetPayload { mesh: ix[0], primitive: ix[1], target: from, position: to }),
    },
];

/// 🧭️ The pointers whose plain `set` the declarative [`RULES`] table answers; every other gesture on a [`FIELDS`] pointer is computed here.
static RULE_SETS: &[&str] = &[
    "document/asset/version",
    "document/scenes/*/name",
    "document/nodes/*/name",
    "document/meshes/*/name",
    "document/nodes/*/weights",
    "document/meshes/*/weights",
    "document/materials/*/alphaMode",
    "document/materials/*/doubleSided",
    "document/meshes/*/primitives/*/mode",
];

/// 📚 The declarative edit table of the glTF editor: one kind per pointer, the addressed indices and the new value carried as the payload.
pub static RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/document/asset/version", "change-asset-version", "version"),
        EntityRule::new("/document/scenes/*/name", "change-scene-name", "value").selecting(&[Selector::Index("scene")]),
        EntityRule::new("/document/nodes/*/name", "change-node-name", "value").selecting(&[Selector::Index("node")]),
        EntityRule::new("/document/meshes/*/name", "change-mesh-name", "value").selecting(&[Selector::Index("mesh")]),
        EntityRule::new("/document/nodes/*/weights", "change-node-morph-weights", "weights").selecting(&[Selector::Index("node")]),
        EntityRule::new("/document/meshes/*/weights", "change-mesh-morph-weights", "weights").selecting(&[Selector::Index("mesh")]),
        EntityRule::new("/document/materials/*/alphaMode", "change-material-alpha-mode", "alphaMode").selecting(&[Selector::Index("material")]),
        EntityRule::new("/document/materials/*/doubleSided", "change-material-double-sided", "doubleSided").selecting(&[Selector::Index("material")]),
        EntityRule::new("/document/meshes/*/primitives/*/mode", "change-primitive-topology-mode", "mode").selecting(&[Selector::Index("mesh"), Selector::Index("primitive")]),
    ],
    inserts: &[
        InsertRule::new("/document/scenes", "create-scene", "scene").at("position"),
        InsertRule::new("/document/nodes", "create-node", "node").at("position"),
        InsertRule::new("/document/meshes", "create-mesh", "mesh").at("position"),
        InsertRule::new("/document/materials", "create-material", "material").at("position"),
        InsertRule::new("/document/textures", "create-texture", "texture").at("position"),
        InsertRule::new("/document/images", "create-image", "image").at("position"),
        InsertRule::new("/document/samplers", "create-sampler", "sampler").at("position"),
        InsertRule::new("/document/skins", "create-skin", "skin").at("position"),
        InsertRule::new("/document/animations", "create-animation", "animation").at("position"),
        InsertRule::new("/document/extensionsUsed", "add-used-extension", "extension").at("position"),
        InsertRule::new("/document/extensionsRequired", "add-required-extension", "extension").at("position"),
        InsertRule::new("/document/scenes/*/nodes", "bind-scene-root-node", "node").at("position").selecting(&[Selector::Index("scene")]),
        InsertRule::new("/document/nodes/*/children", "bind-node-child", "child").at("position").selecting(&[Selector::Index("parent")]),
        InsertRule::new("/document/meshes/*/primitives", "create-primitive", "primitive").at("position").selecting(&[Selector::Index("mesh")]),
        InsertRule::new("/document/meshes/*/primitives/*/targets", "create-morph-target", "target").at("position").selecting(&[Selector::Index("mesh"), Selector::Index("primitive")]),
    ],
    removes: &[
        RemoveRule::by_index("/document/scenes", "delete-scene", "index"),
        RemoveRule::by_index("/document/nodes", "delete-node", "index"),
        RemoveRule::by_index("/document/meshes", "delete-mesh", "index"),
        RemoveRule::by_index("/document/accessors", "delete-accessor", "index"),
        RemoveRule::by_index("/document/bufferViews", "delete-buffer-view", "index"),
        RemoveRule::by_index("/document/buffers", "delete-buffer", "index"),
        RemoveRule::by_index("/document/materials", "delete-material", "index"),
        RemoveRule::by_index("/document/textures", "delete-texture", "index"),
        RemoveRule::by_index("/document/images", "delete-image", "index"),
        RemoveRule::by_index("/document/samplers", "delete-sampler", "index"),
        RemoveRule::by_index("/document/skins", "delete-skin", "index"),
        RemoveRule::by_index("/document/animations", "delete-animation", "index"),
        RemoveRule::by_index("/document/cameras", "delete-camera", "index"),
        RemoveRule::by_index("/document/meshes/*/primitives", "delete-primitive", "primitive").selecting(&[Selector::Index("mesh")]),
        RemoveRule::by_index("/document/meshes/*/primitives/*/targets", "delete-morph-target", "target").selecting(&[Selector::Index("mesh"), Selector::Index("primitive")]),
    ],
};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tokens(pointer: &str) -> Result<Vec<String>, Fault> {
    let Some(rest) = pointer.strip_prefix('/') else {
        return Err(refuse(format!("'{pointer}' is not a pointer into the snapshot")));
    };
    rest.split('/').map(|raw| Ok(raw.replace("~1", "/").replace("~0", "~"))).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn capture(pattern: &str, tokens: &[String]) -> Option<(Vec<usize>, Option<String>)> {
    let parts: Vec<&str> = pattern.split('/').collect();
    if parts.len() != tokens.len() {
        return None;
    }
    let (mut ix, mut key) = (Vec::new(), None);
    for (part, token) in parts.iter().zip(tokens) {
        match *part {
            "*" => ix.push(token.parse().ok()?),
            "$" => key = Some(token.clone()),
            literal if literal == token => {}
            _ => return None,
        }
    }
    Some((ix, key))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn field(base: &GltfSnapshot, pointer: &str, value: Option<&DslValue>) -> Result<Option<Vec<GltfMutation>>, Fault> {
    let path = tokens(pointer)?;
    let Some((rule, ix, key)) = FIELDS.iter().find_map(|rule| capture(rule.path, &path).map(|(ix, key)| (rule, ix, key))) else { return Ok(None) };
    if value.is_some_and(|value| !matches!(value, DslValue::Null)) && RULE_SETS.contains(&rule.path) {
        return Ok(None);
    }
    (rule.resolve)(&Ctx { base, ix, key, value }).map(Some)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn element(pointer: &str, len: impl Fn(&ListRule, &[usize]) -> usize) -> Result<Option<(&'static ListRule, Vec<usize>, usize)>, Fault> {
    let path = tokens(pointer)?;
    let Some((last, parent)) = path.split_last() else { return Ok(None) };
    for rule in LISTS {
        if let Some((ix, _)) = capture(rule.path, parent) {
            let at = if last == "-" { len(rule, &ix) } else { last.parse().map_err(|_| refuse(format!("'{pointer}' does not address a row")))? };
            return Ok(Some((rule, ix, at)));
        }
    }
    Ok(None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn length(base: &GltfSnapshot, rule: &ListRule, ix: &[usize]) -> usize {
    let document = &base.document;
    match rule.path {
        "document/scenes" => document.scenes.len(),
        "document/nodes" => document.nodes.len(),
        "document/meshes" => document.meshes.len(),
        "document/materials" => document.materials.len(),
        "document/textures" => document.textures.len(),
        "document/images" => document.images.len(),
        "document/samplers" => document.samplers.len(),
        "document/skins" => document.skins.len(),
        "document/animations" => document.animations.len(),
        "document/cameras" => document.cameras.len(),
        "document/accessors" => document.accessors.len(),
        "document/bufferViews" => document.buffer_views.len(),
        "document/buffers" => document.buffers.len(),
        "document/extensionsUsed" => document.extensions_used.len(),
        "document/extensionsRequired" => document.extensions_required.len(),
        "document/scenes/*/nodes" => document.scenes.get(ix[0]).map_or(0, |scene| scene.nodes.len()),
        "document/nodes/*/children" => document.nodes.get(ix[0]).map_or(0, |node| node.children.len()),
        "document/meshes/*/primitives" => document.meshes.get(ix[0]).map_or(0, |mesh| mesh.primitives.len()),
        _ => document.meshes.get(ix[0]).and_then(|mesh| mesh.primitives.get(ix[1])).map_or(0, |primitive| primitive.targets.len()),
    }
}

/// 🎯️ The gestures [`RULES`] cannot express because the payload is computed from the addressed entry (a bind or unbind chosen by the new value, a
/// removal by value, a move, a transform composed from the node's other components): the ONE concrete kind they raise, reading only the addressed
/// entry of `base`. `None` hands the edit to [`RULES`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn resolve_special(base: &GltfSnapshot, event: &SnapshotEditEvent) -> Result<Option<Vec<GltfMutation>>, Fault> {
    let measure = |rule: &ListRule, ix: &[usize]| length(base, rule, ix);
    match event {
        SnapshotEditEvent::SetValue { path, value } => field(base, path, Some(value)),
        SnapshotEditEvent::InsertValue { path, value } => match element(path, measure)? {
            Some((rule, ix, at)) => rule.insert.map(|insert| insert(base, &ix, at, value)).transpose(),
            None => field(base, path, Some(value)),
        },
        SnapshotEditEvent::RemoveValue { path } => match element(path, measure)? {
            Some((rule, ix, at)) => rule.remove.map(|remove| remove(base, &ix, at)).transpose(),
            None => field(base, path, None),
        },
        SnapshotEditEvent::MoveValue { from, path } => match (element(from, measure)?, element(path, measure)?) {
            (Some((rule, ix, source)), Some((other, other_ix, target))) if rule.path == other.path && ix == other_ix => (rule.relocate)(base, &ix, source, target).map(Some),
            _ => Ok(None),
        },
        SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::ReplaceSource { .. } => Ok(None),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
