//! 📥️ Closed page metadata validates before exact original-owned string movement.
use super::{StagedFormPage,StagedFormChoicePage};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
#[derive(Default,semio_framework_value::RetireOwned)]
pub(super) struct StagedFormPageCursor {phase:u8,field:usize,offset:usize,number:u8,decimal:u64,root:[usize;3],choice:[usize;2],seen:u8,choice_seen:u8,path_count:usize,path:Vec<String>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.page")}
fn object(value:&DslValue)->Result<&[(String,DslValue)],ValueError>{if let DslValue::Object(rows)=value{Ok(rows)}else{Err(invalid())}}
fn text(value:&DslValue)->Result<&str,ValueError>{if let DslValue::String(value)=value{Ok(value)}else{Err(invalid())}}
fn take_text(value:&mut DslValue)->String{let DslValue::String(value)=value else{unreachable!()};std::mem::take(value)}
impl StagedFormPageCursor{
    pub fn demand(&self)->RetirementDemand{
        if self.phase==7{return Default::default();}
        RetirementDemand{copy_bytes:std::mem::size_of::<Self>()+std::mem::size_of::<StagedFormPage>(),capacity_bytes:if self.phase==4&&self.path_count>0{self.path_count*std::mem::size_of::<String>()}else{0},depth:4,..Default::default()}
    }
    fn inspect(&mut self,source:&DslValue)->Result<(),ValueError>{
        let rows=object(source)?;if !(2..=3).contains(&rows.len()){return Err(invalid());}
        match self.phase{
            0=>{
                let Some((name,value))=rows.get(self.field)else{if self.seen&3!=3{return Err(invalid());}self.phase=1;self.field=0;return Ok(());};
                let index=match name.as_str(){"fieldOffset"=>0,"diagnosticOffset"=>1,"choice"=>2,_=>return Err(invalid())};
                if self.seen&(1<<index)!=0{return Err(invalid());}if index<2{if text(value)?.is_empty(){return Err(invalid());}}else if object(value)?.len()!=2{return Err(invalid());}
                self.seen|=1<<index;self.root[index]=self.field;self.field+=1;
            },
            1=>{
                if self.seen&4==0{self.phase=2;self.field=0;return Ok(());}
                let rows=object(&rows[self.root[2]].1)?;let Some((name,value))=rows.get(self.field)else{if self.choice_seen!=3{return Err(invalid());}self.phase=2;self.field=0;return Ok(());};
                let index=match name.as_str(){"path"=>0,"offset"=>1,_=>return Err(invalid())};if self.choice_seen&(1<<index)!=0{return Err(invalid());}
                if index==0{let DslValue::Array(path)=value else{return Err(invalid())};if path.is_empty()||path.len()>64{return Err(invalid());}self.path_count=path.len();}else if text(value)?.is_empty(){return Err(invalid());}
                self.choice_seen|=1<<index;self.choice[index]=self.field;self.field+=1;
            },
            2=>{
                if self.number==2&&self.seen&4==0||self.number==3{self.phase=3;self.field=0;return Ok(());}
                let value=if self.number<2{&rows[self.root[self.number as usize]].1}else{&object(&rows[self.root[2]].1)?[self.choice[1]].1};let value=text(value)?;
                match value.as_bytes().get(self.offset){
                    Some(byte)if byte.is_ascii_digit()&&(self.offset!=0||value.len()==1||*byte!=b'0')=>{self.decimal=self.decimal.checked_mul(10).and_then(|number|number.checked_add(u64::from(*byte-b'0'))).ok_or_else(invalid)?;self.offset+=1;},
                    Some(_)=>return Err(invalid()),
                    None=>{self.number+=1;self.offset=0;self.decimal=0;},
                }
            },
            3=>{
                if self.field==self.path_count{self.phase=4;self.field=0;return Ok(());}
                let DslValue::Array(path)=&object(&rows[self.root[2]].1)?[self.choice[0]].1 else{return Err(invalid())};
                let part=text(&path[self.field])?;if part.is_empty()||matches!(part,"__proto__"|"prototype"|"constructor"){return Err(invalid());}self.field+=1;
            },
            _=>return Err(invalid()),
        }Ok(())
    }
    pub fn step(&mut self,source:&mut DslValue,output:&mut Option<StagedFormPage>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.phase==7{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if output.is_some(){return Err(invalid());}let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        let result=match self.phase{
            0..=3=>self.inspect(source),
            4=>{
                if self.path_count>0{
                    if self.path.try_reserve_exact(self.path_count).is_err(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"staging.page").with_retained_progress(progress));}
                    progress.retained_capacity_bytes=self.path.capacity()*std::mem::size_of::<String>();if progress.retained_capacity_bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.page").with_retained_progress(progress));}
                }
                self.phase=if self.path_count>0{5}else{6};Ok(())
            },
            5=>{
                if self.field==self.path_count{self.phase=6;Ok(())}else{
                    let DslValue::Object(rows)=source else{unreachable!()};let DslValue::Object(choice)=&mut rows[self.root[2]].1 else{unreachable!()};let DslValue::Array(path)=&mut choice[self.choice[0]].1 else{unreachable!()};
                    self.path.push(take_text(&mut path[self.field]));self.field+=1;Ok(())
                }
            },
            6=>{
                let DslValue::Object(rows)=source else{unreachable!()};
                let choice=if self.seen&4!=0{let DslValue::Object(choice)=&mut rows[self.root[2]].1 else{unreachable!()};Some(StagedFormChoicePage{path:std::mem::take(&mut self.path),offset:take_text(&mut choice[self.choice[1]].1)})}else{None};
                *output=Some(StagedFormPage{field_offset:take_text(&mut rows[self.root[0]].1),diagnostic_offset:take_text(&mut rows[self.root[1]].1),choice});self.phase=7;Ok(())
            },
            _=>Err(invalid()),
        };
        if let Err(error)=result{return Err(error.with_retained_progress(progress));}
        Ok(if self.phase==7{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
