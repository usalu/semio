//! 🧷️ Authored binding views become closed neutral payloads only through the paid original payload cursor.
use super::{StagedPayloadKind as Kind, StagedPayloadNode as Node, StagedPayloadSource, StagedFormBindingSource, StagedBindingEditView as Edit, StagedBindingFixedView as Fixed, StagedBindingPathSource, StagedBindingOptionsSource, StagedBindingOptionView, StagedBindingValueView, validate_staged_binding_source, StagedFormPageBindingSource, StagedChoicePageView, validate_staged_page_binding_source};
use semio_framework::action_staging::*;
use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};

pub enum StagedBindingPayload<'a> { Action(&'a dyn StagedFormBindingSource), Page(&'a StagedFormPageBinding), PageSource(&'a dyn StagedFormPageBindingSource) }
#[derive(Clone, Copy)]
enum View<'a> {
    Action(&'a dyn StagedFormBindingSource), PageBinding(&'a dyn StagedFormPageBindingSource), Token(&'a StagedFormToken),
    Page(&'a dyn StagedFormPageBindingSource), ChoicePage(StagedChoicePageView<'a>), Edits(&'a dyn StagedFormBindingSource),
    Edit(Edit<'a>), Fixed(Fixed<'a>), Path(&'a dyn StagedBindingPathSource), Options(&'a dyn StagedBindingOptionsSource),
    Option(StagedBindingOptionView<'a>), Data(&'a DslValue), External(&'a dyn StagedPayloadSource), Text(&'a str),
}
fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, "staging.binding") }
fn resource() -> ValueError { ValueError::literal(ValueRefusalKind::OwnershipLimit, "staging.resource-limit") }
fn data(value: StagedBindingValueView<'_>) -> View<'_> { match value { StagedBindingValueView::Value(value) => View::Data(value), StagedBindingValueView::Payload(value) => View::External(value) } }
impl<'a> View<'a> {
    fn kind(self) -> Result<Kind<'a>, ValueError> {
        Ok(match self {
            Self::Action(binding) => { validate_staged_binding_source(binding)?; Kind::Map(3) },
            Self::PageBinding(binding) => { validate_staged_page_binding_source(binding)?; Kind::Map(2) },
            Self::Token(_) => Kind::Map(4), Self::Page(page) => Kind::Map(if page.choice().is_some() { 3 } else { 2 }),
            Self::ChoicePage(_) => Kind::Map(2), Self::Edits(binding) => Kind::List(binding.edits()),
            Self::Edit(Edit::Choice { options, .. }) => { if options.len() == 0 || options.len() > 32 { return Err(invalid()); } Kind::Map(4) },
            Self::Edit(_) => Kind::Map(2), Self::Fixed(Fixed::Remove { .. }) => Kind::Map(2), Self::Fixed(_) => Kind::Map(3),
            Self::Path(path) => { if path.len() == 0 || path.len() > 64 { return Err(invalid()); } Kind::List(path.len()) },
            Self::Options(options) => { if options.len() == 0 || options.len() > 32 { return Err(invalid()); } Kind::List(options.len()) },
            Self::Option(_) => Kind::Map(2), Self::Data(value) => super::payload::dsl_kind(value)?,
            Self::External(source) => source.node(&[])?.value, Self::Text(value) => Kind::Text(value),
        })
    }
    fn child(self, index: usize) -> Result<(Option<&'a str>, Self), ValueError> {
        Ok(match self {
            Self::Action(binding) => match index { 0 => (Some("token"), Self::Token(binding.token())), 1 => (Some("layout"), Self::Text(if binding.layout() == StagedFormLayout::Preserve { "preserve" } else { "replace" })), 2 => (Some("edits"), Self::Edits(binding)), _ => return Err(invalid()) },
            Self::PageBinding(binding) => match index { 0 => (Some("token"), Self::Token(binding.token())), 1 => (Some("page"), Self::Page(binding)), _ => return Err(invalid()) },
            Self::Token(token) => match index { 0 => (Some("action"), Self::Text(&token.action)), 1 => (Some("generation"), Self::Text(&token.generation)), 2 => (Some("revision"), Self::Text(&token.revision)), 3 => (Some("window"), Self::Text(&token.window)), _ => return Err(invalid()) },
            Self::Page(page) => match index { 0 => (Some("fieldOffset"), Self::Text(page.field_offset())), 1 => (Some("diagnosticOffset"), Self::Text(page.diagnostic_offset())), 2 => (Some("choice"), Self::ChoicePage(page.choice().ok_or_else(invalid)?)), _ => return Err(invalid()) },
            Self::ChoicePage(page) => match index { 0 => (Some("path"), Self::Path(page.path)), 1 => (Some("offset"), Self::Text(page.offset)), _ => return Err(invalid()) },
            Self::Edits(binding) => (None, Self::Edit(binding.edit(index)?)),
            Self::Edit(edit) => match (edit, index) {
                (Edit::Fixed(_), 0) => (Some("source"), Self::Text("fixed")), (Edit::Text { .. }, 0) => (Some("source"), Self::Text("text")), (Edit::Choice { .. }, 0) => (Some("source"), Self::Text("choice")),
                (Edit::Fixed(edit), 1) => (Some("edit"), Self::Fixed(edit)), (Edit::Text { path }, 1) => (Some("path"), Self::Path(path)),
                (Edit::Choice { kind, .. }, 1) => (Some("kind"), Self::Text(if kind == StagedFormChoiceEditKind::Set { "set" } else { "append" })),
                (Edit::Choice { path, .. }, 2) => (Some("path"), Self::Path(path)), (Edit::Choice { options, .. }, 3) => (Some("options"), Self::Options(options)), _ => return Err(invalid()),
            },
            Self::Fixed(edit) => match (edit, index) {
                (Fixed::Set { .. }, 0) => (Some("kind"), Self::Text("set")), (Fixed::Append { .. }, 0) => (Some("kind"), Self::Text("append")), (Fixed::Remove { .. }, 0) => (Some("kind"), Self::Text("remove")),
                (Fixed::Set { path, .. } | Fixed::Append { path, .. } | Fixed::Remove { path }, 1) => (Some("path"), Self::Path(path)),
                (Fixed::Set { value, .. } | Fixed::Append { value, .. }, 2) => (Some("value"), data(value)), _ => return Err(invalid()),
            },
            Self::Path(path) => { let part = path.segment(index).ok_or_else(invalid)?; if part.is_empty() || matches!(part, "__proto__" | "prototype" | "constructor") { return Err(invalid()); } (None, Self::Text(part)) },
            Self::Options(options) => {
                let option = options.option(index).ok_or_else(invalid)?;
                if option.key.len() > semio_framework_ui_contract::UiText::default().capacity() { return Err(resource()); }
                if option.key.is_empty() { return Err(invalid()); }
                for previous in 0..index { if options.option(previous).ok_or_else(invalid)?.key == option.key { return Err(invalid()); } }
                (None, Self::Option(option))
            },
            Self::Option(option) => match index { 0 => (Some("key"), Self::Text(option.key)), 1 => (Some("value"), data(option.value)), _ => return Err(invalid()) },
            Self::Data(DslValue::Object(fields)) => { let (key, value) = fields.get(index).ok_or_else(invalid)?; (Some(key), Self::Data(value)) },
            Self::Data(DslValue::Array(values)) => (None, Self::Data(values.get(index).ok_or_else(invalid)?)),
            _ => return Err(invalid()),
        })
    }
}
impl StagedPayloadSource for StagedBindingPayload<'_> {
    fn identity(&self) -> usize { match self { Self::Action(binding) => binding.source_identity(), Self::Page(binding) => (*binding as *const _ as usize) ^ 1, Self::PageSource(binding) => binding.source_identity() ^ 1 } }
    fn node(&self, path: &[usize]) -> Result<Node<'_>, ValueError> {
        let mut value = match self { Self::Action(binding) => View::Action(*binding), Self::Page(binding) => View::PageBinding(*binding), Self::PageSource(binding) => View::PageBinding(*binding) };
        let mut key = None;
        for (depth, index) in path.iter().enumerate() {
            if let View::External(source) = value { return source.node(&path[depth..]); }
            let child = value.child(*index)?; key = child.0; value = child.1;
        }
        Ok(Node { key, value: value.kind()? })
    }
}
