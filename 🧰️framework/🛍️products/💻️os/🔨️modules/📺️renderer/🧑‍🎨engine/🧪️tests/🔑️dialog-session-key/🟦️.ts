/** 🔑️ Shell dialog session key: an effect keyed on it re-runs on a new session, never on a republished view state. */
import { describe, expect, it } from "vitest";
import { shellDialogSessionIsCurrentV1, shellDialogSessionKeyV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🟦️.ts";

describe("Shell dialog session key", () => {
  const session = { pluginId: "draw", instanceId: 7, app: { id: "drawing", controllerId: "drawing-controller" } };

  it("is equal for a republished view state, so the attached folder stays attached", () => {
    const republished = { ...session, app: { ...session.app }, viewState: { selected: ["rectangle"] } };
    expect(shellDialogSessionKeyV1(republished)).toBe(shellDialogSessionKeyV1(session));
    expect(shellDialogSessionIsCurrentV1(republished, session)).toBe(true);
  });

  it("differs exactly when the session identity differs", () => {
    for (const other of [{ ...session, pluginId: "fem" }, { ...session, instanceId: 8 }, { ...session, app: { ...session.app, id: "other" } }, { ...session, app: { ...session.app, controllerId: "other" } }]) {
      expect(shellDialogSessionKeyV1(other)).not.toBe(shellDialogSessionKeyV1(session));
      expect(shellDialogSessionIsCurrentV1(other, session)).toBe(false);
    }
    expect(shellDialogSessionKeyV1(null)).toBe("");
  });
});
