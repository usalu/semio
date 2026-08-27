//! 🌳️ In-place typed descendant retirement under the owning arena's exclusive lease.

use super::*;
use crate::*;

//#region 🧭️FixedTraversal
pub(crate) const UI_TYPED_RETIREMENT_DEPTH: usize = 16;

#[derive(Default)]
pub(crate) struct UiTypedRetirementCursor {
    path: [u8; UI_TYPED_RETIREMENT_DEPTH],
    value: Option<UiValueRetirement>,
    complete: bool,
}

impl std::fmt::Debug for UiTypedRetirementCursor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("UiTypedRetirementCursor").field("path", &self.path).field("value_owned", &self.value.is_some()).field("complete", &self.complete).finish()
    }
}

impl UiTypedRetirementCursor {
    /// 🪶️ Visits one leaf of a root that remains exclusively retained by its arena slot.
    pub(crate) fn advance<T: UiTypedRetire>(&mut self, root: &mut T, maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.complete { return Ok(done()); }
        if T::DEPTH > UI_TYPED_RETIREMENT_DEPTH { return Err("typed schema exceeds admitted retirement depth"); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(UiValueRetirementStep::default()); }
        let step = root.retire_typed(&mut self.path, &mut self.value, maximum_bytes)?;
        if step.complete {
            if self.value.is_some() { return Err("typed root completed with a retained value descendant"); }
            self.complete = true;
        }
        Ok(step)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool { self.complete && self.value.is_none() }
}

pub(crate) trait UiTypedRetire {
    const DEPTH: usize = 0;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str>;
}

fn done() -> UiValueRetirementStep { UiValueRetirementStep { complete: true, progressed: true, released_items: 0, released_bytes: 0 } }

fn split(path: &mut [u8]) -> Result<(&mut u8, &mut [u8]), &'static str> {
    path.split_first_mut().ok_or("typed retirement exceeds its schema depth")
}

fn field_step<T: UiTypedRetire>(field: &mut T, index: &mut u8, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize, count: u8) -> Result<UiValueRetirementStep, &'static str> {
    let mut step = field.retire_typed(path, value, bytes)?;
    if step.complete { *index += 1; path.fill(0); }
    step.complete = *index == count;
    Ok(step)
}

macro_rules! typed_fields {
    ($type:ty { $($index:literal => $field:tt : $field_type:ty),* $(,)? }) => {
        impl UiTypedRetire for $type {
            const DEPTH: usize = 1 + maximum_depth(&[$(<$field_type as UiTypedRetire>::DEPTH),*]);
            fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
                let Self { $($field: _),* } = self;
                let (index, path) = split(path)?;
                let count = 0 $(+ { let _ = stringify!($field); 1 })*;
                match *index {
                    $($index => { let field: &mut $field_type = &mut self.$field; field_step(field, index, path, value, bytes, count) },)*
                    _ => Ok(done()),
                }
            }
        }
    };
}

macro_rules! typed_scalar {
    ($($type:ty),* $(,)?) => {$(
        impl UiTypedRetire for $type {
            fn retire_typed(&mut self, _: &mut [u8], _: &mut Option<UiValueRetirement>, _: usize) -> Result<UiValueRetirementStep, &'static str> { assert_copy::<Self>(); Ok(done()) }
        }
    )*};
}

fn assert_copy<T: Copy>() {}

const fn maximum_depth(depths: &[usize]) -> usize {
    let mut maximum = 0;
    let mut index = 0;
    while index < depths.len() { if depths[index] > maximum { maximum = depths[index]; } index += 1; }
    maximum
}

typed_scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);

impl UiTypedRetire for UiText {
    fn retire_typed(&mut self, _: &mut [u8], _: &mut Option<UiValueRetirement>, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let bytes = self.len().min(maximum_bytes);
        self.len -= bytes as u16;
        Ok(UiValueRetirementStep { complete: self.is_empty(), progressed: true, released_items: 0, released_bytes: bytes })
    }
}

impl UiTypedRetire for UiFixedBytes {
    fn retire_typed(&mut self, _: &mut [u8], _: &mut Option<UiValueRetirement>, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.len != 0 {
            let bytes = self.len().min(maximum_bytes);
            self.len -= bytes as u16;
            return Ok(UiValueRetirementStep::progress(0, bytes));
        }
        let released = !self.bytes.is_empty();
        drop(std::mem::take(&mut self.bytes));
        Ok(UiValueRetirementStep { complete: true, ..UiValueRetirementStep::progress(usize::from(released), 0) })
    }
}

