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
    pub(crate) const fn empty() -> Self { Self { path: [0; UI_TYPED_RETIREMENT_DEPTH], value: None, complete: false } }
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
            self.truncate_retired_last()?;
            *index = 0;
            path.fill(0);
            return Ok(UiValueRetirementStep::progress(1, 0));
        }
        let Some(field) = self.last_mut() else {
            let released = self.release_empty_page()?;
            return Ok(UiValueRetirementStep { complete: self.terminal_is_empty(), ..UiValueRetirementStep::progress(usize::from(released.progressed), 0) });
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
include!("../../🧬️typed/📋️fields.rs");
ui_typed_field_catalog!(typed_fields);
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
