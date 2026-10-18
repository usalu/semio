//! 🧭️ Vector, reference list and colour input builders: the vector input, reference list and colour input builders with their labels. See [`super`].

use super::*;

//#region 🔖️Builder
//#region 🧭️VectorInput
/// 🏹️ A vector-valued input recipe: one labelled `ContainerRole::Group` laid out as a wrapping row of
/// `ContainerRole::Field` axes, each holding one `InputKind::Number` field keyed by its axis, all
/// sharing one unit, step, bounds, precision and commit convention. Costs `1 + 2 × axes` records.
/// Build with [`vector_input`], add axes with [`VectorInputBuilder::try_axis`].
pub struct VectorInputBuilder {
    pub(super) group: ContainerBuilder,
    pub(super) unit: Option<crate::UiText>,
    pub(super) step: Option<f64>,
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    pub(super) precision: Option<u16>,
    pub(super) commit: Option<crate::UiText>,
    pub(super) snaps: Vec<f64>,
    pub(super) display_factor: Option<f64>,
    pub(super) limits: Option<crate::UiNumberLimits>,
}

/// 🆕️ An empty vector input reading `label`.
pub fn vector_input(label: crate::Label) -> VectorInputBuilder {
    let group = ContainerBuilder { base: NodeBase::stack(crate::Axis::Horizontal), role: crate::ContainerRole::Group, label, description: None, required: None, error: None, default_open: None, drop_overlay: None }.wrap(true).gap(crate::SpaceToken::Sm);
    VectorInputBuilder { group, unit: None, step: None, min: None, max: None, precision: None, commit: None, snaps: Vec::new(), display_factor: None, limits: None }
}

impl VectorInputBuilder {
    /// 🪧️ Sets the unit every axis shows as its field description.
    pub fn unit(mut self, unit: crate::UiText) -> Self {
        self.unit = Some(unit);
        self
    }

    /// 🦶️ Sets the step every axis shares.
    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    /// ⏬️ Sets the minimum every axis shares.
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// ⏫️ Sets the maximum every axis shares.
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// 🔎️ Sets the fraction digits every axis shows and commits.
    pub fn precision(mut self, precision: u16) -> Self {
        self.precision = Some(precision.min(crate::UI_NUMBER_PRECISION_MAX));
        self
    }

    /// 🤝️ Sets the commit convention every axis shares (e.g. `"blur"`).
    pub fn commit(mut self, commit: crate::UiText) -> Self {
        self.commit = Some(commit);
        self
    }

    /// 🔄️ Sets the display factor every axis shows its stored component at.
    pub fn display_factor(mut self, factor: f64) -> Self {
        self.display_factor = Some(factor);
        self
    }

    /// 🛑️ Sets the hard range every axis's typed component must keep.
    pub fn limits(mut self, limits: crate::UiNumberLimits) -> Self {
        self.limits = Some(limits);
        self
    }

    /// 🧲️ Appends one detent every axis shares — the vector's component snaps. Refused when it would break the detent
    /// law against the bounds set so far or the list is full.
    pub fn try_snap(mut self, snap: f64) -> Result<Self, Self> {
        let previous = self.snaps.last().copied();
        if self.snaps.len() >= crate::UI_FIXED_LIST_ITEMS || !crate::snaps_are_valid(previous.into_iter().chain([snap]), self.min.unwrap_or(f64::NEG_INFINITY), self.max.unwrap_or(f64::INFINITY)) {
            return Err(self);
        }
        self.snaps.push(snap);
        Ok(self)
    }