impl UiTypedRetire for UiValue {
    fn retire_typed(&mut self, _: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if value.is_none() {
            *value = Some(UiValueRetirement::new(std::mem::replace(self, UiValue::Null)));
            return Ok(UiValueRetirementStep::progress(0, 0));
        }
        let step = value.as_mut().unwrap().close_step(1, bytes)?;
        if step.complete { value.take(); }
        Ok(step)
    }
}

impl<T: UiTypedRetire> UiTypedRetire for Option<T> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (index, path) = split(path)?;
        let Some(field) = self.as_mut() else { return Ok(done()) };
        if *index == 1 {
            self.take();
            return Ok(UiValueRetirementStep { complete: true, ..UiValueRetirementStep::progress(1, 0) });
        }
        let mut step = field.retire_typed(path, value, bytes)?;
        if step.complete { *index = 1; path.fill(0); }
        step.complete = false;
        Ok(step)
    }
}

impl<A: UiTypedRetire, B: UiTypedRetire> UiTypedRetire for (A, B) {
    const DEPTH: usize = 1 + maximum_depth(&[A::DEPTH, B::DEPTH]);
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (index, path) = split(path)?;
        match *index { 0 => field_step(&mut self.0, index, path, value, bytes, 2), 1 => field_step(&mut self.1, index, path, value, bytes, 2), _ => Ok(done()) }
    }
}

impl<T: UiTypedRetire, const N: usize> UiTypedRetire for UiFixedList<T, N> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (index, path) = split(path)?;
        if *index == 1 {
            self.pop();
            *index = 0;
            path.fill(0);
            return Ok(UiValueRetirementStep::progress(1, 0));
        }
        let Some(field) = self.last_mut() else {
            let released = self.release_empty_allocation()?;
            return Ok(UiValueRetirementStep { complete: true, ..UiValueRetirementStep::progress(usize::from(released), 0) });
        };
        let mut step = field.retire_typed(path, value, bytes)?;
        if step.complete { *index = 1; path.fill(0); }
        step.complete = false;
        Ok(step)
    }
}

impl<V: UiTypedRetire> UiTypedRetire for UiFixedMap<V> {
    const DEPTH: usize = <UiFixedList<(UiText, V)> as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> { self.entries.retire_typed(path, value, bytes) }
}
//#endregion 🧭️FixedTraversal

