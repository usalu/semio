// #region 🧲️Header
/** 🌐️ `HistoryEntry.label`'s own promise, for the rows the shell journals itself: "a locale switch re-renders the whole
 * ledger instead of leaving logged rows in their dispatch locale". The history body is the guest's own Rust body, so a
 * chrome row can only keep that promise when `noteShellCommand` carries its text in EVERY locale (`{en, de}`, which the
 * guest stores as `LocalizedLabel::native`). A row that carried one resolved string was frozen in its dispatch locale —
 * measured on the live `s` shell (ticket 26/09/18 S10 §2.10 item 2): after switching to German the older rows still read
 * "Switch Panel Tab" while the row for the switch itself read "Panel-Tab wechseln".
 *
 * This suite pins the two-language texts: every one of the nine journalled chrome keys answers real, different English
 * and German text whatever locale the shell stands at, an OS command's manifest label answers per locale, and the note
 * the shell dispatches carries both. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterAll, describe, expect, it } from "vitest";
import { buildNoteShellCommandAction, manifestLabelTextV1, shellLabelTextV1, syncShellLabelLocale } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧱️Fixtures
/** 🧭️ Every chrome key `noteShellCommand` is called with anywhere in the shell. */
const CHROME_COMMAND_KEYS = [
  "ui.shellCommand.dockMove",
  "ui.shellCommand.panelTab",
  "ui.shellCommand.panelToggle",
  "ui.shellCommand.windowActivate",
  "ui.shellCommand.windowClose",
  "ui.shellCommand.windowMove",
  "ui.shellCommand.windowOpenInNewWindow",
  "ui.shellCommand.windowResize",
  "ui.shellCommand.windowSplit",
] as const;
//#endregion 🧱️Fixtures

//#region 🧪️Laws
describe("chrome history rows carry every locale", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("answers real, different English and German text for every journalled chrome key, whatever locale the shell stands at", () => {
    syncShellLabelLocale("en");
    const fromEnglish = CHROME_COMMAND_KEYS.map((key) => shellLabelTextV1(key));
    syncShellLabelLocale("de");
    const fromGerman = CHROME_COMMAND_KEYS.map((key) => shellLabelTextV1(key));
    expect(fromGerman).toEqual(fromEnglish);
    for (const [index, key] of CHROME_COMMAND_KEYS.entries()) {
      const text = fromEnglish[index]!;
      expect(Object.keys(text).sort(), key).toEqual(["de", "en"]);
      for (const locale of ["en", "de"] as const) {
        expect(text[locale], `${key} (${locale})`).not.toBe("");
        expect(text[locale], `${key} (${locale})`).not.toContain("ui.shellCommand.");
      }
      expect(text.de, key).not.toBe(text.en);
    }
    expect(shellLabelTextV1("ui.shellCommand.panelTab").de).toBe("Panel-Tab wechseln");
    expect(shellLabelTextV1("ui.shellCommand.windowActivate").de).toBe("Fenster aktivieren");
  });

  it("answers an OS command's manifest label per locale, and a plain string the same in each", () => {
    expect(manifestLabelTextV1({ native: { en: "Set Locale", de: "Sprache festlegen" }, reuse: { en: "Set Locale", de: "Sprache festlegen" } } as Parameters<typeof manifestLabelTextV1>[0], "native")).toEqual({ en: "Set Locale", de: "Sprache festlegen" });
    expect(manifestLabelTextV1("os.resetDock", "native")).toEqual({ en: "os.resetDock", de: "os.resetDock" });
  });

  it("dispatches the note with both languages as its label", () => {
    syncShellLabelLocale("de");
    const label = shellLabelTextV1("ui.shellCommand.panelTab");
    expect(buildNoteShellCommandAction("app.controller", "shell.panelTab", label, { tabId: "t" })).toEqual({ controllerId: "app.controller", action: "noteShellCommand", args: { commandId: "shell.panelTab", label: { en: label.en, de: "Panel-Tab wechseln" }, detail: { tabId: "t" } } });
  });
});
//#endregion 🧪️Laws