    /// 🪢️ Appends one axis keyed `key`, reading `label`, currently at `value`; `bind` attaches the
    /// axis field's own bindings (its `Trigger::Change`/`Trigger::Commit` action and args). Refused
    /// when the key or label does not fit, `bind` refuses, or the group is full.
    pub fn try_axis(mut self, key: &str, label: crate::Label, value: f64, bind: impl FnOnce(InputBuilder) -> Result<InputBuilder, InputBuilder>) -> Result<Self, Self> {
        let mut field_input = input(crate::InputKind::Number);
        field_input.step = self.step;
        field_input.min = self.min;
        field_input.max = self.max;
        field_input.precision = self.precision;
        field_input.commit = self.commit.clone();
        field_input.display_factor = self.display_factor;
        field_input.limits = self.limits.clone();
        for snap in &self.snaps {
            let Ok(snapped) = field_input.try_snap(*snap) else { return Err(self) };
            field_input = snapped;
        }
        let Ok(field_input) = field_input.number(value).try_id(key) else { return Err(self) };
        let Ok(field_input) = field_input.try_label(label.0.as_str()) else { return Err(self) };
        let Ok(field_input) = bind(field_input) else { return Err(self) };
        let mut axis = ContainerBuilder::new(crate::ContainerRole::Field, label);
        if let Some(unit) = &self.unit {
            axis = axis.description(unit.clone());
        }
        let Ok(axis) = axis.try_id(format!("{key}.axis")) else { return Err(self) };
        let Ok(axis) = axis.try_child(field_input) else { return Err(self) };
        match self.group.try_child(axis) {
            Ok(group) => {
                self.group = group;
                Ok(self)
            }
            Err((group, _)) => {
                self.group = group;
                Err(self)
            }
        }
    }
}

impl HasBase for VectorInputBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        self.group.base_mut()
    }
}

impl From<VectorInputBuilder> for BuiltNode {
    fn from(builder: VectorInputBuilder) -> Self {
        builder.group.into()
    }
}
//#endregion 🧭️VectorInput

//#region 🧷️ReferenceList
/// 🔤️ The copy a [`reference_list`] shows, already localized by the caller — this crate carries no
/// locale, so a plugin resolves both from its own `LocalizedLabel`s.
pub struct ReferenceListLabels {
    /// 🖱️ The button that replaces or extends the references with the current selection.
    pub use_selection: crate::Label,
    /// 🪹️ The line shown while nothing is referenced.
    pub empty: crate::Label,
}

/// 🧷️ A selection-picker recipe: a labelled `ContainerRole::Group` holding a wrapping toolbar of
/// reference chips (each a Button reading the referenced item whose activation removes it), the empty
/// line when there is none, and an actions row with the "use current selection" button and an optional
/// candidate list. Costs `chips + 4` records, one more with a candidate list. Build with
/// [`reference_list`].
pub struct ReferenceListBuilder {
    pub(super) group: ContainerBuilder,
    pub(super) chips: ContainerBuilder,
    pub(super) actions: StackBuilder,
    pub(super) empty: crate::Label,
}

/// 🖇️ An empty reference list reading `label`.
pub fn reference_list(label: crate::Label, labels: ReferenceListLabels) -> ReferenceListBuilder {
    let chips = ContainerBuilder { base: NodeBase::stack(crate::Axis::Horizontal), role: crate::ContainerRole::Toolbar, label: label.clone(), description: None, required: None, error: None, default_open: None, drop_overlay: None }.wrap(true).gap(crate::SpaceToken::Xs);
    let group = ContainerBuilder::new(crate::ContainerRole::Group, label).gap(crate::SpaceToken::Xs);
    let actions = row().gap(crate::SpaceToken::Xs).wrap(true);
    let use_selection = button(labels.use_selection).icon(crate::UiText::clipped("crosshair"));
    let actions = match use_selection.try_id("useSelection") {
        Ok(use_selection) => actions.try_child(use_selection).unwrap_or_else(|(actions, _)| actions),
        Err(_) => actions,
    };
    ReferenceListBuilder { group, chips, actions, empty: labels.empty }
}

