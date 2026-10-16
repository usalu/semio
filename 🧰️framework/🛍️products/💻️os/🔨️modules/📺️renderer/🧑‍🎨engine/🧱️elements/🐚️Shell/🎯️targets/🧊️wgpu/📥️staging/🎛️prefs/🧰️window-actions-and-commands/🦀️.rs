//! 🎛️ `shell-prefs` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
impl PrefsState {
    /// 🎛️ One `build_command_panel_ui` row: a `Select` for the four os commands whose single arg already
    /// has a "framework" `dispatch_action` arm, else a plain (non-interactive) label — see
    /// `build_command_panel_ui`'s doc comment for why `os.resetDock` and any future arg-carrying
    /// Plugin/App/Mode command fall into that last case.
    pub fn build_command_panel_row(&self, entry: &ResolvedCommand) -> UiTreeItemNode {
        let definition = &entry.definition;
        let key = command_address_stable_key(&entry.address);
        let arg_carrying = !definition.args.is_empty();
        UiTreeItemNode {
            id: format!("command.{}", key.replace(':', ".")),
            label: Label::data(if arg_carrying { format!("{}…", definition.label.resolve(self.active_terminology(), self.active_locale())) } else { definition.label.resolve(self.active_terminology(), self.active_locale()).to_string() }),
            icon_id: Some(if arg_carrying { IconName::ChevronUp } else { definition.icon_id.clone() }),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: if arg_carrying { "setCommandExpanded" } else { "executeResolvedCommand" }.into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": key }) }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        }
    }
    // #endregion
}

/// 🎛️ One aggregated command plus its fully-qualified owner address.
#[derive(Clone, Debug)]
pub struct ResolvedCommand {
    pub definition: semio_framework::CommandDefinition,
    pub address: semio_framework::manifest::CommandAddress,
}

impl ResolvedCommand {
    pub fn new(definition: semio_framework::CommandDefinition, owner: semio_framework::manifest::CommandOwnerAddress) -> Self {
        let command_id = definition.id.clone();
        Self { definition, address: semio_framework::manifest::CommandAddress { owner, command_id } }
    }
}

pub fn command_address_stable_key(address: &semio_framework::manifest::CommandAddress) -> String {
    match &address.owner {
        semio_framework::manifest::CommandOwnerAddress::Os => format!("os:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id } => format!("plugin:{plugin_id}:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::App { plugin_id, app_id } => format!("app:{plugin_id}:{app_id}:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id, app_id, mode_id } => format!("mode:{plugin_id}:{app_id}:{mode_id}:{}", address.command_id),
    }
}

//#endregion ActionPanelAndUtilities

