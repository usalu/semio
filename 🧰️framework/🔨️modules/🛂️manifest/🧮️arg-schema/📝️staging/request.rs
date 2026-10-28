//! 📥️ Private parsed request ownership moves only after closed incremental admission.
use super::{StagedFormCensusCursor,StagedFormLimits,StagedFormRequest,StagedFormToken,StagedFormPage,page_request::StagedFormPageCursor};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};

#[derive(semio_framework_value::RetireOwned)]
pub struct StagedFormRequestCursor {
    census:StagedFormCensusCursor,
    source:Option<usize>,
    phase:u8,
    field:usize,
    offset:usize,
    generation:u64,
    root:[usize;4],
    token:[usize;4],
    root_seen:u8,
    token_seen:u8,
    failed:bool,
    page:StagedFormPageCursor,
    owned_page:Option<StagedFormPage>,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.source-invalid")}
fn object(value:&DslValue)->Result<&[(String,DslValue)],ValueError>{if let DslValue::Object(fields)=value{Ok(fields)}else{Err(invalid())}}
fn text(value:&DslValue)->Result<&str,ValueError>{if let DslValue::String(value)=value{Ok(value)}else{Err(invalid())}}
fn take_text(value:&mut DslValue)->String{let DslValue::String(value)=value else{unreachable!()};std::mem::take(value)}
impl StagedFormRequestCursor {
    pub fn new(limits:StagedFormLimits)->Result<Self,ValueError>{Ok(Self{census:StagedFormCensusCursor::new(limits)?,source:None,phase:0,field:0,offset:0,generation:0,root:[0;4],token:[0;4],root_seen:0,token_seen:0,failed:false,page:Default::default(),owned_page:None})}
    /// 🔐️ Incoming production Pack keeps strict key order before any payload can move.
    pub fn new_wire(limits:StagedFormLimits)->Result<Self,ValueError>{let mut cursor=Self::new(limits)?;cursor.census=StagedFormCensusCursor::new_wire(limits)?;Ok(cursor)}
    pub fn demand(&self)->RetirementDemand{
        if self.phase==0{return self.census.demand();}
        if self.phase==5{return Default::default();}
        if self.phase==6{return self.page.demand();}
        RetirementDemand{copy_bytes:std::mem::size_of::<Self>()+std::mem::size_of::<StagedFormRequest>()+std::mem::size_of::<DslValue>(),depth:3,..Default::default()}
    }
    fn inspect(&mut self,source:&DslValue)->Result<(),ValueError>{
        let fields=object(source)?;if !(3..=4).contains(&fields.len()){return Err(invalid());}
        match self.phase {
            1=>{
                let(name,value)=fields.get(self.field).ok_or_else(invalid)?;
                let index=match name.as_str(){"token"=>0,"args"=>1,"locale"=>2,"page"=>3,_=>return Err(invalid())};
                if self.root_seen&(1<<index)!=0{return Err(invalid());}
                match index{0 if object(value)?.len()!=4=>return Err(invalid()),1=>{object(value)?;},2 if !matches!(text(value)?,"en"|"de")=>return Err(invalid()),_=>{}}
                self.root_seen|=1<<index;self.root[index]=self.field;self.field+=1;
                if self.field==fields.len(){if self.root_seen&7!=7{return Err(invalid());}self.field=0;self.phase=2;}
            },
            2=>{
                let token=object(&fields[self.root[0]].1)?;if token.len()!=4{return Err(invalid());}
                let(name,value)=token.get(self.field).ok_or_else(invalid)?;
                let index=match name.as_str(){"action"=>0,"generation"=>1,"revision"=>2,"window"=>3,_=>return Err(invalid())};
                if self.token_seen&(1<<index)!=0||text(value)?.is_empty(){return Err(invalid());}
                self.token_seen|=1<<index;self.token[index]=self.field;self.field+=1;
                if self.field==4{self.phase=3;}
            },
            3=>{
                let token=object(&fields[self.root[0]].1)?;
                let generation=text(&token.get(self.token[1]).ok_or_else(invalid)?.1)?;
                match generation.as_bytes().get(self.offset){
                    Some(byte) if byte.is_ascii_digit()&&(self.offset!=0||generation.len()==1||*byte!=b'0')=>{self.generation=self.generation.checked_mul(10).and_then(|value|value.checked_add(u64::from(*byte-b'0'))).ok_or_else(invalid)?;self.offset+=1;},
                    Some(_)=>return Err(invalid()),
                    None=>self.phase=if self.root_seen&8!=0{6}else{4},
                }
            },
            _=>return Err(invalid()),
        }Ok(())
    }
    fn extract(&mut self,source:&mut DslValue)->Result<StagedFormRequest,ValueError>{
        let fields=object(source)?;
        if !(3..=4).contains(&fields.len())||fields[self.root[0]].0!="token"||fields[self.root[1]].0!="args"||fields[self.root[2]].0!="locale"||self.root_seen&8!=0&&(fields[self.root[3]].0!="page"||self.owned_page.is_none()){return Err(invalid());}
        object(&fields[self.root[1]].1)?;
        let locale=match text(&fields[self.root[2]].1)?{"en"=>semio_framework_ui_locale::Locale::En,"de"=>semio_framework_ui_locale::Locale::De,_=>return Err(invalid())};
        let token=object(&fields[self.root[0]].1)?;
        if token.len()!=4{return Err(invalid());}
        for(index,name)in ["action","generation","revision","window"].iter().enumerate(){let field=&token[self.token[index]];if field.0!=*name||text(&field.1)?.is_empty(){return Err(invalid());}}
        let DslValue::Object(fields)=source else{unreachable!()};
        let DslValue::Object(token)=&mut fields[self.root[0]].1 else{unreachable!()};
        let token=StagedFormToken{action:take_text(&mut token[self.token[0]].1),generation:take_text(&mut token[self.token[1]].1),revision:take_text(&mut token[self.token[2]].1),window:take_text(&mut token[self.token[3]].1)};
        let args=std::mem::replace(&mut fields[self.root[1]].1,DslValue::Null);
        Ok(StagedFormRequest{token,args,locale,page:self.owned_page.take()})
    }
    /// 🧷️ The original owner keeps source private and stationary through census, movement and retirement.
    pub fn step(&mut self,source:&mut DslValue,output:&mut Option<StagedFormRequest>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.failed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"refused staged request cannot resume"));}
        let address=source as *const _ as usize;
        if self.source.is_some_and(|original|original!=address){self.failed=true;return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged request requires its original private source"));}
        if self.phase==5{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if output.is_some(){self.failed=true;return Err(invalid());}
        self.source=Some(address);
        if self.phase==0{
            return match self.census.step(source,grant){Ok(RetainedCloneStep::Complete(progress))=>{self.phase=1;Ok(RetainedCloneStep::Progress(progress))},Ok(step)=>Ok(step),Err(error)=>{self.failed=true;Err(error)}};
        }
        if self.phase==6{
            let DslValue::Object(fields)=source else{self.failed=true;return Err(invalid());};
            return match self.page.step(&mut fields[self.root[3]].1,&mut self.owned_page,grant){
                Ok(RetainedCloneStep::Complete(progress))=>{self.phase=4;Ok(RetainedCloneStep::Progress(progress))},
                Ok(step)=>Ok(step),Err(error)=>{self.failed=true;Err(error)},
            };
        }
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        let result=if self.phase==4{self.extract(source).map(|request|{*output=Some(request);self.phase=5;})}else{self.inspect(source)};
        if let Err(error)=result{self.failed=true;return Err(error.with_retained_progress(progress));}
        Ok(if self.phase==5{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
