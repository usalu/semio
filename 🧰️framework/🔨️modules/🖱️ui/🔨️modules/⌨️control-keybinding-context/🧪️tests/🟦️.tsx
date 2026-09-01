// #region 🧲️Header
// 💻️ framework/ui/modules/⌨️control-keybinding-context/component.test.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { parseOwnedHotkeyChords, keyboardEventMatchesOwnedHotkey, normalizeHotkeyKey, useHotkeys, type ControlKeybindingOptions } from "../🟦️component.tsx";
// #endregion 🔌️Adapters

// #region ⌨️OwnedHotkeys
interface HotkeyHarnessProps {
  readonly callback: () => void;
  readonly keys: string;
  readonly options?: ControlKeybindingOptions;
}

/** @emoji 🧪️ Mounts the owned listener beside a representative form field. */
function HotkeyHarness({ callback, keys, options }: HotkeyHarnessProps) {
  useHotkeys(keys, callback, options);
  return <input aria-label="field" />;
}

describe("owned hotkeys", () => {
  it("normalizes mod to Meta on Apple platforms and Control elsewhere", () => {
    const appleChord = parseOwnedHotkeyChords("mod+shift+p", true)[0]!;
    const otherChord = parseOwnedHotkeyChords("mod+shift+p", false)[0]!;
    expect(keyboardEventMatchesOwnedHotkey(new KeyboardEvent("keydown", { key: "P", metaKey: true, shiftKey: true }), appleChord)).toBe(true);
    expect(keyboardEventMatchesOwnedHotkey(new KeyboardEvent("keydown", { key: "P", ctrlKey: true, shiftKey: true }), appleChord)).toBe(false);
    expect(keyboardEventMatchesOwnedHotkey(new KeyboardEvent("keydown", { key: "P", ctrlKey: true, shiftKey: true }), otherChord)).toBe(true);
    expect(keyboardEventMatchesOwnedHotkey(new KeyboardEvent("keydown", { key: "P", metaKey: true, shiftKey: true }), otherChord)).toBe(false);
    expect(normalizeHotkeyKey(" ")).toBe("space");
  });

  it("parses comma-separated chords and prevents default only after a match", () => {
    const callback = vi.fn();
    render(<HotkeyHarness callback={callback} keys="ctrl+k, meta+k" options={{ preventDefault: true }} />);
    const miss = new KeyboardEvent("keydown", { cancelable: true, key: "j", ctrlKey: true });
    const controlMatch = new KeyboardEvent("keydown", { cancelable: true, key: "k", ctrlKey: true });
    const metaMatch = new KeyboardEvent("keydown", { cancelable: true, key: "K", metaKey: true });
    window.dispatchEvent(miss);
    window.dispatchEvent(controlMatch);
    window.dispatchEvent(metaMatch);
    expect(miss.defaultPrevented).toBe(false);
    expect(controlMatch.defaultPrevented).toBe(true);
    expect(metaMatch.defaultPrevented).toBe(true);
    expect(callback).toHaveBeenCalledTimes(2);
  });

  it("ignores form fields unless they are explicitly enabled", () => {
    const callback = vi.fn();
    const view = render(<HotkeyHarness callback={callback} keys="enter" />);
    fireEvent.keyDown(view.getByRole("textbox"), { key: "Enter" });
    expect(callback).not.toHaveBeenCalled();
    view.rerender(<HotkeyHarness callback={callback} keys="enter" options={{ enableOnFormTags: true }} />);
    fireEvent.keyDown(view.getByRole("textbox"), { key: "Enter" });
    expect(callback).toHaveBeenCalledOnce();
  });

  it("does not attach an enabled action while disabled", () => {
    const callback = vi.fn();
    render(<HotkeyHarness callback={callback} keys="escape" options={{ enabled: false }} />);
    fireEvent.keyDown(window, { key: "Escape" });
    expect(callback).not.toHaveBeenCalled();
  });

  it("removes its listener on cleanup", () => {
    const callback = vi.fn();
    const view = render(<HotkeyHarness callback={callback} keys="escape" />);
    fireEvent.keyDown(window, { key: "Escape" });
    view.unmount();
    fireEvent.keyDown(window, { key: "Escape" });
    expect(callback).toHaveBeenCalledOnce();
  });
});
// #endregion ⌨️OwnedHotkeys
