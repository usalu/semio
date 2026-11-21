//! ↔️ Stages the exact offset of a live native solid certificate.

use super::*;

impl BrepRegistry {
    /// 📦️ Plans a certified constant support offset in the original world frame.
    pub fn plan_certified_solid_offset(&mut self,handle:&str,distance:f64)->Result<SolidPublication,ValueError>{
        let (_,source)=self.preview_provenance(handle).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Offset Solid requires certified stock"))?;
        let boundary=source.offset_boundary(distance)?;
        self.plan_solid_boundary(boundary)
    }
}
