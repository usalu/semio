//! 🧭️ Original source-bound type guidance retains caller reference drafts before strict command admission.
use super::domain_cursor::{DomainCursor, DomainPopulation};
use super::type_input_kind::{Cursor as InputKindCursor, Kind as InputKind};
use super::*;
#[path = "type_input_choices.rs"]
mod choice_rows;
#[path="type_input_list_copy.rs"]
mod list_copy;
#[path="type_input_typed_copy.rs"]mod typed_copy;
#[path="type_input_set_controls.rs"]mod set_controls;
use choice_rows::Choices;
const CARDINALITY:[&str;16]=["0","1","2","3","4","5","6","7","8","9","10","11","12","13","14","15"];
use semio_framework_job::{OriginalConstructionDemand, OriginalSourceBinding, OriginalTurnFailure, StepContext, admit_original_construction};
use semio_framework_plugin::action_staging::*;
use semio_framework_plugin::app::staged_form::*;
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::{ManuallyDrop, size_of, size_of_val};
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "staging.type-input")
}
fn stale() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "staging.stale")
}
fn empty_token() -> StagedFormToken {
    StagedFormToken { action: String::new(), generation: String::new(), revision: String::new(), window: String::new() }
}
fn token_part(token: &StagedFormToken, index: usize) -> &str {
    match index {
        0 => &token.action,
        1 => &token.generation,
        2 => &token.revision,
        _ => &token.window,
    }
}
fn token_part_mut(token: &mut StagedFormToken, index: usize) -> &mut String {
    match index {
        0 => &mut token.action,
        1 => &mut token.generation,
        2 => &mut token.revision,
        _ => &mut token.window,
    }
}
fn marker(request: &StagedFormRequest) -> Result<(&str, &DslValue), ValueError> {
    let DslValue::Object(fields) = &request.args else { return Err(invalid()) };
    if fields.len() != 2 {
        return Err(invalid());
    }
    let path = fields.iter().find(|(key, _)| key == "path").and_then(|(_, value)| value.as_str()).ok_or_else(invalid)?;
    let value = &fields.iter().find(|(key, _)| key == "value").ok_or_else(invalid)?.1;
    super::type_input::request(value).ok_or_else(invalid)?;
    Ok((path, value))
}
fn canonical_decimal(text: &str, positive: bool) -> Option<u64> {
    if text.is_empty() || text.len() > 20 || !text.bytes().all(|byte| byte.is_ascii_digit()) || text.len() > 1 && text.starts_with('0') {
        return None;
    }
    let value = text.parse().ok()?;
    (!positive || value > 0).then_some(value)
}
fn revision_matches(text: &str, revision: [u8; 32]) -> bool {
    text.len() == 64 && text.bytes().enumerate().all(|(index, byte)| byte == b"0123456789abcdef"[if index % 2 == 0 { (revision[index / 2] >> 4) as usize } else { (revision[index / 2] & 15) as usize }])
}
fn rows(request: &StagedFormRequest) -> Result<&[DslValue], ValueError> {
    Ok(super::type_input::request(marker(request)?.1).ok_or_else(invalid)?.1.unwrap_or(&[]))
}
fn row_data(request: &StagedFormRequest, ordinal: usize) -> Result<(usize, InputKind, &str), ValueError> {
    let DslValue::Object(fields) = rows(request)?.get(ordinal).ok_or_else(invalid)? else { return Err(invalid()) };
    if fields.len() != 2 {
        return Err(invalid());
    }
    let index = fields.iter().find(|(key, _)| key == "index").and_then(|(_, value)| value.as_str()).and_then(|text| canonical_decimal(text, false)).and_then(|index| usize::try_from(index).ok()).ok_or_else(invalid)?;
    let DslValue::Object(value) = &fields.iter().find(|(key, _)| key == "value").ok_or_else(invalid)?.1 else { return Err(invalid()) };
    if value.len() != 2 {
        return Err(invalid());
    }
    let kind = match value.iter().find(|(key, _)| key == "kind").and_then(|(_, value)| value.as_str()) {
        Some("reference") => InputKind::Reference,
        Some("string") => InputKind::String,
        Some("integer") => InputKind::Integer,
        Some("enum") => InputKind::Enum,
        Some("real") => InputKind::Real,
        Some("binary") => InputKind::Binary,
        Some("aggregate")=>InputKind::List,
        Some("typedValue")=>InputKind::Select,
        _ => return Err(invalid()),
    };
    let raw=&value.iter().find(|(key,_)|key=="value").ok_or_else(invalid)?.1;
    if kind==InputKind::Select{let(_,items)=super::type_input::typed_constructor(raw).ok_or_else(invalid)?;let text=match items.first(){None=>"",Some(value)=>{let value=value.get("value").ok_or_else(invalid)?;if matches!(value,DslValue::Array(_)){""}else{value.as_str().or_else(||value.get("bits").and_then(DslValue::as_str)).ok_or_else(invalid)?}}};return Ok((index,kind,text));}
    if kind==InputKind::List{let DslValue::Array(items)=raw else{return Err(invalid());};if items.len()>=STAGED_FORM_BODY_LIMITS.content{return Err(invalid());}return Ok((index,kind,""));}
    let text=if kind==InputKind::Real{let DslValue::Object(bits)=raw else{return Err(invalid());};if bits.len()!=1||bits[0].0!="bits"{return Err(invalid());}bits[0].1.as_str().ok_or_else(invalid)?}else{raw.as_str().ok_or_else(invalid)?};
    Ok((index, kind, text))
}
fn row_text(request: &StagedFormRequest, ordinal: usize) -> Result<(usize, &str), ValueError> {
    let (index, _, text) = row_data(request, ordinal)?;
    Ok((index, text))
}
fn list_members(request:&StagedFormRequest,row:usize)->Result<&[DslValue],ValueError>{row_data(request,row)?;let Some(DslValue::Array(items))=rows(request)?.get(row).and_then(|row|row.get("value")).and_then(|value|value.get("value"))else{return Err(invalid());};Ok(items)}
fn list_word(request:&StagedFormRequest,row:usize,member:usize)->Result<&str,ValueError>{let items=list_members(request,row)?;if member>=items.len(){return Err(invalid());}list_copy::word(items,member,list_copy::Mode::Real)}
fn constructor(request:&StagedFormRequest,row:usize)->Result<&DslValue,ValueError>{row_data(request,row)?;rows(request)?.get(row).and_then(|row|row.get("value")).and_then(|value|value.get("value")).ok_or_else(invalid)}
fn selected_numeric(request:&StagedFormRequest,row:usize)->Result<Option<(InputKind,&str)>,ValueError>{let(_,items)=super::type_input::typed_constructor(constructor(request,row)?).ok_or_else(invalid)?;let Some(child)=items.first()else{return Ok(None)};let kind=match child.get("kind").and_then(DslValue::as_str){Some("integer")=>InputKind::Integer,Some("real")=>InputKind::Real,_=>return Err(invalid())};let value=if kind==InputKind::Real{super::type_input::real_word(child).ok_or_else(invalid)?}else{child.get("value").and_then(DslValue::as_str).ok_or_else(invalid)?};Ok(Some((kind,value)))}
const SET_PAGE:usize=(STAGED_FORM_BODY_LIMITS.content-2)/2;
fn set_members(request:&StagedFormRequest,row:usize)->Result<&[DslValue],ValueError>{super::type_input::set_members(constructor(request,row)?).ok_or_else(invalid)}
fn set_word(request:&StagedFormRequest,row:usize,member:usize)->Result<&str,ValueError>{super::type_input::reference_text(set_members(request,row)?.get(member).ok_or_else(invalid)?).ok_or_else(invalid)}
fn collection_word(request:&StagedFormRequest,row:usize,member:usize,kind:InputKind)->Result<&str,ValueError>{let value=set_members(request,row)?.get(member).ok_or_else(invalid)?;match kind{InputKind::Real=>super::type_input::real_word(value),InputKind::Integer=>super::type_input::integer_word(value),InputKind::Reference=>super::type_input::reference_text(value),_=>None}.ok_or_else(invalid)}
fn set_window(request:&StagedFormRequest,position:usize,target:usize,count:usize,leaves:usize)->Result<(usize,usize),ValueError>{let mut offset=0;if let Some(choice)=request.page.as_ref().and_then(|page|page.choice.as_ref()){if choice.path.len()==4&&choice.path[0]=="entities"&&canonical_decimal(&choice.path[1],false)==Some(position as u64)&&choice.path[2]=="args"&&canonical_decimal(&choice.path[3],false)==Some(target as u64){offset=canonical_decimal(&choice.offset,false).and_then(|value|usize::try_from(value).ok()).ok_or_else(invalid)?;if leaves>32&&offset%31==0&&offset<leaves{offset=0;}else if offset%SET_PAGE!=0||offset>=count&&offset!=0{return Err(invalid());}}}Ok((offset,count.saturating_sub(offset).min(SET_PAGE)))}
fn signed_decimal(text: &str) -> Option<i64> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if text.len() > 20 || digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) || digits.len() > 1 && digits.starts_with('0') || text == "-0" {
        return None;
    }
    text.parse().ok()
}
fn row_value_mut(row: &mut DslValue) -> Result<&mut String, ValueError> {
    let DslValue::Object(fields) = row else { return Err(invalid()) };
    let DslValue::Object(fields) = &mut fields.iter_mut().find(|(key, _)| key == "value").ok_or_else(invalid)?.1 else { return Err(invalid()) };
    let value=&mut fields.iter_mut().find(|(key,_)|key=="value").ok_or_else(invalid)?.1;
    match value{DslValue::String(value)=>Ok(value),DslValue::Object(bits)=>{if bits.len()!=1||bits[0].0!="bits"{return Err(invalid());}let DslValue::String(value)=&mut bits[0].1 else{return Err(invalid());};Ok(value)},_=>Err(invalid())}
}
#[derive(semio_framework_value::RetireOwned)]
struct RowParts {
    strings: [Option<String>; 8],
    outer: Option<Vec<(String, DslValue)>>,
    inner: Option<Vec<(String, DslValue)>>,
    bits: Option<Vec<(String, DslValue)>>,
    structured:Option<DslValue>,
}
fn decimal(mut value: usize, buffer: &mut [u8; 20]) -> &str {
    let mut index = 20;
    loop {
        index -= 1;
        buffer[index] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    std::str::from_utf8(&buffer[index..]).expect("decimal digits")
}
#[derive(semio_framework_value::RetireOwned)]
struct ChoiceParts {
    strings: [Option<String>; 6],
    payload: Option<Vec<(String, DslValue)>>,
    bits:Option<Vec<(String,DslValue)>>,
    structured:Option<DslValue>,
}
fn attribute_text<'a>(name:&'a str,member:Option<usize>,buffer:&'a mut[u8;256])->Result<&'a str,ValueError>{let Some(member)=member else{return Ok(name)};let mut digits=[0u8;20];let digits=decimal(member.checked_add(1).ok_or_else(invalid)?,&mut digits);let length=name.len()+digits.len()+2;if length>buffer.len(){return Err(invalid());}buffer[..name.len()].copy_from_slice(name.as_bytes());buffer[name.len()]=b'[';buffer[name.len()+1..length-1].copy_from_slice(digits.as_bytes());buffer[length-1]=b']';std::str::from_utf8(&buffer[..length]).map_err(|_|invalid())}
fn text_path_capacity(field:&Field,row:&str,member:&str)->usize{let(count,bytes)=if field.selection{if field.member.is_some(){if field.kind==InputKind::Real{(11,45+row.len()+member.len())}else{(10,41+row.len()+member.len())}}else if field.kind==InputKind::Real{(9,40+row.len())}else{(8,36+row.len())}}else if field.member.is_some(){(8,34+row.len()+member.len())}else if field.kind==InputKind::Real{(6,29+row.len())}else{(5,25+row.len())};count*size_of::<String>()+bytes}
#[derive(semio_framework_value::RetireOwned)]
struct Navigation {
    binding: StagedFormPageBinding,
    previous: bool,
    field: usize,
}
#[derive(semio_framework_value::RetireOwned)]
struct Page {
    request: Option<StagedFormRequest>,
    status: StagedFormStatus,
    fields: Vec<Field>,
    navigation: Vec<Navigation>,
}
#[derive(semio_framework_value::RetireOwned)]
struct Field {
    binding: StagedFormBinding,
    attribute: String,
    index: usize,
    ordinal:usize,
    member:Option<usize>,
    operation:Option<bool>,
    selection:bool,
    selected_choice:Option<usize>,
    row: usize,
    present: bool,
    kind: InputKind,
    choices: Option<Choices>,
}
/// 🪟️ Only completed original-owned fields reach the common artifact-neutral form body.
pub struct IfcPreparedTypeInputSource {
    page: ManuallyDrop<Option<Page>>,
}
impl IfcPreparedTypeInputSource {
    pub fn context_matches(&self, window: Option<&str>, locale: Locale) -> Result<bool, ValueError> {
        Ok(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?.context_matches(window, locale))
    }
}
impl StagedBodySource for IfcPreparedTypeInputSource {
    fn identity(&self) -> usize {
        self as *const _ as usize
    }
    fn title(&self) -> &str {
        match self.page.as_ref().unwrap().request.as_ref().unwrap().locale {
            Locale::En => "Supply required type values",
            Locale::De => "Erforderliche Typwerte angeben",
        }
    }
    fn status(&self) -> &StagedFormStatus {
        &self.page.as_ref().unwrap().status
    }
    fn controls(&self) -> usize {
        self.page.as_ref().map_or(0, |page| page.fields.len() + page.navigation.len())
    }
    fn control(&self, index: usize) -> Result<StagedBodyControl<'_>, ValueError> {
        let page = self.page.as_ref().ok_or_else(stale)?;
        let request = page.request.as_ref().ok_or_else(stale)?;
        if index >= page.fields.len() {
            let navigation = page.navigation.get(index - page.fields.len()).ok_or_else(invalid)?;
            let label = match (navigation.previous, request.locale) {
                (true, Locale::En) => "Previous choices",
                (true, Locale::De) => "Vorherige Auswahl",
                (false, Locale::En) => "Next choices",
                (false, Locale::De) => "Weitere Auswahl",
            };
            return Ok(StagedBodyControl {
                scalar: StagedScalarControl {
                    owner: self.identity(),
                    address: u64::MAX - 2 + (index - page.fields.len()) as u64,
                    controller: "s.stdio.ifc@4x3/*#editor",
                    label,
                    description: Some(&page.fields[navigation.field].attribute),
                    disabled: false,
                    kind: StagedScalarKind::Button { page: true },
                },
                required: false,
                error: None,
                binding: Some(StagedBodyBinding::Page(&navigation.binding)),
            });
        }
        let field = page.fields.get(index).ok_or_else(invalid)?;
        let value=if field.operation.is_some(){""}else if field.present{if let Some(member)=field.member{if field.selection{collection_word(request,field.row,member,field.kind)?}else{list_word(request,field.row,member)?}}else if field.kind==InputKind::Select{if let Some(index)=field.selected_choice{field.choices.as_ref().ok_or_else(invalid)?.rows().get(index).ok_or_else(invalid)?.key}else{super::type_input::typed_constructor(constructor(request,field.row)?).ok_or_else(invalid)?.0}}else if field.kind==InputKind::List{CARDINALITY[list_members(request,field.row)?.len()]}else if field.kind==InputKind::Number{if field.selection{selected_numeric(request,field.row)?.map_or("",|(kind,_)|kind.tag())}else{row_data(request,field.row)?.1.tag()}}else{row_text(request,field.row)?.1}}else{""};
        let label = if let Some(remove)=field.operation{let array=field.kind==InputKind::Array;let empty=super::type_input::typed_constructor(constructor(request,field.row)?).ok_or_else(invalid)?.1.is_empty();if field.kind==InputKind::List{match(remove,empty,request.locale){(true,_,Locale::En)=>"Remove integer word",(true,_,Locale::De)=>"INTEGER-Wort entfernen",(false,true,Locale::En)=>"Create LIST aggregate",(false,true,Locale::De)=>"LIST-Aggregat erstellen",(false,false,Locale::En)=>"Add integer word",(false,false,Locale::De)=>"INTEGER-Wort hinzufügen"}}else{match(remove,array,empty,request.locale){(true,true,_,Locale::En)=>"Remove real word",(true,true,_,Locale::De)=>"REAL-Wort entfernen",(true,false,_,Locale::En)=>"Remove reference",(true,false,_,Locale::De)=>"Referenz entfernen",(false,true,true,Locale::En)=>"Create ARRAY aggregate",(false,true,true,Locale::De)=>"ARRAY-Aggregat erstellen",(false,true,false,Locale::En)=>"Add real word",(false,true,false,Locale::De)=>"REAL-Wort hinzufügen",(false,false,true,Locale::En)=>"Create SET aggregate",(false,false,true,Locale::De)=>"SET-Aggregat erstellen",(false,false,false,Locale::En)=>"Add reference",(false,false,false,Locale::De)=>"Referenz hinzufügen"}}}else{match (field.kind, field.attribute.as_str(), request.locale) {
            (InputKind::Reference, "Profile", Locale::En) => "Profile reference",
            (InputKind::Reference, "Profile", Locale::De) => "Profilreferenz",
            (InputKind::Reference, _, Locale::En) => "Required reference",
            (InputKind::Reference, _, Locale::De) => "Erforderliche Referenz",
            (InputKind::String, _, Locale::En) => "Required text",
            (InputKind::String, _, Locale::De) => "Erforderlicher Text",
            (InputKind::Integer, _, Locale::En) => "Required integer",
            (InputKind::Integer, _, Locale::De) => "Erforderliche Ganzzahl",
            (InputKind::Enum, _, Locale::En) => "Required declared choice",
            (InputKind::Enum, _, Locale::De) => "Erforderliche deklarierte Auswahl",
            (InputKind::Real, _, Locale::En) => "Required binary64 word",
            (InputKind::Real, _, Locale::De) => "Erforderliches Binary64-Wort",
            (InputKind::Number, _, Locale::En) => "Required numeric representation",
            (InputKind::Number, _, Locale::De) => "Erforderliche Zahlendarstellung",
            (InputKind::Binary, _, Locale::En) => "Required encoded BINARY",
            (InputKind::Binary, _, Locale::De) => "Erforderliche BINARY-Kodierung",
            (InputKind::List,_,Locale::En)=>"Required LIST",
            (InputKind::List,_,Locale::De)=>"Erforderliche LIST",
            (InputKind::Set,_,Locale::En)=>"Required SET",
            (InputKind::Set,_,Locale::De)=>"Erforderliche Menge",
            (InputKind::Array,_,Locale::En)=>"Required ARRAY",
            (InputKind::Array,_,Locale::De)=>"Erforderliches ARRAY",
            (InputKind::Select,_,Locale::En)=>"Required SELECT member",
            (InputKind::Select,_,Locale::De)=>"Erforderlicher Auswahltyp",
        }};
        Ok(StagedBodyControl {
            scalar: StagedScalarControl {
                owner: self.identity(),
                address: index as u64,
                controller: "s.stdio.ifc@4x3/*#editor",
                label,
                description: Some(&field.attribute),
                disabled: false,
                kind: if field.operation.is_some(){StagedScalarKind::Button{page:false}}else if matches!(field.kind,InputKind::Enum|InputKind::Number|InputKind::List|InputKind::Select) { StagedScalarKind::Choice { value, choices: field.choices.as_ref().ok_or_else(invalid)?.rows() } } else { StagedScalarKind::Text { value } },
            },
            required: field.operation.is_none(),
            error: None,
            binding: Some(StagedBodyBinding::Action(&field.binding)),
        })
    }
}
impl Drop for IfcPreparedTypeInputSource {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.page.is_none(), "prepared type input source requires original retirement");
    }
}
impl RetireOwned for IfcPreparedTypeInputSource {
    fn retirement(mut self) -> Box<dyn RetirementCursor> {
        self.page.take().expect("completed type page owns its carrier").retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        self.page.as_ref()?.retirement_birth_bytes()
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
#[derive(Clone, Copy, Default)]
struct State {
    phase: u8,
    position: usize,
    row: usize,
    matches: usize,
    target: usize,
    part: usize,
    argument: usize,
    active: bool,
    ready: bool,
    closing: bool,
    failed: bool,
    profile: usize,
    guard_part: usize,
    guard_slot: usize,
    guard_direct: bool,
    guard_values: bool,
    field: usize,
    copying: usize,
    offset: usize,
    row_part: usize,
    kinds: [InputKind; 16],
    selected: [usize; 16],
    selected_leaves:[usize;16],
    leaf_counts:[usize;16],
    first_leaves:[Option<&'static str>;16],
    leaf_name:Option<&'static str>,
    upper:[usize;16],
    list_resume:u8,
    typed_resume:u8,
    tag_checked:bool,
    leaves:[InputKind;16],
    references:[Option<usize>;16],
    choice_kind:InputKind,
    member: usize,
    active_target: Option<usize>,
    active_found: bool,
    page_offset: usize,
    navigation: [Option<(usize, usize)>; 2],
    nav: usize,
    option: usize,
    choice_part: usize,
    scan: usize,
}
struct CandidatePopulation<'a> {
    source: &'a IfcSnapshot,
    position: usize,
    name: &'a str,
}
impl DomainPopulation for CandidatePopulation<'_> {
    fn len(&self) -> usize {
        self.source.entities.len()
    }
    fn id(&self, row: usize) -> u64 {
        self.source.entities[row].id
    }
    fn constituent_count(&self, row: usize) -> usize {
        self.source.entities[row].complex.len() + 1
    }
    fn constituent_name(&self, row: usize, part: usize) -> &str {
        if row == self.position && part == 0 {
            self.name
        } else if part == 0 {
            &self.source.entities[row].name
        } else {
            &self.source.entities[row].complex[part - 1].name
        }
    }
}
/// 🧵️ The host retains its original snapshot Arc through this source-bound preparation and retirement.
pub struct IfcTypeInputSourceWork<C: IfcDetailsProfile + 'static> {
    request: ManuallyDrop<Option<StagedFormRequest>>,
    page: ManuallyDrop<Option<Page>>,
    edit: ManuallyDrop<Option<StagedFormBindingEdit>>,
    error: ManuallyDrop<Option<String>>,
    placeholders: ManuallyDrop<Option<Vec<super::type_input::Input>>>,
    fields: ManuallyDrop<Option<Vec<Field>>>,
    copied_rows: ManuallyDrop<Option<Vec<DslValue>>>,
    copied_row: ManuallyDrop<Option<DslValue>>,
    row_parts: ManuallyDrop<Option<RowParts>>,
    scalar: ManuallyDrop<Option<String>>,
    list_values:ManuallyDrop<Option<Vec<IfcValue>>>,
    list_copy:list_copy::ListCopy,
    typed_copy:typed_copy::TypedCopy,
    source_loan: ManuallyDrop<Option<super::source_loan::SourceLoan>>,
    candidate: ManuallyDrop<Option<Box<super::type_guidance_candidate::GuidedCandidate<C>>>>,
    choice_parts: ManuallyDrop<Option<ChoiceParts>>,
    navigation: ManuallyDrop<Option<Vec<Navigation>>>,
    nav_item: ManuallyDrop<Option<Navigation>>,
    kind_cursor: InputKindCursor,
    leaf_cursor:super::type_input_select::Cursor,
    domains: [Option<&'static str>; 16],
    leaf_domains:[Option<&'static str>;16],
    plan: super::type_remap::Plan,
    domain: DomainCursor,
    retirement: RetainedCloneClose,
    binding: Option<(OriginalSourceBinding, usize, [u8; 32])>,
    state: State,
    profile: std::marker::PhantomData<fn() -> C>,
}
impl<C: IfcDetailsProfile + 'static> Default for IfcTypeInputSourceWork<C> {
    fn default() -> Self {
        Self {
            request: ManuallyDrop::new(None),
            page: ManuallyDrop::new(None),
            edit: ManuallyDrop::new(None),
            error: ManuallyDrop::new(None),
            placeholders: ManuallyDrop::new(None),
            fields: ManuallyDrop::new(None),
            copied_rows: ManuallyDrop::new(None),
            copied_row: ManuallyDrop::new(None),
            row_parts: ManuallyDrop::new(None),
            scalar: ManuallyDrop::new(None),
            list_values:ManuallyDrop::new(None),
            list_copy:Default::default(),
            typed_copy:Default::default(),
            source_loan: ManuallyDrop::new(None),
            candidate: ManuallyDrop::new(None),
            choice_parts: ManuallyDrop::new(None),
            navigation: ManuallyDrop::new(None),
            nav_item: ManuallyDrop::new(None),
            kind_cursor: Default::default(),
            leaf_cursor:Default::default(),
            domains: Default::default(),
            leaf_domains:Default::default(),
            plan: Default::default(),
            domain: Default::default(),
            retirement: Default::default(),
            binding: None,
            state: State { ready: true, ..Default::default() },
            profile: std::marker::PhantomData,
        }
    }
}
fn metadata(cx: &mut StepContext<'_>, bytes: usize, change: impl FnOnce()) -> Result<bool, OriginalTurnFailure> {
    if cx.is_cancelled() {
        return Err(ValueError::literal(ValueRefusalKind::Canceled, "staging cancelled").into());
    }
    if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() }) {
        return Ok(false);
    }
    change();
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    cx.consume_fuel(1);
    Ok(true)
}
fn copy_text(target: &mut String, text: &str, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    copy_text_extra(target, text, 0, cx)
}
fn copy_text_extra(target: &mut String, text: &str, extra: usize, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    if !target.is_empty() || text.len() > 512 {
        return Err(invalid().into());
    }
    let mut output = None;
    let physical = OriginalConstructionDemand { items: 1, copy_bytes: text.len() + size_of::<String>() + size_of::<State>() + extra, capacity_bytes: text.len(), release_bytes: 0, depth: 1 };
    admit_original_construction(cx, &mut output, physical, |_| {
        target.try_reserve_exact(text.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type guide text"))?;
        target.push_str(text);
        Ok(((), RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: target.capacity(), ..Default::default() }))
    })
}
fn construct_row_string(output: &mut Option<String>, reserve: usize, text: &str, physical: OriginalConstructionDemand, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    admit_original_construction(cx, output, physical, |_| {
        let mut value = String::new();
        value.try_reserve_exact(reserve).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type guide row text"))?;
        value.push_str(text);
        let capacity = value.capacity();
        Ok((value, RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() }))
    })
}
impl<C: IfcDetailsProfile + 'static> IfcTypeInputSourceWork<C> {
    fn source_demands(&self)->Result<RetirementDemand,ValueError>{match self.source_loan.as_ref(){Some(loan)=>loan.demands(),None=>Ok(RetirementDemand{copy_bytes:size_of::<super::source_loan::SourceLoan>()+size_of_val(&self.source_loan),depth:1,..Default::default()})}}
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.source_loan.as_ref().and_then(|loan|loan.source()).is_none(){return self.source_demands();}
        if self.state.phase==80{
            let base=4*size_of::<State>()+2*size_of::<bool>();
            let child=if let Some(candidate)=self.candidate.as_ref(){let source=self.source_loan.as_ref().and_then(|loan|loan.source()).ok_or_else(stale)?;candidate.demands(source,source.try_authority()?.as_ref(),4096)?}else{RetirementDemand{copy_bytes:size_of::<super::type_guidance_candidate::GuidedCandidate<C>>()+size_of::<Box<super::type_guidance_candidate::GuidedCandidate<C>>>(),capacity_bytes:size_of::<super::type_guidance_candidate::GuidedCandidate<C>>(),depth:1,..Default::default()}};
            return Ok(RetirementDemand{copy_bytes:base.checked_add(child.copy_bytes).ok_or_else(invalid)?,capacity_bytes:child.capacity_bytes,release_bytes:child.release_bytes,depth:child.depth.max(1)});
        }
        if matches!(self.state.phase,92|94|95){let own=4*size_of::<State>()+2*size_of::<typed_copy::TypedCopy>();let child=if self.state.phase==95{if self.typed_copy.empty(){RetirementDemand::default()}else{self.typed_copy.close_demands(4096)?}}else{let choice=if self.state.phase==94{Some((self.choice_word()?,self.state.choice_kind))}else{None};let input=Self::typed_spec(self.page.as_ref().ok_or_else(stale)?,&self.state,&self.domains,choice)?;self.typed_copy.demands(&input)?};return Ok(RetirementDemand{copy_bytes:own.checked_add(child.copy_bytes).ok_or_else(invalid)?,capacity_bytes:child.capacity_bytes,release_bytes:child.release_bytes,depth:child.depth.max(1)});}
        if matches!(self.state.phase,84|85|89){let own=4*size_of::<State>()+2*size_of::<list_copy::ListCopy>();let child=if self.state.phase==89{if self.list_copy.empty(){RetirementDemand::default()}else{self.list_copy.close_demands(4096)?}}else{let members=Self::copy_members(self.page.as_ref().ok_or_else(stale)?,&self.state)?;let count=if self.state.phase==84{members.len()}else{self.choice_word()?.parse().map_err(|_|invalid())?};self.list_copy.demands(members,count,list_copy::Mode::Real)?};return Ok(RetirementDemand{copy_bytes:own.checked_add(child.copy_bytes).ok_or_else(invalid)?,capacity_bytes:child.capacity_bytes,release_bytes:child.release_bytes,depth:child.depth.max(1)});}

        let base = 4 * size_of::<State>() + 2 * 256 + size_of::<IfcValue>() + 2*size_of::<super::type_input_select::Cursor>();
        let mut demand = RetirementDemand { copy_bytes: base, depth: 1, ..Default::default() };
        let text = |text: &str| RetirementDemand { copy_bytes: text.len() + size_of::<String>() + size_of::<State>(), capacity_bytes: text.len(), depth: 1, ..Default::default() };
        match self.state.phase {
            4 => demand.copy_bytes += self.next_field_count()?*size_of::<Field>() + size_of::<super::type_input::Input>(),
            33 => demand.copy_bytes += size_of::<StagedFormBindingEdit>() + size_of::<ChoiceParts>(),
            34 => {
                demand.copy_bytes += size_of::<Vec<String>>();
                demand.capacity_bytes = self.choice_path_len()? * size_of::<String>();
            }
            35 => {
                let (_, length, pin) = self.choice_shape()?;
                demand.capacity_bytes = (length + usize::from(pin.is_some())) * size_of::<StagedFormBindingOption>();
            }
            36 => {
                let (_, length, pin) = self.choice_shape()?;
                demand.copy_bytes += size_of::<Choices>();
                demand.capacity_bytes = (length + usize::from(pin.is_some())) * size_of::<StagedScalarChoice>();
            }
            37 => {
                let StagedFormBindingEdit::Choice { path, .. } = self.edit.as_ref().ok_or_else(invalid)? else { return Err(invalid()) };
                if path.len() == self.state.choice_part {
                    demand.copy_bytes += size_of::<String>();
                } else {
                    let mut buffer = [0; 20];
                    let word = self.choice_path_word(self.state.choice_part,&mut buffer)?;
                    let mut demand = text(word);
                    demand.copy_bytes += 40;
                    return Ok(demand);
                }
            }
            38 if self.state.choice_part < self.choice_string_count()? => {
                let word = self.choice_text()?;
                demand.capacity_bytes = word.len();
                demand.copy_bytes += size_of::<String>() + if self.choice_raw()?{0}else{2*word.len()};
            }
            39 => {
                demand.capacity_bytes = 2 * size_of::<(String, DslValue)>();
                demand.copy_bytes += size_of::<Vec<(String, DslValue)>>();
            }
            40 => demand.copy_bytes += 4 * size_of::<ChoiceParts>() + 2 * size_of::<StagedFormBindingOption>() + size_of::<StagedScalarChoice>(),
            82=>demand.copy_bytes+=192+size_of::<String>(),
            83=>{demand.copy_bytes+=size_of::<Vec<(String,DslValue)>>();demand.capacity_bytes=size_of::<(String,DslValue)>();},
            86=>{demand.copy_bytes+=size_of::<Vec<IfcValue>>();demand.capacity_bytes=list_members(self.request.as_ref().ok_or_else(stale)?,self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row)?.len()*size_of::<IfcValue>();},
            87=>demand.copy_bytes+=2*size_of::<Vec<IfcValue>>()+size_of::<IfcValue>(),
            96=>{demand.copy_bytes+=size_of::<Vec<IfcValue>>();demand.capacity_bytes=set_members(self.request.as_ref().ok_or_else(stale)?,self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row)?.len().checked_mul(size_of::<IfcValue>()).ok_or_else(invalid)?;},
            97|100|102=>demand.copy_bytes+=2*size_of::<Vec<IfcValue>>()+2*size_of::<IfcValue>()+2*size_of::<String>(),
            98=>{demand.copy_bytes+=size_of::<String>();demand.capacity_bytes=super::type_input::typed_constructor(constructor(self.request.as_ref().ok_or_else(stale)?,self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row)?).ok_or_else(invalid)?.0.len();},
            99=>demand.copy_bytes+=192+size_of::<String>(),
            101=>demand.capacity_bytes=size_of::<IfcValue>(),
            62 => {
                demand.capacity_bytes = self.state.navigation.iter().flatten().count() * size_of::<Navigation>();
            }
            63 | 70 => demand.copy_bytes += 2 * size_of::<Navigation>(),
            64 => return Ok(text(token_part(&self.request.as_ref().ok_or_else(stale)?.token, self.state.part))),
            65 | 66 => return Ok(text("0")),
            67 => {
                let mut buffer = [0; 20];
                let mut demand = text(decimal(self.nav_spec().ok_or_else(invalid)?.1, &mut buffer));
                demand.copy_bytes += 40;
                return Ok(demand);
            }
            68 => {
                demand.capacity_bytes = 4 * size_of::<String>();
            }
            69 => {
                let path = &self.nav_item.as_ref().ok_or_else(invalid)?.binding.page.choice.as_ref().ok_or_else(invalid)?.path;
                if path.len() == self.state.part {
                    demand.copy_bytes += size_of::<String>();
                } else {
                    let mut buffer = [0; 20];
                    let mut demand = text(self.page_path_text(self.state.part, self.nav_spec().ok_or_else(invalid)?.0, &mut buffer)?);
                    demand.copy_bytes += 40;
                    return Ok(demand);
                }
            }

            30 => {
                demand.copy_bytes += size_of::<String>();
                demand.capacity_bytes = row_data(self.request.as_ref().ok_or_else(stale)?, self.fields.as_ref().and_then(|fields| fields.last()).ok_or_else(invalid)?.row)?.2.len();
            }
            31 => demand.copy_bytes += 192 + size_of::<String>(),
            32 => demand.copy_bytes += 2 * size_of::<String>() + size_of::<IfcValue>(),
            24 if self.page.as_ref().is_some_and(|page| self.state.field == page.fields.len()) => {}
            23 => demand.capacity_bytes = self.state.target * size_of::<super::type_input::Input>(),
            25 => demand.capacity_bytes = STAGED_FORM_BODY_LIMITS.content * size_of::<Field>(),
            91|93=>demand.copy_bytes+=2*size_of::<InputKindCursor>(),
            3 => {
                demand.copy_bytes += super::type_remap::Plan::copy_bytes() + size_of::<super::type_input::Input>() + 2 * size_of::<InputKindCursor>();
                demand.capacity_bytes = self.plan.capacity_bytes(self.state.target)?;
            }
            5 => {
                demand.copy_bytes += 4 * size_of::<DomainCursor>();
                demand.capacity_bytes = 256;
            }
            7 if self.error.is_some() || !self.retirement.is_empty() => return self.retirement.next_owner_demands::<String>(self.error.is_some(), &None, 4096),
            6 => demand.copy_bytes += size_of::<Page>() + 2 * size_of::<StagedFormRequest>() + size_of::<Vec<Field>>() + size_of::<Vec<Navigation>>(),
            8 | 24 => {
                let page = self.page.as_ref().ok_or_else(stale)?;
                return Ok(text(token_part(&page.request.as_ref().ok_or_else(stale)?.token, self.state.part)));
            }
            9 | 10 => return Ok(text("0")),
            11 => {
                let page = self.page.as_ref().ok_or_else(stale)?;
                let (name, _) = super::type_input::request(marker(page.request.as_ref().ok_or_else(stale)?)?.1).ok_or_else(invalid)?;
                let mut bytes = [0; 256];
                let name = super::domain_cursor::canonical_name(name, &mut bytes).ok_or_else(invalid)?;
                let field=&page.fields[self.state.field];let name=C::normalization_attribute(name,field.index,true).map_err(|_|invalid())?.ok_or_else(invalid)?.name;let mut buffer=[0u8;256];let word=attribute_text(name,field.member,&mut buffer)?;let mut demand=text(word);if field.member.is_some(){demand.copy_bytes+=256+word.len()+40;}return Ok(demand);
            }
            12 => {
                demand.capacity_bytes = if self.needs_row()? { 2 } else { 1 } * size_of::<StagedFormBindingEdit>();
            }
            13 => {
                demand.capacity_bytes = self.copy_row_count()? * size_of::<DslValue>();
            }
            14 => demand.copy_bytes += size_of::<RowParts>(),
            28 if self.state.row_part < self.row_string_count()? => {
                let mut buffer = [0; 20];
                let text = self.row_part_text(&mut buffer)?;
                demand.capacity_bytes = text.len();
                demand.copy_bytes += size_of::<String>() + 20 + if self.state.row_part == 6 { 0 } else { 2 * text.len() };
            }
            28 => {
                let limit=self.row_string_count()?;
                demand.capacity_bytes = if self.state.row_part==limit+2{1}else{2} * size_of::<(String, DslValue)>();
                demand.copy_bytes += size_of::<Vec<(String, DslValue)>>();
            }
            29 => {
                demand.copy_bytes += 2 * size_of::<RowParts>() + 5 * size_of::<(String, DslValue)>();
            }
            20 => {
                demand.copy_bytes += 192 + size_of::<String>();
            }
            21 => demand.copy_bytes += 2 * size_of::<DslValue>(),
            26 => {
                demand.capacity_bytes = 2 * size_of::<String>() + 15;
                demand.copy_bytes += demand.capacity_bytes + size_of::<StagedFormBindingEdit>() + size_of::<Vec<DslValue>>();
            }
            15 if self.current_field()?.operation.is_some()=>{let capacity=set_controls::capacity(self.current_field()?,self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?)?;demand.capacity_bytes=capacity;demand.copy_bytes+=2*capacity+2*size_of::<StagedFormBindingEdit>()+2*size_of::<DslValue>();demand.depth=4;},
            15 if matches!(self.current_field()?.kind,InputKind::Enum|InputKind::Number|InputKind::List|InputKind::Select) => {}
            15 => {
                let mut buffer = [0; 20];
                let row=decimal(self.current_field()?.row,&mut buffer);let mut member=[0;20];let member=decimal(self.current_field()?.member.unwrap_or(0),&mut member);demand.capacity_bytes=text_path_capacity(self.current_field()?,row,member);
                demand.copy_bytes += demand.capacity_bytes + size_of::<StagedFormBindingEdit>();
            }
            16 | 27 => demand.copy_bytes += size_of::<StagedFormBindingEdit>(),
            17 if !self.retirement.is_empty() => return self.retirement.next_owner_demands::<DslValue>(false, &None, 4096),
            17 if self.plan.map.is_some() => return self.retirement.next_owner_demands::<Vec<usize>>(true, &None, 4096),
            17 if self.placeholders.is_some() => return self.retirement.next_owner_demands::<Vec<super::type_input::Input>>(true, &None, 4096),
            _ => {}
        }
        Ok(demand)
    }
    fn current_field(&self) -> Result<&Field, ValueError> {
        self.page.as_ref().and_then(|page| page.fields.get(self.state.field)).ok_or_else(stale)
    }
    fn next_field_count(&self)->Result<usize,ValueError>{let values=self.placeholders.as_ref().ok_or_else(invalid)?;let Some(value)=values.get(self.state.part)else{return Ok(0)};let request=self.request.as_ref().ok_or_else(stale)?;let present=rows(request)?.get(self.state.row).is_some_and(|_|row_data(request,self.state.row).is_ok_and(|(index,_,_)|index==value.index));Ok(match self.state.kinds[self.state.part]{InputKind::List if present=>list_members(request,self.state.row)?.len()+1,InputKind::Select if present&&matches!(self.state.leaves[self.state.part],InputKind::Set|InputKind::Array|InputKind::List)=>2+2*set_window(request,self.state.position,value.index,super::type_input::set_members(constructor(request,self.state.row)?).map_or(0,|items|items.len()),self.state.leaf_counts[self.state.part])?.1,InputKind::Select if present&&self.state.leaves[self.state.part]==InputKind::Number=>2+usize::from(selected_numeric(request,self.state.row)?.is_some()),InputKind::Number|InputKind::Select if present=>2,_=>1})}
    fn choice_shape(&self) -> Result<(usize, usize, Option<usize>), ValueError> {
        let field = self.current_field()?;
        if field.kind==InputKind::Number{return Ok((0,2,None));}
        if field.kind==InputKind::Select{let count=self.state.leaf_counts[field.ordinal];if count==0{return Err(invalid());}let start=if count>32&&self.state.active_target==Some(field.index)&&self.state.page_offset%31==0{self.state.page_offset}else{0};if start>=count{return Err(invalid());}let length=if count>32{(count-start).min(31)}else{count};let pin=self.state.selected_leaves[field.ordinal].checked_sub(1).or(field.selected_choice).filter(|index|*index<start||*index>=start+length);return Ok((start,length,pin));}
        if field.kind==InputKind::List{let upper=self.state.upper[field.ordinal];let current=if field.present{Some(list_members(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?,field.row)?.len())}else{None};return Ok((0,upper+1,current.filter(|length|*length>upper)));}
        let domain = if field.selection{self.leaf_domains[field.ordinal]}else{self.domains[field.ordinal]}.ok_or_else(invalid)?;
        let count = InputKindCursor::count::<C>(domain).ok_or_else(invalid)?;
        let start = if count>32&&self.state.active_target == Some(field.index) { self.state.page_offset } else { 0 };
        let length = if count > 32 { (count - start).min(31) } else { count };
        let pin = self.state.selected[field.ordinal].checked_sub(1).filter(|index| *index < start || *index >= start + length);
        Ok((start, length, pin))
    }
    fn choice_word(&self) -> Result<&'static str, ValueError> {
        let field=self.current_field()?;if field.kind==InputKind::Number{return ["integer","real"].get(self.state.option).copied().ok_or_else(invalid);}
        let (start, length, pin) = self.choice_shape()?;
        let index = if self.state.option < length { start + self.state.option } else { pin.ok_or_else(invalid)? };
        if field.kind==InputKind::Select{return self.state.leaf_name.ok_or_else(invalid);}
        if field.kind==InputKind::List{return CARDINALITY.get(index).copied().ok_or_else(invalid);}
        InputKindCursor::member::<C>(if field.selection{self.leaf_domains[field.ordinal]}else{self.domains[field.ordinal]}.ok_or_else(invalid)?, index).ok_or_else(invalid)
    }
    fn choice_path_len(&self)->Result<usize,ValueError>{let field=self.current_field()?;Ok(if field.selection&&matches!(field.kind,InputKind::Number|InputKind::Enum){let request=self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?;if super::type_input::typed_constructor(constructor(request,field.row)?).ok_or_else(invalid)?.1.is_empty(){6}else{7}}else{4})}
    fn choice_path_word<'a>(&self,part:usize,buffer:&'a mut[u8;20])->Result<&'a str,ValueError>{Ok(match part{0=>"value",1=>"$ifcInputs",2=>decimal(self.current_field()?.row,buffer),3|4=>"value",5=>"items",6=>"0",_=>return Err(invalid())})}
    fn choice_real(&self)->Result<bool,ValueError>{Ok(self.current_field()?.kind==InputKind::Number&&self.choice_word()?=="real")}
    fn choice_string_count(&self)->Result<usize,ValueError>{Ok(if self.current_field()?.kind==InputKind::List||self.current_field()?.kind==InputKind::Select&&self.state.choice_kind!=InputKind::Reference{4}else if self.choice_real()?{6}else{5})}
    fn choice_raw(&self)->Result<bool,ValueError>{Ok((self.current_field()?.kind==InputKind::Number||self.current_field()?.kind==InputKind::Select&&self.state.choice_kind==InputKind::Reference)&&self.state.choice_part==4)}
    fn choice_text(&self)->Result<&str,ValueError>{
        let field=self.current_field()?;let number=field.kind==InputKind::Number;let reference=field.kind==InputKind::Select&&self.state.choice_kind==InputKind::Reference;let word=self.choice_word()?;
        Ok(match self.state.choice_part{0=>word,1=>"kind",2=>if reference{"reference"}else if number{word}else if field.kind==InputKind::List{"aggregate"}else if field.kind==InputKind::Select{"typedValue"}else{"enum"},3=>"value",4=>if number||reference{if field.present{let request=self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?;let(kind,text)=if number&&field.selection{selected_numeric(request,field.row)?.unwrap_or((InputKind::Number,""))}else{let(_,kind,text)=row_data(request,field.row)?;(kind,text)};if kind.tag()==if reference{"reference"}else{word}{text}else{""}}else{""}}else{word},5=>"bits",_=>return Err(invalid())})
    }
    fn choice_label(word: &'static str, locale: Locale) -> &'static str {
        match (word, locale) {
            ("integer",Locale::En)=>"Integer",("integer",Locale::De)=>"Ganzzahl",("real",Locale::En)|("real",Locale::De)=>"Binary64",
            ("T", Locale::En) => "True",
            ("T", Locale::De) => "Wahr",
            ("F", Locale::En) => "False",
            ("F", Locale::De) => "Falsch",
            ("U", Locale::En) => "Unknown",
            ("U", Locale::De) => "Unbekannt",
            _ => word,
        }
    }
    fn nav_spec(&self) -> Option<(usize, usize)> {
        self.state.navigation.iter().flatten().copied().nth(self.state.nav)
    }
    fn page_path_text<'a>(&self, part: usize, target: usize, buffer: &'a mut [u8; 20]) -> Result<&'a str, ValueError> {
        Ok(match part {
            0 => "entities",
            1 => decimal(self.state.position, buffer),
            2 => "args",
            3 => decimal(target, buffer),
            _ => return Err(invalid()),
        })
    }
    fn needs_row(&self)->Result<bool,ValueError>{let field=self.current_field()?;Ok(field.operation.is_none()&&!(field.selection&&matches!(field.kind,InputKind::Number|InputKind::Enum))&&(!field.present||field.selection&&super::type_input::typed_constructor(constructor(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?,field.row)?).ok_or_else(invalid)?.1.is_empty()))}
    fn copy_row_count(&self)->Result<usize,ValueError>{Ok(rows(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?)?.len()+usize::from(!self.current_field()?.present))}
    fn typed_spec<'a>(page:&'a Page,state:&State,domains:&[Option<&'static str>;16],choice:Option<(&'static str,InputKind)>)->Result<typed_copy::Input<'a>,ValueError>{
        let field=page.fields.get(state.field).ok_or_else(invalid)?;let request=page.request.as_ref().ok_or_else(stale)?;
        if let Some((name,kind))=choice{let source=if field.present&&row_data(request,field.row)?.1==InputKind::Select&&(!matches!(kind,InputKind::Set|InputKind::Array|InputKind::List)||state.leaves[field.ordinal]==kind){Some(constructor(request,field.row)?)}else{None};let kind=if kind==InputKind::Number{if source.and_then(|source|super::type_input::typed_constructor(source)).and_then(|(_,items)|items.first()).and_then(|child|child.get("kind")).and_then(DslValue::as_str)==Some("real"){InputKind::Real}else{InputKind::Integer}}else{kind};return Ok(typed_copy::Input{source,name,kind,fill:false});}
        if state.copying==field.row&&!field.present{return Ok(typed_copy::Input{source:None,name:state.first_leaves[field.ordinal].ok_or_else(invalid)?,kind:InputKind::String,fill:false});}
        let row=state.copying-usize::from(!field.present&&state.copying>field.row);let source=constructor(request,row)?;let(name,items)=super::type_input::typed_constructor(source).ok_or_else(invalid)?;let kind=if field.selection&&state.copying==field.row{field.kind}else{match items.first().and_then(|value|value.get("kind")).and_then(DslValue::as_str){Some("integer")=>InputKind::Integer,Some("real")=>InputKind::Real,Some("enum")=>InputKind::Enum,Some("binary")=>InputKind::Binary,Some("aggregate")=>{let main=page.fields.iter().find(|value|value.kind==InputKind::Select&&value.present&&value.row==row).ok_or_else(invalid)?;state.leaves[main.ordinal]},_=>InputKind::String}};Ok(typed_copy::Input{source:Some(source),name,kind,fill:field.selection&&state.copying==field.row&&!matches!(kind,InputKind::Set|InputKind::Array|InputKind::List)})
    }
    fn advance_typed_copy(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let own=4*size_of::<State>()+2*size_of::<typed_copy::TypedCopy>();
        if self.state.phase==95&&self.typed_copy.empty(){metadata(cx,own,||{self.typed_copy=Default::default();self.state.phase=self.state.typed_resume;})?;return Ok(());}
        let grant=cx.retained_grant();let loan=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(own),..grant};let before=cx.retained_progress();
        let complete=if self.state.phase==95{let demand=self.typed_copy.close_demands(loan.maximum_copy_bytes)?;cx.with_retained_loan(loan,|cx|{semio_framework_job::close_original_retained_step(cx,demand,|grant|self.typed_copy.close_owned(grant))?;Ok::<_,OriginalTurnFailure>(())})?;false}else{let choice=if self.state.phase==94{Some((self.choice_word()?,self.state.choice_kind))}else{None};let input=Self::typed_spec(self.page.as_ref().ok_or_else(stale)?,&self.state,&self.domains,choice)?;let output=if self.state.phase==92{&mut self.row_parts.as_mut().ok_or_else(invalid)?.structured}else{&mut self.choice_parts.as_mut().ok_or_else(invalid)?.structured};let copy=&mut self.typed_copy;cx.with_retained_loan(loan,|cx|{if !copy.advance(&input,cx)?{return Ok(false);}copy.take(output,cx)})?};
        if complete{self.state.typed_resume=if self.state.phase==92{29}else{40};self.state.phase=95;self.typed_copy.begin_close();let delegated=cx.retained_progress()!=before;cx.consume_retained(RetainedCloneProgress{copied_items:usize::from(!delegated),copied_bytes:own,..Default::default()})?;if !delegated{cx.consume_fuel(1);}}Ok(())
    }
    fn copying_row(&self) -> Result<(usize, InputKind, &str), ValueError> {
        let field = self.current_field()?;
        if self.state.copying == field.row {
            return Ok((field.index, if field.selection{InputKind::Select}else if field.kind==InputKind::Select&&!field.present&&self.state.references[field.ordinal]==Some(0){InputKind::Reference}else if field.kind==InputKind::Number{InputKind::Integer}else{field.kind}, ""));
        }
        let ordinal = self.state.copying - usize::from(!field.present&&self.state.copying > field.row);
        row_data(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?, ordinal)
    }
    fn row_part_text<'a>(&'a self, buffer: &'a mut [u8; 20]) -> Result<&'a str, ValueError> {
        let (index, kind, text) = self.copying_row()?;
        Ok(match self.state.row_part {
            0 => "index",
            1 | 4 => "value",
            2 => "kind",
            3 => kind.tag(),
            5 => decimal(index, buffer),
            6 => text,
            7 => "bits",
            _ => return Err(invalid()),
        })
    }
    fn row_string_count(&self)->Result<usize,ValueError>{Ok(match self.copying_row()?.1{InputKind::Real=>8,InputKind::List|InputKind::Select=>6,_=>7})}
    fn copy_members<'a>(page:&'a Page,state:&State)->Result<&'a[DslValue],ValueError>{let field=page.fields.get(state.field).ok_or_else(invalid)?;if state.phase==84{if state.copying==field.row{return Ok(&[]);}let row=state.copying-usize::from(!field.present&&state.copying>field.row);list_members(page.request.as_ref().ok_or_else(stale)?,row)}else{if !field.present{return Ok(&[]);}list_members(page.request.as_ref().ok_or_else(stale)?,field.row)}}
    fn advance_list_copy(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let own=4*size_of::<State>()+2*size_of::<list_copy::ListCopy>();
        if self.state.phase==89&&self.list_copy.empty(){metadata(cx,own,||{self.list_copy=Default::default();self.state.phase=self.state.list_resume;})?;return Ok(());}
        let grant=cx.retained_grant();let loan=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(own),..grant};let before=cx.retained_progress();
        let complete=if self.state.phase==89{let demand=self.list_copy.close_demands(loan.maximum_copy_bytes)?;cx.with_retained_loan(loan,|cx|{semio_framework_job::close_original_retained_step(cx,demand,|grant|self.list_copy.close_owned(grant))?;Ok::<_,OriginalTurnFailure>(())})?;false}else{let phase=self.state.phase;let count=if phase==85{Some(self.choice_word()?.parse().map_err(|_|invalid())?)}else{None};let members=Self::copy_members(self.page.as_ref().ok_or_else(stale)?,&self.state)?;let count=count.unwrap_or(members.len());let output=if phase==84{&mut self.row_parts.as_mut().ok_or_else(invalid)?.structured}else{&mut self.choice_parts.as_mut().ok_or_else(invalid)?.structured};let copy=&mut self.list_copy;cx.with_retained_loan(loan,|cx|{if !copy.advance(members,count,list_copy::Mode::Real,cx)?{return Ok(false);}copy.take(output,cx)})?};
        if complete{self.state.list_resume=if self.state.phase==84{29}else{40};self.state.phase=89;self.list_copy.begin_close();let delegated=cx.retained_progress()!=before;cx.consume_retained(RetainedCloneProgress{copied_items:usize::from(!delegated),copied_bytes:own,..Default::default()})?;if !delegated{cx.consume_fuel(1);}}Ok(())
    }
    fn receive_source(&mut self,owner:&std::sync::Arc<IfcSnapshot>,source:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.binding.is_none_or(|(binding,pointer,hash)|!binding.matches(cx)||pointer!=source as *const _ as usize||hash!=revision)||!std::ptr::eq(owner.as_ref(),source)||self.state.closing||self.state.failed{return Err(stale().into());}
        if self.source_loan.is_none(){metadata(cx,size_of::<super::source_loan::SourceLoan>()+size_of_val(&self.source_loan),||*self.source_loan=Some(Default::default()))?;return Ok(false);}
        self.source_loan.as_mut().ok_or_else(stale)?.advance(owner,source,revision,cx)
    }
    pub fn original_request_received(&self) -> bool {
        self.binding.is_some()
    }
    pub fn context_matches(&self, window: Option<&str>, locale: Locale) -> Result<bool, ValueError> {
        self.request.as_ref().or_else(|| self.page.as_ref().and_then(|page| page.request.as_ref())).or_else(||self.candidate.as_ref().and_then(|candidate|candidate.request())).map(|request| request.context_matches(window, locale)).ok_or_else(stale)
    }
    pub fn receive(&mut self, donor: &mut Option<StagedFormRequest>, source: &IfcSnapshot, revision: [u8; 32], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.binding.is_some() || donor.is_none() || self.state.closing {
            return Err(stale().into());
        }
        let binding = OriginalSourceBinding::borrow(cx);
        metadata(cx, 2 * size_of::<StagedFormRequest>() + size_of_val(&self.binding), || {
            *self.request = donor.take();
            self.binding = Some((binding, source as *const _ as usize, revision));
            self.state.profile = C::normalization_attribute as usize;
        })
    }
    pub fn advance(&mut self, owner: &std::sync::Arc<IfcSnapshot>, revision: [u8; 32], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let source=owner.as_ref();
        if self.binding.is_none_or(|(binding, pointer, hash)| !binding.matches(cx) || pointer != source as *const _ as usize || hash != revision) || self.state.profile != C::normalization_attribute as usize || self.state.closing || self.state.failed
        {
            return Err(stale().into());
        }
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "staging cancelled").into());
        }
        if self.source_loan.as_ref().and_then(|loan|loan.source()).is_none(){self.receive_source(owner,source,revision,cx)?;return Ok(false);}
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), self.demands()?) {
            return Ok(false);
        }
        let result = self.advance_inner(source, revision, cx);
        if result.is_err() {
            self.state.failed = true;
        }
        result
    }
    fn advance_inner(&mut self, source: &IfcSnapshot, revision: [u8; 32], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let base = 4 * size_of::<State>() + 2 * 256 + size_of::<IfcValue>() + 2*size_of::<super::type_input_select::Cursor>();
        match self.state.phase {
            0 => {
                let request = self.request.as_ref().ok_or_else(stale)?;
                if !request.token.admitted()
                    || request.token.action != editing::SET_SNAPSHOT_VALUE_ACTION_ID
                    || !revision_matches(&request.token.revision, revision)
                    || request.page.as_ref().is_some_and(|page| !page.admitted() || page.field_offset != "0" || page.diagnostic_offset != "0")
                {
                    return Err(stale().into());
                }
                let (path, _) = marker(request)?;
                let ordinal = path.strip_prefix("/entities/").and_then(|value| canonical_decimal(value, false)).and_then(|value| usize::try_from(value).ok()).ok_or_else(invalid)?;
                if !metadata(cx, base, || {
                    self.state.position = ordinal;
                    self.state.phase = 1;
                })? {
                    return Ok(false);
                }
            }
            1 => {
                let entity = source.entities.get(self.state.position).ok_or_else(invalid)?;
                if self.state.row == source.entities.len() {
                    if self.state.matches != 1 {
                        return Err(invalid().into());
                    }
                    metadata(cx, base, || self.state.phase = 2)?;
                } else {
                    let row = self.state.row;
                    metadata(cx, base, || {
                        self.state.matches += usize::from(source.entities[row].id == entity.id);
                        self.state.row += 1;
                    })?;
                }
            }
            2 => {
                let entity = &source.entities[self.state.position];
                let (_, value) = marker(self.request.as_ref().ok_or_else(stale)?)?;
                let (name, _) = super::type_input::request(value).ok_or_else(invalid)?;
                let mut bytes = [0; 256];
                let name = super::domain_cursor::canonical_name(name, &mut bytes).ok_or_else(invalid)?;
                let count = C::direct_attribute_count(name).ok_or_else(invalid)?;
                let repair = name.eq_ignore_ascii_case(&entity.name) && count == entity.args.len();
                if count == 0
                    || entity.args.is_empty()
                    || C::direct_attribute_count(&entity.name) != Some(entity.args.len())
                    || if repair { entity.complex.is_empty() && C::rootless_attribute_count(name) != Some(count) } else { entity.complex.is_empty() || count == entity.args.len() }
                {
                    return Err(invalid().into());
                }
                metadata(cx, base, || {
                    self.state.target = count;
                    self.state.phase = 23;
                })?;
            }
            23 => {
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base, capacity_bytes: self.state.target * size_of::<super::type_input::Input>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.placeholders, physical, |_| {
                    let mut values = Vec::new();
                    values.try_reserve_exact(self.state.target).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type required roster"))?;
                    let capacity = values.capacity() * size_of::<super::type_input::Input>();
                    Ok((values, RetainedCloneProgress { copied_items: 1, copied_bytes: base, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.phase = 25;
                }
            }
            25 => {
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base, capacity_bytes: STAGED_FORM_BODY_LIMITS.content * size_of::<Field>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.fields, physical, |_| {
                    let mut values = Vec::new();
                    values.try_reserve_exact(STAGED_FORM_BODY_LIMITS.content).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type required fields"))?;
                    let capacity = values.capacity() * size_of::<Field>();
                    Ok((values, RetainedCloneProgress { copied_items: 1, copied_bytes: base, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.phase = 3;
                }
            }
            3 => {
                let entity = &source.entities[self.state.position];
                let (_, value) = marker(self.request.as_ref().ok_or_else(stale)?)?;
                let (name, _) = super::type_input::request(value).ok_or_else(invalid)?;
                let mut bytes = [0; 256];
                let name = super::domain_cursor::canonical_name(name, &mut bytes).ok_or_else(invalid)?;
                if let Some(index) = self.plan.required_frontier(entity.args.len()).filter(|index| self.placeholders.as_ref().unwrap().last().is_none_or(|input| input.index != *index)) {
                    if self.placeholders.as_ref().unwrap().len() >= STAGED_FORM_BODY_LIMITS.content {
                        return Err(invalid().into());
                    }
                    let attribute = C::normalization_attribute(name, index, true).map_err(|_| invalid())?.ok_or_else(invalid)?;
                    let mut cursor = self.kind_cursor;
                    let result = cursor.advance::<C>(attribute.type_expr);
                    let kind = result.as_ref().ok().copied().flatten();
                    if !metadata(cx, base + size_of::<super::type_input::Input>() + 2 * size_of::<InputKindCursor>(), || {
                        self.kind_cursor = cursor;
                        if let Some(kind) = kind {
                            let index_in_roster = self.placeholders.as_ref().unwrap().len();
                            self.state.kinds[index_in_roster] = kind;
                            self.domains[index_in_roster] = cursor.domain();
                            self.state.references[index_in_roster] = cursor.reference_member();
                            self.state.leaf_counts[index_in_roster]=cursor.leaf_count().unwrap_or(0);
                            self.state.first_leaves[index_in_roster]=cursor.first_leaf();
                            if let Some((_,upper))=cursor.list_bounds(){self.state.upper[index_in_roster]=upper;}
                            self.placeholders.as_mut().unwrap().push(super::type_input::Input { index, value: Some(IfcValue::Unset) });
                            self.kind_cursor.clear();
                        }
                    })? {
                        return Ok(false);
                    }
                    result?;
                } else {
                    let mut previous = [0; 256];
                    let previous = super::domain_cursor::canonical_name(&entity.name, &mut previous).ok_or_else(invalid)?;
                    let demand = RetirementDemand { copy_bytes: base + super::type_remap::Plan::copy_bytes(), capacity_bytes: self.plan.capacity_bytes(self.state.target)?, depth: 1, ..Default::default() };
                    if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
                        return Ok(false);
                    }
                    let (complete, capacity) = self.plan.advance::<C>(previous, name, &entity.args, self.state.target, self.placeholders.as_deref())?;
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() })?;
                    cx.consume_fuel(1);
                    if complete {
                        self.state.phase = 4;
                        self.state.row = 0;
                        self.state.part = 0;
                    }
                }
            }
            4 => {
                let request = self.request.as_ref().ok_or_else(stale)?;
                let values = self.placeholders.as_ref().ok_or_else(invalid)?;
                if self.state.part == values.len() {
                    if values.is_empty() || self.state.row != rows(request)?.len() {
                        return Err(invalid().into());
                    }
                    metadata(cx, base, || {
                        self.state.scan = 0;
                        self.state.phase = 61;
                    })?;
                    return Ok(false);
                }
                let index = values[self.state.part].index;
                let current = if self.state.row < rows(request)?.len() { Some(row_data(request, self.state.row)?) } else { None };
                if current.is_some_and(|(ordinal, _, _)| ordinal < index) {
                    return Err(invalid().into());
                }
                let present = current.is_some_and(|(ordinal, _, _)| ordinal == index);
                let declared=self.state.kinds[self.state.part];
                let kind=if declared==InputKind::Number{current.filter(|_|present).map_or(InputKind::Number,|(_,kind,_)|kind)}else{declared};
                let reference=declared==InputKind::Select&&current.is_some_and(|(ordinal,actual,_)|ordinal==index&&actual==InputKind::Reference)&&self.state.references[self.state.part].is_some();
                if current.is_some_and(|(ordinal, actual, _)| ordinal == index && if declared==InputKind::Number{!matches!(actual,InputKind::Integer|InputKind::Real)}else{declared!=actual&&!reference}) {
                    return Err(invalid().into());
                }
                if declared==InputKind::Select&&present&&!reference&&!self.state.tag_checked{metadata(cx,base,||{self.state.member=0;self.state.offset=0;self.state.leaf_name=None;self.leaf_cursor.clear();self.state.phase=90;})?;return Ok(false);}
                let text = current.filter(|_| present).map(|(_, _, text)| text);
                let value = match kind {
                    InputKind::Reference => text.and_then(|text| canonical_decimal(text, true)).map(IfcValue::Reference),
                    InputKind::Integer => text.and_then(signed_decimal).map(IfcValue::Integer),
                    InputKind::String | InputKind::Enum => None,
                    InputKind::Real=>text.and_then(|text|canonical_decimal(text,false)).filter(|bits|(bits>>52)&2047!=2047).map(|bits|IfcValue::Real(f64::from_bits(bits))),
                    InputKind::Number | InputKind::Binary | InputKind::List | InputKind::Set | InputKind::Array | InputKind::Select=>None,
                };
                let valid = if reference{text.and_then(|text|canonical_decimal(text,true)).is_some()}else if declared==InputKind::Select&&present{let(_,items)=super::type_input::typed_constructor(constructor(request,self.state.row)?).ok_or_else(invalid)?;items.len()==1&&(match self.state.leaves[self.state.part]{InputKind::String|InputKind::Enum|InputKind::Binary|InputKind::Set|InputKind::Array|InputKind::List=>true,InputKind::Integer=>text.and_then(signed_decimal).is_some(),InputKind::Number=>selected_numeric(request,self.state.row)?.is_some_and(|(kind,text)|if kind==InputKind::Integer{signed_decimal(text).is_some()}else{canonical_decimal(text,false).is_some_and(|bits|(bits>>52)&2047!=2047)}),_=>text.and_then(|text|canonical_decimal(text,false)).is_some_and(|bits|(bits>>52)&2047!=2047)})}else if matches!(kind, InputKind::String | InputKind::Enum | InputKind::Binary | InputKind::List) { present } else { value.is_some() };
                let members=if declared==InputKind::List&&present{list_members(request,self.state.row)?.len()}else{0};
                let set=declared==InputKind::Select&&present&&!reference&&matches!(self.state.leaves[self.state.part],InputKind::Set|InputKind::Array|InputKind::List);
                let set_count=if set{super::type_input::set_members(constructor(request,self.state.row)?).map_or(0,|items|items.len())}else{0};let set_window=if set{set_window(request,self.state.position,index,set_count,self.state.leaf_counts[self.state.part])?}else{(0,0)};
                let physical=self.next_field_count()?;
                if self.fields.as_ref().ok_or_else(invalid)?.len()+physical>STAGED_FORM_BODY_LIMITS.content{return Err(invalid().into());}
                metadata(cx, base + physical*size_of::<Field>() + size_of::<super::type_input::Input>(), || {
                    self.placeholders.as_mut().unwrap()[self.state.part].value = Some(value.unwrap_or(IfcValue::Unset));
                    self.state.ready &= valid;
                    if declared==InputKind::Number&&present{self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:StagedFormLayout::Replace,edits:Vec::new()},attribute:String::new(),index,ordinal:self.state.part,member:None,operation:None,selection:false,selected_choice:None,row:self.state.row,present,kind:InputKind::Number,choices:None});}
                    self.fields.as_mut().unwrap().push(Field {
                        binding: StagedFormBinding { token: empty_token(), layout: if present && !matches!(kind,InputKind::Enum|InputKind::Select|InputKind::List) { StagedFormLayout::Preserve } else { StagedFormLayout::Replace }, edits: Vec::new() },
                        attribute: String::new(),
                        index,
                        ordinal:self.state.part,
                        member:None,operation:None,selection:false,selected_choice:if reference{self.state.references[self.state.part]}else{None},
                        row: self.state.row,
                        present,
                        kind,
                        choices: None,
                    });
                    if declared==InputKind::List&&present{for member in 0..members{self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:StagedFormLayout::Preserve,edits:Vec::new()},attribute:String::new(),index,ordinal:self.state.part,member:Some(member),operation:None,selection:false,selected_choice:None,row:self.state.row,present:true,kind:InputKind::Real,choices:None});}}
                    if declared==InputKind::Select&&present{let leaf=if reference{InputKind::Reference}else{self.state.leaves[self.state.part]};let empty=if reference{false}else{super::type_input::typed_constructor(constructor(request,self.state.row).expect("closed original constructor")).expect("closed original constructor").1.is_empty()};self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:if empty||set||matches!(leaf,InputKind::Number|InputKind::Enum){StagedFormLayout::Replace}else{StagedFormLayout::Preserve},edits:Vec::new()},attribute:String::new(),index,ordinal:self.state.part,member:None,operation:if set{Some(false)}else{None},selection:!reference,selected_choice:None,row:self.state.row,present:true,kind:leaf,choices:None});if leaf==InputKind::Number{if let Some((kind,_))=selected_numeric(request,self.state.row).expect("closed original numeric child"){self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:StagedFormLayout::Preserve,edits:Vec::new()},attribute:String::new(),index,ordinal:self.state.part,member:None,operation:None,selection:true,selected_choice:None,row:self.state.row,present:true,kind,choices:None});}}if set{for member in set_window.0..set_window.0+set_window.1{for remove in [false,true]{self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:if remove{StagedFormLayout::Replace}else{StagedFormLayout::Preserve},edits:Vec::new()},attribute:String::new(),index,ordinal:self.state.part,member:Some(member),operation:remove.then_some(true),selection:true,selected_choice:None,row:self.state.row,present:true,kind:if leaf==InputKind::Array{InputKind::Real}else if leaf==InputKind::List{InputKind::Integer}else{InputKind::Reference},choices:None});}}}}
                    self.state.tag_checked=false;
                    self.state.row += usize::from(present);
                    self.state.part += 1;
                    if set&&valid{self.state.phase=96;}else if declared==InputKind::Select&&present&&valid&&self.state.leaves[self.state.part-1]==InputKind::Enum{self.state.member=0;self.state.offset=0;self.state.phase=103;}else if declared==InputKind::Select&&present&&valid&&self.state.leaves[self.state.part-1]==InputKind::Binary{self.state.offset=0;self.state.phase=104;}else if kind==InputKind::List&&present{self.state.phase=86;}else if kind == InputKind::String && present {
                        self.state.phase = 30;
                    } else if kind==InputKind::Binary&&present{
                        self.state.offset=0;self.state.phase=81;
                    } else if kind == InputKind::Enum && present {
                        self.state.member = 0;
                        self.state.offset = 0;
                        self.state.phase = 60;
                    }
                })?;
            }
            90=>{
                let request=self.request.as_ref().ok_or_else(stale)?;let(name,_)=super::type_input::typed_constructor(constructor(request,self.state.row)?).ok_or_else(invalid)?;let mut next=self.state;let mut cursor=self.leaf_cursor;
                if let Some(member)=next.leaf_name{if name.len()!=member.len(){next.leaf_name=None;next.offset=0;}else if next.offset==member.len(){next.selected_leaves[next.part]=next.member+1;next.leaf_name=None;cursor.clear();next.phase=91;}else{let end=(next.offset+64).min(member.len());if name.as_bytes()[next.offset..end]==member.as_bytes()[next.offset..end]{next.offset=end;}else{next.leaf_name=None;next.offset=0;}}}else{match cursor.advance::<C>(self.domains[next.part].ok_or_else(invalid)?)?{super::type_input_select::Step::Pending=>{},super::type_input_select::Step::Complete{..}=>return Err(invalid().into()),super::type_input_select::Step::Leaf{index,name}=>{next.member=index;next.leaf_name=Some(name);}}}metadata(cx,base,||{self.state=next;self.leaf_cursor=cursor;})?;
            }
            91=>{
                let request=self.request.as_ref().ok_or_else(stale)?;let(name,items)=super::type_input::typed_constructor(constructor(request,self.state.row)?).ok_or_else(invalid)?;let expression=C::defined_type_expression(name).ok_or_else(invalid)?;let mut cursor=self.kind_cursor;let answer=cursor.advance_selected::<C>(expression)?;if answer.is_some_and(|kind|!matches!(kind,InputKind::String|InputKind::Integer|InputKind::Real|InputKind::Number|InputKind::Enum|InputKind::Binary|InputKind::List|InputKind::Set|InputKind::Array)||items.first().is_some_and(|value|{let tag=value.get("kind").and_then(DslValue::as_str);if kind==InputKind::Number{!matches!(tag,Some("integer"|"real"))}else{tag!=Some(kind.tag())}})){return Err(invalid().into());}metadata(cx,base+2*size_of::<InputKindCursor>(),||{self.kind_cursor=cursor;if let Some(kind)=answer{self.state.leaves[self.state.part]=kind;self.leaf_domains[self.state.part]=cursor.domain();self.kind_cursor.clear();self.state.tag_checked=true;self.state.phase=4;}})?;
            }
            60|103 => {
                let field = self.fields.as_ref().and_then(|fields| fields.last()).ok_or_else(invalid)?;
                let text = row_data(self.request.as_ref().ok_or_else(stale)?, field.row)?.2;
                let domain = if self.state.phase==103{self.leaf_domains[self.state.part-1]}else{self.domains[self.state.part-1]}.ok_or_else(invalid)?;
                let word = InputKindCursor::member::<C>(domain, self.state.member);
                let mut next = self.state;
                match word {
                    None => {
                        next.ready = false;
                        next.phase = 4;
                    }
                    Some(word) if word.len() != text.len() => {
                        next.member += 1;
                        next.offset = 0;
                    }
                    Some(word) if next.offset == word.len() => {
                        next.selected[next.part - 1] = next.member + 1;
                        next.phase = if self.state.phase==103{4}else{30};
                    }
                    Some(word) => {
                        let end = word.len().min(next.offset + 64);
                        if word.as_bytes()[next.offset..end] == text.as_bytes()[next.offset..end] {
                            next.offset = end;
                        } else {
                            next.member += 1;
                            next.offset = 0;
                        }
                    }
                }
                metadata(cx, base, || self.state = next)?;
            }
            61 => {
                let mut next = self.state;
                let request = self.request.as_ref().ok_or_else(stale)?;
                if next.scan == 0 {
                    if let Some(choice) = request.page.as_ref().and_then(|page| page.choice.as_ref()) {
                        if choice.path.len() != 4 || choice.path[0] != "entities" || canonical_decimal(&choice.path[1], false).and_then(|value| usize::try_from(value).ok()) != Some(next.position) || choice.path[2] != "args" {
                            return Err(invalid().into());
                        }
                        next.active_target = Some(canonical_decimal(&choice.path[3], false).and_then(|value| usize::try_from(value).ok()).ok_or_else(invalid)?);
                        next.page_offset = canonical_decimal(&choice.offset, false).and_then(|value| usize::try_from(value).ok()).ok_or_else(invalid)?;
                    }
                }
                if let Some(field) = self.fields.as_ref().and_then(|fields| fields.get(next.scan)) {
                    let paging=if field.kind==InputKind::Select{Some((next.leaf_counts[field.ordinal],31,32))}else if matches!(field.kind,InputKind::Set|InputKind::Array|InputKind::List)&&field.operation==Some(false)&&next.leaf_counts[field.ordinal]<=32{Some((super::type_input::set_members(constructor(request,field.row)?).map_or(0,|items|items.len()),SET_PAGE,SET_PAGE))}else if let Some(domain)=self.domains[field.ordinal].filter(|_|self.state.kinds[field.ordinal]==InputKind::Enum){Some((InputKindCursor::count::<C>(domain).ok_or_else(invalid)?,31,32))}else{None};
                    if let Some((count,step,limit))=paging {
                        if count > limit {
                            let target = *next.active_target.get_or_insert(field.index);
                            if field.index < target {
                                next.navigation[0] = Some((field.index, (count - 1) / step * step));
                            } else if field.index == target {
                                let members=if field.kind==InputKind::Select&&field.present&&matches!(next.leaves[field.ordinal],InputKind::Set|InputKind::Array|InputKind::List){super::type_input::set_members(constructor(request,field.row)?).map_or(0,|items|items.len())}else{0};
                                if field.kind==InputKind::Select&&next.page_offset>0&&next.page_offset%SET_PAGE==0&&next.page_offset<members{next.active_found=true;next.navigation[0]=Some((field.index,if next.page_offset==SET_PAGE{(count-1)/step*step}else{next.page_offset-SET_PAGE}));if next.page_offset+SET_PAGE<members{next.navigation[1]=Some((field.index,next.page_offset+SET_PAGE));}next.scan+=1;metadata(cx,base,||self.state=next)?;return Ok(false);}
                                if next.page_offset >= count || next.page_offset%step!=0 {return Err(invalid().into());}
                                next.active_found = true;
                                if field.kind==InputKind::Select&&next.page_offset+step>=count&&members>SET_PAGE{next.navigation[1]=Some((field.index,SET_PAGE));}
                                if next.page_offset > 0 {
                                    next.navigation[0] = Some((field.index, next.page_offset - step));
                                }
                                if next.page_offset + step < count {
                                    next.navigation[1] = Some((field.index, next.page_offset + step));
                                }
                            } else if next.navigation[1].is_none() {
                                next.navigation[1] = Some((field.index, 0));
                            }
                        }
                    }
                    next.scan += 1;
                } else {
                    if next.active_target.is_some() && !next.active_found {
                        return Err(invalid().into());
                    }
                    next.phase = 62;
                    next.nav = 0;
                }
                metadata(cx, base, || self.state = next)?;
            }
            62 => {
                let count = self.state.navigation.iter().flatten().count();
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base, capacity_bytes: count * size_of::<Navigation>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.navigation, physical, |_| {
                    let mut rows = Vec::new();
                    rows.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type choice pages"))?;
                    let capacity = rows.capacity() * size_of::<Navigation>();
                    Ok((rows, RetainedCloneProgress { copied_items: 1, copied_bytes: base, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.phase = 63;
                }
            }
            63 => {
                if self.nav_spec().is_none() {
                    metadata(cx, base + 2 * size_of::<Navigation>(), || self.state.phase = 22)?;
                } else {
                    metadata(cx, base + 2 * size_of::<Navigation>(), || {
                        *self.nav_item = Some(Navigation {
                            previous: self.state.navigation[0] == self.nav_spec(),
                            field: self.fields.as_ref().unwrap().iter().position(|field| Some(field.index) == self.nav_spec().map(|value| value.0)).expect("source-owned page target"),
                            binding: StagedFormPageBinding {
                                token: empty_token(),
                                page: StagedFormPage { field_offset: String::new(), diagnostic_offset: String::new(), choice: Some(StagedFormChoicePage { path: Vec::new(), offset: String::new() }) },
                            },
                        });
                        self.state.part = 0;
                        self.state.phase = 64;
                    })?;
                }
            }
            64 => {
                let token = &self.request.as_ref().ok_or_else(stale)?.token;
                if copy_text(token_part_mut(&mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.token, self.state.part), token_part(token, self.state.part), cx)? {
                    self.state.part += 1;
                    if self.state.part == 4 {
                        self.state.phase = 65;
                    }
                }
            }
            65 | 66 => {
                let page = &mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.page;
                let target = if self.state.phase == 65 { &mut page.diagnostic_offset } else { &mut page.field_offset };
                if copy_text(target, "0", cx)? {
                    self.state.phase += 1;
                }
            }
            67 => {
                let mut buffer = [0; 20];
                let text = decimal(self.nav_spec().ok_or_else(invalid)?.1, &mut buffer);
                if copy_text_extra(&mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.page.choice.as_mut().ok_or_else(invalid)?.offset, text, 40, cx)? {
                    self.state.phase = 68;
                }
            }
            68 => {
                let path = &mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.page.choice.as_mut().ok_or_else(invalid)?.path;
                let mut output = None;
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base, capacity_bytes: 4 * size_of::<String>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut output, physical, |_| {
                    path.try_reserve_exact(4).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type page address"))?;
                    Ok(((), RetainedCloneProgress { copied_items: 1, copied_bytes: base, retained_capacity_bytes: path.capacity() * size_of::<String>(), ..Default::default() }))
                })? {
                    self.state.part = 0;
                    self.state.phase = 69;
                }
            }
            69 => {
                if self.state.part == 4 {
                    metadata(cx, base + size_of::<String>(), || self.state.phase = 70)?;
                } else if self.nav_item.as_ref().unwrap().binding.page.choice.as_ref().unwrap().path.len() == self.state.part {
                    metadata(cx, base + size_of::<String>(), || self.nav_item.as_mut().unwrap().binding.page.choice.as_mut().unwrap().path.push(String::new()))?;
                } else {
                    let mut buffer = [0; 20];
                    let text = self.page_path_text(self.state.part, self.nav_spec().ok_or_else(invalid)?.0, &mut buffer)?;
                    if copy_text_extra(&mut self.nav_item.as_mut().unwrap().binding.page.choice.as_mut().unwrap().path[self.state.part], text, 40, cx)? {
                        self.state.part += 1;
                    }
                }
            }
            70 => {
                metadata(cx, base + 2 * size_of::<Navigation>(), || {
                    self.navigation.as_mut().unwrap().push(self.nav_item.take().unwrap());
                    self.state.nav += 1;
                    self.state.phase = 63;
                })?;
            }
            81|104=>{
                let field=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?;let text=row_data(self.request.as_ref().ok_or_else(stale)?,field.row)?.2;let bytes=text.as_bytes();let mut next=self.state;
                let mut valid=!bytes.is_empty()&&matches!(bytes[0],b'0'..=b'3')&&(bytes.len()>1||bytes[0]==b'0');
                let end=(next.offset+64).min(bytes.len());
                for index in next.offset..end{if index==0{continue;}let nibble=match bytes[index]{b'0'..=b'9'=>Some(bytes[index]-b'0'),b'A'..=b'F'=>Some(bytes[index]-b'A'+10),_=>None};valid&=nibble.is_some();if index==1&&valid&&bytes[0]!=b'0'{valid&=nibble.unwrap()<(1u8<<(4-(bytes[0]-b'0')));}}
                if !valid{next.ready=false;next.phase=4;}else{next.offset=end;if end==bytes.len(){next.phase=if self.state.phase==104{4}else{30};}}
                metadata(cx,base,||self.state=next)?;
            }
            86=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let count=list_members(self.request.as_ref().ok_or_else(stale)?,row)?.len();let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<Vec<IfcValue>>(),capacity_bytes:count*size_of::<IfcValue>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.list_values,physical,|_|{let mut values=Vec::new();values.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"LIST candidate members"))?;let capacity=values.capacity()*size_of::<IfcValue>();Ok((values,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=87;}
            }
            87=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let request=self.request.as_ref().ok_or_else(stale)?;let member=self.list_values.as_ref().ok_or_else(invalid)?.len();let count=list_members(request,row)?.len();
                if member==count{metadata(cx,base+2*size_of::<Vec<IfcValue>>()+size_of::<IfcValue>(),||{self.placeholders.as_mut().unwrap()[self.state.part-1].value=Some(IfcValue::Aggregate(self.list_values.take().unwrap()));self.state.phase=4;})?;}else{let value=canonical_decimal(list_word(request,row,member)?,false).filter(|bits|(bits>>52)&2047!=2047).map(|bits|IfcValue::Real(f64::from_bits(bits)));metadata(cx,base+2*size_of::<Vec<IfcValue>>()+size_of::<IfcValue>(),||{self.state.ready&=value.is_some();self.list_values.as_mut().unwrap().push(value.unwrap_or(IfcValue::Unset));})?;}
            }
            96=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let count=set_members(self.request.as_ref().ok_or_else(stale)?,row)?.len();let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<Vec<IfcValue>>(),capacity_bytes:count.checked_mul(size_of::<IfcValue>()).ok_or_else(invalid)?,release_bytes:0,depth:1};if admit_original_construction(cx,&mut self.list_values,physical,|_|{let mut values=Vec::new();values.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SET candidate members"))?;let capacity=values.capacity()*size_of::<IfcValue>();Ok((values,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=97;}
            }
            97=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let request=self.request.as_ref().ok_or_else(stale)?;let member=self.list_values.as_ref().ok_or_else(invalid)?.len();if member==set_members(request,row)?.len(){metadata(cx,base+2*size_of::<Vec<IfcValue>>()+2*size_of::<IfcValue>()+2*size_of::<String>(),||self.state.phase=98)?;}else{let array=self.state.leaves[self.state.part-1]==InputKind::Array;let integer=self.state.leaves[self.state.part-1]==InputKind::List;let value=if integer{signed_decimal(collection_word(request,row,member,InputKind::Integer)?).map(IfcValue::Integer)}else if array{canonical_decimal(collection_word(request,row,member,InputKind::Real)?,false).filter(|bits|(bits>>52)&2047!=2047).map(|bits|IfcValue::Real(f64::from_bits(bits)))}else{canonical_decimal(set_word(request,row,member)?,true).map(IfcValue::Reference)};metadata(cx,base+2*size_of::<Vec<IfcValue>>()+2*size_of::<IfcValue>()+2*size_of::<String>(),||{self.state.ready&=value.is_some();self.list_values.as_mut().unwrap().push(value.unwrap_or(IfcValue::Unset));})?;}
            }
            98=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let name=super::type_input::typed_constructor(constructor(self.request.as_ref().ok_or_else(stale)?,row)?).ok_or_else(invalid)?.0;let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<String>(),capacity_bytes:name.len(),release_bytes:0,depth:1};if construct_row_string(&mut self.scalar,name.len(),"",physical,cx)?{self.state.offset=0;self.state.phase=99;}
            }
            99=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let name=super::type_input::typed_constructor(constructor(self.request.as_ref().ok_or_else(stale)?,row)?).ok_or_else(invalid)?.0;let end=(self.state.offset+64).min(name.len());let mut buffer=[0u8;64];let chunk=&name.as_bytes()[self.state.offset..end];buffer[..chunk.len()].copy_from_slice(chunk);let length=chunk.len();let complete=end==name.len();metadata(cx,base+192+size_of::<String>(),||{self.scalar.as_mut().unwrap().push_str(std::str::from_utf8(&buffer[..length]).expect("admitted static SELECT member"));self.state.offset=end;if complete{self.state.phase=100;}})?;
            }
            100=>{metadata(cx,base+2*size_of::<Vec<IfcValue>>()+2*size_of::<IfcValue>()+2*size_of::<String>(),||{self.placeholders.as_mut().unwrap()[self.state.part-1].value=Some(IfcValue::TypedValue(IfcTypedValue{name:self.scalar.take().unwrap(),items:Vec::new()}));self.state.phase=101;})?;}
            101=>{
                let Some(IfcValue::TypedValue(value))=self.placeholders.as_mut().and_then(|values|values.get_mut(self.state.part-1)).and_then(|value|value.value.as_mut())else{return Err(invalid().into())};let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:size_of::<IfcValue>(),release_bytes:0,depth:1};let mut output=None;if admit_original_construction(cx,&mut output,physical,|_|{value.items.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SET constructor item"))?;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:value.items.capacity()*size_of::<IfcValue>(),..Default::default()}))})?{self.state.phase=102;}
            }
            102=>{metadata(cx,base+2*size_of::<Vec<IfcValue>>()+2*size_of::<IfcValue>()+2*size_of::<String>(),||{let Some(IfcValue::TypedValue(value))=self.placeholders.as_mut().unwrap()[self.state.part-1].value.as_mut()else{unreachable!("original SET constructor")};value.items.push(IfcValue::Aggregate(self.list_values.take().unwrap()));self.state.phase=4;})?;}
            30 => {
                let row = self.fields.as_ref().and_then(|fields| fields.last()).ok_or_else(invalid)?.row;
                let reserve = row_data(self.request.as_ref().ok_or_else(stale)?, row)?.2.len();
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base + size_of::<String>(), capacity_bytes: reserve, release_bytes: 0, depth: 1 };
                if construct_row_string(&mut self.scalar, reserve, "", physical, cx)? {
                    self.state.offset = 0;
                    self.state.phase = 31;
                }
            }
            31 => {
                let row = self.fields.as_ref().and_then(|fields| fields.last()).ok_or_else(invalid)?.row;
                let text = row_data(self.request.as_ref().ok_or_else(stale)?, row)?.2;
                let mut end = (self.state.offset + 64).min(text.len());
                while !text.is_char_boundary(end) {
                    end -= 1;
                }
                let mut buffer = [0u8; 64];
                let chunk = &text[self.state.offset..end];
                buffer[..chunk.len()].copy_from_slice(chunk.as_bytes());
                let length = chunk.len();
                let complete = end == text.len();
                metadata(cx, base + 192 + size_of::<String>(), || {
                    self.scalar.as_mut().unwrap().push_str(std::str::from_utf8(&buffer[..length]).expect("UTF8 boundary"));
                    self.state.offset = end;
                    if complete {
                        self.state.phase = 32;
                    }
                })?;
            }
            32 => {
                metadata(cx, base + 2 * size_of::<String>() + size_of::<IfcValue>(), || {
                    self.placeholders.as_mut().unwrap()[self.state.part - 1].value =
                        Some(match self.state.kinds[self.state.part-1]{InputKind::Enum=>IfcValue::Enum(self.scalar.take().unwrap()),InputKind::Binary=>IfcValue::Binary(self.scalar.take().unwrap()),_=>IfcValue::String(self.scalar.take().unwrap())});
                    self.state.phase = 4;
                })?;
            }
            22 => {
                let entity = &source.entities[self.state.position];
                if self.state.guard_part > entity.complex.len() {
                    metadata(cx, base, || self.state.phase = 5)?;
                    return Ok(false);
                }
                let (_, request) = marker(self.request.as_ref().ok_or_else(stale)?)?;
                let (target, _) = super::type_input::request(request).ok_or_else(invalid)?;
                let mut bytes = [0; 256];
                let original = if self.state.guard_part == 0 { target } else { &entity.complex[self.state.guard_part - 1].name };
                let name = super::domain_cursor::canonical_name(original, &mut bytes).ok_or_else(invalid)?;
                if self.state.guard_part > 0 && name != original {
                    return Err(invalid().into());
                }
                let value_at = |index: usize| -> Option<&IfcValue> {
                    if self.state.guard_part > 0 {
                        entity.complex[self.state.guard_part - 1].args.get(index)
                    } else {
                        let ordinal = *self.plan.map.as_ref()?.get(index)?;
                        if ordinal == usize::MAX {
                            Some(&IfcValue::Unset)
                        } else if let Some(input) = super::type_input::input_ordinal(ordinal) {
                            self.placeholders.as_ref()?.get(input)?.value.as_ref()
                        } else {
                            entity.args.get(ordinal)
                        }
                    }
                };
                let mut next = self.state;
                if next.guard_values {
                    if let Some(value) = value_at(next.guard_slot) {
                        if matches!(value, IfcValue::Derived) {
                            next.ready = false;
                        }
                        next.guard_slot += 1;
                    } else {
                        next.guard_part += 1;
                        next.guard_slot = 0;
                        next.guard_values = false;
                        next.guard_direct = false;
                    }
                } else {
                    match C::normalization_attribute(name, next.guard_slot, next.guard_direct).map_err(|_| invalid())? {
                        Some(attribute) => {
                            if attribute.derived || next.guard_direct && attribute.global_id && !matches!(value_at(next.guard_slot),Some(IfcValue::String(value))if !value.is_empty()) {
                                next.ready = false;
                            }
                            next.guard_slot += 1;
                        }
                        None => {
                            next.guard_slot = 0;
                            if next.guard_direct {
                                next.guard_values = true;
                            } else {
                                next.guard_direct = true;
                            }
                        }
                    }
                }
                metadata(cx, base, || self.state = next)?;
            }
            80=>{
                let own=4*size_of::<State>()+2*size_of::<bool>();
                if self.candidate.is_none(){
                    let bytes=size_of::<super::type_guidance_candidate::GuidedCandidate<C>>();let physical=OriginalConstructionDemand{items:1,copy_bytes:own+bytes+size_of::<Box<super::type_guidance_candidate::GuidedCandidate<C>>>(),capacity_bytes:bytes,release_bytes:0,depth:1};
                    admit_original_construction(cx,&mut self.candidate,physical,|_|Ok((Box::new(Default::default()),RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:bytes,..Default::default()})))?;
                }else{
                    let lease=self.source_loan.as_ref().and_then(|loan|loan.source()).ok_or_else(stale)?;let grant=cx.retained_grant();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(own),..grant};let before=cx.retained_progress();
                    if cx.with_retained_loan(child,|cx|self.candidate.as_mut().ok_or_else(stale)?.advance(&mut self.request,lease,source,revision,cx))?{
                        self.state.ready=self.candidate.as_ref().ok_or_else(stale)?.valid();self.state.phase=6;let delegated=cx.retained_progress()!=before;cx.consume_retained(RetainedCloneProgress{copied_items:usize::from(!delegated),copied_bytes:own,..Default::default()})?;if !delegated{cx.consume_fuel(1);}
                    }
                }
            }
            84|85|89=>self.advance_list_copy(cx)?,
            92|94|95=>self.advance_typed_copy(cx)?,
            5 => {
                let entity = &source.entities[self.state.position];
                let request = self.request.as_ref().ok_or_else(stale)?;
                let (_, value) = marker(request)?;
                let (name, _) = super::type_input::request(value).ok_or_else(invalid)?;
                let mut bytes = [0; 256];
                let name = super::domain_cursor::canonical_name(name, &mut bytes).ok_or_else(invalid)?;
                if self.state.argument == self.state.target {
                    metadata(cx, base, || self.state.phase = if self.state.ready{80}else{6})?;
                    return Ok(false);
                }
                let index = self.state.argument;
                let ordinal = *self.plan.map.as_ref().and_then(|map| map.get(index)).ok_or_else(invalid)?;
                if super::type_input::input_ordinal(ordinal).is_some_and(|input| matches!(self.placeholders.as_ref().and_then(|values| values.get(input)).and_then(|value| value.value.as_ref()), Some(IfcValue::Unset))) {
                    metadata(cx, base, || self.state.argument += 1)?;
                    return Ok(false);
                }
                let after = if ordinal == usize::MAX {
                    &IfcValue::Unset
                } else if let Some(input) = super::type_input::input_ordinal(ordinal) {
                    self.placeholders.as_ref().and_then(|values| values.get(input)).and_then(|value| value.value.as_ref()).ok_or_else(invalid)?
                } else {
                    entity.args.get(ordinal).ok_or_else(invalid)?
                };
                let demand = RetirementDemand { copy_bytes: base + 4 * size_of::<DomainCursor>(), capacity_bytes: 256, depth: 1, ..Default::default() };
                if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
                    return Ok(false);
                }
                let result = if !self.state.active {
                    let declaration = C::catalog().entities.get(name).and_then(|entity| entity.attributes.get(index)).ok_or_else(invalid)?;
                    self.domain.start(&declaration.type_expr, &[], domain::ValueChange::Set, declaration.optional, true, false).map(|_| false)
                } else {
                    self.domain.advance(C::catalog(), after, after, &CandidatePopulation { source, position: self.state.position, name })
                };
                let mut capacity = 0;
                match result {
                    Ok(true) => {
                        self.domain.clear();
                        self.state.active = false;
                        self.state.argument += 1;
                    }
                    Ok(false) => self.state.active = true,
                    Err(error) => {
                        capacity = error.capacity();
                        *self.error = Some(error);
                        self.state.ready = false;
                        self.domain.clear();
                        self.state.active = false;
                        self.state.argument += 1;
                        self.state.phase = 7;
                    }
                }
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() })?;
                cx.consume_fuel(1);
            }
            7 => {
                if self.error.is_none() && self.retirement.is_empty() {
                    metadata(cx, base, || self.state.phase = 5)?;
                } else {
                    let demand = self.retirement.next_owner_demands::<String>(self.error.is_some(), &None, 4096)?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| if self.retirement.is_empty() { self.retirement.begin_granted(&mut self.error, grant)?.ok_or_else(invalid) } else { self.retirement.step_granted(grant) })?;
                }
            }
            6 => {
                metadata(cx, base + size_of::<Page>() + 2 * size_of::<StagedFormRequest>() + size_of::<Vec<Field>>() + size_of::<Vec<Navigation>>(), || {
                    *self.page = Some(Page {
                        request: self.request.take(),
                        status: StagedFormStatus { token: empty_token(), ready: self.state.ready, diagnostics: Vec::new(), diagnostic_offset: String::new(), diagnostic_total: String::new() },
                        fields: self.fields.take().unwrap(),
                        navigation: self.navigation.take().unwrap(),
                    });
                    self.state.part = 0;
                    self.state.field = 0;
                    self.state.phase = 8;
                })?;
            }
            8 | 24 => {
                let page = self.page.as_mut().ok_or_else(stale)?;
                if self.state.phase == 24 && self.state.field == page.fields.len() {
                    metadata(cx, base, || self.state.phase = 17)?;
                    return Ok(false);
                }
                let token = &page.request.as_ref().ok_or_else(stale)?.token;
                let target = if self.state.phase == 8 { token_part_mut(&mut page.status.token, self.state.part) } else { token_part_mut(&mut page.fields[self.state.field].binding.token, self.state.part) };
                if copy_text(target, token_part(token, self.state.part), cx)? {
                    self.state.part += 1;
                    if self.state.part == 4 {
                        self.state.part = 0;
                        self.state.phase = if self.state.phase == 8 { 9 } else { 11 };
                    }
                }
            }
            9 | 10 => {
                let page = self.page.as_mut().ok_or_else(stale)?;
                let target = if self.state.phase == 9 { &mut page.status.diagnostic_offset } else { &mut page.status.diagnostic_total };
                if copy_text(target, "0", cx)? {
                    self.state.phase = if self.state.phase == 9 { 10 } else { 24 };
                }
            }
            11 => {
                let page = self.page.as_mut().ok_or_else(stale)?;
                let (name, _) = super::type_input::request(marker(page.request.as_ref().ok_or_else(stale)?)?.1).ok_or_else(invalid)?;
                let mut bytes = [0; 256];
                let name = super::domain_cursor::canonical_name(name, &mut bytes).ok_or_else(invalid)?;
                let field = &mut page.fields[self.state.field];
                let attribute = C::normalization_attribute(name, field.index, true).map_err(|_| invalid())?.ok_or_else(invalid)?;
                let mut buffer=[0u8;256];let word=attribute_text(attribute.name,field.member,&mut buffer)?;let extra=if field.member.is_some(){256+word.len()+40}else{0};if copy_text_extra(&mut field.attribute,word,extra,cx)? {
                    self.state.phase = 12;
                }
            }
            12 => {
                let page = self.page.as_mut().ok_or_else(stale)?;
                let field = &mut page.fields[self.state.field];
                let needs_row=field.operation.is_none()&&!(field.selection&&matches!(field.kind,InputKind::Number|InputKind::Enum))&&(!field.present||field.selection&&super::type_input::typed_constructor(constructor(page.request.as_ref().ok_or_else(stale)?,field.row)?).ok_or_else(invalid)?.1.is_empty());
                let count = if needs_row { 2 } else { 1 };
                let mut output = None;
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base, capacity_bytes: count * size_of::<StagedFormBindingEdit>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut output, physical, |_| {
                    field.binding.edits.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type guide bindings"))?;
                    Ok(((), RetainedCloneProgress { copied_items: 1, copied_bytes: base, retained_capacity_bytes: field.binding.edits.capacity() * size_of::<StagedFormBindingEdit>(), ..Default::default() }))
                })? {
                    self.state.phase = if needs_row { 13 } else { 15 };
                }
            }
            13 => {
                let count = self.copy_row_count()?;
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base, capacity_bytes: count * size_of::<DslValue>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.copied_rows, physical, |_| {
                    let mut rows = Vec::new();
                    rows.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type guide sorted rows"))?;
                    let capacity = rows.capacity() * size_of::<DslValue>();
                    Ok((rows, RetainedCloneProgress { copied_items: 1, copied_bytes: base, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.copying = 0;
                    self.state.offset = 0;
                    self.state.phase = 14;
                }
            }
            14 => {
                metadata(cx, base + size_of::<RowParts>(), || {
                    *self.row_parts = Some(RowParts { strings: Default::default(), outer: None, inner: None, bits:None, structured:None });
                    self.state.row_part = 0;
                    self.state.phase = 28;
                })?;
            }
            28 if self.state.row_part < self.row_string_count()? => {
                let mut buffer = [0; 20];
                let text = self.row_part_text(&mut buffer)?;
                let part = self.state.row_part;
                let reserve = text.len();
                let bytes = if part == 6 { 0 } else { text.len() };
                let mut literal = [0u8; 20];
                if part != 6 {
                    literal[..bytes].copy_from_slice(text.as_bytes());
                }
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base + size_of::<String>() + 20 + 2 * bytes, capacity_bytes: reserve, release_bytes: 0, depth: 1 };
                if construct_row_string(&mut self.row_parts.as_mut().unwrap().strings[part], reserve, std::str::from_utf8(&literal[..bytes]).expect("ASCII reference metadata"), physical, cx)? {
                    self.state.row_part += 1;
                }
            }
            28 => {
                let limit=self.row_string_count()?;let kind=self.copying_row()?.1;let list=kind==InputKind::List;let typed=kind==InputKind::Select;let count=if self.state.row_part==limit+2{1}else{2};
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base + size_of::<Vec<(String, DslValue)>>(), capacity_bytes: count * size_of::<(String, DslValue)>(), release_bytes: 0, depth: 1 };
                let parts = self.row_parts.as_mut().ok_or_else(invalid)?;
                let target=if self.state.row_part==limit{&mut parts.outer}else if self.state.row_part==limit+1{&mut parts.inner}else{&mut parts.bits};
                if admit_original_construction(cx, target, physical, |_| {
                    let mut values = Vec::new();
                    values.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type guide row fields"))?;
                    let capacity = values.capacity() * size_of::<(String, DslValue)>();
                    Ok((values, RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.row_part += 1;
                    if self.state.row_part==limit+if limit==8{3}else{2}{
                        self.state.phase = if list{84}else if typed{92}else{29};
                    }
                }
            }
            29 => {
                let kind=self.copying_row()?.1;let real=kind==InputKind::Real;let list=matches!(kind,InputKind::List|InputKind::Select);
                metadata(cx, base + 2 * size_of::<RowParts>() + 5 * size_of::<(String, DslValue)>(), || {
                    let mut parts = self.row_parts.take().unwrap();
                    let mut inner = parts.inner.take().unwrap();
                    let mut outer = parts.outer.take().unwrap();
                    inner.push((parts.strings[2].take().unwrap(), DslValue::String(parts.strings[3].take().unwrap())));
                    let payload=if list{parts.structured.take().unwrap()}else{let text=DslValue::String(parts.strings[6].take().unwrap());if real{let mut bits=parts.bits.take().unwrap();bits.push((parts.strings[7].take().unwrap(),text));DslValue::Object(bits)}else{text}};
                    inner.push((parts.strings[4].take().unwrap(),payload));
                    outer.push((parts.strings[0].take().unwrap(), DslValue::String(parts.strings[5].take().unwrap())));
                    outer.push((parts.strings[1].take().unwrap(), DslValue::Object(inner)));
                    *self.copied_row = Some(DslValue::Object(outer));
                    self.state.offset = 0;
                    self.state.phase = if list{21}else{20};
                })?;
            }
            20 => {
                let (_, _, text) = self.copying_row()?;
                let mut end = (self.state.offset + 64).min(text.len());
                while !text.is_char_boundary(end) {
                    end -= 1;
                }
                let chunk = &text[self.state.offset..end];
                let mut chunk_buffer = [0u8; 64];
                chunk_buffer[..chunk.len()].copy_from_slice(chunk.as_bytes());
                let length = chunk.len();
                let complete = end == text.len();
                metadata(cx, base + 192 + size_of::<String>(), || {
                    row_value_mut(self.copied_row.as_mut().unwrap()).expect("closed row").push_str(std::str::from_utf8(&chunk_buffer[..length]).expect("UTF8 boundary"));
                    self.state.offset = end;
                    if complete {
                        self.state.phase = 21;
                    }
                })?;
            }
            21 => {
                let count = self.copy_row_count()?;
                metadata(cx, base + 2 * size_of::<DslValue>(), || {
                    self.copied_rows.as_mut().unwrap().push(self.copied_row.take().unwrap());
                    self.state.copying += 1;
                    self.state.phase = if self.state.copying == count { 26 } else { 14 };
                })?;
            }
            26 => {
                let capacity = 2 * size_of::<String>() + 15;
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base + capacity + size_of::<StagedFormBindingEdit>() + size_of::<Vec<DslValue>>(), capacity_bytes: capacity, release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.edit, physical, |_| {
                    Ok((
                        StagedFormBindingEdit::Fixed { edit: StagedFormEdit::Set { path: vec!["value".into(), "$ifcInputs".into()], value: DslValue::Array(self.copied_rows.take().unwrap()) } },
                        RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() },
                    ))
                })? {
                    self.state.phase = 27;
                }
            }
            27 => {
                metadata(cx, base + size_of::<StagedFormBindingEdit>(), || {
                    self.page.as_mut().unwrap().fields[self.state.field].binding.edits.push(self.edit.take().unwrap());
                    self.state.phase = 15;
                })?;
            }
            33 => {
                let append=self.current_field()?.selection&&matches!(self.current_field()?.kind,InputKind::Number|InputKind::Enum)&&self.choice_path_len()?==6;
                metadata(cx, base + size_of::<StagedFormBindingEdit>() + size_of::<ChoiceParts>(), || {
                    *self.edit = Some(StagedFormBindingEdit::Choice { kind: if append{StagedFormChoiceEditKind::Append}else{StagedFormChoiceEditKind::Set}, path: Vec::new(), options: Vec::new() });
                    *self.choice_parts = Some(ChoiceParts { strings: Default::default(), payload: None, bits:None, structured:None });
                    self.state.phase = 34;
                })?;
            }
            34 | 35 => {
                let (_, length, pin) = self.choice_shape()?;
                let count = length + usize::from(pin.is_some());
                let physical = OriginalConstructionDemand {
                    items: 1,
                    copy_bytes: base + if self.state.phase == 34 { size_of::<Vec<String>>() } else { 0 },
                    capacity_bytes: if self.state.phase == 34 { self.choice_path_len()? * size_of::<String>() } else { count * size_of::<StagedFormBindingOption>() },
                    release_bytes: 0,
                    depth: 1,
                };
                let path_count=self.choice_path_len()?;
                let StagedFormBindingEdit::Choice { path, options, .. } = self.edit.as_mut().ok_or_else(invalid)? else { return Err(invalid().into()) };
                let mut output = None;
                if admit_original_construction(cx, &mut output, physical, |_| {
                    let capacity = if self.state.phase == 34 {
                        path.try_reserve_exact(path_count).map_err(|_| invalid())?;
                        path.capacity() * size_of::<String>()
                    } else {
                        options.try_reserve_exact(count).map_err(|_| invalid())?;
                        options.capacity() * size_of::<StagedFormBindingOption>()
                    };
                    Ok(((), RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.phase += 1;
                }
            }
            36 => {
                let (_, length, pin) = self.choice_shape()?;
                let count = length + usize::from(pin.is_some());
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base + size_of::<Choices>(), capacity_bytes: count * size_of::<StagedScalarChoice>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.page.as_mut().unwrap().fields[self.state.field].choices, physical, |_| {
                    let mut rows = Vec::new();
                    rows.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type declared choices"))?;
                    let capacity = rows.capacity() * size_of::<StagedScalarChoice>();
                    Ok((Choices::new(rows), RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.choice_part = 0;
                    self.state.phase = 37;
                }
            }
            37 => {
                if self.state.choice_part == self.choice_path_len()? {
                    let select=self.current_field()?.kind==InputKind::Select;
                    metadata(cx, base + size_of::<String>(), || {
                        self.state.choice_part = 0;
                        self.state.phase = if select{93}else{38};
                    })?;
                } else {
                    let mut buffer=[0;20];let text=self.choice_path_word(self.state.choice_part,&mut buffer)?;
                    let StagedFormBindingEdit::Choice { path, .. } = self.edit.as_mut().ok_or_else(invalid)? else { return Err(invalid().into()) };
                    if path.len() == self.state.choice_part {
                        metadata(cx, base + size_of::<String>(), || path.push(String::new()))?;
                    } else {
                        if copy_text_extra(&mut path[self.state.choice_part], text, 40, cx)? {
                            self.state.choice_part += 1;
                        }
                    }
                }
            }
            38 if self.state.choice_part < self.choice_string_count()? => {
                let text=self.choice_text()?;let length=text.len();let raw=self.choice_raw()?;let mut literal=[0u8;128];
                if !raw{if length>literal.len(){return Err(invalid().into());}literal[..length].copy_from_slice(text.as_bytes());}
                let bytes=if raw{0}else{length};let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<String>()+2*bytes,capacity_bytes:length,release_bytes:0,depth:1};
                if construct_row_string(&mut self.choice_parts.as_mut().unwrap().strings[self.state.choice_part],length,std::str::from_utf8(&literal[..bytes]).expect("declaration choice text"),physical,cx)?{
                    if raw{self.state.offset=0;self.state.phase=82;}else{self.state.choice_part+=1;}
                }
            }
            82=>{
                let text=self.choice_text()?;let mut end=(self.state.offset+64).min(text.len());while !text.is_char_boundary(end){end-=1;}
                let chunk=&text[self.state.offset..end];let mut buffer=[0u8;64];buffer[..chunk.len()].copy_from_slice(chunk.as_bytes());let length=chunk.len();let complete=end==text.len();
                metadata(cx,base+192+size_of::<String>(),||{self.choice_parts.as_mut().unwrap().strings[4].as_mut().unwrap().push_str(std::str::from_utf8(&buffer[..length]).expect("UTF8 boundary"));self.state.offset=end;if complete{self.state.choice_part+=1;self.state.phase=38;}})?;
            }
            38 => {
                metadata(cx, base, || self.state.phase = 39)?;
            }
            39 => {
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base + size_of::<Vec<(String, DslValue)>>(), capacity_bytes: 2 * size_of::<(String, DslValue)>(), release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.choice_parts.as_mut().unwrap().payload, physical, |_| {
                    let mut fields = Vec::new();
                    fields.try_reserve_exact(2).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type enum payload"))?;
                    let capacity = fields.capacity() * size_of::<(String, DslValue)>();
                    Ok((fields, RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() }))
                })? {
                    self.state.phase = if self.current_field()?.kind==InputKind::List{85}else if self.current_field()?.kind==InputKind::Select&&self.state.choice_kind!=InputKind::Reference{94}else if self.choice_real()?{83}else{40};
                }
            }
            93=>{
                if self.state.leaf_name.is_none(){let field=self.current_field()?;let(start,length,pin)=self.choice_shape()?;let target=if self.state.option<length{start+self.state.option}else{pin.ok_or_else(invalid)?};let mut cursor=self.leaf_cursor;let answer=cursor.advance::<C>(self.domains[field.ordinal].ok_or_else(invalid)?)?;if matches!(answer,super::type_input_select::Step::Complete{..}){return Err(invalid().into());}metadata(cx,base,||{self.leaf_cursor=cursor;if let super::type_input_select::Step::Leaf{index,name}=answer{if index==target{self.state.leaf_name=Some(name);self.leaf_cursor.clear();}}})?;return Ok(false);}
                let name=self.choice_word()?;if C::direct_attribute_count(name).is_some(){metadata(cx,base+2*size_of::<InputKindCursor>(),||{self.state.choice_kind=InputKind::Reference;self.kind_cursor.clear();self.state.phase=38;})?;}else{let expression=C::defined_type_expression(name).ok_or_else(invalid)?;let mut cursor=self.kind_cursor;let answer=cursor.advance_selected::<C>(expression)?;if answer.is_some_and(|kind|!matches!(kind,InputKind::String|InputKind::Integer|InputKind::Enum|InputKind::Real|InputKind::Number|InputKind::Binary|InputKind::List|InputKind::Set|InputKind::Array)){return Err(invalid().into());}metadata(cx,base+2*size_of::<InputKindCursor>(),||{self.kind_cursor=cursor;if let Some(kind)=answer{self.state.choice_kind=kind;self.kind_cursor.clear();self.state.phase=38;}})?;}
            }
            83=>{
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<Vec<(String,DslValue)>>(),capacity_bytes:size_of::<(String,DslValue)>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.choice_parts.as_mut().unwrap().bits,physical,|_|{let mut fields=Vec::new();fields.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"numeric bit carrier"))?;let capacity=fields.capacity()*size_of::<(String,DslValue)>();Ok((fields,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=40;}
            }
            40 => {
                let word = self.choice_word()?;
                let select=self.current_field()?.kind==InputKind::Select;
                let list=self.current_field()?.kind==InputKind::List||select&&self.state.choice_kind!=InputKind::Reference;
                let locale = self.page.as_ref().unwrap().request.as_ref().unwrap().locale;
                metadata(cx, base + 4 * size_of::<ChoiceParts>() + 2 * size_of::<StagedFormBindingOption>() + size_of::<StagedScalarChoice>(), || {
                    let parts = self.choice_parts.as_mut().unwrap();
                    let mut payload = parts.payload.take().unwrap();
                    payload.push((parts.strings[1].take().unwrap(), DslValue::String(parts.strings[2].take().unwrap())));
                    let value=if list{parts.structured.take().unwrap()}else{let text=DslValue::String(parts.strings[4].take().unwrap());if let Some(mut bits)=parts.bits.take(){bits.push((parts.strings[5].take().unwrap(),text));DslValue::Object(bits)}else{text}};
                    payload.push((parts.strings[3].take().unwrap(),value));
                    let StagedFormBindingEdit::Choice { options, .. } = self.edit.as_mut().unwrap() else { unreachable!() };
                    options.push(StagedFormBindingOption { key: parts.strings[0].take().unwrap(), value: DslValue::Object(payload) });
                    self.page.as_mut().unwrap().fields[self.state.field].choices.as_mut().unwrap().push(word, Self::choice_label(word, locale));
                    self.state.option += 1;
                    self.state.leaf_name=None;
                    self.leaf_cursor.clear();
                    self.state.choice_part = 0;
                    let (_, length, pin) = self.choice_shape().expect("completed declaration window");
                    self.state.phase = if self.state.option == length + usize::from(pin.is_some()) { 16 } else if select{93}else{38};
                })?;
            }
            15 if self.current_field()?.operation.is_some()=>{let page=self.page.as_ref().ok_or_else(stale)?;if set_controls::advance(&page.fields[self.state.field],page.request.as_ref().ok_or_else(stale)?,&mut self.edit,base,cx)?{self.state.phase=16;}},
            15 if matches!(self.current_field()?.kind,InputKind::Enum|InputKind::Number|InputKind::List|InputKind::Select) => {
                metadata(cx, base, || {
                    self.state.option = 0;
                    self.state.leaf_name=None;
                    self.leaf_cursor.clear();
                    self.state.choice_part = 0;
                    self.state.phase = 33;
                })?;
            }
            15 => {
                let mut buffer = [0; 20];
                let row = decimal(self.current_field()?.row, &mut buffer);
                let real=self.current_field()?.kind==InputKind::Real;
                let selection=self.current_field()?.selection;
                let indexed=self.current_field()?.member.is_some();let mut member=[0;20];let member=decimal(self.current_field()?.member.unwrap_or(0),&mut member);let capacity=text_path_capacity(self.current_field()?,row,member);
                let physical = OriginalConstructionDemand { items: 1, copy_bytes: base + capacity + size_of::<StagedFormBindingEdit>(), capacity_bytes: capacity, release_bytes: 0, depth: 1 };
                if admit_original_construction(cx, &mut self.edit, physical, |_| {
                    Ok((
                        StagedFormBindingEdit::Text { path: if selection&&indexed{set_controls::text_path(row,member,real)}else if selection{let mut path=vec!["value".into(),"$ifcInputs".into(),row.into(),"value".into(),"value".into(),"items".into(),"0".into(),"value".into()];if real{path.push("bits".into());}path}else if indexed{vec!["value".into(),"$ifcInputs".into(),row.into(),"value".into(),"value".into(),member.into(),"value".into(),"bits".into()]}else if real{vec!["value".into(), "$ifcInputs".into(), row.into(), "value".into(), "value".into(),"bits".into()]}else{vec!["value".into(), "$ifcInputs".into(), row.into(), "value".into(), "value".into()]} },
                        RetainedCloneProgress { copied_items: 1, copied_bytes: physical.copy_bytes, retained_capacity_bytes: capacity, ..Default::default() },
                    ))
                })? {
                    self.state.phase = 16;
                }
            }
            16 => {
                let selected_choice=if self.current_field()?.kind==InputKind::Select{if let Some(index)=self.current_field()?.selected_choice{let(start,length,_)=self.choice_shape()?;Some(if index>=start&&index<start+length{index-start}else{length})}else{None}}else{self.current_field()?.selected_choice};
                metadata(cx, base + size_of::<StagedFormBindingEdit>(), || {
                    self.page.as_mut().unwrap().fields[self.state.field].binding.edits.push(self.edit.take().unwrap());
                    self.page.as_mut().unwrap().fields[self.state.field].selected_choice=selected_choice;
                    self.state.field += 1;
                    self.state.part = 0;
                    self.state.phase = 24;
                })?;
            }
            17 => {
                if self.plan.map.is_some() || self.placeholders.is_some() || !self.retirement.is_empty() {
                    let demand = self.demands()?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                        if !self.retirement.is_empty() {
                            self.retirement.step_granted(grant)
                        } else if self.plan.map.is_some() {
                            self.retirement.begin_granted(&mut self.plan.map, grant)?.ok_or_else(invalid)
                        } else {
                            self.retirement.begin_granted(&mut self.placeholders, grant)?.ok_or_else(invalid)
                        }
                    })?;
                } else {
                    metadata(cx, base, || {
                        self.plan.clear_metadata();
                        self.state.phase = 18;
                    })?;
                }
            }
            18 => return Ok(true),
            _ => return Err(invalid().into()),
        }
        Ok(false)
    }
    pub fn take(&mut self, output: &mut Option<IfcPreparedTypeInputSource>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.state.phase != 18 || self.state.failed || self.state.closing || output.is_some() || self.binding.is_none_or(|(binding, _, _)| !binding.matches(cx)) {
            return Err(stale().into());
        }
        metadata(cx, size_of::<Page>() + size_of::<IfcPreparedTypeInputSource>(), || {
            *output = Some(IfcPreparedTypeInputSource { page: ManuallyDrop::new(self.page.take()) });
            self.state.phase = 19;
        })
    }
    pub fn begin_close(&mut self) {
        self.state.closing = true;
        self.list_copy.begin_close();
        self.typed_copy.begin_close();
    }
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.binding.is_some_and(|(binding, _, _)| !binding.matches(cx)) {
            return Err(stale().into());
        }
        if !self.state.closing {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
        }
        if self.terminal_is_empty() {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() });
        }
        let demand = self.close_demand(4096)?;
        let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| self.close_owned(grant))?;
        Ok(match step {
            semio_framework_job::InteractiveJobCloseStep::Complete { progress } if !self.terminal_is_empty() => semio_framework_job::InteractiveJobCloseStep::Pending { progress },
            other => other,
        })
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.request.is_none()
            && self.page.is_none()
            && self.edit.is_none()
            && self.error.is_none()
            && self.placeholders.is_none()
            && self.fields.is_none()
            && self.copied_rows.is_none()
            && self.copied_row.is_none()
            && self.row_parts.is_none()
            && self.scalar.is_none()
            && self.list_values.is_none()
            && self.list_copy.empty()
            && self.typed_copy.empty()
            && self.source_loan.is_none()
            && self.candidate.is_none()
            && self.choice_parts.is_none()
            && self.navigation.is_none()
            && self.nav_item.is_none()
            && self.plan.map.is_none()
            && self.retirement.is_empty()
    }
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.typed_copy.empty(){return self.typed_copy.close_demands(body);}
        if !self.list_copy.empty(){return self.list_copy.close_demands(body);}
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<DslValue>(false, &None, body);
        }
        macro_rules! owner {
            ($value:expr,$kind:ty) => {
                if ($value).is_some() {
                    return self.retirement.next_owner_demands::<$kind>(true, &None, body);
                }
            };
        }
        owner!(&self.candidate, Box<super::type_guidance_candidate::GuidedCandidate<C>>);
        owner!(&self.edit, StagedFormBindingEdit);
        owner!(&self.error, String);
        owner!(&self.scalar, String);
        owner!(&self.list_values,Vec<IfcValue>);
        owner!(&self.choice_parts, ChoiceParts);
        owner!(&self.navigation, Vec<Navigation>);
        owner!(&self.nav_item, Navigation);
        owner!(&self.row_parts, RowParts);
        owner!(&self.copied_row, DslValue);
        owner!(&self.copied_rows, Vec<DslValue>);
        owner!(&self.fields, Vec<Field>);
        owner!(&self.page, Page);
        owner!(&self.request, StagedFormRequest);
        owner!(&self.placeholders, Vec<super::type_input::Input>);
        owner!(&self.plan.map, Vec<usize>);
        owner!(&self.source_loan, super::source_loan::SourceLoan);
        Ok(Default::default())
    }
    fn close_owned(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if !retained_clone_grant_admits(grant, self.close_demand(grant.maximum_copy_bytes)?) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if !self.typed_copy.empty(){return self.typed_copy.close_owned(grant);}
        if !self.list_copy.empty(){return self.list_copy.close_owned(grant);}
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        macro_rules! owner {
            ($value:expr) => {
                if ($value).is_some() {
                    return self.retirement.begin_granted($value, grant)?.ok_or_else(invalid);
                }
            };
        }
        owner!(&mut self.candidate);
        owner!(&mut self.edit);
        owner!(&mut self.error);
        owner!(&mut self.scalar);
        owner!(&mut self.list_values);
        owner!(&mut self.choice_parts);
        owner!(&mut self.navigation);
        owner!(&mut self.nav_item);
        owner!(&mut self.row_parts);
        owner!(&mut self.copied_row);
        owner!(&mut self.copied_rows);
        owner!(&mut self.fields);
        owner!(&mut self.page);
        owner!(&mut self.request);
        owner!(&mut self.placeholders);
        owner!(&mut self.plan.map);
        owner!(&mut self.source_loan);
        Err(invalid())
    }
}
impl<C: IfcDetailsProfile + 'static> RetireOwned for IfcTypeInputSourceWork<C> {
    fn retirement(mut self) -> Box<dyn RetirementCursor> {
        self.begin_close();
        Box::new(self)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
impl<C: IfcDetailsProfile + 'static> RetirementCursor for IfcTypeInputSourceWork<C> {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demand(body)
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
        if self.terminal_is_empty() {
            return semio_framework_value::retirement::RetirementStep::Complete;
        }
        match self.close_owned(grant) {
            Ok(step) => semio_framework_value::retirement::RetirementStep::Progress(step.progress()),
            Err(error) => semio_framework_value::retirement::RetirementStep::Failure(error),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        IfcTypeInputSourceWork::terminal_is_empty(self)
    }
    fn next_close_byte_demand(&self) -> Option<usize> {
        Some(0)
    }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        Some(0)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(usize::MAX)?.copy_bytes)
    }
    fn next_selected_work_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand(body)?.copy_bytes)
    }
}
impl<C: IfcDetailsProfile + 'static> Drop for IfcTypeInputSourceWork<C> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "type input guidance requires original paid retirement");
    }
}
#[cfg(test)]
#[path = "type-guidance-tests.rs"]
mod tests;
