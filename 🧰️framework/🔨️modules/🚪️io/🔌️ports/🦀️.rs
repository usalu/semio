//! 🔌️ Ports through which a parent owner carries one child's inline content without importing the child's owner.
//!
//! The Rust twin of `🟦️.ts` (`embeddedChildSlotV1`): the parent declares one slot, the composition root that knows both
//! owners binds exactly one codec to it, and the parent only ever speaks to the codec contract and to the neutral
//! [`VectorScene`] that content lowers to and lifts from.
//!
//! @see 🧷️ `🧪️tests/🦀️.rs` for the laws: an unbound slot refuses, rebinding the same codec is idempotent, binding another codec is refused.

use semio_framework_io_sqlite_snapshot::artifact::RowWriter;
use semio_framework_io_sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl};
use semio_framework_value::{DslValue, NativeDecodeControl, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::OnceLock;

//#region 🔖️Codec
/// 🧱️ The validate, lower, lift, SQLite component and external format facets of one child content type, supplied by the composition that knows the child's owner.
pub trait EmbeddedChildCodecV1: Send + Sync {
    /// ✅️ Admits `content` as a well-formed child under the caller's decode budget and cancellation, or refuses it.
    fn validate(&self, content: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError>;
    /// 🔽️ Reads `content` as the neutral vector scene the parent renders, exports and edits.
    fn lower(&self, content: &DslValue) -> Result<VectorScene, ValueError>;
    /// 🔼️ Writes a neutral vector scene back as child content.
    fn lift(&self, scene: &VectorScene) -> Result<DslValue, ValueError>;
    /// 🗄️ The DDL the child contributes to the parent's SQLite schema.
    fn sqlite_schema(&self) -> &'static str;
    /// 🏷️ Whether `table` belongs to the child's DDL.
    fn sqlite_owns_table(&self, table: &str) -> bool;
    /// 📤️ Projects `content` into the child's tables of the parent's database.
    fn sqlite_project(&self, content: &DslValue, writer: &mut RowWriter<'_, '_>) -> Result<(), ValueError>;
    /// 📥️ Reconstructs the content from the child's tables of the parent's database; `declared_schema` is the parent's composed DDL.
    fn sqlite_reconstruct(&self, database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>, declared_schema: &str) -> Result<DslValue, ValueError>;
    /// 📥️ The external formats `import` reads, by stable lowercase name.
    fn import_formats(&self) -> &'static [&'static str];
    /// 📤️ The external formats `export` writes, by stable lowercase name.
    fn export_formats(&self) -> &'static [&'static str];
    /// 📥️ Decodes one external document into child content.
    fn import(&self, format: &str, bytes: &[u8]) -> Result<DslValue, String>;
    /// 📤️ Encodes child content as one external document.
    fn export(&self, format: &str, content: &DslValue) -> Result<Vec<u8>, String>;
}
//#endregion 🔖️Codec

//#region 🔖️Slot
/// ⚠️ Why a slot refused to hand out or accept a codec.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EmbeddedChildSlotError {
    Unbound { id: &'static str },
    Rebound { id: &'static str },
}

impl std::fmt::Display for EmbeddedChildSlotError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unbound { id } => write!(formatter, "embedded child slot {id} has no bound codec"),
            Self::Rebound { id } => write!(formatter, "embedded child slot {id} is already bound to a different codec"),
        }
    }
}

impl std::error::Error for EmbeddedChildSlotError {}

/// 🧩️ A parent-declared position that exactly one child codec fills, bound once by the composition root.
pub struct EmbeddedChildSlotV1 {
    id: &'static str,
    bound: OnceLock<&'static dyn EmbeddedChildCodecV1>,
}

impl EmbeddedChildSlotV1 {
    /// 🧩️ Declares an unbound slot.
    pub const fn new(id: &'static str) -> Self {
        Self { id, bound: OnceLock::new() }
    }

