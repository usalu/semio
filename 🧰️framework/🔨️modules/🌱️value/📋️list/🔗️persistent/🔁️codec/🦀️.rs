//! 🔁️ The persistent paged list encodes as an ordered array with exact path reads.
use crate::list::persistent::PersistentList;
use crate::{ValueError,ValueRefusalKind};
use semio_framework_value_intrinsic::{controlled_sequence,path_index,DslValue,ToValue,ValueShape};
use semio_framework_value_native_encoding::NativeEncodeControl;

impl<T:ToValue+crate::retirement::RetireOwned+Send+Sync+'static> ToValue for PersistentList<T>{
    fn to_value(&self)->DslValue{DslValue::Array(self.iter().map(ToValue::to_value).collect())}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{controlled_sequence(self.iter(),control)}
    fn value_at_path(&self,path:&[&str])->Result<DslValue,ValueError>{
        let Some((segment,rest))=path.split_first()else{return Ok(self.to_value())};
        self.get(path_index(segment,self.len(),false)?).expect("admitted immutable row rank").value_at_path(rest).map_err(|error|error.under(segment))
    }
    fn value_shape_at_path(&self,path:&[&str])->Result<ValueShape,ValueError>{
        let Some((segment,rest))=path.split_first()else{return Ok(ValueShape::Array{len:self.len()})};
        self.get(path_index(segment,self.len(),false)?).expect("admitted immutable row rank").value_shape_at_path(rest).map_err(|error|error.under(segment))
    }
    fn value_key_at_path(&self,path:&[&str],index:usize)->Result<String,ValueError>{
        let(segment,rest)=path.split_first().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"immutable array root has no object keys"))?;
        self.get(path_index(segment,self.len(),false)?).expect("admitted immutable row rank").value_key_at_path(rest,index).map_err(|error|error.under(segment))
    }
}