//#region 🧬️TypedFields
impl UiTypedRetire for Label {
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> { self.0.retire_typed(path, value, bytes) }
}
impl UiTypedRetire for SurfaceId {
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> { self.0.retire_typed(path, value, bytes) }
}
typed_fields!(ActionId { 0 => scope: UiText, 1 => name: UiText, 2 => version: u16 });
typed_fields!(ActionBinding { 0 => trigger: Trigger, 1 => action: ActionId, 2 => args: Option<UiValue>, 3 => capability: Option<UiText> });
typed_fields!(MenuRef { 0 => id: UiText, 1 => args: Option<UiValue> });
typed_fields!(UiIntent { 0 => surface: SurfaceId, 1 => revision: UiRevision, 2 => node: UiNodeId, 3 => node_key: UiText, 4 => trigger: Trigger, 5 => action: ActionId, 6 => args: Option<UiValue>, 7 => input: Option<UiValue>, 8 => seq: u64 });
typed_fields!(AccessibilitySpec { 0 => label: Option<Label>, 1 => description: Option<Label>, 2 => live: Liveness, 3 => shortcut: Option<UiText>, 4 => hidden: bool });
typed_fields!(DropOverlaySpec { 0 => title: Label, 1 => hint: Label, 2 => accept: Option<UiText> });
typed_fields!(SelectItem { 0 => value: UiText, 1 => label: Label });
typed_fields!(KeyValueEntry { 0 => label: Label, 1 => value: UiText });
typed_fields!(RowAction { 0 => icon: UiText, 1 => label: Option<Label>, 2 => action: ActionBinding, 3 => placement: RowActionPlacement });
typed_fields!(ContainerProps { 0 => role: ContainerRole, 1 => label: Option<Label>, 2 => description: Option<UiText>, 3 => required: Option<bool>, 4 => error: Option<UiText>, 5 => default_open: Option<bool>, 6 => drop_overlay: Option<DropOverlaySpec> });
typed_fields!(TextProps { 0 => value: Label, 1 => emphasize: Option<bool>, 2 => data_attributes: Option<UiFixedMap<UiText>> });
typed_fields!(ButtonProps { 0 => icon: UiText, 1 => label: Label });
typed_fields!(SeparatorProps {});
typed_fields!(InputProps { 0 => kind: InputKind, 1 => value: UiText, 2 => placeholder: Option<Label>, 3 => commit: Option<UiText>, 4 => min: Option<f64>, 5 => max: Option<f64>, 6 => step: Option<f64>, 7 => accept: Option<UiText> });
typed_fields!(SelectProps { 0 => value: UiText, 1 => items: UiFixedList<SelectItem>, 2 => placeholder: Option<Label> });
typed_fields!(ToggleProps { 0 => on: bool, 1 => icon: UiText, 2 => text: Option<Label> });
typed_fields!(KeyValueListProps { 0 => entries: UiFixedList<KeyValueEntry> });
typed_fields!(SliderProps { 0 => value: f64, 1 => min: f64, 2 => max: f64, 3 => step: f64, 4 => unit: Option<UiText> });
typed_fields!(NumberStepperProps { 0 => value: f64, 1 => step: f64, 2 => uniform: bool });
typed_fields!(RingProps { 0 => orb_id: UiText, 1 => t: f64 });
typed_fields!(IconSelectProps { 0 => value: UiText, 1 => uniform: bool, 2 => classifier_kind: UiText });
typed_fields!(TreeProps { 0 => interaction_domain: Option<UiText> });
typed_fields!(TreeSectionProps { 0 => label: Option<Label>, 1 => default_open: Option<bool> });
typed_fields!(TreeItemProps { 0 => label: Label, 1 => description: Option<UiText>, 2 => icon: Option<UiText>, 3 => default_open: Option<bool>, 4 => draggable: Option<bool>, 5 => drag_data: Option<UiFixedMap<UiText>>, 6 => dimmed: Option<bool>, 7 => row_actions: UiFixedList<RowAction> });
typed_fields!(ImageProps { 0 => src: UiText, 1 => alt: Option<Label> });
typed_fields!(ExtensionProps { 0 => extension: UiText, 1 => props: UiValue });
typed_fields!(SurfaceProps { 0 => kind: SurfaceKind, 1 => doc_schema: UiText, 2 => doc: SurfaceDoc, 3 => bindings: UiNodeBindings });
typed_fields!(SurfaceDoc { 0 => bytes: UiFixedBytes });
typed_fields!(GridLayout { 0 => columns: UiGridTracks, 1 => rows: UiGridTracks, 2 => column_gap: SpaceToken, 3 => row_gap: SpaceToken, 4 => padding: EdgeSpace, 5 => align: Align, 6 => justify: Justify });
typed_fields!(StackLayout { 0 => axis: Axis, 1 => gap: SpaceToken, 2 => padding: EdgeSpace, 3 => align: Align, 4 => justify: Justify, 5 => grow: bool, 6 => wrap: bool });
typed_fields!(OverlayLayout { 0 => anchor: Anchor, 1 => inset: EdgeSpace, 2 => dismissible: bool });
typed_fields!(ScrollLayout { 0 => axes: ScrollAxes, 1 => padding: EdgeSpace, 2 => sizing: Sizing });
typed_fields!(AbsoluteLayout { 0 => sizing_width: Sizing, 1 => sizing_height: Sizing });
typed_fields!(LeafLayout { 0 => width: Sizing, 1 => height: Sizing });
typed_fields!(UiNodeRecord { 0 => id: UiNodeId, 1 => key: UiText, 2 => component: Component, 3 => layout: LayoutSpec, 4 => style: StyleSpec, 5 => activity: Activity, 6 => disabled: bool, 7 => transition: Option<TransitionHint>, 8 => accessibility: AccessibilitySpec, 9 => bindings: UiNodeBindings, 10 => menu: Option<MenuRef>, 11 => children: UiNodeChildren });
typed_fields!(UiSnapshot { 0 => surface: SurfaceId, 1 => revision: UiRevision, 2 => root: UiNodeId, 3 => nodes: UiSnapshotNodes, 4 => layout_epoch: u64 });
typed_fields!(UiPatch { 0 => surface: SurfaceId, 1 => base_revision: UiRevision, 2 => revision: UiRevision, 3 => ops: UiPatchOps });

