//! 👁️ Original edit ordinals borrow genuine native records and expose their actual access depth.
use super::{HistoryEdit,HistoryEdits};
use semio_framework_value::ValueError;
pub trait OriginalHistoryEdits{fn original_edit(&self,ordinal:usize)->Option<&HistoryEdit>;fn original_count(&self)->usize;fn original_depth(&self,ordinal:usize)->Result<usize,ValueError>;}
impl OriginalHistoryEdits for HistoryEdits{
 fn original_edit(&self,ordinal:usize)->Option<&HistoryEdit>{self.get(ordinal)}
 fn original_count(&self)->usize{self.len()}
 fn original_depth(&self,ordinal:usize)->Result<usize,ValueError>{self.next_get_depth_demand(ordinal).map_err(|error|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,error.reason))}
}
impl OriginalHistoryEdits for Vec<HistoryEdit>{
 fn original_edit(&self,ordinal:usize)->Option<&HistoryEdit>{self.get(ordinal)}
 fn original_count(&self)->usize{self.len()}
 fn original_depth(&self,_:usize)->Result<usize,ValueError>{Ok(1)}
}
impl<const N:usize> OriginalHistoryEdits for [HistoryEdit;N]{
 fn original_edit(&self,ordinal:usize)->Option<&HistoryEdit>{self.get(ordinal)}
 fn original_count(&self)->usize{N}
 fn original_depth(&self,_:usize)->Result<usize,ValueError>{Ok(1)}
}
