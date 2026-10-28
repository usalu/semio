//! 🎟️ Missing identities expose source-authored blank text controls with exact fixed row creation.
use super::*;
#[derive(Default)]
pub(super) struct IdentityControlState{role:usize,slot:usize,row:Option<usize>,stage:u8,part:usize,field:usize,remove:usize}
#[cfg(test)]
impl IdentityControlState{pub(super) fn stage(&self)->u8{self.stage}}
fn role(encoded:usize,buffer:&mut[u8;32])->&str{let(prefix,value)=if encoded<MAXIMUM_ENTRIES{("entry:",Some(encoded))}else if encoded<MAXIMUM_ENTRIES*2{("enum:",Some(encoded-MAXIMUM_ENTRIES))}else{(match encoded-MAXIMUM_ENTRIES*2{0=>"set",1=>"relation",2=>"performance",_=>"control"},None)};buffer[..prefix.len()].copy_from_slice(prefix.as_bytes());let mut len=prefix.len();if let Some(value)=value{let mut number=[0;20];let text=decimal(value as u64,&mut number);buffer[len..len+text.len()].copy_from_slice(text.as_bytes());len+=text.len();}std::str::from_utf8(&buffer[..len]).expect("static role and decimal coordinates are UTF8")}
fn fixed(binding:&mut StagedFormBinding)->Result<(&mut Vec<String>,&mut Vec<(String,DslValue)>),ValueError>{match binding.edits.first_mut(){Some(StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Append{path,value:DslValue::Object(fields)}})=>Ok((path,fields)),_=>Err(refused())}}
fn text_path(binding:&mut StagedFormBinding)->Result<&mut Vec<String>,ValueError>{match binding.edits.last_mut(){Some(StagedFormBindingEdit::Text{path})=>Ok(path),_=>Err(refused())}}
impl IfcTemplateSourceWork{
    pub(super) fn advance_identity_control(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        if self.phase==34{return self.advance_identity_removal(cx);}
        if self.identity_control.stage==0{
            if self.identity_control.role==self.identities.count{self.phase=34;return Ok(());}let(encoded,rooted)=self.identities.requirements[self.identity_control.role].ok_or_else(refused)?;if self.identity_control.slot==2||self.identity_control.slot==1&&!rooted{self.identity_control.role+=1;self.identity_control.slot=0;return Ok(());}let source=self.page.as_ref().unwrap().source.as_ref().unwrap().try_authority()?;let rows=drafts::rows(&source.args)?;let mut existing=None;for(index,row)in rows.iter().enumerate(){if key_matches(encoded,drafts::fields(row)?.0){existing=Some(index);break;}}let present=if let Some(row)=existing{let(_,id,guid)=drafts::fields(&rows[row])?;if self.identity_control.slot==0{id.is_some()}else{guid.is_some()}}else{false};if present{self.identity_control.slot+=1;return Ok(());}self.identity_control.row=existing;if self.ordinal<self.offset||self.ordinal-self.offset>=STAGED_FORM_BODY_LIMITS.content as u64{self.ordinal=self.ordinal.checked_add(1).ok_or_else(refused)?;self.identity_control.slot+=1;return Ok(());}let page=self.page.as_mut().unwrap();metadata(cx,size_of::<Control>(),||{page.controls.push(Control::Identity{address:self.ordinal,global_id:self.identity_control.slot==1,role:String::new(),binding:StagedFormBinding{token:empty_token(),layout:StagedFormLayout::Replace,edits:Vec::new()}});self.identity_control.stage=1;self.identity_control.part=0;})?;return Ok(());
        }
        let page=self.page.as_mut().unwrap();let source=page.source.as_ref().unwrap().try_authority()?;let rows=drafts::rows(&source.args)?;let(encoded,rooted)=self.identities.requirements[self.identity_control.role].ok_or_else(refused)?;let Control::Identity{role:role_text,binding,..}=page.controls.last_mut().unwrap()else{return Err(refused().into())};
        match self.identity_control.stage{
            1=>{let mut buffer=[0;32];if copy_text(role_text,role(encoded,&mut buffer),cx)?{self.identity_control.stage=2;}},
            2=>{if copy_text(token_part_mut(&mut binding.token,self.identity_control.part),token_part(&source.token,self.identity_control.part),cx)?{self.identity_control.part+=1;if self.identity_control.part==4{self.identity_control.stage=3;}}},
            3=>{if reserve(&mut binding.edits,if self.identity_control.row.is_some(){1}else{2},cx)?{self.identity_control.stage=if self.identity_control.row.is_some(){11}else{4};}},
            4=>{metadata(cx,size_of::<StagedFormBindingEdit>(),||{binding.edits.push(StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Append{path:Vec::new(),value:DslValue::Object(Vec::new())}});self.identity_control.stage=5;})?;},
            5=>{let(path,_)=fixed(binding)?;if reserve(path,1,cx)?{self.identity_control.stage=6;}},
            6=>{let(path,_)=fixed(binding)?;if path.is_empty(){metadata(cx,size_of::<String>(),||path.push(String::new()))?;}else if copy_text(&mut path[0],"identities",cx)?{self.identity_control.stage=7;}},
            7=>{let(_,fields)=fixed(binding)?;if reserve(fields,if rooted{3}else{2},cx)?{self.identity_control.stage=8;self.identity_control.field=0;}},
            8=>{let(_,fields)=fixed(binding)?;if fields.len()==self.identity_control.field{metadata(cx,size_of::<(String,DslValue)>(),||fields.push((String::new(),DslValue::String(String::new()))))?;}else{self.identity_control.stage=9;}},
            9=>{let(_,fields)=fixed(binding)?;let name=match self.identity_control.field{0=>"key",1=>"id",_=>"guid"};if copy_text(&mut fields[self.identity_control.field].0,name,cx)?{self.identity_control.stage=10;}},
            10=>{let(_,fields)=fixed(binding)?;if self.identity_control.field==0{let DslValue::String(value)=&mut fields[0].1 else{return Err(refused().into())};if !copy_text(value,role_text,cx)?{return Ok(());}}self.identity_control.field+=1;self.identity_control.stage=if self.identity_control.field==if rooted{3}else{2}{11}else{8};},
            11=>{metadata(cx,size_of::<StagedFormBindingEdit>(),||{binding.edits.push(StagedFormBindingEdit::Text{path:Vec::new()});self.identity_control.stage=12;self.identity_control.part=0;})?;},
            12=>{if reserve(text_path(binding)?,3,cx)?{self.identity_control.stage=13;}},
            13=>{let path=text_path(binding)?;if path.len()==self.identity_control.part{metadata(cx,size_of::<String>(),||path.push(String::new()))?;}else{self.identity_control.stage=14;}},
            14=>{let path=text_path(binding)?;let mut number=[0;20];let text=match self.identity_control.part{0=>"identities",1=>decimal(self.identity_control.row.unwrap_or(rows.len())as u64,&mut number),_=>if self.identity_control.slot==0{"id"}else{"guid"}};if copy_text(&mut path[self.identity_control.part],text,cx)?{self.identity_control.part+=1;if self.identity_control.part==3{self.ordinal=self.ordinal.checked_add(1).ok_or_else(refused)?;self.identity_control.slot+=1;self.identity_control.stage=0;}else{self.identity_control.stage=13;}}},
            _=>return Err(refused().into()),
        }Ok(())
    }
    fn advance_identity_removal(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let page=self.page.as_mut().unwrap();let source=page.source.as_ref().unwrap().try_authority()?;let rows=drafts::rows(&source.args)?;
        if self.identity_control.stage==0{if self.identity_control.remove==rows.len(){if self.offset>=self.ordinal{return Err(refused().into());}self.phase=18;return Ok(());}if self.ordinal<self.offset||self.ordinal-self.offset>=STAGED_FORM_BODY_LIMITS.content as u64{self.ordinal=self.ordinal.checked_add(1).ok_or_else(refused)?;self.identity_control.remove+=1;return Ok(());}metadata(cx,size_of::<Control>(),||{page.controls.push(Control::Removal{address:self.ordinal,row:self.identity_control.remove,binding:StagedFormBinding{token:empty_token(),layout:StagedFormLayout::Replace,edits:Vec::new()}});self.identity_control.stage=1;self.identity_control.part=0;})?;return Ok(());}
        let Control::Removal{binding,..}=page.controls.last_mut().unwrap()else{return Err(refused().into())};match self.identity_control.stage{
            1=>{if copy_text(token_part_mut(&mut binding.token,self.identity_control.part),token_part(&source.token,self.identity_control.part),cx)?{self.identity_control.part+=1;if self.identity_control.part==4{self.identity_control.stage=2;}}},
            2=>{if reserve(&mut binding.edits,1,cx)?{self.identity_control.stage=3;}},
            3=>{metadata(cx,size_of::<StagedFormBindingEdit>(),||{binding.edits.push(StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Remove{path:Vec::new()}});self.identity_control.stage=4;})?;},
            4=>{let[StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Remove{path}}]=binding.edits.as_mut_slice()else{return Err(refused().into())};if reserve(path,2,cx)?{self.identity_control.stage=5;self.identity_control.part=0;}},
            5=>{let[StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Remove{path}}]=binding.edits.as_mut_slice()else{return Err(refused().into())};if path.len()==self.identity_control.part{metadata(cx,size_of::<String>(),||path.push(String::new()))?;}else{self.identity_control.stage=6;}},
            6=>{let[StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Remove{path}}]=binding.edits.as_mut_slice()else{return Err(refused().into())};let mut number=[0;20];let text=if self.identity_control.part==0{"identities"}else{decimal(self.identity_control.remove as u64,&mut number)};if copy_text(&mut path[self.identity_control.part],text,cx)?{self.identity_control.part+=1;if self.identity_control.part==2{self.identity_control.remove+=1;self.ordinal=self.ordinal.checked_add(1).ok_or_else(refused)?;self.identity_control.stage=0;}else{self.identity_control.stage=5;}}},
            _=>return Err(refused().into()),
        }Ok(())
    }
}
