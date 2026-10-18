//! 🔢️ Number scale and slider props: the number scale, slider appearance, number bound and limits, the slider and number stepper props and the range and snap validity checks. See [`super`].

use super::*;

//#region 🎨️Props
/// 📈️ How a slider's travel maps onto its track: evenly (`linear`), or by ratio (`log`, a strictly positive
/// travel) — see [`slider_axis_position`]/[`slider_axis_value`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum UiNumberScale {
    #[default]
    Linear,
    Log,
}

impl UiNumberScale {
    pub fn is_linear(&self) -> bool {
        matches!(self, Self::Linear)
    }
}

/// 🎡️ How a slider draws its travel: a straight `track`, or a rotary `dial` sweeping one full counter-clockwise
/// turn with the travel's centre at three o'clock — so a `-180°..180°` angle points where it turns
/// ([`dial_angle`]/[`dial_position`]). Both take the same keys, pointer law and accessibility.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum SliderAppearance {
    #[default]
    Track,
    Dial,
}

impl SliderAppearance {
    pub fn is_track(&self) -> bool {
        matches!(self, Self::Track)
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_inclusive(exclusive: &bool) -> bool {
    !*exclusive
}

/// 🧱️ One hard bound of a numeric control's value: the limit, whether the limit itself is excluded, and the
/// refusal a renderer shows when a typed value crosses it — already localized by the producer and naming the
/// bound in display units (the contract carries no locale). A crossing value is never dispatched: the draft is
/// kept and the refusal shown.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct UiNumberBound {
    pub value: f64,
    #[serde(default, skip_serializing_if = "is_inclusive")]
    #[value(default, skip_serializing_if = "is_inclusive")]
    pub exclusive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<Label>,
}

/// 🛤️ The hard range a numeric control's value must keep, each side open when absent. A slider's `min`/`max`
/// are then only its travel (a soft range): a typed value may leave the travel while the limits admit it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct UiNumberLimits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<UiNumberBound>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<UiNumberBound>,
}

impl UiNumberBound {
    /// 🙅️ Whether `value` lies beyond this bound on the side `below` names (an excluded limit refuses itself).
    pub fn refuses(&self, value: f64, below: bool) -> bool {
        match (below, self.exclusive) {
            (true, false) => value < self.value,
            (true, true) => value <= self.value,
            (false, false) => value > self.value,
            (false, true) => value >= self.value,
        }
    }
}

impl UiNumberLimits {
    /// 🔎️ The bound `value` crosses, the lower one first; `None` when both admit it.
    pub fn crossed(&self, value: f64) -> Option<&UiNumberBound> {
        self.min.as_ref().filter(|bound| bound.refuses(value, true)).or_else(|| self.max.as_ref().filter(|bound| bound.refuses(value, false)))
    }
}

/// 🪤️ The hard bound a typed `value` crosses: `limits` when the control declares them, else its inclusive
/// `min`/`max` (unlabelled, so the renderer's refusal names no bound). A non-finite value crosses nothing — a
/// renderer refuses it as unreadable before it gets here.
pub fn ui_number_crossed_bound(value: f64, min: Option<f64>, max: Option<f64>, limits: Option<&UiNumberLimits>) -> Option<UiNumberBound> {
    if !value.is_finite() {
        return None;
    }
    match limits {
        Some(limits) => limits.crossed(value).cloned(),
        None => UiNumberLimits { min: min.map(|value| UiNumberBound { value, exclusive: false, refusal: None }), max: max.map(|value| UiNumberBound { value, exclusive: false, refusal: None }) }.crossed(value).cloned(),
    }
}

/// 🎚️ Props for `Component::Slider`. `on_change` moved to the record's `bindings`. `min`/`max` are the travel;
/// `snaps` are its detents: strictly ascending, finite, inside `min..=max`, at most [`crate::UI_FIXED_LIST_ITEMS`]
/// of them — every renderer paints one tick per snap at its [`slider_axis_position`] and resolves a pointer value
/// through [`slider_pointer_value`] and a key through [`slider_key_value`]. Values are stored values; the readout,
/// the spoken value and a typed value are display values (`stored × display_factor`, at `precision`, beside
/// `display_unit` else `unit`, read back by [`ui_number_typed_value`]). A typed value is refused by `limits` (the
/// travel itself when absent).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct SliderProps {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub snaps: crate::UiFixedList<f64>,
    #[serde(default, skip_serializing_if = "SliderAppearance::is_track")]
    #[value(default, skip_serializing_if = "SliderAppearance::is_track")]
    pub appearance: SliderAppearance,
    #[serde(default, skip_serializing_if = "UiNumberScale::is_linear")]
    #[value(default, skip_serializing_if = "UiNumberScale::is_linear")]
    pub scale: UiNumberScale,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
}

