//! 🗺️ Receives each authored adjustment map entry into its preadmitted Raster page.

use super::RasterOwnedMap;
use semio_framework_dsl_record::{FieldValue,receiving::owned::*};
use semio_framework_value::{DslValue,RetainedAllocationEffect,RetirementDemand,ValueError};
use std::mem::size_of;

type Map=RasterOwnedMap<DslValue>;

#[derive(Default)]
pub struct RasterAdjustmentMapReceiving {
    output: OwnedSlot<Map>,
    ready: bool,
}

impl DslOwnedReceivingField for Map {
    type Cursor=RasterAdjustmentMapReceiving;
    fn receiving_cursor()->Self::Cursor{Self::Cursor::default()}
}

impl OwnedFieldReceiving<Map> for RasterAdjustmentMapReceiving {
    fn demands(&self,source:&Option<FieldValue>,_:usize)->Result<RetirementDemand,ValueError>{
        let Some(FieldValue::Map(entries))=source else {return Err(invalid());};
        let Some(output)=self.output.value.as_ref() else {
            Map::new().validate_additional_unique_entries(entries.len()).map_err(ValueError::invariant)?;
            return Ok(payload::<Option<Map>>());
        };
        let Some((key,value))=entries.last() else{return Ok(payload::<bool>());};
        if !matches!(value,FieldValue::Value(_))||output.contains_key(key){return Err(invalid());}
        if output.page_required_for_insert(key){
            return Ok(RetirementDemand {
                copy_bytes:Map::conservative_page_credit_bytes()+size_of::<usize>(),
                capacity_bytes:Map::conservative_page_credit_bytes(),
                depth:1,..Default::default()
            });
        }
        Ok(RetirementDemand {
            copy_bytes:size_of::<(String,FieldValue)>()+size_of::<(String,DslValue)>()+super::RASTER_OWNED_MAP_CAPACITY,
            depth:1,..Default::default()
        })
    }

    fn select_advance(&self,source:&Option<FieldValue>,body:usize,_:u64)->Result<OwnedAdvanceSelection,ValueError>{
        self.demands(source,body)?;
        Ok(OwnedAdvanceSelection::Local {work:1})
    }

    fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
        let demand=match self.demands(source,body){Ok(demand)=>demand,Err(error)=>return done(effect,Err(error.into()))};
        if !admits(effect,demand)||self.ready{return idle(effect);}
        let result=(||{
            let Some(FieldValue::Map(entries))=source else{return Err(invalid());};
            perform(effect)?;
            let Some(output)=self.output.value.as_mut() else{
                *self.output.value=Some(Map::new());
                return Ok(receipt(demand));
            };
            if entries.is_empty(){self.ready=true;return Ok(receipt(demand));}
            if demand.capacity_bytes!=0{
                if effect.actual().is_some(){return Err(invariant());}
                *effect.actual()=Some(0);
                output.admit_one_page().map_err(ValueError::invariant)?;
                *effect.actual()=Some(demand.capacity_bytes);
            }else{
                let (key,FieldValue::Value(value))=entries.pop().expect("validated Raster map item") else{unreachable!()};
                let inserted=output.insert_pre_admitted(key,value);
                assert!(matches!(inserted,Ok(super::RasterOwnedMapInsert::Inserted)),"validated Raster map insertion");
            }
            Ok(receipt(demand))
        })();
        done(effect,result.map_err(Into::into))
    }

    fn is_ready(&self)->bool{self.ready&&self.output.value.is_some()}
    fn take_into(&mut self,output:&mut Option<Map>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{
        if !self.is_ready(){return done(effect,Err(invariant()));}
        move_into(&mut self.output.value,output,effect)
    }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.output.close_demands(body)}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.output.close_work_demand(body)}
    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{self.output.close_step(effect)}
    fn terminal_is_empty(&self)->bool{self.output.terminal_is_empty()}
}
