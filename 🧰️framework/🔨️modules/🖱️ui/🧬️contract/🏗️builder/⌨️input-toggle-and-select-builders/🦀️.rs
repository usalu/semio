//! ⌨️ Input, toggle and select builders: the input builder with its text constraints, the toggle builder and the select builder. See [`super`].

use super::*;

//#region 🔖️Builder
//#region ⌨️Input
/// ⌨️ A text/number/date/color/file input — `Component::Input`. Build with [`input`].
pub struct InputBuilder {
    pub(super) base: NodeBase,
    pub(super) kind: crate::InputKind,
    pub(super) value: crate::UiText,
    pub(super) placeholder: Option<crate::Label>,
    pub(super) commit: Option<crate::UiText>,
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    pub(super) step: Option<f64>,
    accept: Option<crate::UiText>,
    pub(super) precision: Option<u16>,
    pub(super) snaps: crate::UiFixedList<f64>,
    pub(super) display_factor: Option<f64>,
    pub(super) limits: Option<crate::UiNumberLimits>,
    pub(super) draft_target: Option<crate::UiText>,
    pub(super) publication_revision: Option<crate::UiPublicationRevision>,
}

/// ⌨️ An input of `kind`, initially empty.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn input(kind: crate::InputKind) -> InputBuilder {
    InputBuilder { base: NodeBase::leaf(), kind, value: crate::UiText::default(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, precision: None, snaps: crate::UiFixedList::default(), display_factor: None, limits: None, draft_target: None, publication_revision: None }
}

impl InputBuilder {
    /// ✍️ Sets the current value.
    pub fn value(mut self, value: crate::UiText) -> Self {
        self.value = value;
        self
    }

    /// 💬️ Sets placeholder copy.
    pub fn placeholder(mut self, placeholder: crate::Label) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// 🫳️ Sets the commit convention (e.g. `"blur"`).
    pub fn commit(mut self, commit: crate::UiText) -> Self {
        self.commit = Some(commit);
        self
    }

    /// ⬇️ Sets the minimum value (`InputKind::Number`).
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// ⬆️ Sets the maximum value (`InputKind::Number`).
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// 🔢️ Sets the step increment (`InputKind::Number`).
    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    /// 📎️ Sets the accepted file-type filter (`InputKind::File`).
    pub fn accept(mut self, accept: crate::UiText) -> Self {
        self.accept = Some(accept);
        self
    }

    /// 🥅️ Sets the fraction digits a number field shows and commits, capped at
    /// [`crate::UI_NUMBER_PRECISION_MAX`] (`InputKind::Number`).
    pub fn precision(mut self, precision: u16) -> Self {
        self.precision = Some(precision.min(crate::UI_NUMBER_PRECISION_MAX));
        self
    }

    /// 💯️ Sets a number field's value, printed at the field's precision when one is set — or, once a display factor
    /// is set, as the exact stored number the renderer shows `× factor` at that precision.
    pub fn number(mut self, value: f64) -> Self {
        let text = match (self.display_factor, self.precision) {
            (None, Some(precision)) => crate::format_ui_number_fixed(value, precision),
            _ => crate::format_ui_number(value),
        };
        self.value = crate::UiText::clipped(&text);
        self
    }

    /// 🪞️ Sets the factor a number field shows its stored value at (`stored × factor`) and divides a typed one back by;
    /// set it before [`Self::number`].
    pub fn display_factor(mut self, factor: f64) -> Self {
        self.display_factor = Some(factor);
        self
    }

    /// 🥏️ Sets the hard range a typed number must keep, with the refusals naming its bounds.
    pub fn limits(mut self, limits: crate::UiNumberLimits) -> Self {
        self.limits = Some(limits);
        self
    }

    /// 🎯️ Sets the opaque semantic identity used to retain an in-progress draft.
    pub fn draft_target(mut self, draft_target: crate::UiText) -> Self {
        self.draft_target = Some(draft_target);
        self
    }

    /// 🧾️ Pins the input to the exact native document revision that authored it.
    pub fn publication_revision(mut self, publication_revision: crate::UiPublicationRevision) -> Self {
        self.publication_revision = Some(publication_revision);
        self
    }

    /// 📌️ Appends one detent of a number field. Refused when it would break the detent law against the bounds set
    /// so far (not strictly above the previous snap, outside `min..=max`, non-finite) or the list is full.
    pub fn try_snap(mut self, snap: f64) -> Result<Self, Self> {
        let previous = self.snaps.iter().last().copied();
        if !crate::snaps_are_valid(previous.into_iter().chain([snap]), self.min.unwrap_or(f64::NEG_INFINITY), self.max.unwrap_or(f64::INFINITY)) || self.snaps.try_push(snap).is_err() {
            return Err(self);
        }
        Ok(self)
    }
}

