//! 🧷️ Embedded child slot: unbound refusal, idempotent rebinding and refused replacement agree with a hand-written truth table, and the vector scene edits what it reads.

use super::*;
use semio_framework_value::{FromValue, ToValue};

struct StubCodec {
    schema: &'static str,
}

impl EmbeddedChildCodecV1 for StubCodec {
    fn validate(&self, _content: &DslValue, _control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        Ok(())
    }
    fn lower(&self, content: &DslValue) -> Result<VectorScene, ValueError> {
        VectorScene::from_value(content.clone())
    }
    fn lift(&self, scene: &VectorScene) -> Result<DslValue, ValueError> {
        Ok(scene.to_value())
    }
    fn sqlite_schema(&self) -> &'static str {
        self.schema
    }
    fn sqlite_owns_table(&self, table: &str) -> bool {
        table.starts_with("stub_")
    }
    fn sqlite_project(&self, _content: &DslValue, _writer: &mut RowWriter<'_, '_>) -> Result<(), ValueError> {
        Ok(())
    }
    fn sqlite_reconstruct(&self, _database: &SqliteDatabase, _control: &mut SqliteSnapshotControl<'_>, _declared_schema: &str) -> Result<DslValue, ValueError> {
        Ok(DslValue::Null)
    }
    fn import_formats(&self) -> &'static [&'static str] {
        &["stub"]
    }
    fn export_formats(&self) -> &'static [&'static str] {
        &["stub"]
    }
    fn import(&self, format: &str, bytes: &[u8]) -> Result<DslValue, String> {
        if format == "stub" { Ok(DslValue::String(String::from_utf8_lossy(bytes).into_owned())) } else { Err(format!("unknown format {format}")) }
    }
    fn export(&self, format: &str, content: &DslValue) -> Result<Vec<u8>, String> {
        match (format, content) {
            ("stub", DslValue::String(text)) => Ok(text.clone().into_bytes()),
            _ => Err(format!("unknown format {format}")),
        }
    }
}

static FIRST: StubCodec = StubCodec { schema: "-- first" };
static OTHER: StubCodec = StubCodec { schema: "-- other" };

fn point(x: f64, y: f64) -> VectorPoint2 {
    VectorPoint2 { x, y }
}

fn scene() -> VectorScene {
    let text = |value: &str| VectorNode::Text { value: value.into(), at: point(1.0, 2.0), style: None };
    VectorScene {
        schema: "test.scene".into(),
        canvas: VectorCanvas { width: 10.0, height: 20.0, background: Some(VectorRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: vec![VectorStyle { name: "wall".into(), stroke: Some(VectorRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }), stroke_width: Some(2.0), ..VectorStyle::default() }],
        layers: vec![
            VectorLayer { id: "a".into(), name: "A".into(), visible: true, root: VectorNode::Group { transform: VectorTransform::identity(), children: vec![text("north"), VectorNode::Path { segments: vec![VectorSegment::MoveTo { to: point(0.0, 0.0) }, VectorSegment::Close], style: Some("wall".into()) }, text("south")] } },
            VectorLayer { id: "b".into(), name: "B".into(), visible: false, root: text("east") },
        ],
    }
}

#[test]
fn an_unbound_slot_refuses_to_hand_out_a_codec() {
    let slot = EmbeddedChildSlotV1::new("test.unbound");
    assert_eq!(slot.codec().err(), Some(EmbeddedChildSlotError::Unbound { id: "test.unbound" }));
    assert_eq!(slot.codec().err().map(|error| error.to_string()), Some("embedded child slot test.unbound has no bound codec".to_string()));
}

#[test]
fn a_slot_returns_exactly_the_codec_bound_to_it_and_tolerates_identical_rebinding() {
    let slot = EmbeddedChildSlotV1::new("test.bound");
    assert!(slot.bind(&FIRST).is_ok());
    assert!(slot.bind(&FIRST).is_ok());
    let codec = slot.codec().expect("bound");
    assert_eq!(codec.sqlite_schema(), "-- first");
    assert!(codec.sqlite_owns_table("stub_row"));
    assert!(!codec.sqlite_owns_table("layout_page"));
    assert_eq!(codec.import("stub", b"x").ok(), Some(DslValue::String("x".into())));
    assert_eq!(codec.export("stub", &DslValue::String("y".into())).ok(), Some(b"y".to_vec()));
    assert!(codec.import("other", b"x").is_err());
}

#[test]
fn a_bound_slot_refuses_a_different_codec_and_slots_stay_independent() {
    let left = EmbeddedChildSlotV1::new("test.left");
    let right = EmbeddedChildSlotV1::new("test.right");
    assert!(left.bind(&FIRST).is_ok());
    let refusal = left.bind(&OTHER).err().expect("refused");
    assert_eq!(refusal, EmbeddedChildSlotError::Rebound { id: "test.left" });
    assert_eq!(refusal.to_string(), "embedded child slot test.left is already bound to a different codec");
    assert_eq!(right.codec().err(), Some(EmbeddedChildSlotError::Unbound { id: "test.right" }));
    assert!(right.bind(&OTHER).is_ok());
    assert_eq!(left.codec().expect("left").sqlite_schema(), "-- first");
    assert_eq!(right.codec().expect("right").sqlite_schema(), "-- other");
    assert_eq!(left.id(), "test.left");
}

#[test]
fn lifting_what_was_lowered_returns_the_same_scene() {
    let original = scene();
    let content = FIRST.lift(&original).expect("lift");
    assert_eq!(FIRST.lower(&content).expect("lower"), original);
}

#[test]
fn texts_run_depth_first_over_every_layer_and_set_text_follows_the_same_order() {
    let mut value = scene();
    assert_eq!(value.texts(), vec!["north", "south", "east"]);
    assert!(value.set_text(1, "SOUTH"));
    assert!(value.set_text(2, "EAST"));
    assert_eq!(value.texts(), vec!["north", "SOUTH", "EAST"]);
    assert!(!value.set_text(3, "none"));
    assert_eq!(value.texts(), vec!["north", "SOUTH", "EAST"]);
}
