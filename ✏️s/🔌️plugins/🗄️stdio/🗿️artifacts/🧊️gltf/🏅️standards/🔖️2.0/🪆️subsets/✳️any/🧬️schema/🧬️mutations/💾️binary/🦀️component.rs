//! binary rep for stdio.gltf 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");

use crate::artifacts::gltf::schema::diff::{
    gltf_bin_err, read_bin_accessor, read_bin_animation, read_bin_asset, read_bin_blob, read_bin_buffer, read_bin_gltf_snapshot, read_bin_material, read_bin_mesh, read_bin_node, read_bin_option, read_bin_scene, write_bin_accessor,
    write_bin_animation, write_bin_asset, write_bin_blob, write_bin_buffer, write_bin_gltf_snapshot, write_bin_material, write_bin_mesh, write_bin_node, write_bin_option, write_bin_scene,
};
use crate::artifacts::gltf::schema::mutations::*;

/// ⚡️ P2-FG3: real binary op-frame — upgraded from the F6-era `print_op().into_bytes()` text-as-
/// binary shortcut (18 standards, gltf among them, were still on this shortcut per the P2-W0
/// census). Matches `../💾️binary/📡️component.protocol.semio`'s real fixed header exactly:
/// `format u8` (the repo-wide `store::pack_rt::OP_BINARY_FORMAT` convention byte) + `tag u8` (this
/// variant's own ordinal, `GltfMutation`'s declaration order, `NoMutation`=0) — both individually,
/// genuinely protocol-walkable — then one opaque `payload bytes` tail (`§2.5`'s recursive/opaque-
/// tail pattern: the payload itself IS real, fully structured binary on the Rust side via this
/// artifact's own `write_bin_*`/`read_bin_*` value codecs, just not further protocol-walkable past
/// the fixed 2-byte header, `protocol-prim-ref-recursion`).
fn write_bin_array<const N: usize>(writer: &mut dsl::ByteWriter, values: &[f64; N]) {
    for value in values {
        writer.write_f64_le(*value);
    }
}

fn read_bin_array<const N: usize>(reader: &mut dsl::ByteReader<'_>) -> Result<[f64; N], dsl::PackError> {
    let mut values = [0.0; N];
    for value in &mut values {
        *value = reader.read_f64_le()?;
    }
    Ok(values)
}

