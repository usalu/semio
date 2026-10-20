import { type ControlKeybindingCallback, type ControlKeybindingOptions, type ControlKeybindingDependencies, useUiKeybindingsByControlId, resolveControlKeybindingRaw, SHELL_KEYBINDINGS, useHotkeys } from "@semio-tech/ui/modules/control-keybinding-context";
import { reactHostPort } from "@semio-tech/ui/elements/Ports/react";
import type { Anchor } from "@semio-tech/ui/modules/panel-anchor";
import { SHELL_PANEL_ANCHOR_KEY_IDS } from "../⌨️keybindings/🟦️.ts";

/**
 * 🎮️ Hook binding a keyboard shortcut from the control registry, shell table, or a raw chord literal.
 **/
export function useActionHotkey(
  hotkeyOrControlId: string,
  callback: ControlKeybindingCallback,
  options?: ControlKeybindingOptions,
  dependencies?: ControlKeybindingDependencies,
  configuration?: {
    overrides?: Record<string, string> | undefined;
  },
) {
  const bindings = useUiKeybindingsByControlId();
  const finalHotkey = reactHostPort.useMemo(() => {
    const override = configuration?.overrides?.[hotkeyOrControlId];
    if (override) return override;
    const fromRegistry = resolveControlKeybindingRaw(hotkeyOrControlId, bindings) ?? SHELL_KEYBINDINGS[hotkeyOrControlId];
    if (fromRegistry) return fromRegistry;
    return hotkeyOrControlId;
  }, [bindings, configuration?.overrides, hotkeyOrControlId]);

  useHotkeys(finalHotkey, callback, options ?? {}, dependencies ?? []);
}

/** ⌨️ Requires the actual declared chord for one authored panel anchor. */
function requiredPanelToggleHotkey(anchor:Anchor):string{const chord=SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS[anchor]];if(chord===undefined)throw Error("Declared panel toggle chord required");return chord;}

/** ⌨️ Chords for toggling each panel's fold/unfold state (derived from {@link SHELL_KEYBINDINGS}). */
export const PANEL_TOGGLE_HOTKEYS: Record<Anchor, string> = {
  "top-left": requiredPanelToggleHotkey("top-left"),
  "top-middle": requiredPanelToggleHotkey("top-middle"),
  "top-right": requiredPanelToggleHotkey("top-right"),
  "right-middle": requiredPanelToggleHotkey("right-middle"),
  "bottom-right": requiredPanelToggleHotkey("bottom-right"),
  "bottom-middle": requiredPanelToggleHotkey("bottom-middle"),
  "bottom-left": requiredPanelToggleHotkey("bottom-left"),
  "left-middle": requiredPanelToggleHotkey("left-middle"),
};

/**
 * ⌨️ Binds {@link PANEL_TOGGLE_HOTKEYS} for all eight anchors when a handler is provided.
 **/
export function usePanelChromeHotkeys(options: { readonly onToggle?: (anchor: Anchor) => void }): void {
  const { onToggle } = options;
  useHotkeys(PANEL_TOGGLE_HOTKEYS["top-left"], () => onToggle?.("top-left"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["top-middle"], () => onToggle?.("top-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["top-right"], () => onToggle?.("top-right"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["right-middle"], () => onToggle?.("right-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["bottom-right"], () => onToggle?.("bottom-right"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["bottom-middle"], () => onToggle?.("bottom-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["bottom-left"], () => onToggle?.("bottom-left"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["left-middle"], () => onToggle?.("left-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
}
