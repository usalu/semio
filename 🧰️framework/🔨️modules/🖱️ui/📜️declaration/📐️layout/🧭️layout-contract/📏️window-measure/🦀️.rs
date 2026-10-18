//! 📏️ Window measure: the measure select item, the `WindowMeasure` union, its accessors and the measure partition. See [`super`].

use super::*;

// #region layout
//#region 🔖️WindowMeasure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MeasureSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum WindowMeasure {
    Select {
        id: String,
        label: Option<String>,
        value: String,
        items: Vec<MeasureSelectItem>,
        on_change: ActionDescriptor,
    },
    Slider {
        id: String,
        label: Option<String>,
        value: f64,
        min: f64,
        max: f64,
        step: Option<f64>,
        /// 🎚️ Absolute value on the fixed `[min, max]` range that is already preloaded/ready.
        /// Renderers keep `max` stable and draw a highlight from the knob to this extent.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        ready: Option<f64>,
        /// 🌀️ When true, the measure tree leaf shows a loading ring while preload continues.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        loading: Option<bool>,
        /// 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        waiting: Option<bool>,
        /// 🚫️ When true, the slider is inert — used when a parent weight is zero so joint percentages cannot change anything.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        on_change: ActionDescriptor,
    },
    /// 🔢️ Unbounded numeric entry — the measure-overlay sibling of `UiNumberStepperNode`.
    /// Both bounds are optional: `max: None` means "no ceiling, type any value", which a
    /// [`WindowMeasure::Slider`] can never express because its `min`/`max` are mandatory.
    Number {
        id: String,
        label: Option<String>,
        value: f64,
        /// ⬇️ Inclusive floor; `None` leaves the value unbounded below.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        /// ⬆️ Inclusive ceiling; `None` leaves the value unbounded above — renderers must not clamp.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        /// 🎚️ Absolute value already prepared/committed, drawn as a soft extent behind the entry.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        ready: Option<f64>,
        /// 🌀️ When true, the measure tree leaf shows a loading ring while work continues.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        loading: Option<bool>,
        /// 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        waiting: Option<bool>,
        /// 🚫️ When true, the entry is inert.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        on_change: ActionDescriptor,
    },
    Toggle {
        id: String,
        icon_id: IconName,
        label: Option<String>,
        pressed: bool,
        text: Option<String>,
        on_change: ActionDescriptor,
    },
    Group {
        id: String,
        label: String,
        default_open: Option<bool>,
        /// 🎯️ When `Some(utility_id)`, this group is *utility-scoped chrome*: the shell surfaces it only while
        /// `ViewModel.active_utility_id == utility_id`, and renders it in the dedicated "Utility Options" rail
        /// beside the utility bar — never in the always-on Measures overlay. When absent, the group is a
        /// general measure and stays in the Measures overlay exactly as before. See [`partition_window_measures`].
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        active_utility_id: Option<String>,
        /// 🎚️ Optional header slider — when set with `on_change`, the group row hosts a weight control (e.g. object-kind probability).
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        value: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        ready: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        loading: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        waiting: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        on_change: Option<ActionDescriptor>,
        children: Vec<WindowMeasure>,
    },
}
//#endregion 🔖️WindowMeasure

impl WindowMeasure {
    /// 🌳️ Builds a measure group with default slider/header fields unset.
    pub fn measure_group(id: impl Into<String>, label: impl Into<String>, children: Vec<WindowMeasure>) -> Self {
        Self::Group { id: id.into(), label: label.into(), default_open: None, active_utility_id: None, value: None, min: None, max: None, step: None, ready: None, loading: None, waiting: None, on_change: None, children }
    }
}

//#region 🔖️PartitionWindowMeasures
pub const WINDOW_MEASURE_PARTITION_CAPACITY: usize = 64;

#[derive(Debug)]
pub struct WindowMeasurePartition<'a> {
    pub general: UiFixedList<&'a WindowMeasure, WINDOW_MEASURE_PARTITION_CAPACITY>,
    pub utility_options: UiFixedList<&'a WindowMeasure, WINDOW_MEASURE_PARTITION_CAPACITY>,
}

/// 🎯️ Splits a window's top-level measures into `(general, utility_options)`.
///
/// A top-level [`WindowMeasure::Group`] tagged with `active_utility_id: Some(id)` is *utility-scoped chrome*:
/// its **children** land in `utility_options` **only** when `id == active_utility_id`, and the tagged wrapper
/// is dropped from both buckets otherwise (it is irrelevant to whichever utility — or no utility — is
/// currently active). The wrapper itself is a routing envelope only — never rendered — so activating a
/// utility shows its option tree directly (no duplicate utility-name group header). Every untagged group
/// and every non-group top-level measure stays in `general`, unchanged. Tagging is a top-level concept only.
pub fn partition_window_measures<'a>(measures: &'a [WindowMeasure], active_utility_id: Option<&str>) -> Result<WindowMeasurePartition<'a>, &'a WindowMeasure> {
    let mut partition = WindowMeasurePartition { general: UiFixedList::default(), utility_options: UiFixedList::default() };
    for measure in measures {
        match measure {
            WindowMeasure::Group { active_utility_id: Some(scoped), children, .. } => {
                if active_utility_id == Some(scoped.as_str()) {
                    for child in children {
                        partition.utility_options.try_push(child)?;
                    }
                }
            }
            _ => partition.general.try_push(measure)?,
        }
    }
    Ok(partition)
}
//#endregion 🔖️PartitionWindowMeasures
// #endregion layout
