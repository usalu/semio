//! 🥣️ semio wgpu input laws: hit registry, pointer and keyboard actions against the widget kit.

#![cfg(test)]

use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_action::{BoundedAction, BoundedActionBatchReservation, BoundedActionFault, BoundedActionQueue, BoundedActionReservation, QueuedActionDescriptor, RetainedStringAction, RetainedStringActionStep, RETAINED_ACTION_STRING_PAGE_BYTES};
use std::rc::Rc;
use std::collections::HashMap;
use semio_framework_ui_wgpu_input::*;
use semio_framework_ui_wgpu_mounted_layout::hit::*;

#[path = "../../../../../🧪️tests/🔬️targets-wgpu-input-unit/🦀️.rs"]
mod tests;

