//! 📸️ Native IDML persisted snapshot with strict package admission.
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind, NativeDecodeControl};
use crate::schema::{IdmlPart, PackageWire};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdmlSnapshot {
    pub dom_version: String,
    pub parts: Vec<IdmlPart>,
}

impl IdmlSnapshot {
    /// 📥️ Captures the native intrinsic IDML value without adopting ZIP artifact identity.
    pub fn from_package(package: crate::IdmlPackage) -> Self {
        Self { dom_version: package.dom_version.clone(), parts: package.parts().map(|(path, bytes)| IdmlPart { path: path.into(), bytes: bytes.into() }).collect() }
    }
    /// 🛂️ Re-admits every owned path, XML member and package invariant before external output.
    pub fn package(&self) -> Result<crate::IdmlPackage, ValueError> {
        let package = crate::IdmlPackage::from_parts(self.parts.iter().map(|part| (part.path.clone(), part.bytes.clone())).collect()).map_err(invalid)?;
        if package.dom_version != self.dom_version { return Err(invalid("stdio.idml.dom-version-mismatch")); }
        Ok(package)
    }
    pub(crate) fn admit(wire: PackageWire) -> Result<Self, ValueError> {
        let snapshot = Self { dom_version: wire.dom_version, parts: wire.parts };
        snapshot.package()?;
        Ok(snapshot)
    }
}

pub(crate) fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }

impl FromValue for IdmlSnapshot {
    fn from_value(value: DslValue) -> Result<Self, ValueError> { Self::admit(PackageWire::from_value(value)?) }
    fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.checkpoint()?;
        let wire = PackageWire::from_value_controlled(value, control)?;
        let snapshot = Self::admit(wire)?;
        control.checkpoint()?;
        Ok(snapshot)
    }
}

/// 📽️ A new unpublished post copies genuine owned fields through the existing bounded projection cursor.
impl semio_framework_value::snapshot_projection::SnapshotProjection for IdmlSnapshot{
 type Output=Self;
 type Projection=semio_framework_value::snapshot_projection::OwnedFieldProjection<Self>;
 fn projection_birth_demand()->semio_framework_value::RetirementDemand{Self::Projection::birth_demand()}
 fn begin_projection(grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Self::Projection,semio_framework_value::retained_clone::RetainedCloneProgress),ValueError>{Self::Projection::begin(grant)}
}
