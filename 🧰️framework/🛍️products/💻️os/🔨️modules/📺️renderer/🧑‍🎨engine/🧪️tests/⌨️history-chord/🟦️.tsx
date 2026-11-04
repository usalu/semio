import { cleanup, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { controlOwnsKeyboardEvent, isHistoryChord } from "../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";

afterEach(cleanup);
const chord = (key: string, extra: Partial<{ ctrlKey: boolean; metaKey: boolean; shiftKey: boolean; altKey: boolean }> = {}) => ({ key, ctrlKey: false, metaKey: false, shiftKey: false, altKey: false, ...extra });

describe("history chords reach editor history from a clean Details field", () => {
  it("recognises undo and redo chords", () => {
    expect(isHistoryChord(chord("z", { ctrlKey: true }))).toBe(true);
    expect(isHistoryChord(chord("Z", { metaKey: true, shiftKey: true }))).toBe(true);
    expect(isHistoryChord(chord("y", { ctrlKey: true }))).toBe(true);
    expect(isHistoryChord(chord("z"))).toBe(false);
    expect(isHistoryChord(chord("c", { ctrlKey: true }))).toBe(false);
  });

  it("lets a clean interpreter field yield, keeps native undo for a dirty draft and for foreign fields", () => {
    const view = render(<><input id="clean" data-ui-node-id="1" /><input id="dirty" data-ui-node-id="2" data-draft-dirty="true" /><input id="foreign" /></>);
    const target = (id: string) => view.container.querySelector(`#${id}`)!;
    const undo = (id: string) => ({ ...chord("z", { ctrlKey: true }), target: target(id) });
    expect(controlOwnsKeyboardEvent(undo("clean"))).toBe(false);
    expect(controlOwnsKeyboardEvent(undo("dirty"))).toBe(true);
    expect(controlOwnsKeyboardEvent(undo("foreign"))).toBe(true);
    expect(controlOwnsKeyboardEvent({ ...chord("a"), target: target("clean") })).toBe(true);
  });
});
