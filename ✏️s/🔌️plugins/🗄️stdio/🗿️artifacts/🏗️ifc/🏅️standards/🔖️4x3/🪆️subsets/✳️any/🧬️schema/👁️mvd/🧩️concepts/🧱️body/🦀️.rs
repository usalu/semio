//! 🧱️ A heap-free family child borrows only its parent's authenticated source turn and original quote.
use super::native::*;
use super::concept_types::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use crate::catalog::query::{IfcAttributeQueryWork, IfcAttributeQueryStatus, Ifc4x3AttributeMetadata};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcSnapshot, IfcValue};
use std::{mem::size_of, sync::Arc};
mod domain;
mod helpers;
mod clause;
mod work;
pub use work::IfcBodyConceptWork;
pub use domain::{IfcBodyConceptRequest, IfcBodyConceptReport};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BodyPolicy { Tessellation, SweptSolid, AdvancedSweptSolid, SurfaceModel, Clipping, Brep, AdvancedBrep }
semio_framework_value::artifact_retire_leaf!(BodyPolicy);
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
impl BodyPolicy {
    fn representation_type(self) -> &'static str { match self { Self::Tessellation => "Tessellation", Self::SweptSolid => "SweptSolid", Self::AdvancedSweptSolid => "AdvancedSweptSolid", Self::SurfaceModel => "SurfaceModel", Self::Clipping => "Clipping", Self::Brep => "Brep", Self::AdvancedBrep => "AdvancedBrep" } }
    fn concept(self) -> &'static str { match self {
        Self::Tessellation => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Tessellation_Geometry",
        Self::SweptSolid => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SweptSolid_Geometry",
        Self::AdvancedSweptSolid => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_AdvancedSweptSolid_Geometry",
        Self::SurfaceModel => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SurfaceModel_Geometry",
        Self::Clipping => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Clipping_Geometry",
        Self::Brep => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Brep_Geometry",
        Self::AdvancedBrep => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_AdvancedBrep_Geometry",
    } }
    fn source(self) -> IfcConceptSource { IfcConceptSource {
        commit: "610ee3f95db9d0380b1eb8de4dc6e17833531518",
        url: match self {
            Self::Tessellation => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Tessellation_Geometry/content.html",
            Self::SweptSolid => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SweptSolid_Geometry/content.html",
            Self::AdvancedSweptSolid => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_AdvancedSweptSolid_Geometry/content.html",
            Self::SurfaceModel => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SurfaceModel_Geometry/content.html",
            Self::Clipping => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Clipping_Geometry/content.html",
            Self::Brep => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Brep_Geometry/content.html",
            Self::AdvancedBrep => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_AdvancedBrep_Geometry/content.html",
        },
        sha256: match self { Self::Tessellation => "b8f905fdf6d7bcb4afa69d20111600589e52f4bef43043b8a7814e0b5da78d32", Self::SweptSolid => "c571f3ac400b669781c09cc666b5cd55031b6a480333df6ac47d8ebb1f8dd7c0", Self::AdvancedSweptSolid => "c492bb1ef2c769a85c1b4dfd6d4e7caf03614bd03b9b8bf59669d705dd04658b", Self::SurfaceModel => "f4ac42de0851bd18e4a3ca56104a6ec69dd547b5c3a3fff1ddc4a5912239cc95", Self::Clipping => "35ca73abc9904a2e97a0ad5ff0896a1df91285259537ebab269df2cfe7c01210", Self::Brep => "1e1658d668430d01c2f796a3d6ca217ebd89a9f7c18b9a17d9937ff170cfed17", Self::AdvancedBrep => "368bf65e75dcf96e3a6c778fc6ac29be57b8f1bea2f4315e8df0ea68fa4eb5a1" },
    } }
    fn families(self) -> &'static [(&'static str, bool)] {
        match self {
            Self::Tessellation => &[("IFCTESSELLATEDFACESET", false)],
            Self::SweptSolid => &[("IFCEXTRUDEDAREASOLID", false), ("IFCREVOLVEDAREASOLID", false), ("IFCEXTRUDEDAREASOLIDTAPERED", true), ("IFCREVOLVEDAREASOLIDTAPERED", true)],
            Self::AdvancedSweptSolid => &[("IFCSWEPTAREASOLID", false), ("IFCSWEPTDISKSOLID", false)],
            Self::SurfaceModel => &[("IFCTESSELLATEDITEM", false), ("IFCSHELLBASEDSURFACEMODEL", false), ("IFCFACEBASEDSURFACEMODEL", false)],
            Self::Clipping => &[("IFCBOOLEANCLIPPINGRESULT", false)],
            Self::Brep => &[("IFCFACETEDBREP", false)],
            Self::AdvancedBrep => &[("IFCADVANCEDBREP", false), ("IFCFACETEDBREP", false)],
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BodyFamilyResult { Pending, Resolved(bool), Unknown }
pub(super) struct BodyFamilyMembership {
    policy: BodyPolicy,
    member: Membership,
    position: usize,
    family: usize,
    started: bool,
    complete: bool,
    matched: bool,
    excluded: bool,
    unknown: bool,
}
semio_framework_value::artifact_retire_leaf!(BodyFamilyMembership);
impl BodyFamilyMembership {
    pub fn new(policy: BodyPolicy) -> Self {
        Self { policy, member: Membership::default(), position: 0, family: 0, started: false, complete: false, matched: false, excluded: false, unknown: false }
    }
    /// 🧾 The enclosing original-funded clause pays this bound before start, advance or reset.
    pub fn copy_bound(&self) -> usize {
        self.member.copy_bound() + 4 * size_of::<Self>() + size_of::<BodyFamilyResult>()
    }
    /// 📍 The parent supplies an ordinal from its actual pinned prepared-population lookup.
    pub fn start(&mut self, position: usize) {
        self.position = position;
        self.family = 0;
        self.started = true;
        self.complete = false;
        self.matched = false;
        self.excluded = false;
        self.unknown = false;
        self.member.start(position, self.policy.families()[0].0);
    }
    /// 🪜 One primitive advances no more than one existing membership continuation.
    pub fn advance(&mut self, source: &IfcSnapshot) -> bool {
        if self.complete { return true; }
        if !self.started { return false; }
        if self.member.advance(source) {
            match self.member.result() {
                None => self.unknown = true,
                Some(true) if self.policy.families()[self.family].1 => self.excluded = true,
                Some(true) => self.matched = true,
                Some(false) => {}
            }
            self.family += 1;
            if self.family == self.policy.families().len() {
                self.complete = true;
            } else {
                self.member.start(self.position, self.policy.families()[self.family].0);
            }
        }
        self.complete
    }
    pub fn result(&self) -> BodyFamilyResult {
        if !self.complete { BodyFamilyResult::Pending } else if self.unknown { BodyFamilyResult::Unknown } else { BodyFamilyResult::Resolved(self.matched && !self.excluded) }
    }
}
#[cfg(test)]
#[path = "family-tests.rs"]
mod tests;

#[cfg(test)]
#[path = "native-tests.rs"]
mod native_tests;

#[cfg(test)]
#[path = "clause-tests.rs"]
mod clause_tests;

#[cfg(test)]
#[path = "advanced-tests.rs"]
mod advanced_tests;

#[cfg(test)]
#[path = "surface-tests.rs"]
mod surface_tests;

#[cfg(test)]
#[path = "clipping-tests.rs"]
mod clipping_tests;
#[cfg(test)]
#[path = "brep-tests.rs"]
mod brep_tests;
#[cfg(test)]
#[path = "tessellation-tests.rs"]
mod tessellation_tests;
