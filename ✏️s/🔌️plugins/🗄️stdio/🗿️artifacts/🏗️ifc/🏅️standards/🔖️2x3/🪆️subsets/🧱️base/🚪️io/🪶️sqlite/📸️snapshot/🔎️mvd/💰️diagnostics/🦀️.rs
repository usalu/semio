//! 💰️ MVD diagnostics admit concrete vectors, stable codes and exact formatted UTF-8 messages.
use super::{SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use crate::standards::v2x3::subsets::base::io::sqlite::snapshot::native;
use semio_framework_diagnostic::{Diagnostic,FaultCode,FaultScope,Severity,TextSpan};
use semio_framework_value::NativeEncodeControl;
use semio_framework_dsl_record::__rt::DecodedFieldOwner;

/// 🩺️ Owns controlled IFC2x3 conformance diagnostics through success and partial failure.
#[derive(Default)]
pub struct MvdDiagnostics{values:Vec<Diagnostic>,bytes:usize}
impl Drop for MvdDiagnostics{fn drop(&mut self){native::close(std::mem::take(&mut self.values));}}
impl MvdDiagnostics{
    /// 🛫️ Transfers the completed diagnostic vector without an additional backing request.
    pub fn finish(mut self)->Vec<Diagnostic>{std::mem::take(&mut self.values)}
    /// 🧵️ Formats each exact diagnostic directly into an admitted UTF-8 buffer.
    pub fn emit(&mut self,code:&'static str,severity:Severity,message:std::fmt::Arguments<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
        let mut measure=Measure(0);std::fmt::write(&mut measure,message).map_err(|_|ValueError::new(ValueRefusalKind::OwnershipLimit,"IFC2x3 diagnostic byte count overflow"))?;
        let count=self.values.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 diagnostic count overflow"))?;control.check_rows(count)?;
        let bytes=self.bytes.checked_add(measure.0).and_then(|bytes|bytes.checked_add(code.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IFC2x3 diagnostic semantic bytes overflow"))?;control.check_value_bytes(bytes)?;
        control.allocation_stage(SqliteSnapshotPhase::ProjectSnapshot,|remaining,progress|{
            let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
            let mut native=NativeEncodeControl::new(remaining,&mut callback);
            let result=(||{
                native.begin_stage(0)?;
                if self.values.len()==self.values.capacity(){
                    let capacity=self.values.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IFC2x3 diagnostic vector size overflow"))?;
                    let mut next=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(native.allocate_vec(capacity)?,native::close::<Vec<Diagnostic>>);
                    while !self.values.is_empty(){native.step()?;next.as_mut().push(self.values.pop().unwrap());}
                    let length=next.as_mut().len();for index in 0..length/2{native.step()?;next.as_mut().swap(index,length-index-1);}self.values=next.take();
                }
                let code=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(native.copy_text(code)?,native::close::<String>);
                let mut bytes=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(native.allocate_vec::<u8>(measure.0)?,native::close::<Vec<u8>>);
                native.begin_stage(measure.0)?;let mut writer=Writer{bytes:bytes.as_mut(),control:&mut native,error:None};
                if std::fmt::write(&mut writer,message).is_err(){return Err(writer.error.unwrap_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"IFC2x3 diagnostic formatting differs from measurement")));}
                if bytes.as_mut().len()!=measure.0{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"IFC2x3 diagnostic formatting length changed"));}
                let message=match String::from_utf8(bytes.take()){Ok(message)=>message,Err(error)=>{native::close(error.into_bytes());return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"IFC2x3 diagnostic formatting produced invalid UTF-8"));}};
                let diagnostic=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Diagnostic{code:FaultCode(code.take()),severity,span:TextSpan::at(1,1),message,expected:None,scope:FaultScope::default()},native::close::<Diagnostic>);
                native.checkpoint()?;self.values.push(diagnostic.take());Ok(())
            })();(result,native.owned_bytes())
        })??;
        self.bytes=bytes;Ok(())
    }
}
struct Measure(usize);
impl std::fmt::Write for Measure{fn write_str(&mut self,value:&str)->std::fmt::Result{self.0=self.0.checked_add(value.len()).ok_or(std::fmt::Error)?;Ok(())}}
struct Writer<'a,'p>{bytes:&'a mut Vec<u8>,control:&'a mut NativeEncodeControl<'p>,error:Option<ValueError>}
impl std::fmt::Write for Writer<'_,'_>{
    fn write_str(&mut self,value:&str)->std::fmt::Result{
        let Some(end)=self.bytes.len().checked_add(value.len()).filter(|end|*end<=self.bytes.capacity())else{return Err(std::fmt::Error)};
        for chunk in value.as_bytes().chunks(65536){self.bytes.extend_from_slice(chunk);if let Err(error)=self.control.advance(chunk.len()){self.error=Some(error);return Err(std::fmt::Error);}}
        if self.bytes.len()!=end{return Err(std::fmt::Error)}Ok(())
    }
}
