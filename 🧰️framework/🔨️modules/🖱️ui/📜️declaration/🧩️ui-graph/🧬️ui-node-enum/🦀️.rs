//! 🧬️ Ui node enum: the `UiNode` union and its constructors, accessors and conversions. See [`super`].

use super::*;

// #region ui
//#region 🔖️UiNode
/// ⚠️ `ComponentScene`'s payload dwarfs the other variants (nested scene JSON blobs) — boxing it
/// would be a breaking public-API change (every construction/match site across ~30 plugins would
/// need `Box::new`/deref updates), out of scope for a mechanical pass.
#[allow(clippy::large_enum_variant, reason = "boxing is a breaking public API change, out of T1 scope")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "type", rename_all = "camelCase")]
#[value(tag = "type", rename_all = "camelCase")]
pub enum UiNode {
    Stack(UiStackNode),
    Text(UiTextNode),
    Button(UiButtonNode),
    Separator(UiSeparatorNode),
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
    Progress(UiProgressNode),
    Field(UiFieldNode),
    Section(UiSectionNode),
    Group(UiGroupNode),
    Tree(UiTreeNode),
    Image(UiImageNode),
    ComponentScene(UiComponentSceneNode),
    ExternalSlot(UiExternalSlotNode),
}

impl UiNode {
    /// 🧭️ Exhaustive presence accessor — adding a `UiNode` variant fails to compile here (and in
    /// `presence_mut` and `paint_node`'s match) until the new element's `presence` field is wired up.
    pub fn presence(&self) -> &UiPresence {
        match self {
            UiNode::Stack(n) => &n.presence,
            UiNode::Text(n) => &n.presence,
            UiNode::Button(n) => &n.presence,
            UiNode::Separator(n) => &n.presence,
            UiNode::Input(n) => &n.presence,
            UiNode::Select(n) => &n.presence,
            UiNode::Toggle(n) => &n.presence,
            UiNode::KeyValue(n) => &n.presence,
            UiNode::Slider(n) => &n.presence,
            UiNode::NumberStepper(n) => &n.presence,
            UiNode::Ring(n) => &n.presence,
            UiNode::IconSelect(n) => &n.presence,
            UiNode::Progress(n) => &n.presence,
            UiNode::Field(n) => &n.presence,
            UiNode::Section(n) => &n.presence,
            UiNode::Group(n) => &n.presence,
            UiNode::Tree(n) => &n.presence,
            UiNode::Image(n) => &n.presence,
            UiNode::ComponentScene(n) => &n.presence,
            UiNode::ExternalSlot(n) => &n.presence,
        }
    }
    pub fn presence_mut(&mut self) -> &mut UiPresence {
        match self {
            UiNode::Stack(n) => &mut n.presence,
            UiNode::Text(n) => &mut n.presence,
            UiNode::Button(n) => &mut n.presence,
            UiNode::Separator(n) => &mut n.presence,
            UiNode::Input(n) => &mut n.presence,
            UiNode::Select(n) => &mut n.presence,
            UiNode::Toggle(n) => &mut n.presence,
            UiNode::KeyValue(n) => &mut n.presence,
            UiNode::Slider(n) => &mut n.presence,
            UiNode::NumberStepper(n) => &mut n.presence,
            UiNode::Ring(n) => &mut n.presence,
            UiNode::IconSelect(n) => &mut n.presence,
            UiNode::Progress(n) => &mut n.presence,
            UiNode::Field(n) => &mut n.presence,
            UiNode::Section(n) => &mut n.presence,
            UiNode::Group(n) => &mut n.presence,
            UiNode::Tree(n) => &mut n.presence,
            UiNode::Image(n) => &mut n.presence,
            UiNode::ComponentScene(n) => &mut n.presence,
            UiNode::ExternalSlot(n) => &mut n.presence,
        }
    }
    /// 🖱️ Exhaustive context-menu-ref accessor — adding a `UiNode` variant fails to compile here
    /// (and in `menu_mut`) until the new element's `menu` field is wired up. `None` means the
    /// element bubbles right-clicks to its nearest menu-bearing ancestor.
    pub fn menu(&self) -> Option<&UiMenuRef> {
        match self {
            UiNode::Stack(n) => n.menu.as_ref(),
            UiNode::Text(n) => n.menu.as_ref(),
            UiNode::Button(n) => n.menu.as_ref(),
            UiNode::Separator(n) => n.menu.as_ref(),
            UiNode::Input(n) => n.menu.as_ref(),
            UiNode::Select(n) => n.menu.as_ref(),
            UiNode::Toggle(n) => n.menu.as_ref(),
            UiNode::KeyValue(n) => n.menu.as_ref(),
            UiNode::Slider(n) => n.menu.as_ref(),
            UiNode::NumberStepper(n) => n.menu.as_ref(),
            UiNode::Ring(n) => n.menu.as_ref(),
            UiNode::IconSelect(n) => n.menu.as_ref(),
            UiNode::Progress(n) => n.menu.as_ref(),
            UiNode::Field(n) => n.menu.as_ref(),
            UiNode::Section(n) => n.menu.as_ref(),
            UiNode::Group(n) => n.menu.as_ref(),
            UiNode::Tree(n) => n.menu.as_ref(),
            UiNode::Image(n) => n.menu.as_ref(),
            UiNode::ComponentScene(n) => n.menu.as_ref(),
            UiNode::ExternalSlot(n) => n.menu.as_ref(),
        }
    }
    pub fn menu_mut(&mut self) -> &mut Option<UiMenuRef> {
        match self {
            UiNode::Stack(n) => &mut n.menu,
            UiNode::Text(n) => &mut n.menu,
            UiNode::Button(n) => &mut n.menu,
            UiNode::Separator(n) => &mut n.menu,
            UiNode::Input(n) => &mut n.menu,
            UiNode::Select(n) => &mut n.menu,
            UiNode::Toggle(n) => &mut n.menu,
            UiNode::KeyValue(n) => &mut n.menu,
            UiNode::Slider(n) => &mut n.menu,
            UiNode::NumberStepper(n) => &mut n.menu,
            UiNode::Ring(n) => &mut n.menu,
            UiNode::IconSelect(n) => &mut n.menu,
            UiNode::Progress(n) => &mut n.menu,
            UiNode::Field(n) => &mut n.menu,
            UiNode::Section(n) => &mut n.menu,
            UiNode::Group(n) => &mut n.menu,
            UiNode::Tree(n) => &mut n.menu,
            UiNode::Image(n) => &mut n.menu,
            UiNode::ComponentScene(n) => &mut n.menu,
            UiNode::ExternalSlot(n) => &mut n.menu,
        }
    }
}
//#endregion 🔖️UiNode
// #endregion ui
