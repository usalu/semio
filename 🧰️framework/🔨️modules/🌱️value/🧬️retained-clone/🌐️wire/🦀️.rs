//! 🌐️ Canonical Value wire traits own the five independent retained currencies.
use crate::{RetainedCloneGrant,RetainedCloneProgress};
use std::mem::size_of;
impl crate::ToValue for RetainedCloneGrant {
    fn to_value(&self)->crate::DslValue{
        let fields=self.wire_fields();let bytes=5*size_of::<(String,crate::DslValue)>()+fields.iter().map(|(name,_)|name.len()).sum::<usize>();
        let mut accepted=|_|true;self.to_value_controlled(&mut crate::NativeEncodeControl::new(bytes,&mut accepted)).expect("fixed canonical grant encoding")
    }
    fn to_value_controlled(&self,control:&mut crate::NativeEncodeControl<'_>)->Result<crate::DslValue,crate::ValueError>{
        control.scoped_stage(|control|{control.begin_stage(5)?;
        control.checkpoint()?;let mut entries=control.allocate_vec(5)?;
        for(name,value)in self.wire_fields(){entries.push((control.copy_text(name)?,crate::DslValue::Number(crate::Number::UInt(u64::try_from(value).map_err(|_|crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical grant exceeds unsigned wire range"))?))));control.step()?;}
        Ok(crate::DslValue::Object(entries))
            })
    }
}
impl crate::FromValue for RetainedCloneGrant {
    fn from_value(value:crate::DslValue)->Result<Self,crate::ValueError>{
        let mut accepted=|_|true;Self::from_value_controlled(&value,&mut crate::NativeDecodeControl::new(0,&mut accepted))
    }
    fn from_value_controlled(value:&crate::DslValue,control:&mut crate::NativeDecodeControl<'_>)->Result<Self,crate::ValueError>{
        control.scoped_stage(|control|{control.begin_stage(5)?;
        control.checkpoint()?;
        let crate::DslValue::Object(entries)=value else{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained grant requires an object"));};
        let names=Self::default().wire_fields().map(|(name,_)|name);let mut fields=[None;5];
        for(name,value)in entries{
            let index=names.iter().position(|expected|*expected==name).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained grant has an unknown axis"))?;
            if fields[index].is_some(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained grant repeats an axis"));}
            let crate::DslValue::Number(crate::Number::UInt(value))=value else{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained grant requires unsigned axes"));};
            fields[index]=Some(usize::try_from(*value).map_err(|_|crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"canonical retained grant exceeds this address space"))?);control.step()?;
        }
        let [Some(maximum_items),Some(maximum_copy_bytes),Some(maximum_capacity_bytes),Some(maximum_release_bytes),Some(maximum_depth)]=fields else{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained grant is missing an axis"));};
        Ok(Self {maximum_items,maximum_copy_bytes,maximum_capacity_bytes,maximum_release_bytes,maximum_depth})
            })
    }
}



/// 🌱️ Exact allocation-only constructor demand borrowed before original owner transfer.





/// 🧾️ Canonical cumulative physical receipt, independent of the per-turn grant.


impl crate::ToValue for RetainedCloneProgress {
    fn to_value(&self)->crate::DslValue{
        let fields=self.wire_fields();let bytes=4*size_of::<(String,crate::DslValue)>()+fields.iter().map(|(name,_)|name.len()).sum::<usize>();
        let mut accepted=|_|true;self.to_value_controlled(&mut crate::NativeEncodeControl::new(bytes,&mut accepted)).expect("fixed canonical progress encoding")
    }
    fn to_value_controlled(&self,control:&mut crate::NativeEncodeControl<'_>)->Result<crate::DslValue,crate::ValueError>{
        control.scoped_stage(|control|{control.begin_stage(4)?;
        control.checkpoint()?;let mut entries=control.allocate_vec(4)?;
        for(name,value)in self.wire_fields(){entries.push((control.copy_text(name)?,crate::DslValue::Number(crate::Number::UInt(u64::try_from(value).map_err(|_|crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical progress exceeds unsigned wire range"))?))));control.step()?;}
        Ok(crate::DslValue::Object(entries))
            })
    }
}
impl crate::FromValue for RetainedCloneProgress {
    fn from_value(value:crate::DslValue)->Result<Self,crate::ValueError>{
        let mut accepted=|_|true;Self::from_value_controlled(&value,&mut crate::NativeDecodeControl::new(0,&mut accepted))
    }
    fn from_value_controlled(value:&crate::DslValue,control:&mut crate::NativeDecodeControl<'_>)->Result<Self,crate::ValueError>{
        control.scoped_stage(|control|{control.begin_stage(4)?;
        control.checkpoint()?;
        let crate::DslValue::Object(entries)=value else{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained progress requires an object"));};
        let names=Self::default().wire_fields().map(|(name,_)|name);let mut fields=[None;4];
        for(name,value)in entries{
            let index=names.iter().position(|expected|*expected==name).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained progress has an unknown axis"))?;
            if fields[index].is_some(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained progress repeats an axis"));}
            let crate::DslValue::Number(crate::Number::UInt(value))=value else{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained progress requires unsigned axes"));};
            fields[index]=Some(usize::try_from(*value).map_err(|_|crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"canonical retained progress exceeds this address space"))?);control.step()?;
        }
        let [Some(copied_items),Some(copied_bytes),Some(retained_capacity_bytes),Some(released_bytes)]=fields else{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"canonical retained progress is missing an axis"));};
        Ok(Self {copied_items,copied_bytes,retained_capacity_bytes,released_bytes})
            })
    }
}






/// 🛡️ Checks both granted closure work and the child's exact terminal ownership witness.






/// 🧬️ Creates a one-shot cursor for an immutable owner retained at one stable address until completion or cancellation.
/// The cursor owner retains it through `begin_close` and capacity-bearing `close_step` turns until

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod original_wire_tests;