    /// 🪪️ The slot identity.
    pub const fn id(&self) -> &'static str {
        self.id
    }

    /// 🔗️ Binds `codec`; binding the same codec again is idempotent and binding a different codec is refused.
    pub fn bind(&self, codec: &'static dyn EmbeddedChildCodecV1) -> Result<(), EmbeddedChildSlotError> {
        let bound = *self.bound.get_or_init(|| codec);
        if std::ptr::eq(bound as *const dyn EmbeddedChildCodecV1 as *const (), codec as *const dyn EmbeddedChildCodecV1 as *const ()) {
            Ok(())
        } else {
            Err(EmbeddedChildSlotError::Rebound { id: self.id })
        }
    }

    /// 🔎️ The bound codec, or a refusal when the composition root has not bound one.
    pub fn codec(&self) -> Result<&'static dyn EmbeddedChildCodecV1, EmbeddedChildSlotError> {
        self.bound.get().copied().ok_or(EmbeddedChildSlotError::Unbound { id: self.id })
    }
}
//#endregion 🔖️Slot

//#region 🔖️Scene
/// 📍️ A point on the vector plane.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorPoint2 {
    pub x: f64,
    pub y: f64,
}

/// 📍️ A point in space, used by transforms.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorPoint3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🧭️ A rotation as a named quaternion; the default is the identity rotation.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Default for VectorQuaternion {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}

/// 🎨️ A color with straight-alpha float channels.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorRgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// 🧭️ Translation, rotation and scale of a group.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorTransform {
    pub translation: VectorPoint3,
    pub rotation: VectorQuaternion,
    pub scale: VectorPoint3,
}

impl VectorTransform {
    /// 🧭️ Zero translation, identity rotation, unit scale.
    pub fn identity() -> Self {
        Self { translation: VectorPoint3::default(), rotation: VectorQuaternion::default(), scale: VectorPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
    }
}

impl Default for VectorTransform {
    fn default() -> Self {
        Self::identity()
    }
}

/// ✏️ One path command in SVG order.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum VectorSegment {
    MoveTo { to: VectorPoint2 },
    LineTo { to: VectorPoint2 },
    CubicTo { c1: VectorPoint2, c2: VectorPoint2, to: VectorPoint2 },
    QuadTo { c: VectorPoint2, to: VectorPoint2 },
    ArcTo { rx: f64, ry: f64, x_rotation: f64, large_arc: bool, sweep: bool, to: VectorPoint2 },
    Close,
}

/// 🖍️ One node of the recursive scene graph; `style` names an entry of [`VectorScene::styles`].
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum VectorNode {
    Path {
        segments: Vec<VectorSegment>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Text {
        value: String,
        at: VectorPoint2,
        #[value(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Group {
        transform: VectorTransform,
        #[value(default)]
        children: Vec<VectorNode>,
    },
    Image { at: VectorPoint2, width: f64, height: f64, mime: String, bytes: Vec<u8> },
}

impl Default for VectorNode {
    fn default() -> Self {
        Self::Group { transform: VectorTransform::identity(), children: Vec::new() }
    }
}

/// 🎨️ A named presentation style.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorStyle {
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<VectorRgba>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<VectorRgba>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
}

/// 🗂️ One ordered layer.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorLayer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub root: VectorNode,
}

/// 🖼️ The viewport and backdrop of a scene.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorCanvas {
    pub width: f64,
    pub height: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<VectorRgba>,
}

/// 🎬️ The neutral vector scene a child's content lowers to and lifts from: the child's schema id, canvas, name-keyed styles and ordered layers.
/// An empty `schema` asks the codec to lift with its own schema id.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct VectorScene {
    #[value(default)]
    pub schema: String,
    pub canvas: VectorCanvas,
    #[value(default)]
    pub styles: Vec<VectorStyle>,
    #[value(default)]
    pub layers: Vec<VectorLayer>,
}