impl protocol::OpBinary for GltfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        w.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        let tag: u8 = match self {
            GltfMutation::NoMutation(NoMutation {}) => 0,
            GltfMutation::SetSnapshot(SetSnapshot { .. }) => 1,
            GltfMutation::SetAsset(SetAsset { .. }) => 2,
            GltfMutation::InsertScene(InsertScene { .. }) => 3,
            GltfMutation::RemoveScene(RemoveScene { .. }) => 4,
            GltfMutation::SetScene(SetScene { .. }) => 5,
            GltfMutation::InsertNode(InsertNode { .. }) => 6,
            GltfMutation::RemoveNode(RemoveNode { .. }) => 7,
            GltfMutation::SetNode(SetNode { .. }) => 8,
            GltfMutation::InsertMesh(InsertMesh { .. }) => 9,
            GltfMutation::RemoveMesh(RemoveMesh { .. }) => 10,
            GltfMutation::SetMesh(SetMesh { .. }) => 11,
            GltfMutation::InsertAccessor(InsertAccessor { .. }) => 12,
            GltfMutation::RemoveAccessor(RemoveAccessor { .. }) => 13,
            GltfMutation::SetAccessor(SetAccessor { .. }) => 14,
            GltfMutation::InsertMaterial(InsertMaterial { .. }) => 15,
            GltfMutation::RemoveMaterial(RemoveMaterial { .. }) => 16,
            GltfMutation::SetMaterial(SetMaterial { .. }) => 17,
            GltfMutation::InsertBuffer(InsertBuffer { .. }) => 18,
            GltfMutation::RemoveBuffer(RemoveBuffer { .. }) => 19,
            GltfMutation::SetBuffer(SetBuffer { .. }) => 20,
            GltfMutation::InsertAnimation(InsertAnimation { .. }) => 21,
            GltfMutation::RemoveAnimation(RemoveAnimation { .. }) => 22,
            GltfMutation::SetAnimation(SetAnimation { .. }) => 23,
            GltfMutation::TransformNode(TransformNode { .. }) => 24,
            GltfMutation::ReparentNode(ReparentNode { .. }) => 25,
            GltfMutation::BindNodeMesh(BindNodeMesh { .. }) => 26,
            GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { .. }) => 27,
        };
        w.write_u8(tag);
        match self {
            GltfMutation::NoMutation(NoMutation {}) => {}
            GltfMutation::SetSnapshot(SetSnapshot { snapshot }) => write_bin_gltf_snapshot(&mut w, snapshot),
            GltfMutation::SetAsset(SetAsset { asset }) => write_bin_asset(&mut w, asset),
            GltfMutation::InsertScene(InsertScene { index, scene }) => {
                w.write_varint_u64(*index as u64);
                write_bin_scene(&mut w, scene);
            }
            GltfMutation::RemoveScene(RemoveScene { index }) => w.write_varint_u64(*index as u64),
            GltfMutation::SetScene(SetScene { index, scene }) => {
                w.write_varint_u64(*index as u64);
                write_bin_scene(&mut w, scene);
            }
            GltfMutation::InsertNode(InsertNode { index, node }) => {
                w.write_varint_u64(*index as u64);
                write_bin_node(&mut w, node);
            }
            GltfMutation::RemoveNode(RemoveNode { index }) => w.write_varint_u64(*index as u64),
            GltfMutation::SetNode(SetNode { index, node }) => {
                w.write_varint_u64(*index as u64);
                write_bin_node(&mut w, node);
            }
            GltfMutation::TransformNode(TransformNode { index, matrix, translation, rotation, scale }) => {
                w.write_varint_u64(*index as u64);
                write_bin_option(&mut w, matrix, write_bin_array);
                write_bin_option(&mut w, translation, write_bin_array);
                write_bin_option(&mut w, rotation, write_bin_array);
                write_bin_option(&mut w, scale, write_bin_array);
            }
            GltfMutation::ReparentNode(ReparentNode { index, parent, scene, position }) => {
                w.write_varint_u64(*index as u64);
                write_bin_option(&mut w, parent, |w, value| w.write_varint_u64(*value as u64));
                write_bin_option(&mut w, scene, |w, value| w.write_varint_u64(*value as u64));
                w.write_varint_u64(*position as u64);
            }
            GltfMutation::BindNodeMesh(BindNodeMesh { index, mesh }) => {
                w.write_varint_u64(*index as u64);
                write_bin_option(&mut w, mesh, |w, value| w.write_varint_u64(*value as u64));
            }
            GltfMutation::InsertMesh(InsertMesh { index, mesh }) => {
                w.write_varint_u64(*index as u64);
                write_bin_mesh(&mut w, mesh);
            }
            GltfMutation::RemoveMesh(RemoveMesh { index }) => w.write_varint_u64(*index as u64),
            GltfMutation::SetMesh(SetMesh { index, mesh }) => {
                w.write_varint_u64(*index as u64);
                write_bin_mesh(&mut w, mesh);
            }
            GltfMutation::InsertAccessor(InsertAccessor { index, accessor }) => {
                w.write_varint_u64(*index as u64);
                write_bin_accessor(&mut w, accessor);
            }
            GltfMutation::RemoveAccessor(RemoveAccessor { index }) => w.write_varint_u64(*index as u64),
            GltfMutation::SetAccessor(SetAccessor { index, accessor }) => {
                w.write_varint_u64(*index as u64);
                write_bin_accessor(&mut w, accessor);
            }
            GltfMutation::InsertMaterial(InsertMaterial { index, material }) => {
                w.write_varint_u64(*index as u64);
                write_bin_material(&mut w, material);
            }
            GltfMutation::RemoveMaterial(RemoveMaterial { index }) => w.write_varint_u64(*index as u64),
            GltfMutation::SetMaterial(SetMaterial { index, material }) => {
                w.write_varint_u64(*index as u64);
                write_bin_material(&mut w, material);
            }
            GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh, primitive, material }) => {
                w.write_varint_u64(*mesh as u64);
                w.write_varint_u64(*primitive as u64);
                write_bin_option(&mut w, material, |w, value| w.write_varint_u64(*value as u64));
            }
            GltfMutation::InsertBuffer(InsertBuffer { index, buffer, bytes }) => {
                w.write_varint_u64(*index as u64);
                write_bin_buffer(&mut w, buffer);
                write_bin_blob(&mut w, bytes);
            }
            GltfMutation::RemoveBuffer(RemoveBuffer { index }) => w.write_varint_u64(*index as u64),
            GltfMutation::SetBuffer(SetBuffer { index, buffer, bytes }) => {
                w.write_varint_u64(*index as u64);
                write_bin_buffer(&mut w, buffer);
                write_bin_blob(&mut w, bytes);
            }
            GltfMutation::InsertAnimation(InsertAnimation { index, animation }) => {
                w.write_varint_u64(*index as u64);
                write_bin_animation(&mut w, animation);
            }
            GltfMutation::RemoveAnimation(RemoveAnimation { index }) => w.write_varint_u64(*index as u64),
            GltfMutation::SetAnimation(SetAnimation { index, animation }) => {
                w.write_varint_u64(*index as u64);
                write_bin_animation(&mut w, animation);
            }
        }
        Ok(w.into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let format = r.read_u8().map_err(gltf_bin_err)?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "gltf op format", offset: 0, detail: format!("expected format {}, got {format}", store::pack_rt::OP_BINARY_FORMAT) });
        }
        let tag = r.read_u8().map_err(gltf_bin_err)?;
        let idx = |r: &mut dsl::ByteReader<'_>| -> Result<usize, protocol::ProtocolError> { Ok(r.read_varint_u64().map_err(gltf_bin_err)? as usize) };
        let mutation = match tag {
            0 => GltfMutation::NoMutation(NoMutation {}),
            1 => GltfMutation::SetSnapshot(SetSnapshot { snapshot: read_bin_gltf_snapshot(&mut r).map_err(gltf_bin_err)? }),
            2 => GltfMutation::SetAsset(SetAsset { asset: read_bin_asset(&mut r).map_err(gltf_bin_err)? }),
            3 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertScene(InsertScene { index, scene: read_bin_scene(&mut r).map_err(gltf_bin_err)? })
            }
            4 => GltfMutation::RemoveScene(RemoveScene { index: idx(&mut r)? }),
            5 => {
                let index = idx(&mut r)?;
                GltfMutation::SetScene(SetScene { index, scene: read_bin_scene(&mut r).map_err(gltf_bin_err)? })
            }
            6 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertNode(InsertNode { index, node: read_bin_node(&mut r).map_err(gltf_bin_err)? })
            }
            7 => GltfMutation::RemoveNode(RemoveNode { index: idx(&mut r)? }),
            8 => {
                let index = idx(&mut r)?;
                GltfMutation::SetNode(SetNode { index, node: read_bin_node(&mut r).map_err(gltf_bin_err)? })
            }
            9 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertMesh(InsertMesh { index, mesh: read_bin_mesh(&mut r).map_err(gltf_bin_err)? })
            }
            10 => GltfMutation::RemoveMesh(RemoveMesh { index: idx(&mut r)? }),
            11 => {
                let index = idx(&mut r)?;
                GltfMutation::SetMesh(SetMesh { index, mesh: read_bin_mesh(&mut r).map_err(gltf_bin_err)? })
            }
            12 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertAccessor(InsertAccessor { index, accessor: read_bin_accessor(&mut r).map_err(gltf_bin_err)? })
            }
            13 => GltfMutation::RemoveAccessor(RemoveAccessor { index: idx(&mut r)? }),
            14 => {
                let index = idx(&mut r)?;
                GltfMutation::SetAccessor(SetAccessor { index, accessor: read_bin_accessor(&mut r).map_err(gltf_bin_err)? })
            }
            15 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertMaterial(InsertMaterial { index, material: read_bin_material(&mut r).map_err(gltf_bin_err)? })
            }
            16 => GltfMutation::RemoveMaterial(RemoveMaterial { index: idx(&mut r)? }),
            17 => {
                let index = idx(&mut r)?;
                GltfMutation::SetMaterial(SetMaterial { index, material: read_bin_material(&mut r).map_err(gltf_bin_err)? })
            }
            18 => {
                let index = idx(&mut r)?;
                let buffer = read_bin_buffer(&mut r).map_err(gltf_bin_err)?;
                let bytes = read_bin_blob(&mut r).map_err(gltf_bin_err)?;
                GltfMutation::InsertBuffer(InsertBuffer { index, buffer, bytes })
            }
            19 => GltfMutation::RemoveBuffer(RemoveBuffer { index: idx(&mut r)? }),
            20 => {
                let index = idx(&mut r)?;
                let buffer = read_bin_buffer(&mut r).map_err(gltf_bin_err)?;
                let bytes = read_bin_blob(&mut r).map_err(gltf_bin_err)?;
                GltfMutation::SetBuffer(SetBuffer { index, buffer, bytes })
            }
            21 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertAnimation(InsertAnimation { index, animation: read_bin_animation(&mut r).map_err(gltf_bin_err)? })
            }
            22 => GltfMutation::RemoveAnimation(RemoveAnimation { index: idx(&mut r)? }),
            23 => {
                let index = idx(&mut r)?;
                GltfMutation::SetAnimation(SetAnimation { index, animation: read_bin_animation(&mut r).map_err(gltf_bin_err)? })
            }
            24 => GltfMutation::TransformNode(TransformNode {
                index: idx(&mut r)?,
                matrix: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
                translation: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
                rotation: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
                scale: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
            }),
            25 => GltfMutation::ReparentNode(ReparentNode {
                index: idx(&mut r)?,
                parent: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)?,
                scene: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)?,
                position: idx(&mut r)?,
            }),
            26 => GltfMutation::BindNodeMesh(BindNodeMesh { index: idx(&mut r)?, mesh: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)? }),
            27 => GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: idx(&mut r)?, primitive: idx(&mut r)?, material: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)? }),
            other => return Err(protocol::ProtocolError::Malformed { what: "gltf op tag", offset: 0, detail: format!("unknown tag {other}") }),
        };
        if r.remaining() != 0 {
            return Err(protocol::ProtocolError::Malformed { what: "gltf op trailing bytes", offset: (bytes.len() - r.remaining()) as u64, detail: format!("{} trailing bytes", r.remaining()) });
        }
        Ok(mutation)
    }
}
