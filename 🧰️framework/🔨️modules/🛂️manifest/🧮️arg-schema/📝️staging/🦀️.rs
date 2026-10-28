//! 📝️ Exact source-bound descriptions for ephemeral action arguments shared by native and web hosts.
use semio_framework_value::DslValue;
use serde::{Deserialize, Serialize};
#[path="limits.rs"]
mod limits;
pub use limits::*;
#[path="request.rs"]
mod request;
pub use request::*;
#[path="page.rs"]
mod page;
pub use page::*;
#[path="page-request.rs"]
mod page_request;
pub const STAGED_FORM_BODY_KEY: &str = "framework.action.form";
pub const STAGED_FORM_STATUS_EXTENSION: &str = "framework.stagedFormStatus";
pub const STAGED_FORM_EVENT_ACTION: &str = "framework.stageActionForm";
pub const STAGED_FORM_STATUS_DIAGNOSTICS: usize = 16;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormToken {
    pub action: String,
    pub generation: String,
    pub revision: String,
    pub window: String,
}
impl StagedFormToken {
    /// 🎟️ An issued generation is canonical exact u64 text and all other authorities are explicit.
    pub fn admitted(&self)->bool{!self.action.is_empty()&&!self.revision.is_empty()&&!self.window.is_empty()&&self.generation.len()<=20&&StagedFormTextFormat::DecimalU64.admits(&self.generation)}
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormLabel {
    pub en: String,
    pub de: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormChoice {
    pub id: String,
    pub label: StagedFormLabel,
    pub value: DslValue,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum StagedFormTextFormat {
    Text,
    DecimalI64,
    DecimalU64,
    Binary64,
    Guid,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum StagedFormProvenance {
    Declaration,
    Source,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum StagedFormField {
    Text {
        path: Vec<String>,
        label: StagedFormLabel,
        required: bool,
        format: StagedFormTextFormat,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        description: Option<StagedFormLabel>,
    },
    Choice {
        path: Vec<String>,
        label: StagedFormLabel,
        required: bool,
        provenance: StagedFormProvenance,
        choices: Vec<StagedFormChoice>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        resets: Vec<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        description: Option<StagedFormLabel>,
    },
    Constant {
        path: Vec<String>,
        label: StagedFormLabel,
        value: DslValue,
    },
    Record {
        path: Vec<String>,
        label: StagedFormLabel,
        fields: Vec<StagedFormField>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        create: Option<DslValue>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        #[value(default, skip_serializing_if = "std::ops::Not::not")]
        optional: bool,
    },
    Array {
        path: Vec<String>,
        label: StagedFormLabel,
        rows: Vec<StagedFormField>,
        add: Vec<StagedFormChoice>,
    },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormDiagnostic {
    pub path: Vec<String>,
    pub code: String,
    pub label: StagedFormLabel,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormRequest {
    pub token: StagedFormToken,
    pub args: DslValue,
    pub locale: semio_framework_ui_locale::Locale,
    #[serde(skip_serializing_if="Option::is_none")]
    #[value(skip_serializing_if="Option::is_none")]
    pub page:Option<StagedFormPage>,
}
impl StagedFormRequest{
    /// 📨️ Match one admitted donor to the actual real window without inspecting command values.
    pub fn context_matches(&self,window:Option<&str>,locale:semio_framework_ui_locale::Locale)->bool{self.token.admitted()&&window.is_some_and(|window|!window.is_empty()&&self.token.window==window)&&self.locale==locale}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormResponse {
    pub token: StagedFormToken,
    pub fields: Vec<StagedFormField>,
    pub diagnostics: Vec<StagedFormDiagnostic>,
    pub ready: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct StagedActionDefinition {
    pub body_key: String,
    #[serde(skip_serializing_if="Option::is_none")]
    #[value(skip_serializing_if="Option::is_none")]
    pub limits:Option<StagedFormLimits>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct StagedFormStatus {
    pub token: StagedFormToken,
    pub ready: bool,
    pub diagnostics: Vec<StagedFormDiagnostic>,
    pub diagnostic_offset: String,
    pub diagnostic_total: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum StagedFormEdit {
    Set { path: Vec<String>, value: DslValue },
    Append { path: Vec<String>, value: DslValue },
    Remove { path: Vec<String> },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormEvent {
    pub token: StagedFormToken,
    pub edits: Vec<StagedFormEdit>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum StagedFormLayout { Preserve, Replace }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum StagedFormChoiceEditKind { Set, Append }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormBindingOption { pub key:String, pub value:DslValue }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "source", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "source", rename_all = "camelCase", deny_unknown_fields)]
pub enum StagedFormBindingEdit {
    Fixed { edit:StagedFormEdit },
    Text { path:Vec<String> },
    Choice { kind:StagedFormChoiceEditKind, path:Vec<String>, options:Vec<StagedFormBindingOption> },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct StagedFormBinding { pub token:StagedFormToken, pub layout:StagedFormLayout, pub edits:Vec<StagedFormBindingEdit> }
pub enum StagedFormBorrowedEdit<'a> {
    Set { path:&'a[String], value:&'a DslValue },
    Append { path:&'a[String], value:&'a DslValue },
    Remove { path:&'a[String] },
}
pub struct StagedFormBindingPlan<'a> { binding:&'a StagedFormBinding, input:Option<&'a DslValue>, index:usize }
impl StagedFormBinding {
    /// 🧷️ Borrows edits only from an original resident UI binding already authorized by the host.
    pub fn plan<'a>(&'a self,status:&StagedFormStatus,input:Option<&'a DslValue>)->Result<StagedFormBindingPlan<'a>,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"staged form binding or original input is invalid");
        if !self.token.admitted()||!status.token.admitted()||self.token!=status.token{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"staging.stale"));}
        if self.edits.is_empty()||self.edits.len()>8{return Err(invalid());}
        if self.layout==StagedFormLayout::Preserve&&(self.edits.len()!=1||!matches!(&self.edits[0],StagedFormBindingEdit::Text{..})){return Err(invalid());}
        let mut inputs=0;
        for edit in &self.edits{
            let path=match edit{
                StagedFormBindingEdit::Fixed{edit}=>match edit{StagedFormEdit::Set{path,..}|StagedFormEdit::Append{path,..}|StagedFormEdit::Remove{path}=>path},
                StagedFormBindingEdit::Text{path}=>{inputs+=1;if !matches!(input,Some(DslValue::String(_))){return Err(invalid());}path},
                StagedFormBindingEdit::Choice{path,options,..}=>{
                    inputs+=1;
                    let Some(DslValue::String(key))=input else{return Err(invalid())};
                    if options.is_empty()||options.len()>32||!options.iter().any(|option|option.key==*key){return Err(invalid());}
                    for(index,option)in options.iter().enumerate(){if option.key.is_empty()||options[..index].iter().any(|previous|previous.key==option.key){return Err(invalid());}}
                    path
                },
            };
            if path.is_empty()||path.len()>64||path.iter().any(|part|part.is_empty()||matches!(part.as_str(),"__proto__"|"prototype"|"constructor")){return Err(invalid());}
        }
        if inputs>1||inputs==0&&input.is_some(){return Err(invalid());}
        Ok(StagedFormBindingPlan{binding:self,input,index:0})
    }
}
impl<'a> Iterator for StagedFormBindingPlan<'a>{
    type Item=StagedFormBorrowedEdit<'a>;
    fn next(&mut self)->Option<Self::Item>{
        let edit=self.binding.edits.get(self.index)?;self.index+=1;
        Some(match edit{
            StagedFormBindingEdit::Fixed{edit}=>match edit{
                StagedFormEdit::Set{path,value}=>StagedFormBorrowedEdit::Set{path,value},
                StagedFormEdit::Append{path,value}=>StagedFormBorrowedEdit::Append{path,value},
                StagedFormEdit::Remove{path}=>StagedFormBorrowedEdit::Remove{path},
            },
            StagedFormBindingEdit::Text{path}=>StagedFormBorrowedEdit::Set{path,value:self.input.expect("validated original text input remains borrowed")},
            StagedFormBindingEdit::Choice{kind,path,options}=>{
                let Some(DslValue::String(key))=self.input else{unreachable!("validated original choice input remains borrowed")};
                let value=&options.iter().find(|option|option.key==*key).expect("validated resident choice remains borrowed").value;
                match kind{StagedFormChoiceEditKind::Set=>StagedFormBorrowedEdit::Set{path,value},StagedFormChoiceEditKind::Append=>StagedFormBorrowedEdit::Append{path,value}}
            },
        })
    }
}
impl StagedFormTextFormat {
    /// 🔢️ Validate exact text without routing decimal ids or binary64 words through floating point.
    pub fn admits(self, value: &str) -> bool {
        match self {
            Self::Text => true,
            Self::DecimalI64 => canonical_integer(value) && value.parse::<i64>().is_ok(),
            Self::DecimalU64 | Self::Binary64 => canonical_integer(value) && value.parse::<u64>().is_ok(),
            Self::Guid => value.len() == 22 && matches!(value.as_bytes()[0], b'0'..=b'3') && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$')),
        }
    }
}
fn canonical_integer(value: &str) -> bool {
    value == "0" || {
        let digits = value.strip_prefix('-').unwrap_or(value);
        !digits.is_empty() && digits.as_bytes()[0] != b'0' && digits.bytes().all(|byte| byte.is_ascii_digit())
    }
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
