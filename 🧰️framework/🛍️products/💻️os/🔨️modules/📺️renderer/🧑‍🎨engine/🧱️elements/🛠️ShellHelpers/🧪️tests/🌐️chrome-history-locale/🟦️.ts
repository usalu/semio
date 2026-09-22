// #region 🧲️Header
/** @emoji 🌐️ `HistoryEntry.label`'s own promise, for the rows that cannot keep it by themselves:
 * "a locale switch re-renders the whole ledger instead of leaving logged rows in their dispatch
 * locale". A plugin row carries a real `LocalizedLabel`, so re-resolving it is enough. A CHROME row
 * carries `LocalizedLabel::data(<one resolved string>)` — `noteShellCommand` takes a `string` and
 * every call site hands it `shellLabel("ui.shellCommand.…")` already resolved — so the row is frozen
 * in its dispatch locale and no amount of re-resolving moves it. Measured on the live `s` shell
 * (ticket 26/09/18 S10 §2.10 item 2): after switching to German the older rows still read
 * "Switch Panel Tab" while the row for the switch itself read "Panel-Tab wechseln".
 *
 * So the shell resolves chrome text itself, from the row's `actionId`, and this suite pins the
 * resolver: every one of the nine journalled chrome ids answers, each answers DIFFERENTLY in the two
 * locales, and an id that is not chrome answers `null` so the caller keeps the row's own label. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterAll, describe, expect, it } from "vitest";
import { shellChromeCommandLabel, syncShellLabelLocale } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧱️Fixtures
/** 🧭️ Every `commandId` `noteShellCommand` is called with anywhere in the shell. */
const CHROME_COMMAND_IDS = [
  "shell.dockMove",
  "shell.panelTab",
  "shell.panelToggle",
  "shell.windowActivate",
  "shell.windowClose",
  "shell.windowMove",
  "shell.windowOpenInNewWindow",
  "shell.windowResize",
  "shell.windowSplit",
] as const;
//#endregion 🧱️Fixtures

//#region 🧪️Laws
describe("chrome history rows follow the shell's current locale", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("answers real text for every journalled chrome command id", () => {
    syncShellLabelLocale("en");
    for (const id of CHROME_COMMAND_IDS) {
      const label = shellChromeCommandLabel(id);
      expect(label, id).not.toBeNull();
      expect(label, id).not.toBe("");
      expect(label, id).not.toContain("ui.shellCommand.");
    }
  });

  it("answers a DIFFERENT text once the shell is German — which is the whole defect", () => {
    syncShellLabelLocale("en");
    const english = CHROME_COMMAND_IDS.map((id) => shellChromeCommandLabel(id));
    syncShellLabelLocale("de");
    const german = CHROME_COMMAND_IDS.map((id) => shellChromeCommandLabel(id));
    for (const [index, id] of CHROME_COMMAND_IDS.entries()) {
      expect(german[index], id).not.toBeNull();
      expect(german[index], id).not.toBe(english[index]);
    }
    expect(shellChromeCommandLabel("shell.panelTab")).toBe("Panel-Tab wechseln");
    expect(shellChromeCommandLabel("shell.windowActivate")).toBe("Fenster aktivieren");
  });

  it("answers null for anything that is not one of the nine, so the row keeps its own label", () => {
    syncShellLabelLocale("en");
    expect(shellChromeCommandLabel("addWidget")).toBeNull();
    expect(shellChromeCommandLabel("os.setLocale")).toBeNull();
    expect(shellChromeCommandLabel("shell.notAChromeCommand")).toBeNull();
    expect(shellChromeCommandLabel("")).toBeNull();
  });
});
//#endregion 🧪️Laws
