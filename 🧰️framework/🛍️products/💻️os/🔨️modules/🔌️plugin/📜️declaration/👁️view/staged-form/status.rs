//! 🚦️ Bounded nonvisual source metadata borrows the neutral status owner without serializing a model.
use super::{StagedPayloadKind as Kind,StagedPayloadNode as Node,StagedPayloadSource};
use semio_framework::action_staging::{StagedFormStatus,StagedFormTextFormat};
use semio_framework_value::{ValueError,ValueRefusalKind};
pub struct StagedStatusPayload<'a>{pub status:&'a StagedFormStatus}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.status")}
fn node<'a>(key:Option<&'a str>,value:Kind<'a>)->Node<'a>{Node{key,value}}
impl StagedPayloadSource for StagedStatusPayload<'_>{
    fn identity(&self)->usize{self.status as*const _ as usize}
    fn node(&self,path:&[usize])->Result<Node<'_>,ValueError>{
        let status=self.status;
        match path{
            []=>{if !status.token.admitted()||status.diagnostics.len()>16||!StagedFormTextFormat::DecimalU64.admits(&status.diagnostic_offset)||!StagedFormTextFormat::DecimalU64.admits(&status.diagnostic_total){return Err(invalid());}Ok(node(None,Kind::Map(5)))},
            [0]=>Ok(node(Some("token"),Kind::Map(4))),
            [0,index]=>{let(key,value)=match index{0=>("action",status.token.action.as_str()),1=>("generation",status.token.generation.as_str()),2=>("revision",status.token.revision.as_str()),3=>("window",status.token.window.as_str()),_=>return Err(invalid())};Ok(node(Some(key),Kind::Text(value)))},
            [1]=>Ok(node(Some("ready"),Kind::Bool(status.ready))),
            [2]=>Ok(node(Some("diagnostics"),Kind::List(status.diagnostics.len()))),
            [2,index] if *index<status.diagnostics.len()=>Ok(node(None,Kind::Map(3))),
            [2,index,field]=>{let value=status.diagnostics.get(*index).ok_or_else(invalid)?;match field{0=>{if value.path.len()>64{return Err(invalid());}Ok(node(Some("path"),Kind::List(value.path.len())))},1=>Ok(node(Some("code"),Kind::Text(&value.code))),2=>Ok(node(Some("label"),Kind::Map(2))),_=>Err(invalid())}},
            [2,index,0,item]=>Ok(node(None,Kind::Text(status.diagnostics.get(*index).and_then(|value|value.path.get(*item)).ok_or_else(invalid)?))),
            [2,index,2,locale]=>{let label=&status.diagnostics.get(*index).ok_or_else(invalid)?.label;let(key,value)=match locale{0=>("en",label.en.as_str()),1=>("de",label.de.as_str()),_=>return Err(invalid())};Ok(node(Some(key),Kind::Text(value)))},
            [3]=>Ok(node(Some("diagnosticOffset"),Kind::Text(&status.diagnostic_offset))),
            [4]=>Ok(node(Some("diagnosticTotal"),Kind::Text(&status.diagnostic_total))),
            _=>Err(invalid()),
        }
    }
}