impl ReferenceListBuilder {
    /// 🎫️ Appends one chip keyed `key` reading `label`; activating it fires `remove` (with `args`),
    /// and a reader announces it as `remove_label` (for example "Remove Piece 3").
    pub fn try_chip(mut self, key: &str, label: crate::Label, remove_label: &str, remove: crate::ActionId, args: Option<crate::UiValue>) -> Result<Self, Self> {
        let chip = button(label).icon(crate::UiText::clipped("x")).variant(crate::Variant::Outline);
        let Ok(chip) = chip.try_id(key) else { return Err(self) };
        let Ok(chip) = chip.try_label(remove_label) else { return Err(self) };
        let bound = match args {
            Some(args) => chip.try_on_with(crate::Trigger::Activate, remove, args),
            None => chip.try_on(crate::Trigger::Activate, remove),
        };
        let Ok(chip) = bound else { return Err(self) };
        match self.chips.try_child(chip) {
            Ok(chips) => {
                self.chips = chips;
                Ok(self)
            }
            Err((chips, _)) => {
                self.chips = chips;
                Err(self)
            }
        }
    }

    /// 👉️ Binds the "use current selection" button to `action` (with `args`); `enabled: false` paints
    /// it disabled, the state while the selection holds nothing this list can reference.
    pub fn try_use_selection(mut self, action: crate::ActionId, args: Option<crate::UiValue>, enabled: bool) -> Result<Self, Self> {
        let Some(button) = self.actions.base.children.get_mut(0) else { return Err(self) };
        let binding = crate::ActionBinding { trigger: crate::Trigger::Activate, action, args, capability: None };
        if button.bindings.try_push(binding).is_err() {
            return Err(self);
        }
        button.disabled = !enabled;
        Ok(self)
    }

    /// 🗳️ Adds the optional candidate list: a select reading `placeholder` whose `Trigger::Change`
    /// fires `add` with the picked candidate's value.
    pub fn try_candidates(mut self, placeholder: crate::Label, add: crate::ActionId, candidates: impl IntoIterator<Item = (crate::UiText, crate::Label)>) -> Result<Self, Self> {
        let mut list = select(crate::UiText::default()).placeholder(placeholder.clone());
        for (value, label) in candidates {
            list = match list.try_item(value, label) {
                Ok(list) => list,
                Err(_) => return Err(self),
            };
        }
        let Ok(list) = list.try_id("candidates") else { return Err(self) };
        let Ok(list) = list.try_label(placeholder.0.as_str()) else { return Err(self) };
        let Ok(list) = list.try_on(crate::Trigger::Change, add) else { return Err(self) };
        match self.actions.try_child(list) {
            Ok(actions) => {
                self.actions = actions;
                Ok(self)
            }
            Err((actions, _)) => {
                self.actions = actions;
                Err(self)
            }
        }
    }
}

impl HasBase for ReferenceListBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        self.group.base_mut()
    }
}

impl From<ReferenceListBuilder> for BuiltNode {
    fn from(builder: ReferenceListBuilder) -> Self {
        let references: BuiltNode = if builder.chips.base.children.is_empty() {
            let mut empty: BuiltNode = text(builder.empty).tone(crate::Tone::Secondary).into();
            empty.key = crate::UiText::clipped("empty");
            empty
        } else {
            let mut chips: BuiltNode = builder.chips.into();
            chips.key = crate::UiText::clipped("chips");
            chips
        };
        let mut actions: BuiltNode = builder.actions.into();
        actions.key = crate::UiText::clipped("actions");
        let group = builder.group.try_child(references).and_then(|group| group.try_child(actions));
        match group {
            Ok(group) => group.into(),
            Err((group, _)) => group.into(),
        }
    }
}
//#endregion 🧷️ReferenceList

//#region 🎨️ColorInput
/// 🔠️ The copy a [`color_input`] shows beside its swatch, already localized by the caller.
pub struct ColorInputLabels {
    /// #️⃣ The hex field's label.
    pub hex: crate::Label,
    /// 🌫️ The alpha slider's label.
    pub alpha: crate::Label,
}