impl VectorScene {
    /// 🏷️ Every text value in depth-first order over the layers, hidden layers included.
    pub fn texts(&self) -> Vec<&str> {
        fn collect<'a>(node: &'a VectorNode, out: &mut Vec<&'a str>) {
            match node {
                VectorNode::Text { value, .. } => out.push(value.as_str()),
                VectorNode::Group { children, .. } => children.iter().for_each(|child| collect(child, out)),
                VectorNode::Path { .. } | VectorNode::Image { .. } => {}
            }
        }
        let mut out = Vec::new();
        self.layers.iter().for_each(|layer| collect(&layer.root, &mut out));
        out
    }

    /// ✏️ Replaces the text at `index` of [`VectorScene::texts`]; `false` when the scene has no such text.
    pub fn set_text(&mut self, index: u32, text: &str) -> bool {
        fn replace(node: &mut VectorNode, index: &mut u32, text: &str) -> bool {
            match node {
                VectorNode::Text { value, .. } => {
                    if *index == 0 {
                        *value = text.to_string();
                        return true;
                    }
                    *index -= 1;
                    false
                }
                VectorNode::Group { children, .. } => children.iter_mut().any(|child| replace(child, index, text)),
                VectorNode::Path { .. } | VectorNode::Image { .. } => false,
            }
        }
        let mut cursor = index;
        self.layers.iter_mut().any(|layer| replace(&mut layer.root, &mut cursor, text))
    }
}
//#endregion 🔖️Scene

//#region 🪝️Testkit
/// 🪝️ The codec a parent binds in its own unit tests: the content is the vector scene itself, so a test authors content with [`testkit::scene_content`] and reads it back with the scene helpers.
#[cfg(feature = "testkit")]
pub mod testkit {
    use super::*;
    use semio_framework_io_sqlite_snapshot::artifact::Cell;
    use semio_framework_value::{FromValue, ToValue, ValueRefusalKind};

    /// 🪝️ Content equals scene; SQLite carries a marker row only, and there are no external formats.
    pub struct VectorSceneCodec;

    /// 🔗️ The one instance to bind.
    pub static VECTOR_SCENE_CODEC: VectorSceneCodec = VectorSceneCodec;

    /// 🔗️ Binds the testkit codec to a parent's slot; idempotent, so every test of a binary may call it.
    pub fn bind(slot: &EmbeddedChildSlotV1) {
        slot.bind(&VECTOR_SCENE_CODEC).expect("the testkit codec binds");
    }

    /// ✍️ Authors child content from a scene.
    pub fn scene_content(scene: &VectorScene) -> DslValue {
        scene.to_value()
    }

    impl EmbeddedChildCodecV1 for VectorSceneCodec {
        fn validate(&self, content: &DslValue, _control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
            VectorScene::from_value(content.clone()).map(|_| ())
        }
        fn lower(&self, content: &DslValue) -> Result<VectorScene, ValueError> {
            VectorScene::from_value(content.clone())
        }
        fn lift(&self, scene: &VectorScene) -> Result<DslValue, ValueError> {
            Ok(scene.to_value())
        }
        fn sqlite_schema(&self) -> &'static str {
            "CREATE TABLE semio_drawing_document (id INTEGER PRIMARY KEY, marker INTEGER NOT NULL);"
        }
        fn sqlite_owns_table(&self, table: &str) -> bool {
            table.starts_with("semio_drawing_")
        }
        fn sqlite_project(&self, _content: &DslValue, writer: &mut RowWriter<'_, '_>) -> Result<(), ValueError> {
            writer.insert_key("semio_drawing_document", 1, &[Cell::Integer(1)]).map(|_| ())
        }
        fn sqlite_reconstruct(&self, _database: &SqliteDatabase, _control: &mut SqliteSnapshotControl<'_>, _declared_schema: &str) -> Result<DslValue, ValueError> {
            Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "the testkit codec does not reconstruct SQLite content"))
        }
        fn import_formats(&self) -> &'static [&'static str] {
            &[]
        }
        fn export_formats(&self) -> &'static [&'static str] {
            &[]
        }
        fn import(&self, format: &str, _bytes: &[u8]) -> Result<DslValue, String> {
            Err(format!("the testkit codec cannot import {format}"))
        }
        fn export(&self, format: &str, _content: &DslValue) -> Result<Vec<u8>, String> {
            Err(format!("the testkit codec cannot export {format}"))
        }
    }
}
//#endregion 🪝️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
