//! 📝️ Text representation codec surface for `stdio.gltf` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");

use crate::artifacts::gltf::schema::diff::{
    dec_accessor, dec_animation, dec_asset, dec_buffer, dec_bytes, dec_gltf_snapshot, dec_material, dec_mesh, dec_node, dec_scene, enc_accessor, enc_animation, enc_asset, enc_buffer, enc_bytes, enc_gltf_snapshot, enc_material, enc_mesh, enc_node,
    enc_scene,
};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;

/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `GltfMutation` — CONFIRMED by a real
/// `cargo check -p semio-s-plugin-stdio --lib` failure with `#[derive(dsl::DslOps)]` temporarily
/// added to this enum (33 `E0277` errors, captured in `f6-gltf-mutation-derive-check1.txt` in the
/// ticket folder, then reverted): `SetSnapshot{snapshot: GltfSnapshot}` recursively requires
/// `DslField` on `GltfAsset`/`GltfScene`/`GltfNode`/`GltfMesh`/`GltfAccessor`/`GltfMaterial`/
/// `GltfBuffer`/`GltfAnimation`/`GltfSnapshot` itself, none of which are `DslRecord`-derived, and
/// even fully deriving all of them would still fail once the walk reaches `GltfJson`/
/// `GltfCameraProjection` (real data-carrying enums, no `DslField` impl possible — see
/// `🔺️diff/component.rs`'s `HandcraftedDiffCodec` doc comment for the full citation). Reuses the
/// diff module's `pub(crate)` grammar primitives and value codecs (`hex_encode`/`enc_asset`/
/// `enc_scene`/.../`enc_gltf_snapshot`/...) rather than duplicating them a second time in this
/// file — same intra-artifact reuse `SvgMutation` uses off `SvgDiff`. Grammar: `keyword arg=value
/// ...` (space-separated), one match arm per variant, matching the derive's own handcrafted-wrapper
/// convention (`f6-recon-report.md` §2) in shape even though nothing here actually derives
/// `DslVariants`.
fn enc_optional_index(value: Option<usize>) -> String {
    value.map(|value| value.to_string()).unwrap_or_else(|| "-".into())
}

fn dec_optional_index(value: &str) -> Result<Option<usize>, String> {
    if value == "-" {
        Ok(None)
    } else {
        value.parse().map(Some).map_err(|error: std::num::ParseIntError| error.to_string())
    }
}

fn enc_optional_array<const N: usize>(value: Option<[f64; N]>) -> String {
    value.map(|values| values.into_iter().map(|value| value.to_bits().to_string()).collect::<Vec<_>>().join(",")).unwrap_or_else(|| "-".into())
}

fn dec_optional_array<const N: usize>(value: &str) -> Result<Option<[f64; N]>, String> {
    if value == "-" {
        return Ok(None);
    }
    let values = value.split(',').map(|part| part.parse::<u64>().map(f64::from_bits).map_err(|error| error.to_string())).collect::<Result<Vec<_>, _>>()?;
    values.try_into().map(Some).map_err(|values: Vec<f64>| format!("expected {N} values, got {}", values.len()))
}

