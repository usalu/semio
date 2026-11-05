//! 🧷️ Immutable binding views borrow resident paths, choices and values before the paid payload writer constructs UI ownership.
use super::StagedPayloadSource;
use semio_framework::action_staging::*;
use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};

/// 🛤️ A resident decoded path; each segment stays owned by its original prepared source.
pub trait StagedBindingPathSource: Send + Sync {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn segment(&self, index: usize) -> Option<&str>;
}
impl StagedBindingPathSource for Vec<String> {
    fn len(&self) -> usize { Vec::len(self) }
    fn segment(&self, index: usize) -> Option<&str> { self.get(index).map(String::as_str) }
}
impl<const N: usize> StagedBindingPathSource for [&str; N] {
    fn len(&self) -> usize { N }
    fn segment(&self, index: usize) -> Option<&str> { self.get(index).copied() }
}
/// 🎁️ Existing data and virtual borrowed value trees share the same paid payload construction boundary.
#[derive(Clone, Copy)]
pub enum StagedBindingValueView<'a> {
    Value(&'a DslValue),
    Payload(&'a dyn StagedPayloadSource),
}
#[derive(Clone, Copy)]
pub struct StagedBindingOptionView<'a> { pub key: &'a str, pub value: StagedBindingValueView<'a> }
/// 🔖️ A source supplies at most thirty-two exact resident options for this control page.
pub trait StagedBindingOptionsSource: Send + Sync {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn option(&self, index: usize) -> Option<StagedBindingOptionView<'_>>;
}
impl StagedBindingOptionsSource for Vec<StagedFormBindingOption> {
    fn len(&self) -> usize { Vec::len(self) }
    fn option(&self, index: usize) -> Option<StagedBindingOptionView<'_>> { self.get(index).map(|option| StagedBindingOptionView { key: &option.key, value: StagedBindingValueView::Value(&option.value) }) }
}
#[derive(Clone, Copy)]
pub enum StagedBindingFixedView<'a> {
    Set { path: &'a dyn StagedBindingPathSource, value: StagedBindingValueView<'a> },
    Append { path: &'a dyn StagedBindingPathSource, value: StagedBindingValueView<'a> },
    Remove { path: &'a dyn StagedBindingPathSource },
}
#[derive(Clone, Copy)]
pub enum StagedBindingEditView<'a> {
    Fixed(StagedBindingFixedView<'a>),
    Text { path: &'a dyn StagedBindingPathSource },
    Choice { kind: StagedFormChoiceEditKind, path: &'a dyn StagedBindingPathSource, options: &'a dyn StagedBindingOptionsSource },
}
/// 🧷️ The immutable source view is independent of an owned DTO and never constructs data from its accessors.
pub trait StagedFormBindingSource: Send + Sync {
    fn source_identity(&self) -> usize { self as *const Self as *const () as usize }
    fn token(&self) -> &StagedFormToken;
    fn layout(&self) -> StagedFormLayout;
    fn edits(&self) -> usize;
    fn edit(&self, index: usize) -> Result<StagedBindingEditView<'_>, ValueError>;
}
fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, "staging.binding") }
fn resource() -> ValueError { ValueError::literal(ValueRefusalKind::OwnershipLimit, "staging.resource-limit") }
impl StagedFormBindingSource for StagedFormBinding {
    fn token(&self) -> &StagedFormToken { &self.token }
    fn layout(&self) -> StagedFormLayout { self.layout }
    fn edits(&self) -> usize { self.edits.len() }
    fn edit(&self, index: usize) -> Result<StagedBindingEditView<'_>, ValueError> {
        Ok(match self.edits.get(index).ok_or_else(invalid)? {
            StagedFormBindingEdit::Text { path } => StagedBindingEditView::Text { path },
            StagedFormBindingEdit::Choice { kind, path, options } => StagedBindingEditView::Choice { kind: *kind, path, options },
            StagedFormBindingEdit::Fixed { edit } => StagedBindingEditView::Fixed(match edit {
                StagedFormEdit::Set { path, value } => StagedBindingFixedView::Set { path, value: StagedBindingValueView::Value(value) },
                StagedFormEdit::Append { path, value } => StagedBindingFixedView::Append { path, value: StagedBindingValueView::Value(value) },
                StagedFormEdit::Remove { path } => StagedBindingFixedView::Remove { path },
            }),
        })
    }
}
impl<'a> StagedBindingEditView<'a> {
    pub fn path(self) -> &'a dyn StagedBindingPathSource {
        match self { Self::Text { path } | Self::Choice { path, .. } | Self::Fixed(StagedBindingFixedView::Set { path, .. } | StagedBindingFixedView::Append { path, .. } | StagedBindingFixedView::Remove { path }) => path }
    }
}
/// 🛡️ Borrowed and owned bindings obey the same token, path, single-input, layout and exact-option rules.
pub fn validate_staged_binding_source(source: &dyn StagedFormBindingSource) -> Result<(), ValueError> {
    if !source.token().admitted() || source.edits() == 0 || source.edits() > 8 { return Err(invalid()); }
    if source.layout() == StagedFormLayout::Preserve && (source.edits() != 1 || !matches!(source.edit(0)?, StagedBindingEditView::Text { .. })) { return Err(invalid()); }
    let mut inputs = 0;
    for index in 0..source.edits() {
        let edit = source.edit(index)?;
        let path = edit.path();
        if path.len() == 0 || path.len() > 64 { return Err(invalid()); }
        for index in 0..path.len() {
            let part = path.segment(index).ok_or_else(invalid)?;
            if part.is_empty() || matches!(part, "__proto__" | "prototype" | "constructor") { return Err(invalid()); }
        }
        match edit {
            StagedBindingEditView::Text { .. } => inputs += 1,
            StagedBindingEditView::Choice { options, .. } => {
                inputs += 1;
                if options.len() == 0 || options.len() > 32 { return Err(invalid()); }
                for index in 0..options.len() {
                    let option = options.option(index).ok_or_else(invalid)?;
                    if option.key.len() > semio_framework_ui_contract::UiText::default().capacity() { return Err(resource()); }
                    if option.key.is_empty() { return Err(invalid()); }
                    for previous in 0..index { if options.option(previous).ok_or_else(invalid)?.key == option.key { return Err(invalid()); } }
                }
            },
            _ => {},
        }
    }
    if inputs > 1 { return Err(invalid()); }
    Ok(())
}
