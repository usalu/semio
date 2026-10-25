//! 🔢️ Number facets of action arguments: the declared bounds, step and unit, the unit symbol and the bound refusal. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionArgFacets
/// 💠️ The UI-contract number facets of one input — the single mapping every renderer of an [`ActionArgDef`] shares (the
/// time-travel editor, both shells' staged dialog fields): its key range (a slider's or dial's travel; a stepper's, number
/// field's or vector axis' hard bounds, an excluded bound being none since no key lands on it), the step, the look (`Dial`
/// for a dial presentation) and axis, the stored and shown unit as symbols ([`action_arg_unit_symbol`]), the display factor
/// and precision, the declared detents the detent law and the limits admit, and the hard limits of the schema with their
/// refusals naming the bound in display units ("Must be greater than 0", "Darf höchstens 180 ° sein"). TypeScript twin:
/// `actionArgNumberFacets`; fixture `🧫️fixtures/🧫️number-facets`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionArgNumberFacets {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub appearance: semio_framework_ui_contract::SliderAppearance,
    pub scale: semio_framework_ui_contract::UiNumberScale,
    pub unit: Option<String>,
    pub display_unit: Option<String>,
    pub display_factor: Option<f64>,
    pub precision: Option<u16>,
    pub snaps: Vec<f64>,
    pub limits: semio_framework_ui_contract::UiNumberLimits,
}

impl ActionArgNumberFacets {
    /// 🏷️ The unit a value shows with — the display unit, else the stored one.
    pub fn shown_unit(&self) -> Option<&str> {
        self.display_unit.as_deref().or(self.unit.as_deref())
    }
}

/// 🔣️ The symbol a descriptor's unit name shows as (`deg` reads `°`, `percent` reads `%`); any other unit as written.
pub fn action_arg_unit_symbol(unit: &str) -> String {
    match unit {
        "deg" | "degree" | "degrees" => "°".to_string(),
        "percent" => "%".to_string(),
        other => other.to_string(),
    }
}

/// 🚧️ The refusal of a typed value crossing a hard bound, `{bound}` naming it — framework copy, EN and DE, no default.
fn action_arg_bound_refusal(below: bool, exclusive: bool, locale: Locale) -> &'static str {
    match (locale, below, exclusive) {
        (Locale::En, true, false) => "Must be at least {bound}",
        (Locale::En, true, true) => "Must be greater than {bound}",
        (Locale::En, false, false) => "Must be at most {bound}",
        (Locale::En, false, true) => "Must be less than {bound}",
        (Locale::De, true, false) => "Muss mindestens {bound} sein",
        (Locale::De, true, true) => "Muss größer als {bound} sein",
        (Locale::De, false, false) => "Darf höchstens {bound} sein",
        (Locale::De, false, true) => "Muss kleiner als {bound} sein",
    }
}

impl ActionArgDef {
    /// 🎛️ The [`ActionArgNumberFacets`] of the control [`Self::control`] derives, its refusals in `locale` — `None` unless
    /// the input is a number field, stepper, slider, dial or vector.
    pub fn number_facets(&self, locale: Locale) -> Option<ActionArgNumberFacets> {
        use semio_framework_ui_contract::{SliderAppearance, UiNumberBound, UiNumberLimits, UiNumberScale};
        let (hard_min, hard_max, min_exclusive, max_exclusive) = match &self.schema {
            ArgSchema::Number { min, max, min_exclusive, max_exclusive, .. } => (*min, *max, *min_exclusive, *max_exclusive),
            ArgSchema::Vector { min, max, .. } => (*min, *max, false, false),
            _ => return None,
        };
        let number = |min: Option<f64>, max: Option<f64>, step: Option<f64>, unit: Option<String>, precision: Option<u32>, display_unit: Option<String>, display_factor: Option<f64>, snaps: Vec<f64>| ActionArgNumberFacets {
            min,
            max,
            step: step.filter(|step| step.is_finite() && *step > 0.0),
            unit: unit.as_deref().map(action_arg_unit_symbol),
            display_unit: display_unit.as_deref().map(action_arg_unit_symbol),
            display_factor,
            precision: precision.map(|precision| u16::try_from(precision).unwrap_or(u16::MAX).min(semio_framework_ui_contract::UI_NUMBER_PRECISION_MAX)),
            snaps,
            ..ActionArgNumberFacets::default()
        };
        let key = |bound: Option<f64>, excluded: bool| bound.filter(|_| !excluded);
        let mut facets = match self.control() {
            ActionArgControl::Slider { min, max, step, unit, precision, display_unit, display_factor, snaps, scale, .. } => ActionArgNumberFacets { scale: if scale == Some(NumberScale::Log) { UiNumberScale::Log } else { UiNumberScale::Linear }, ..number(Some(min), Some(max), step, unit, precision, display_unit, display_factor, snaps) },
            ActionArgControl::Dial { min, max, step, unit, precision, display_unit, display_factor, snaps, .. } => ActionArgNumberFacets { appearance: SliderAppearance::Dial, ..number(Some(min), Some(max), step, unit, precision, display_unit, display_factor, snaps) },
            ActionArgControl::Stepper { min, max, step, unit, precision, display_unit, display_factor, snaps, .. } => number(key(min, min_exclusive), key(max, max_exclusive), step, unit, precision, display_unit, display_factor, snaps),
            ActionArgControl::Number { min, max, step, unit, precision, display_unit, display_factor } => number(key(min, min_exclusive), key(max, max_exclusive), step, unit, precision, display_unit, display_factor, Vec::new()),
            ActionArgControl::Vector { min, max, step, unit, precision, display_unit, display_factor, snaps, .. } => number(min, max, step, unit, precision, display_unit, display_factor, snaps),
            _ => return None,
        };
        let shown_unit = facets.shown_unit().map(ToOwned::to_owned);
        let bound = |value: f64, exclusive: bool, below: bool| {
            let text = semio_framework_ui_contract::ui_number_display_text(value, facets.display_factor, None);
            let text = shown_unit.as_ref().map_or_else(|| text.clone(), |unit| format!("{text} {unit}"));
            let refusal = action_arg_bound_refusal(below, exclusive, locale).replace("{bound}", &text);
            UiNumberBound { value, exclusive, refusal: Some(semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::clipped(&refusal))) }
        };
        let limits = UiNumberLimits { min: hard_min.map(|value| bound(value, min_exclusive, true)), max: hard_max.map(|value| bound(value, max_exclusive, false)) };
        let (low, high) = (facets.min.unwrap_or(f64::NEG_INFINITY), facets.max.unwrap_or(f64::INFINITY));
        let mut previous = f64::NEG_INFINITY;
        facets.snaps.retain(|&snap| {
            let kept = snap.is_finite() && snap > previous && (low..=high).contains(&snap) && limits.crossed(snap).is_none();
            if kept {
                previous = snap;
            }
            kept
        });
        facets.limits = limits;
        Some(facets)
    }
}
//#endregion 🔖️ActionArgFacets
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