fn print_gltf_mutation(m: &GltfMutation) -> String {
    match m {
        GltfMutation::NoMutation(NoMutation {}) => "no-mutation".to_string(),
        GltfMutation::SetSnapshot(SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_gltf_snapshot(snapshot)),
        GltfMutation::SetAsset(SetAsset { asset }) => format!("set-asset asset={}", enc_asset(asset)),

        GltfMutation::InsertScene(InsertScene { index, scene }) => format!("insert-scene index={index} scene={}", enc_scene(scene)),
        GltfMutation::RemoveScene(RemoveScene { index }) => format!("remove-scene index={index}"),
        GltfMutation::SetScene(SetScene { index, scene }) => format!("set-scene index={index} scene={}", enc_scene(scene)),

        GltfMutation::InsertNode(InsertNode { index, node }) => format!("insert-node index={index} node={}", enc_node(node)),
        GltfMutation::RemoveNode(RemoveNode { index }) => format!("remove-node index={index}"),
        GltfMutation::SetNode(SetNode { index, node }) => format!("set-node index={index} node={}", enc_node(node)),
        GltfMutation::TransformNode(TransformNode { index, matrix, translation, rotation, scale }) => {
            format!("transform-node index={index} matrix={} translation={} rotation={} scale={}", enc_optional_array(*matrix), enc_optional_array(*translation), enc_optional_array(*rotation), enc_optional_array(*scale))
        }
        GltfMutation::ReparentNode(ReparentNode { index, parent, scene, position }) => format!("reparent-node index={index} parent={} scene={} position={position}", enc_optional_index(*parent), enc_optional_index(*scene)),
        GltfMutation::BindNodeMesh(BindNodeMesh { index, mesh }) => format!("bind-node-mesh index={index} mesh={}", enc_optional_index(*mesh)),

        GltfMutation::InsertMesh(InsertMesh { index, mesh }) => format!("insert-mesh index={index} mesh={}", enc_mesh(mesh)),
        GltfMutation::RemoveMesh(RemoveMesh { index }) => format!("remove-mesh index={index}"),
        GltfMutation::SetMesh(SetMesh { index, mesh }) => format!("set-mesh index={index} mesh={}", enc_mesh(mesh)),

        GltfMutation::InsertAccessor(InsertAccessor { index, accessor }) => format!("insert-accessor index={index} accessor={}", enc_accessor(accessor)),
        GltfMutation::RemoveAccessor(RemoveAccessor { index }) => format!("remove-accessor index={index}"),
        GltfMutation::SetAccessor(SetAccessor { index, accessor }) => format!("set-accessor index={index} accessor={}", enc_accessor(accessor)),

        GltfMutation::InsertMaterial(InsertMaterial { index, material }) => format!("insert-material index={index} material={}", enc_material(material)),
        GltfMutation::RemoveMaterial(RemoveMaterial { index }) => format!("remove-material index={index}"),
        GltfMutation::SetMaterial(SetMaterial { index, material }) => format!("set-material index={index} material={}", enc_material(material)),
        GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh, primitive, material }) => format!("bind-primitive-material mesh={mesh} primitive={primitive} material={}", enc_optional_index(*material)),

        GltfMutation::InsertBuffer(InsertBuffer { index, buffer, bytes }) => format!("insert-buffer index={index} buffer={} bytes={}", enc_buffer(buffer), enc_bytes(bytes)),
        GltfMutation::RemoveBuffer(RemoveBuffer { index }) => format!("remove-buffer index={index}"),
        GltfMutation::SetBuffer(SetBuffer { index, buffer, bytes }) => format!("set-buffer index={index} buffer={} bytes={}", enc_buffer(buffer), enc_bytes(bytes)),

        GltfMutation::InsertAnimation(InsertAnimation { index, animation }) => format!("insert-animation index={index} animation={}", enc_animation(animation)),
        GltfMutation::RemoveAnimation(RemoveAnimation { index }) => format!("remove-animation index={index}"),
        GltfMutation::SetAnimation(SetAnimation { index, animation }) => format!("set-animation index={index} animation={}", enc_animation(animation)),
    }
}

