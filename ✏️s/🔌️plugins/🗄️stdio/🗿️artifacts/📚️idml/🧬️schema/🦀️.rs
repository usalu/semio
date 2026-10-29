//! 🧬️ Owned IDML intrinsic package values retain ordered paths and payloads.
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdmlPart {
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct PackageWire {
    dom_version: String,
    parts: Vec<IdmlPart>,
}

impl ToValue for crate::IdmlPackage {
    fn to_value(&self) -> DslValue {
        PackageWire { dom_version: self.dom_version.clone(), parts: self.parts().map(|(path, bytes)| IdmlPart { path: path.into(), bytes: bytes.into() }).collect() }.to_value()
    }
}

impl FromValue for crate::IdmlPackage {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let wire = PackageWire::from_value(value)?;
        let package = Self::from_parts(wire.parts.into_iter().map(|part| (part.path, part.bytes)).collect()).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error))?;
        if package.dom_version != wire.dom_version { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "stdio.idml.dom-version-mismatch")); }
        Ok(package)
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.checkpoint()?;
        let wire = PackageWire::from_value_controlled(value, control)?;
        let package = Self::from_parts(wire.parts.into_iter().map(|part| (part.path, part.bytes)).collect()).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error))?;
        if package.dom_version != wire.dom_version { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "stdio.idml.dom-version-mismatch")); }
        control.checkpoint()?;
        Ok(package)
    }
    fn retire_decoded(value: Self) {
        <semio_s_artifact_stdio_zip::ZipSnapshot as FromValue>::retire_decoded(value.into_archive());
    }
}

#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;

pub use semio_s_artifact_stdio_idml_schema as descriptor;