impl SliderProps {
    /// 🧯️ Whether `snaps` satisfies the detent law: finite, strictly ascending, inside the bounds.
    pub fn snaps_are_valid(&self) -> bool {
        crate::snaps_are_valid(self.snaps.iter().copied(), self.min, self.max)
    }

    /// 🥽️ Whether the travel, scale, display factor and limits agree ([`crate::number_range_is_valid`]).
    pub fn number_range_is_valid(&self) -> bool {
        crate::number_range_is_valid(Some(self.min), Some(self.max), self.scale, self.display_factor, self.limits.as_ref(), self.snaps.iter().copied())
    }
}

/// 🔢️ Props for `Component::NumberStepper`. `on_absolute`/`on_delta` both moved to the record's
/// `bindings`, distinguished by `Trigger`. `precision` is the fraction digits it shows and commits (in display
/// units); `snaps` are its detents under the slider's detent law against `min`/`max` (the page keys stop on them,
/// [`ui_number_key_value`]); the display facets and `limits` read as on [`SliderProps`], `limits` refusing a typed
/// value (`min`/`max` themselves when absent).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct NumberStepperProps {
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub snaps: crate::UiFixedList<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
}

impl NumberStepperProps {
    /// 🪝️ Whether `snaps` satisfies the detent law against the optional bounds.
    pub fn snaps_are_valid(&self) -> bool {
        crate::snaps_are_valid(self.snaps.iter().copied(), self.min.unwrap_or(f64::NEG_INFINITY), self.max.unwrap_or(f64::INFINITY))
    }

    /// 🪢️ Whether the bounds, display factor and limits agree ([`crate::number_range_is_valid`]).
    pub fn number_range_is_valid(&self) -> bool {
        crate::number_range_is_valid(self.min, self.max, UiNumberScale::Linear, self.display_factor, self.limits.as_ref(), self.snaps.iter().copied())
    }
}
//#endregion 🎨️Props

/// 🎚️ The number-range law every renderer shares (TS twin `numberRangeIsValid`): a display factor is positive; a
/// `log` scale has a strictly positive travel; declared limits keep their lower bound below their upper one
/// (equal only when both include it) and admit the key range `min..=max` and every detent — so a slider's travel
/// is a soft range inside its hard one. The admission gate behind [`UiContractViolation::InvalidNumberRange`].
pub fn number_range_is_valid(min: Option<f64>, max: Option<f64>, scale: crate::UiNumberScale, display_factor: Option<f64>, limits: Option<&crate::UiNumberLimits>, snaps: impl IntoIterator<Item = f64>) -> bool {
    let factor = display_factor.is_none_or(|factor| factor > 0.0);
    let axis = scale == crate::UiNumberScale::Linear || min.zip(max).is_some_and(|(min, max)| min > 0.0 && max > 0.0);
    let Some(limits) = limits else { return factor && axis };
    let ordered = limits.min.as_ref().zip(limits.max.as_ref()).is_none_or(|(low, high)| low.value < high.value || (low.value == high.value && !low.exclusive && !high.exclusive));
    let admitted = |value: f64| limits.crossed(value).is_none();
    factor && axis && ordered && min.is_none_or(admitted) && max.is_none_or(admitted) && snaps.into_iter().all(admitted)
}

/// 🧲️ The detent law every renderer shares: finite, strictly ascending, inside `min..=max` — the admission
/// gate behind [`UiContractViolation::InvalidSnaps`] (TS twin `snapsAreValid`, `🛡️limits/🟦️.ts`).
pub fn snaps_are_valid(snaps: impl IntoIterator<Item = f64>, min: f64, max: f64) -> bool {
    let mut previous = f64::NEG_INFINITY;
    snaps.into_iter().all(|snap| {
        let valid = snap.is_finite() && snap > previous && snap >= min && snap <= max;
        previous = snap;
        valid
    })
}