fn parse_gltf_mutation(line: &str) -> Result<GltfMutation, String> {
    if line == "no-mutation" {
        return Ok(GltfMutation::NoMutation(NoMutation {}));
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("gltf mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("gltf mutation: missing arg '{k}' for '{keyword}'"));
    let idx = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(GltfMutation::SetSnapshot(SetSnapshot { snapshot: dec_gltf_snapshot(arg("snapshot")?)? })),
        "set-asset" => Ok(GltfMutation::SetAsset(SetAsset { asset: dec_asset(arg("asset")?)? })),

        "insert-scene" => Ok(GltfMutation::InsertScene(InsertScene { index: idx("index")?, scene: dec_scene(arg("scene")?)? })),
        "remove-scene" => Ok(GltfMutation::RemoveScene(RemoveScene { index: idx("index")? })),
        "set-scene" => Ok(GltfMutation::SetScene(SetScene { index: idx("index")?, scene: dec_scene(arg("scene")?)? })),

        "insert-node" => Ok(GltfMutation::InsertNode(InsertNode { index: idx("index")?, node: dec_node(arg("node")?)? })),
        "remove-node" => Ok(GltfMutation::RemoveNode(RemoveNode { index: idx("index")? })),
        "set-node" => Ok(GltfMutation::SetNode(SetNode { index: idx("index")?, node: dec_node(arg("node")?)? })),
        "transform-node" => Ok(GltfMutation::TransformNode(TransformNode {
            index: idx("index")?,
            matrix: dec_optional_array(arg("matrix")?)?,
            translation: dec_optional_array(arg("translation")?)?,
            rotation: dec_optional_array(arg("rotation")?)?,
            scale: dec_optional_array(arg("scale")?)?,
        })),
        "reparent-node" => Ok(GltfMutation::ReparentNode(ReparentNode { index: idx("index")?, parent: dec_optional_index(arg("parent")?)?, scene: dec_optional_index(arg("scene")?)?, position: idx("position")? })),
        "bind-node-mesh" => Ok(GltfMutation::BindNodeMesh(BindNodeMesh { index: idx("index")?, mesh: dec_optional_index(arg("mesh")?)? })),

        "insert-mesh" => Ok(GltfMutation::InsertMesh(InsertMesh { index: idx("index")?, mesh: dec_mesh(arg("mesh")?)? })),
        "remove-mesh" => Ok(GltfMutation::RemoveMesh(RemoveMesh { index: idx("index")? })),
        "set-mesh" => Ok(GltfMutation::SetMesh(SetMesh { index: idx("index")?, mesh: dec_mesh(arg("mesh")?)? })),

        "insert-accessor" => Ok(GltfMutation::InsertAccessor(InsertAccessor { index: idx("index")?, accessor: dec_accessor(arg("accessor")?)? })),
        "remove-accessor" => Ok(GltfMutation::RemoveAccessor(RemoveAccessor { index: idx("index")? })),
        "set-accessor" => Ok(GltfMutation::SetAccessor(SetAccessor { index: idx("index")?, accessor: dec_accessor(arg("accessor")?)? })),

        "insert-material" => Ok(GltfMutation::InsertMaterial(InsertMaterial { index: idx("index")?, material: dec_material(arg("material")?)? })),
        "remove-material" => Ok(GltfMutation::RemoveMaterial(RemoveMaterial { index: idx("index")? })),
        "set-material" => Ok(GltfMutation::SetMaterial(SetMaterial { index: idx("index")?, material: dec_material(arg("material")?)? })),
        "bind-primitive-material" => Ok(GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: idx("mesh")?, primitive: idx("primitive")?, material: dec_optional_index(arg("material")?)? })),

        "insert-buffer" => Ok(GltfMutation::InsertBuffer(InsertBuffer { index: idx("index")?, buffer: dec_buffer(arg("buffer")?)?, bytes: dec_bytes(arg("bytes")?)? })),
        "remove-buffer" => Ok(GltfMutation::RemoveBuffer(RemoveBuffer { index: idx("index")? })),
        "set-buffer" => Ok(GltfMutation::SetBuffer(SetBuffer { index: idx("index")?, buffer: dec_buffer(arg("buffer")?)?, bytes: dec_bytes(arg("bytes")?)? })),

        "insert-animation" => Ok(GltfMutation::InsertAnimation(InsertAnimation { index: idx("index")?, animation: dec_animation(arg("animation")?)? })),
        "remove-animation" => Ok(GltfMutation::RemoveAnimation(RemoveAnimation { index: idx("index")? })),
        "set-animation" => Ok(GltfMutation::SetAnimation(SetAnimation { index: idx("index")?, animation: dec_animation(arg("animation")?)? })),

        other => Err(format!("gltf mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for GltfMutation {
    fn print_op(&self) -> String {
        print_gltf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_gltf_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}
