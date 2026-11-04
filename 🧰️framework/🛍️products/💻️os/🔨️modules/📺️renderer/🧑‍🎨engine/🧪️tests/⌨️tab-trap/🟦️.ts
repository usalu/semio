import { describe, expect, it } from "vitest";
import { isTextEditingKey, TAB_TRAP_ANNOUNCEMENTS_V1, tabTrapStepV1, type TabTrapStateV1 } from "../../🧱️elements/✏️TextEditor/🚫️edit-refusals-and-keys/🟦️.ts";

const key = (k: string, extra: Partial<{ shiftKey: boolean; ctrlKey: boolean; metaKey: boolean; altKey: boolean }> = {}) => ({ key: k, shiftKey: false, ctrlKey: false, metaKey: false, altKey: false, ...extra });
const idle: TabTrapStateV1 = { armed: false, leaveMode: false };

describe("editor Tab trap", () => {
  it("never treats Tab as a text edit, so a read-only editor does not swallow it", () => {
    expect(isTextEditingKey(key("Tab"))).toBe(false);
    expect(isTextEditingKey(key("a"))).toBe(true);
  });

  it("lets a read-only editor and Shift+Tab leave, and inserts a tab otherwise", () => {
    expect(tabTrapStepV1(idle, key("Tab"), true).action).toBe("leave");
    expect(tabTrapStepV1(idle, key("Tab", { shiftKey: true }), false).action).toBe("leave");
    expect(tabTrapStepV1(idle, key("Tab"), false).action).toBe("insert-tab");
  });

  it("Escape followed by Tab leaves with an announcement; any other key disarms", () => {
    const armed = tabTrapStepV1(idle, key("Escape"), false).state;
    expect(armed.armed).toBe(true);
    const left = tabTrapStepV1(armed, key("Tab"), false);
    expect(left).toMatchObject({ action: "leave", announce: "leaving" });
    expect(tabTrapStepV1(tabTrapStepV1(armed, key("a"), false).state, key("Tab"), false).action).toBe("insert-tab");
  });

  it("Ctrl+M toggles a persistent leave mode, announced both ways", () => {
    const on = tabTrapStepV1(idle, key("m", { ctrlKey: true }), false);
    expect(on).toMatchObject({ action: "consume", announce: "leave-mode-on" });
    expect(tabTrapStepV1(on.state, key("Tab"), false).action).toBe("leave");
    expect(tabTrapStepV1(on.state, key("m", { ctrlKey: true }), false)).toMatchObject({ announce: "leave-mode-off", state: { leaveMode: false } });
  });

  it("words every announcement in en and de", () => {
    for (const row of Object.values(TAB_TRAP_ANNOUNCEMENTS_V1)) expect(row.en).not.toBe(row.de);
  });
});