impl HasBase for InputBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<InputBuilder> for BuiltNode {
    fn from(builder: InputBuilder) -> Self {
        assemble(
            builder.base,
            crate::Component::Input(crate::InputProps { kind: builder.kind, value: builder.value, placeholder: builder.placeholder, commit: builder.commit, min: builder.min, max: builder.max, step: builder.step, accept: builder.accept, precision: builder.precision, snaps: builder.snaps, display_factor: builder.display_factor, limits: builder.limits, draft_target: builder.draft_target, publication_revision: builder.publication_revision }),
        )
    }
}
//#endregion ⌨️Input

//#region 🔀️Toggle
/// 🔀️ A binary switch — `Component::Toggle`. Build with [`toggle`].
pub struct ToggleBuilder {
    pub(super) base: NodeBase,
    pub(super) appearance: crate::ToggleAppearance,
    on: bool,
    pub(super) icon: crate::UiText,
    pub(super) text: Option<crate::Label>,
}

/// 🔀️ A toggle currently `on` or off.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn toggle(on: bool) -> ToggleBuilder {
    ToggleBuilder { base: NodeBase::leaf(), appearance: crate::ToggleAppearance::Button, on, icon: crate::UiText::default(), text: None }
}

impl ToggleBuilder {
    /// ☑️ Selects the shared binary-control presentation.
    pub fn appearance(mut self, appearance: crate::ToggleAppearance) -> Self {
        self.appearance = appearance;
        self
    }

    /// 🖼️ Sets the icon key.
    pub fn icon(mut self, icon: crate::UiText) -> Self {
        self.icon = icon;
        self
    }

    /// 📝️ Sets the visible text, and — unless [`HasBase::label`] already set one explicitly — the
    /// accessible name too, for the same "hard to omit" reason [`button`] auto-derives its own.
    pub fn text(mut self, text: crate::Label) -> Self {
        self.base.accessibility.label.get_or_insert_with(|| text.clone());
        self.text = Some(text);
        self
    }
}

impl HasBase for ToggleBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ToggleBuilder> for BuiltNode {
    fn from(builder: ToggleBuilder) -> Self {
        assemble(builder.base, crate::Component::Toggle(crate::ToggleProps { appearance: builder.appearance, on: builder.on, icon: builder.icon, text: builder.text }))
    }
}
//#endregion 🔀️Toggle

//#region 🔽️Select
/// 🔽️ A single-choice dropdown — `Component::Select`. Build with [`select`].
pub struct SelectBuilder {
    pub(super) base: NodeBase,
    pub(super) value: crate::UiText,
    pub(super) items: crate::UiFixedList<crate::SelectItem>,
    pub(super) placeholder: Option<crate::Label>,
    pub(super) appearance: crate::SelectAppearance,
}

/// 🔽️ A select currently holding `value`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn select(value: crate::UiText) -> SelectBuilder {
    SelectBuilder { base: NodeBase::leaf(), value, items: crate::UiFixedList::default(), placeholder: None, appearance: crate::SelectAppearance::Menu }
}

impl SelectBuilder {
    /// ➕️ Appends one option.
    #[expect(clippy::result_large_err, reason = "A full select returns its builder and exact bounded option without allocating on refusal.")]
    pub fn try_item(mut self, value: crate::UiText, label: crate::Label) -> Result<Self, (Self, crate::SelectItem)> {
        let item = crate::SelectItem { value, label };
        match self.items.try_push(item) {
            Ok(()) => Ok(self),
            Err(item) => Err((self, item)),
        }
    }

    /// 💬️ Sets placeholder copy shown while no option is selected.
    pub fn placeholder(mut self, placeholder: crate::Label) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// 🗳️ Shows the choice as a menu (the default) or as one segmented row of its options.
    pub fn appearance(mut self, appearance: crate::SelectAppearance) -> Self {
        self.appearance = appearance;
        self
    }
}

impl HasBase for SelectBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<SelectBuilder> for BuiltNode {
    fn from(builder: SelectBuilder) -> Self {
        assemble(builder.base, crate::Component::Select(crate::SelectProps { value: builder.value, items: builder.items, placeholder: builder.placeholder, appearance: builder.appearance }))
    }
}
//#endregion 🔽️Select
//#endregion 🔖️Builder
