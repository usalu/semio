//! 🎚️ Icon select, slider, stepper and progress builders: the icon select, slider, number stepper and progress builders. See [`super`].

use super::*;

//#region 🔖️Builder
//#region 🖼️IconSelect
/// 🖼️ A single-choice icon picker — `Component::IconSelect`. Build with [`icon_select`].
pub struct IconSelectBuilder {
    pub(super) base: NodeBase,
    pub(super) value: crate::UiText,
    pub(super) uniform: bool,
    pub(super) classifier_kind: crate::UiText,
}

/// 🖼️ An icon picker currently holding the icon id `value`, offering the icons its host classifies as `classifier_kind`
/// (`"icon"` is the whole catalogue — what a descriptor of format `iconId` asks for).
pub fn icon_select(value: crate::UiText, classifier_kind: crate::UiText) -> IconSelectBuilder {
    IconSelectBuilder { base: NodeBase::leaf(), value, uniform: true, classifier_kind }
}

impl IconSelectBuilder {
    /// 🌫️ Marks the picker as showing a mixed selection: no icon is the current one.
    pub fn mixed(mut self) -> Self {
        self.uniform = false;
        self
    }
}

impl HasBase for IconSelectBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<IconSelectBuilder> for BuiltNode {
    fn from(builder: IconSelectBuilder) -> Self {
        assemble(builder.base, crate::Component::IconSelect(crate::IconSelectProps { value: builder.value, uniform: builder.uniform, classifier_kind: builder.classifier_kind }))
    }
}
//#endregion 🖼️IconSelect

//#region 🎚️Slider
/// 🎚️ A continuous range control — `Component::Slider`. Build with [`slider`].
pub struct SliderBuilder {
    pub(super) base: NodeBase,
    pub(super) value: f64,
    pub(super) min: f64,
    pub(super) max: f64,
    pub(super) step: f64,
    pub(super) unit: Option<crate::UiText>,
    pub(super) snaps: crate::UiFixedList<f64>,
    pub(super) appearance: crate::SliderAppearance,
    scale: crate::UiNumberScale,
    pub(super) precision: Option<u16>,
    pub(super) display_unit: Option<crate::UiText>,
    pub(super) display_factor: Option<f64>,
    pub(super) limits: Option<crate::UiNumberLimits>,
}

/// 🎚️ A slider currently at `value`, defaulting to the `0.0..=1.0` normalized range with a `0.1`
/// step — the common case for an unlabeled proportion.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn slider(value: f64) -> SliderBuilder {
    SliderBuilder {
        base: NodeBase::leaf(),
        value,
        min: 0.0,
        max: 1.0,
        step: 0.1,
        unit: None,
        snaps: crate::UiFixedList::default(),
        appearance: crate::SliderAppearance::Track,
        scale: crate::UiNumberScale::Linear,
        precision: None,
        display_unit: None,
        display_factor: None,
        limits: None,
    }
}

impl SliderBuilder {
    /// ⬇️ Sets the minimum value.
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    /// ⬆️ Sets the maximum value.
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// 🔢️ Sets the step increment.
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// 📏️ Sets the displayed unit suffix.
    pub fn unit(mut self, unit: crate::UiText) -> Self {
        self.unit = Some(unit);
        self
    }

    /// 🧲️ Appends one detent. Refused when it would break the detent law against the bounds set so
    /// far (not strictly above the previous snap, outside `min..=max`, non-finite) or the list is full.
    pub fn try_snap(mut self, snap: f64) -> Result<Self, Self> {
        let previous = self.snaps.iter().last().copied();
        if !crate::snaps_are_valid(previous.into_iter().chain([snap]), self.min, self.max) || self.snaps.try_push(snap).is_err() {
            return Err(self);
        }
        Ok(self)
    }

    /// 🎡️ Draws the travel as a straight track or a rotary dial.
    pub fn appearance(mut self, appearance: crate::SliderAppearance) -> Self {
        self.appearance = appearance;
        self
    }

    /// 📉️ Maps the travel onto the track evenly or by ratio.
    pub fn scale(mut self, scale: crate::UiNumberScale) -> Self {
        self.scale = scale;
        self
    }

    /// 🔬️ Sets the fraction digits the readout, the spoken value and a typed value use, capped at
    /// [`crate::UI_NUMBER_PRECISION_MAX`].
    pub fn precision(mut self, precision: u16) -> Self {
        self.precision = Some(precision.min(crate::UI_NUMBER_PRECISION_MAX));
        self
    }

    /// 🪪️ Sets the unit the display value shows beside it (the stored `unit` otherwise).
    pub fn display_unit(mut self, unit: crate::UiText) -> Self {
        self.display_unit = Some(unit);
        self
    }

    /// 🫗️ Sets the factor the slider shows its stored value at (`stored × factor`) and divides a typed one back by.
    pub fn display_factor(mut self, factor: f64) -> Self {
        self.display_factor = Some(factor);
        self
    }

    /// 🏔️ Sets the hard range a typed value must keep, making the travel a soft range inside it.
    pub fn limits(mut self, limits: crate::UiNumberLimits) -> Self {
        self.limits = Some(limits);
        self
    }
}