impl UiTypedRetire for Component {
    const DEPTH: usize = maximum_depth(&[ContainerProps::DEPTH, TextProps::DEPTH, ButtonProps::DEPTH, SeparatorProps::DEPTH, InputProps::DEPTH, SelectProps::DEPTH, ToggleProps::DEPTH, KeyValueListProps::DEPTH, SliderProps::DEPTH, NumberStepperProps::DEPTH, RingProps::DEPTH, IconSelectProps::DEPTH, TreeProps::DEPTH, TreeSectionProps::DEPTH, TreeItemProps::DEPTH, ImageProps::DEPTH, SurfaceProps::DEPTH, ExtensionProps::DEPTH]);
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        match self {
            Self::Container(field) => field.retire_typed(path, value, bytes), Self::Text(field) => field.retire_typed(path, value, bytes),
            Self::Button(field) => field.retire_typed(path, value, bytes), Self::Separator(field) => field.retire_typed(path, value, bytes),
            Self::Input(field) => field.retire_typed(path, value, bytes), Self::Select(field) => field.retire_typed(path, value, bytes),
            Self::Toggle(field) => field.retire_typed(path, value, bytes), Self::KeyValueList(field) => field.retire_typed(path, value, bytes),
            Self::Slider(field) => field.retire_typed(path, value, bytes), Self::NumberStepper(field) => field.retire_typed(path, value, bytes),
            Self::Ring(field) => field.retire_typed(path, value, bytes), Self::IconSelect(field) => field.retire_typed(path, value, bytes),
            Self::Tree(field) => field.retire_typed(path, value, bytes), Self::TreeSection(field) => field.retire_typed(path, value, bytes),
            Self::TreeItem(field) => field.retire_typed(path, value, bytes), Self::Image(field) => field.retire_typed(path, value, bytes),
            Self::Surface(field) => field.retire_typed(path, value, bytes), Self::Extension(field) => field.retire_typed(path, value, bytes),
        }
    }
}

impl UiTypedRetire for LayoutSpec {
    const DEPTH: usize = maximum_depth(&[LeafLayout::DEPTH, StackLayout::DEPTH, GridLayout::DEPTH, OverlayLayout::DEPTH, ScrollLayout::DEPTH, AbsoluteLayout::DEPTH]);
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        match self {
            Self::Leaf(field) => field.retire_typed(path, value, bytes), Self::Stack(field) => field.retire_typed(path, value, bytes),
            Self::Grid(field) => field.retire_typed(path, value, bytes), Self::Overlay(field) => field.retire_typed(path, value, bytes),
            Self::Scroll(field) => field.retire_typed(path, value, bytes), Self::Absolute(field) => field.retire_typed(path, value, bytes),
        }
    }
}

impl UiTypedRetire for UiPatchOp {
    const DEPTH: usize = maximum_depth(&[UiNodeRecord::DEPTH, Component::DEPTH, LayoutSpec::DEPTH, UiNodeChildren::DEPTH, AccessibilitySpec::DEPTH, UiNodeBindings::DEPTH, <Option<MenuRef>>::DEPTH]);
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        match self {
            Self::Upsert(field) => field.retire_typed(path, value, bytes), Self::SetComponent { id: _, component } => component.retire_typed(path, value, bytes),
            Self::SetLayout { id: _, layout } => layout.retire_typed(path, value, bytes), Self::SetActivity { id: _, activity: _, disabled: _ } => Ok(done()),
            Self::SetChildren { id: _, children } => children.retire_typed(path, value, bytes), Self::SetStyle { id: _, style: _ } => Ok(done()),
            Self::SetAccessibility { id: _, accessibility } => accessibility.retire_typed(path, value, bytes), Self::SetBindings { id: _, bindings } => bindings.retire_typed(path, value, bytes),
            Self::SetMenu { id: _, menu } => menu.retire_typed(path, value, bytes), Self::Remove { id: _ } | Self::SetRoot { id: _ } => Ok(done()),
        }
    }
}

const _: () = assert!(UiSnapshot::DEPTH <= UI_TYPED_RETIREMENT_DEPTH);
const _: () = assert!(UiPatch::DEPTH <= UI_TYPED_RETIREMENT_DEPTH);
const _: () = assert!(UiIntent::DEPTH <= UI_TYPED_RETIREMENT_DEPTH);
//#endregion 🧬️TypedFields
