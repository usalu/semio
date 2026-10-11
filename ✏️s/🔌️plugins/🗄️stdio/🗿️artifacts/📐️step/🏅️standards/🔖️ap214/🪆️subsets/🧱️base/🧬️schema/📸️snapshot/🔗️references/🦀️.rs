//! 🔗️ Complete internal Part-21 graph admission for typed STEP snapshots.
use super::{StepSnapshot,StepTypedValue, StepValue};
use protocol::{MutationApplyError,MutationApplyResult};
use std::collections::BTreeSet;

/// 🔗️ Validates unique instance identities and every nested internal reference before publication.
pub fn validate_step_references(snapshot:&StepSnapshot)->MutationApplyResult<()> {
    let mut identities=BTreeSet::new();
    for entity in &snapshot.entities {
        if !identities.insert(entity.id) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-entity-id","STEP entity identities must be unique").at(vec!["entities".into(),entity.id.to_string()]));
        }
    }
    for entity in &snapshot.entities {
        for (index,value) in entity.args.iter().enumerate() {
            validate_value(value,&identities,vec!["entities".into(),entity.id.to_string(),"args".into(),index.to_string()])?;
        }
        for (part,complex) in entity.complex.iter().enumerate() {
            for (index,value) in complex.args.iter().enumerate() {
                validate_value(value,&identities,vec!["entities".into(),entity.id.to_string(),"complex".into(),part.to_string(),"args".into(),index.to_string()])?;
            }
        }
    }
    Ok(())
}

fn validate_value<'a>(value:&'a StepValue,identities:&BTreeSet<u64>,path:Vec<String>)->MutationApplyResult<()> {
    let mut pending=vec![(value,path)];
    while let Some((value,path))=pending.pop() {
        match value {
            StepValue::Reference(id) if !identities.contains(id)=>return Err(MutationApplyError::new("mutation.apply.dangling-reference","STEP internal reference must resolve to a retained entity").at(path)),
            StepValue::Aggregate(items)=>for (index,item) in items.iter().enumerate().rev() {let mut target=path.clone();target.push(index.to_string());pending.push((item,target));},
            StepValue::TypedValue(StepTypedValue {value,..})=>{let mut target=path;target.push("value".into());pending.push((value.as_ref(),target));},
            _=>{},
        }
    }
    Ok(())
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
