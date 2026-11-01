//! 🧪️ The IFC4 catalog profile every details layer law runs against.
use super::*;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::catalog::{ifc4_catalog, IFC4_MAXIMUM_ATTRIBUTES};
pub struct DetailsTestProfile;
impl IfcDetailsProfile for DetailsTestProfile {
    type CandidateRules = ();
    const MAXIMUM_ATTRIBUTES: usize = IFC4_MAXIMUM_ATTRIBUTES;
    fn catalog() -> &'static IfcCatalog {
        ifc4_catalog()
    }
}