/// 🎨️ A colour-picker recipe over an sRGB colour with components in `0..=1`: a labelled, wrapping
/// `ContainerRole::Group` holding the swatch (`InputKind::Color` keyed `swatch`, `#rrggbb`), the hex field
/// (`InputKind::Text` keyed `hex`, `#rrggbb` — `#rrggbbaa` with alpha — committed on blur) and, with alpha, the alpha
/// slider (keyed `alpha`, `0..=1` in steps of `0.01`). The swatch and the hex field commit a hex string
/// ([`crate::parse_ui_color_hex`] reads it back), the slider the straight alpha. Costs 3 records, 4 with alpha. Build
/// with [`color_input`], fill with [`ColorInputBuilder::try_color`].
pub struct ColorInputBuilder {
    pub(super) group: ContainerBuilder,
    pub(super) labels: ColorInputLabels,
}

/// 🫙️ An empty colour input reading `label`.
pub fn color_input(label: crate::Label, labels: ColorInputLabels) -> ColorInputBuilder {
    let group = ContainerBuilder { base: NodeBase::stack(crate::Axis::Horizontal), role: crate::ContainerRole::Group, label, description: None, required: None, error: None, default_open: None, drop_overlay: None }.wrap(true).gap(crate::SpaceToken::Sm);
    ColorInputBuilder { group, labels }
}

impl ColorInputBuilder {
    /// 🖌️ Adds the swatch and the hex field showing `rgba` ([`crate::ui_color_hex`]) and, with `alpha`, the alpha
    /// slider; `bind_hex` attaches the swatch's and the hex field's bindings, `bind_alpha` the slider's. A missing
    /// alpha reads 1, a non-finite one 0. Refused when a label does not fit, a binding refuses, or the group is full.
    pub fn try_color(mut self, rgba: &[f64], alpha: bool, bind_hex: impl Fn(InputBuilder) -> Result<InputBuilder, InputBuilder>, bind_alpha: impl FnOnce(SliderBuilder) -> Result<SliderBuilder, SliderBuilder>) -> Result<Self, Self> {
        let group_label = self.group.label.0.as_str().to_string();
        let swatch = input(crate::InputKind::Color).value(crate::UiText::clipped(&crate::ui_color_hex(rgba, false)));
        let Ok(swatch) = swatch.try_id("swatch").and_then(|swatch| swatch.try_label(&group_label)).and_then(&bind_hex) else { return Err(self) };
        let hex = input(crate::InputKind::Text).value(crate::UiText::clipped(&crate::ui_color_hex(rgba, alpha))).commit(crate::UiText::clipped("blur"));
        let Ok(hex) = hex.try_id("hex").and_then(|hex| hex.try_label(self.labels.hex.0.as_str())).and_then(&bind_hex) else { return Err(self) };
        let mut parts: Vec<BuiltNode> = vec![swatch.into(), hex.into()];
        if alpha {
            let value = rgba.get(3).copied().map_or(1.0, |alpha| if alpha.is_finite() { alpha.clamp(0.0, 1.0) } else { 0.0 });
            let slider = slider(value).min(0.0).max(1.0).step(0.01);
            let Ok(slider) = slider.try_id("alpha").and_then(|slider| slider.try_label(self.labels.alpha.0.as_str())).and_then(bind_alpha) else { return Err(self) };
            parts.push(slider.into());
        }
        for part in parts {
            match self.group.try_child(part) {
                Ok(group) => self.group = group,
                Err((group, _)) => {
                    self.group = group;
                    return Err(self);
                }
            }
        }
        Ok(self)
    }
}

impl HasBase for ColorInputBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        self.group.base_mut()
    }
}

impl From<ColorInputBuilder> for BuiltNode {
    fn from(builder: ColorInputBuilder) -> Self {
        builder.group.into()
    }
}
//#endregion 🎨️ColorInput
//#endregion 🔖️Builder
