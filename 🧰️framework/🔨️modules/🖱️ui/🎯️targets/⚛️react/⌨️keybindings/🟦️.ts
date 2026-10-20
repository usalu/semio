import type { Anchor } from "@semio-tech/ui/modules/panel-anchor";
import { SHELL_KEYBINDINGS } from "@semio-tech/ui/modules/control-keybinding-context";

/** ⌨️ Maps dock {@link Anchor} values to {@link SHELL_KEYBINDINGS} control ids. */
export const SHELL_PANEL_ANCHOR_KEY_IDS: Readonly<Record<Anchor, keyof typeof SHELL_KEYBINDINGS & string>> = {
  "top-left": "ui.shell.panelAnchor.topLeft",
  "top-middle": "ui.shell.panelAnchor.topMiddle",
  "top-right": "ui.shell.panelAnchor.topRight",
  "right-middle": "ui.shell.panelAnchor.rightMiddle",
  "bottom-right": "ui.shell.panelAnchor.bottomRight",
  "bottom-middle": "ui.shell.panelAnchor.bottomMiddle",
  "bottom-left": "ui.shell.panelAnchor.bottomLeft",
  "left-middle": "ui.shell.panelAnchor.leftMiddle",
};
