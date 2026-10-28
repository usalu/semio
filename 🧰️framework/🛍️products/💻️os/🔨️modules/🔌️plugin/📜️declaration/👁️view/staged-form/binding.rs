//! 🧷️ Native authored bindings borrow the closed neutral payload without an intermediate dynamic copy.
use super::{StagedPayloadKind as Kind,StagedPayloadNode as Node,StagedPayloadSource};
use semio_framework::action_staging::*;
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
pub enum StagedBindingPayload<'a>{Action(&'a StagedFormBinding),Page(&'a StagedFormPageBinding)}
#[derive(Clone,Copy)]
enum View<'a>{Action(&'a StagedFormBinding),PageBinding(&'a StagedFormPageBinding),Token(&'a StagedFormToken),Page(&'a StagedFormPage),ChoicePage(&'a StagedFormChoicePage),Edits(&'a[StagedFormBindingEdit]),Edit(&'a StagedFormBindingEdit),Fixed(&'a StagedFormEdit),Path(&'a[String]),Options(&'a[StagedFormBindingOption]),Option(&'a StagedFormBindingOption),Data(&'a DslValue),Text(&'a str)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.binding")}
fn resource()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.resource-limit")}
fn path(value:&[String])->Result<(),ValueError>{if value.is_empty()||value.len()>64{return Err(invalid());}Ok(())}
impl<'a> View<'a>{
    fn kind(self)->Result<Kind<'a>,ValueError>{Ok(match self{
        Self::Action(binding)=>{if !binding.token.admitted()||binding.edits.is_empty()||binding.edits.len()>8||binding.layout==StagedFormLayout::Preserve&&(binding.edits.len()!=1||!matches!(binding.edits[0],StagedFormBindingEdit::Text{..}))||binding.edits.iter().filter(|edit|!matches!(edit,StagedFormBindingEdit::Fixed{..})).count()>1{return Err(invalid());}Kind::Map(3)},
        Self::PageBinding(binding)=>{if !binding.token.admitted()||!binding.page.admitted(){return Err(invalid());}Kind::Map(2)},Self::Token(_)=>Kind::Map(4),Self::Page(page)=>Kind::Map(if page.choice.is_some(){3}else{2}),Self::ChoicePage(_)=>Kind::Map(2),Self::Edits(rows)=>Kind::List(rows.len()),
        Self::Edit(StagedFormBindingEdit::Choice{options,..})=>{if options.is_empty()||options.len()>32{return Err(invalid());}Kind::Map(4)},Self::Edit(_)=>Kind::Map(2),Self::Fixed(StagedFormEdit::Remove{..})=>Kind::Map(2),Self::Fixed(_)=>Kind::Map(3),Self::Path(rows)=>{path(rows)?;Kind::List(rows.len())},Self::Options(rows)=>{if rows.is_empty()||rows.len()>32{return Err(invalid());}Kind::List(rows.len())},Self::Option(_)=>Kind::Map(2),Self::Data(value)=>super::payload::dsl_kind(value)?,Self::Text(value)=>Kind::Text(value),
    })}
    fn child(self,index:usize)->Result<(Option<&'a str>,Self),ValueError>{
        let pair=match self{
            Self::Action(binding)=>match index{0=>(Some("token"),Self::Token(&binding.token)),1=>(Some("layout"),Self::Text(if binding.layout==StagedFormLayout::Preserve{"preserve"}else{"replace"})),2=>(Some("edits"),Self::Edits(&binding.edits)),_=>return Err(invalid())},
            Self::PageBinding(binding)=>match index{0=>(Some("token"),Self::Token(&binding.token)),1=>(Some("page"),Self::Page(&binding.page)),_=>return Err(invalid())},
            Self::Token(token)=>match index{0=>(Some("action"),Self::Text(&token.action)),1=>(Some("generation"),Self::Text(&token.generation)),2=>(Some("revision"),Self::Text(&token.revision)),3=>(Some("window"),Self::Text(&token.window)),_=>return Err(invalid())},
            Self::Page(page)=>match index{0=>(Some("fieldOffset"),Self::Text(&page.field_offset)),1=>(Some("diagnosticOffset"),Self::Text(&page.diagnostic_offset)),2=>(Some("choice"),Self::ChoicePage(page.choice.as_ref().ok_or_else(invalid)?)),_=>return Err(invalid())},
            Self::ChoicePage(page)=>match index{0=>(Some("path"),Self::Path(&page.path)),1=>(Some("offset"),Self::Text(&page.offset)),_=>return Err(invalid())},
            Self::Edits(rows)=>(None,Self::Edit(rows.get(index).ok_or_else(invalid)?)),
            Self::Edit(edit)=>match(edit,index){
                (StagedFormBindingEdit::Fixed{..},0)=>(Some("source"),Self::Text("fixed")),(StagedFormBindingEdit::Text{..},0)=>(Some("source"),Self::Text("text")),(StagedFormBindingEdit::Choice{..},0)=>(Some("source"),Self::Text("choice")),
                (StagedFormBindingEdit::Fixed{edit},1)=>(Some("edit"),Self::Fixed(edit)),(StagedFormBindingEdit::Text{path},1)=>(Some("path"),Self::Path(path)),
                (StagedFormBindingEdit::Choice{kind,..},1)=>(Some("kind"),Self::Text(if *kind==StagedFormChoiceEditKind::Set{"set"}else{"append"})),(StagedFormBindingEdit::Choice{path,..},2)=>(Some("path"),Self::Path(path)),(StagedFormBindingEdit::Choice{options,..},3)=>(Some("options"),Self::Options(options)),_=>return Err(invalid()),
            },
            Self::Fixed(edit)=>match(edit,index){
                (StagedFormEdit::Set{..},0)=>(Some("kind"),Self::Text("set")),(StagedFormEdit::Append{..},0)=>(Some("kind"),Self::Text("append")),(StagedFormEdit::Remove{..},0)=>(Some("kind"),Self::Text("remove")),
                (StagedFormEdit::Set{path,..}|StagedFormEdit::Append{path,..}|StagedFormEdit::Remove{path},1)=>(Some("path"),Self::Path(path)),(StagedFormEdit::Set{value,..}|StagedFormEdit::Append{value,..},2)=>(Some("value"),Self::Data(value)),_=>return Err(invalid()),
            },
            Self::Path(rows)=>{let part=rows.get(index).ok_or_else(invalid)?;if part.is_empty()||matches!(part.as_str(),"__proto__"|"prototype"|"constructor"){return Err(invalid());}(None,Self::Text(part))},
            Self::Options(rows)=>{if rows.len()>32{return Err(invalid());}let option=rows.get(index).ok_or_else(invalid)?;if option.key.len()>semio_framework_ui_contract::UiText::default().capacity(){return Err(resource());}if option.key.is_empty()||rows[..index].iter().any(|previous|previous.key.len()<=std::mem::size_of::<semio_framework_ui_contract::UiText>()&&previous.key==option.key){return Err(invalid());}(None,Self::Option(option))},
            Self::Option(option)=>match index{0=>(Some("key"),Self::Text(&option.key)),1=>(Some("value"),Self::Data(&option.value)),_=>return Err(invalid())},
            Self::Data(DslValue::Object(rows))=>{let(key,value)=rows.get(index).ok_or_else(invalid)?;(Some(key.as_str()),Self::Data(value))},Self::Data(DslValue::Array(rows))=>(None,Self::Data(rows.get(index).ok_or_else(invalid)?)),_=>return Err(invalid()),
        };Ok(pair)
    }
}
impl StagedPayloadSource for StagedBindingPayload<'_>{
    fn identity(&self)->usize{match self{Self::Action(binding)=>*binding as*const _ as usize,Self::Page(binding)=>(*binding as*const _ as usize)^1}}
    fn node(&self,path:&[usize])->Result<Node<'_>,ValueError>{let mut value=match self{Self::Action(binding)=>View::Action(binding),Self::Page(binding)=>View::PageBinding(binding)};let mut key=None;for index in path{let child=value.child(*index)?;key=child.0;value=child.1;}Ok(Node{key,value:value.kind()?})}
}
