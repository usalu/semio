//! 🗂️ A funded field partition for metadata read by the retained GLB scene.
use super::{SceneResult, Value};
use semio_framework_value::{RetainedAllocationEffect, retained_clone::RetainedCloneProgress};
use std::mem::size_of;

const PREFIX: usize = 64;
const MEMBER_SWAP_BYTES: usize = 2 * size_of::<(String, Value)>();
const ROOT_KEYS: &[&str] = &["scene", "scenes", "nodes", "meshes", "buffers", "bufferViews", "accessors"];

pub(super) fn field<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.as_object()?.iter().take(PREFIX).find_map(|(name, value)| (name == key).then_some(value))
}

fn field_mut<'a>(value: &'a mut Value, key: &str) -> Option<&'a mut Value> {
    value.as_object_mut()?.iter_mut().take(PREFIX).find_map(|(name, value)| (name == key).then_some(value))
}

fn array_member<'a>(document: &'a mut Value, key: &str, slot: usize) -> Option<&'a mut Value> {
    field_mut(document, key)?.as_array_mut()?.get_mut(slot)
}

#[derive(Clone, Copy, Default, semio_framework_value_derive::RetireOwned)]
enum Group {
    #[default]
    Root,
    Scenes,
    Nodes,
    Meshes,
    Buffers,
    Views,
    Accessors,
    Complete,
}

impl Group {
    fn next(self) -> Self {
        match self {
            Self::Root => Self::Scenes,
            Self::Scenes => Self::Nodes,
            Self::Nodes => Self::Meshes,
            Self::Meshes => Self::Buffers,
            Self::Buffers => Self::Views,
            Self::Views => Self::Accessors,
            Self::Accessors | Self::Complete => Self::Complete,
        }
    }
}

#[derive(Default, semio_framework_value_derive::RetireOwned)]
pub(super) struct MetadataCursor {
    group: Group,
    row: usize,
    part: u8,
    primitive: usize,
    scanned: usize,
    known: usize,
    receipt: [usize; 2],
}

impl MetadataCursor {
    pub(super) fn take_progress(&mut self) -> RetainedCloneProgress {
        let [copied_items, copied_bytes] = std::mem::take(&mut self.receipt);
        RetainedCloneProgress { copied_items, copied_bytes, ..Default::default() }
    }

    pub(super) fn step(&mut self, document: &mut Value, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<bool> {
        self.receipt = [0; 2];
        let grant = effect.grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < MEMBER_SWAP_BYTES
            || effect.remaining_work_units().map_err(|error| error.to_string())? == 0
            || !effect.performed(1).map_err(|error| error.to_string())?
        {
            return Ok(false);
        }
        self.receipt[0] = 1;
        match self.group {
            Group::Root => {
                if self.partition(Some(document), ROOT_KEYS)? { self.next_group(); }
            }
            Group::Scenes => self.flat(document, "scenes", &["nodes"])?,
            Group::Nodes => self.flat(document, "nodes", &["mesh", "children", "matrix", "translation", "rotation", "scale"])?,
            Group::Buffers => self.flat(document, "buffers", &["uri"])?,
            Group::Views => self.flat(document, "bufferViews", &["buffer", "byteOffset", "byteLength", "byteStride"])?,
            Group::Accessors => self.accessor(document)?,
            Group::Meshes => self.mesh(document)?,
            Group::Complete => return Ok(true),
        }
        Ok(matches!(self.group, Group::Complete))
    }

    fn partition(&mut self, record: Option<&mut Value>, known: &[&str]) -> SceneResult<bool> {
        let Some(object) = record.and_then(Value::as_object_mut).filter(|object| object.len() > PREFIX) else {
            return Ok(true);
        };
        let Some((name, _)) = object.member_at(self.scanned) else {
            self.scanned = 0;
            self.known = 0;
            return Ok(true);
        };
        if known.contains(&name) {
            if self.scanned != self.known {
                if !object.swap_members(self.scanned, self.known) { return Err("glb: metadata slot changed".into()); }
                self.receipt[1] = MEMBER_SWAP_BYTES;
            }
            self.known += 1;
        }
        self.scanned += 1;
        if self.known == known.len() {
            self.scanned = 0;
            self.known = 0;
            return Ok(true);
        }
        Ok(false)
    }

    fn next_group(&mut self) {
        self.group = self.group.next();
        self.row = 0;
        self.part = 0;
        self.primitive = 0;
    }

    fn next_row(&mut self) {
        self.row += 1;
        self.part = 0;
        self.primitive = 0;
    }

    fn flat(&mut self, document: &mut Value, kind: &str, keys: &[&str]) -> SceneResult<()> {
        if let Some(record) = array_member(document, kind, self.row) {
            if self.partition(Some(record), keys)? { self.next_row(); }
        } else {
            self.next_group();
        }
        Ok(())
    }

    fn mesh(&mut self, document: &mut Value) -> SceneResult<()> {
        let Some(mesh) = array_member(document, "meshes", self.row) else {
            self.next_group();
            return Ok(());
        };
        if self.part == 0 {
            if self.partition(Some(mesh), &["primitives"])? { self.part = 1; }
            return Ok(());
        }
        let Some(primitive) = array_member(mesh, "primitives", self.primitive) else {
            self.next_row();
            return Ok(());
        };
        if self.part == 1 {
            if self.partition(Some(primitive), &["mode", "attributes", "indices"])? { self.part = 2; }
        } else if self.partition(field_mut(primitive, "attributes"), &["POSITION", "NORMAL", "TEXCOORD_0"])? {
            self.primitive += 1;
            self.part = 1;
        }
        Ok(())
    }

    fn accessor(&mut self, document: &mut Value) -> SceneResult<()> {
        let Some(accessor) = array_member(document, "accessors", self.row) else {
            self.next_group();
            return Ok(());
        };
        if self.part == 0 {
            let keys = &["componentType", "type", "count", "normalized", "bufferView", "byteOffset", "sparse"];
            if self.partition(Some(accessor), keys)? { self.part = 1; }
            return Ok(());
        }
        let Some(sparse) = field_mut(accessor, "sparse") else {
            self.next_row();
            return Ok(());
        };
        match self.part {
            1 => if self.partition(Some(sparse), &["count", "indices", "values"])? { self.part = 2; },
            2 => if self.partition(field_mut(sparse, "indices"), &["bufferView", "byteOffset", "componentType"])? { self.part = 3; },
            _ => if self.partition(field_mut(sparse, "values"), &["bufferView", "byteOffset"])? { self.next_row(); },
        }
        Ok(())
    }
}
