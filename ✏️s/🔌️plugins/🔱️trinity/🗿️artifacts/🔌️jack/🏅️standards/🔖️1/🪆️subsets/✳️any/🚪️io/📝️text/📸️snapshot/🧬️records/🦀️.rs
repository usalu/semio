//! 🔌️ Literal native Jack parent record: the manifest is named by `manifest_id`, never carried inline.
use crate::standards::v1::subsets::any::schema::graph::Camera;
use crate::standards::v1::subsets::any::schema::content::JackContentChild;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_value::{DecodedValue, NativeEncodeControl, ValueError};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
#[derive(semio_framework_dsl_record_derive::DslRecord,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
#[dsl(id = "trinity.jack", layout = "lines")]
pub(crate) struct JackPackRecord {
    pub(crate) schema: String,
    pub(crate) name: String,
    pub(crate) manifest_id: String,
    pub(crate) camera: Camera,
    pub(crate) content: ContentRecord,
    pub(crate) root_node_id: Option<String>,
    pub(crate) query: String,
}
#[derive(semio_framework_dsl_record_derive::DslRecord,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
pub(crate) struct ContentRecord {
    pub(crate) child_id: String,
    pub(crate) artifact_id: String,
    pub(crate) artifact_kind: String,
    pub(crate) standard: String,
    pub(crate) subset: String,
}
trait Copies {
    fn text(&mut self, value: &str) -> Result<String, ValueError>;
    fn scalar(&mut self, bytes: usize) -> Result<(), ValueError>;
    fn step(&mut self) -> Result<(), ValueError>;
}
struct Ordinary;
impl Copies for Ordinary {
    fn text(&mut self, value: &str) -> Result<String, ValueError> {
        Ok(value.into())
    }
    fn scalar(&mut self, _: usize) -> Result<(), ValueError> {
        Ok(())
    }
    fn step(&mut self) -> Result<(), ValueError> {
        Ok(())
    }
}
impl Copies for NativeDecodeControl<'_> {
    fn text(&mut self, value: &str) -> Result<String, ValueError> {
        self.copy_text(value)
    }
    fn scalar(&mut self, bytes: usize) -> Result<(), ValueError> {
        self.charge(bytes)
    }
    fn step(&mut self) -> Result<(), ValueError> {
        NativeDecodeControl::step(self)
    }
}
impl Copies for NativeEncodeControl<'_> {
    fn text(&mut self, value: &str) -> Result<String, ValueError> {
        self.copy_text(value)
    }
    fn scalar(&mut self, bytes: usize) -> Result<(), ValueError> {
        self.charge(bytes)
    }
    fn step(&mut self) -> Result<(), ValueError> {
        NativeEncodeControl::step(self)
    }
}
fn optional(value: &Option<String>, c: &mut impl Copies) -> Result<Option<String>, ValueError> {
    value.as_deref().map(|v| c.text(v)).transpose()
}
impl JackPackRecord {
    pub(crate) fn admit_rows(&self, maximum: usize, c: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(0)?;
            if maximum < 3 {
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Jack native row limit"));
            }
            c.step()?;
            c.checkpoint()
        })
    }

    fn copy(value: &JackSnapshot, c: &mut impl Copies) -> Result<Self, ValueError> {
        c.scalar(size_of::<Self>())?;
        c.scalar(size_of::<Camera>())?;
        let child = &value.content;
        let target = &child.target;
        Ok(Self {
            schema: c.text(&value.schema)?,
            name: c.text(&value.name)?,
            manifest_id: c.text(&value.manifest_id)?,
            camera: Camera { x: value.camera.x, y: value.camera.y, zoom: value.camera.zoom },
            content: ContentRecord {
                child_id: c.text(&child.child_id)?,
                artifact_id: c.text(&target.artifact_id)?,
                artifact_kind: c.text(&target.dialect.artifact_kind)?,
                standard: c.text(&target.dialect.standard)?,
                subset: c.text(&target.dialect.subset)?,
            },
            root_node_id: optional(&value.root_node_id, c)?,
            query: c.text(&value.query)?,
        })
    }
    pub(crate) fn from_snapshot(value: &JackSnapshot) -> Self {
        Self::copy(value, &mut Ordinary).expect("owned native Jack records")
    }
    pub(crate) fn from_snapshot_controlled(value: &JackSnapshot, c: &mut NativeEncodeControl<'_>) -> Result<Self, ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(0)?;
            Self::copy(value, c)
        })
    }
    fn materialize(self, c: &mut impl Copies) -> Result<JackSnapshot, ValueError> {
        c.scalar(size_of::<JackSnapshot>())?;
        let value = DecodedValue::new(
            JackSnapshot {
                schema: self.schema,
                name: self.name,
                manifest_id: self.manifest_id,
                camera: self.camera,
                content: JackContentChild::new(
                    self.content.child_id,
                    semio_framework_artifact_reference::ArtifactRef { artifact_id: self.content.artifact_id, dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: self.content.artifact_kind, standard: self.content.standard, subset: self.content.subset } },
                ),
                root_node_id: self.root_node_id,
                query: self.query,
            },
            crate::standards::v1::subsets::any::io::sqlite::snapshot::retire_snapshot,
        );
        crate::standards::v1::subsets::any::io::sqlite::snapshot::validate(value.get())?;
        c.step()?;
        Ok(value.take())
    }
    pub(crate) fn into_snapshot(self) -> Result<JackSnapshot, ValueError> {
        self.materialize(&mut Ordinary)
    }
    pub(crate) fn into_snapshot_controlled(self, c: &mut NativeDecodeControl<'_>) -> Result<JackSnapshot, ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(0)?;
            self.materialize(c)
        })
    }
}