impl HasBase for SliderBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<SliderBuilder> for BuiltNode {
    fn from(builder: SliderBuilder) -> Self {
        assemble(
            builder.base,
            crate::Component::Slider(crate::SliderProps {
                value: builder.value,
                min: builder.min,
                max: builder.max,
                step: builder.step,
                unit: builder.unit,
                snaps: builder.snaps,
                appearance: builder.appearance,
                scale: builder.scale,
                precision: builder.precision,
                display_unit: builder.display_unit,
                display_factor: builder.display_factor,
                limits: builder.limits,
            }),
        )
    }
}
//#endregion 🎚️Slider

//#region 🔢️NumberStepper
/// 🪜️ A numeric spin button with decrement/increment steps — `Component::NumberStepper`. Build with
/// [`number_stepper`]; bind `Trigger::Change` for the absolute value and `Trigger::Delta` for the
/// relative nudge a mixed selection needs.
pub struct NumberStepperBuilder {
    pub(super) base: NodeBase,
    pub(super) value: f64,
    pub(super) step: f64,
    pub(super) uniform: bool,
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    pub(super) precision: Option<u16>,
    pub(super) snaps: crate::UiFixedList<f64>,
    pub(super) unit: Option<crate::UiText>,
    pub(super) display_unit: Option<crate::UiText>,
    pub(super) display_factor: Option<f64>,
    pub(super) limits: Option<crate::UiNumberLimits>,
}

/// 🧮️ A stepper currently at `value`, stepping by `1.0`, unbounded, showing one uniform value.
pub fn number_stepper(value: f64) -> NumberStepperBuilder {
    NumberStepperBuilder { base: NodeBase::leaf(), value, step: 1.0, uniform: true, min: None, max: None, precision: None, snaps: crate::UiFixedList::default(), unit: None, display_unit: None, display_factor: None, limits: None }
}

impl NumberStepperBuilder {
    /// 👣️ Sets the step increment.
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// 🔻️ Sets the minimum value.
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// 🔺️ Sets the maximum value.
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// 🌗️ Marks the value mixed: a multi-selection whose members disagree shows no single value.
    pub fn mixed(mut self) -> Self {
        self.uniform = false;
        self
    }

    /// 🔍️ Sets the fraction digits it shows and commits, capped at [`crate::UI_NUMBER_PRECISION_MAX`].
    pub fn precision(mut self, precision: u16) -> Self {
        self.precision = Some(precision.min(crate::UI_NUMBER_PRECISION_MAX));
        self
    }

    /// 📍️ Appends one detent its page keys stop on. Refused when it would break the detent law against the bounds set
    /// so far or the list is full.
    pub fn try_snap(mut self, snap: f64) -> Result<Self, Self> {
        let previous = self.snaps.iter().last().copied();
        if !crate::snaps_are_valid(previous.into_iter().chain([snap]), self.min.unwrap_or(f64::NEG_INFINITY), self.max.unwrap_or(f64::INFINITY)) || self.snaps.try_push(snap).is_err() {
            return Err(self);
        }
        Ok(self)
    }

    /// ⚗️ Sets the unit of the stored value (shown beside it without a display unit).
    pub fn unit(mut self, unit: crate::UiText) -> Self {
        self.unit = Some(unit);
        self
    }

    /// 🔖️ Sets the unit the display value shows beside it.
    pub fn display_unit(mut self, unit: crate::UiText) -> Self {
        self.display_unit = Some(unit);
        self
    }

    /// ✖️ Sets the factor it shows its stored value at (`stored × factor`) and divides a typed one back by.
    pub fn display_factor(mut self, factor: f64) -> Self {
        self.display_factor = Some(factor);
        self
    }

    /// ⛔️ Sets the hard range a typed value must keep, with the refusals naming its bounds.
    pub fn limits(mut self, limits: crate::UiNumberLimits) -> Self {
        self.limits = Some(limits);
        self
    }
}

impl HasBase for NumberStepperBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<NumberStepperBuilder> for BuiltNode {
    fn from(builder: NumberStepperBuilder) -> Self {
        assemble(
            builder.base,
            crate::Component::NumberStepper(crate::NumberStepperProps {
                value: builder.value,
                step: builder.step,
                uniform: builder.uniform,
                min: builder.min,
                max: builder.max,
                precision: builder.precision,
                snaps: builder.snaps,
                unit: builder.unit,
                display_unit: builder.display_unit,
                display_factor: builder.display_factor,
                limits: builder.limits,
            }),
        )
    }
}
//#endregion 🔢️NumberStepper

//#region 📶️Progress
/// 📶️ A read-only progress bar — `Component::Progress`. Build with [`progress`].
pub struct ProgressBuilder {
    pub(super) base: NodeBase,
    completed: f64,
    total: Option<f64>,
    value_text: crate::Label,
}

/// 📶️ An indeterminate bar at `completed` units announcing `value_text`; call [`ProgressBuilder::total`]
/// once the amount of work is known.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn progress(completed: f64, value_text: crate::Label) -> ProgressBuilder {
    ProgressBuilder { base: NodeBase::leaf(), completed, total: None, value_text }
}

impl ProgressBuilder {
    /// 🏁️ Makes the bar determinate on `0..=total`.
    pub fn total(mut self, total: f64) -> Self {
        self.total = Some(total);
        self
    }
}

impl HasBase for ProgressBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ProgressBuilder> for BuiltNode {
    fn from(builder: ProgressBuilder) -> Self {
        assemble(builder.base, crate::Component::Progress(crate::ProgressProps { completed: builder.completed, total: builder.total, value_text: builder.value_text }))
    }
}
//#endregion 📶️Progress
//#endregion 🔖️Builder
