//! 📐️ STEP-owned session operations over the general computational geometry authority.
use semio_framework_3d::brep::engine::{Brep, GeometryHandle};
use semio_s_spatial_kernel_semio_session::{BrepModuleError, GeometryOperations, Session};
use semio_framework_pack_json::{Value, object};
/// 📐️ Closed STEP operation table for explicitly assembled geometry sessions.
pub struct StepGeometryOperations;
impl GeometryOperations for StepGeometryOperations {
    fn export(&self,kernel:&Brep,format:&str,shapes:&[GeometryHandle],_:f64) -> Result<(String,bool),BrepModuleError> {
        if format != "step" { return Err(BrepModuleError::UnsupportedExportFormat(format.into())); }
        super::export_step(kernel,shapes).map(|text|(text,false)).map_err(BrepModuleError::from)
    }
    fn import(&self,kernel:&mut Brep,format:&str,text:&str,_:f64) -> Result<Vec<GeometryHandle>,BrepModuleError> {
        if format != "step" { return Err(BrepModuleError::UnsupportedImportFormat(format.into())); }
        super::import_step(kernel,text).map_err(BrepModuleError::from)
    }
    fn invoke(&self,kernel:&mut Brep,method:&str,args:&Value) -> Result<Value,BrepModuleError> {
        match method {
            "exportStep" => {
                let shapes=args.get("shapes").and_then(Value::as_array).ok_or_else(||BrepModuleError::InvalidArgs("shapes must be a geometry handle list".into()))?.iter().map(|value|value.as_str().map(|value|GeometryHandle(value.into())).ok_or_else(||BrepModuleError::InvalidArgs("shapes must contain geometry handles".into()))).collect::<Result<Vec<_>,_>>()?;
                let text=super::export_step(kernel,&shapes).map_err(BrepModuleError::from)?;
                Ok(object([("value".into(),Value::String(text))]))
            }
            "importStep" => {
                let text=args.get("data").and_then(Value::as_str).ok_or_else(||BrepModuleError::InvalidArgs("data must be STEP text".into()))?;
                let handles=super::import_step(kernel,text).map_err(BrepModuleError::from)?;
                Ok(object([("handles".into(),Value::Array(handles.into_iter().map(|handle|Value::String(handle.0)).collect()))]))
            }
            other => Err(BrepModuleError::UnknownMethod(other.into())),
        }
    }
}
/// 🌊️ Creates a session with the explicitly owned STEP operation contribution.
pub fn geometry_session() -> Session { Session::with_operations(&StepGeometryOperations) }
