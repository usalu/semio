//! 📄️ A source-owned page transition replaces ephemeral layout while keeping command arguments exact.
use super::{StagedFormStatus,StagedFormToken,StagedFormTextFormat};
use serde::{Deserialize,Serialize};
use semio_framework_value::{ValueError,ValueRefusalKind};
pub const STAGED_FORM_PAGE_ACTION:&str="framework.stageActionFormPage";
pub const STAGED_FORM_CONTROL_PAGE:usize=16;
pub const STAGED_FORM_CHOICE_PAGE:usize=32;
#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedFormBodyLimits{pub content:usize,pub pages:usize,pub controls:usize}
pub const STAGED_FORM_BODY_LIMITS:StagedFormBodyLimits=StagedFormBodyLimits{content:16,pages:2,controls:18};
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct StagedFormPage {pub field_offset:String,pub diagnostic_offset:String,#[serde(skip_serializing_if="Option::is_none")]#[value(skip_serializing_if="Option::is_none")]pub choice:Option<StagedFormChoicePage>}
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormChoicePage {pub path:Vec<String>,pub offset:String}
impl StagedFormPage {
    pub fn admitted(&self)->bool{
        let valid=|value:&str|value.len()<=20&&StagedFormTextFormat::DecimalU64.admits(value);
        valid(&self.field_offset)&&valid(&self.diagnostic_offset)&&self.choice.as_ref().is_none_or(|choice|valid(&choice.offset)&&!choice.path.is_empty()&&choice.path.len()<=64&&choice.path.iter().all(|part|!part.is_empty()&&!matches!(part.as_str(),"__proto__"|"prototype"|"constructor")))
    }
}
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormPageBinding {pub token:StagedFormToken,pub page:StagedFormPage}
impl StagedFormPageBinding {
    /// 🎫️ The host authenticates the current resident node, publication, trigger and fixed binding first.
    pub fn destination(&self,status:&StagedFormStatus,current:&StagedFormToken)->Result<&StagedFormPage,ValueError>{
        if !current.admitted()||&status.token!=current||&self.token!=current{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"staging.stale"));}
        if !self.page.admitted(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"staging.page"));}Ok(&self.page)
    }
}
