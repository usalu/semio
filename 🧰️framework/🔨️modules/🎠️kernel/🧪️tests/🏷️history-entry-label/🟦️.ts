/** 🏷️ `historyEntryLabelText`'s two promises, pinned separately.
 *
 * 1. No English fallback: an axis the carrier did not fill renders EMPTY, visibly wrong.
 * 2. It never throws. The function is called from a `useMemo` inside `FrameworkOsShellInner`'s
 *    render, so a throw does not cost one ledger row its text — it unmounts the whole shell. That is
 *    what a single guest carrying a pre-`LocalizedLabel` label shape did on 2026-09-21: every window
 *    gone, `plugin-handle.closed`, an empty `document.body` (ticket 26/09/18 S10 §2.1). Promise 1
 *    was already kept for the locale axis and not for the terminology axis.
 */
import { describe, expect, it } from "vitest";
import { historyEntryLabelText, type LocalizedLabel } from "../../🟦️.ts";

const filled = { native: { en: "Add Widget", de: "Widget hinzufügen" }, reuse: { en: "Add Part", de: "Teil hinzufügen" } } as unknown as LocalizedLabel;

describe("historyEntryLabelText", () => {
  it("reads the requested terminology and locale when the carrier filled both", () => {
    expect(historyEntryLabelText(filled, "native", "en")).toBe("Add Widget");
    expect(historyEntryLabelText(filled, "native", "de")).toBe("Widget hinzufügen");
    expect(historyEntryLabelText(filled, "reuse", "en")).toBe("Add Part");
  });

  it("renders an unfilled locale as empty rather than falling back to English", () => {
    const missingLocale = { native: { de: "Nur Deutsch" }, reuse: {} } as unknown as LocalizedLabel;
    expect(historyEntryLabelText(missingLocale, "native", "en")).toBe("");
  });

  it("renders an unfilled TERMINOLOGY as empty instead of throwing and taking the shell down", () => {
    const preLocalizedLabelShape = { en: "Add Widget", de: "Widget hinzufügen" } as unknown as LocalizedLabel;
    expect(() => historyEntryLabelText(preLocalizedLabelShape, "native", "en")).not.toThrow();
    expect(historyEntryLabelText(preLocalizedLabelShape, "native", "en")).toBe("");
    expect(historyEntryLabelText({ reuse: { en: "x" } } as unknown as LocalizedLabel, "native", "en")).toBe("");
  });

  it("renders an absent label as empty instead of throwing", () => {
    expect(() => historyEntryLabelText(undefined as unknown as LocalizedLabel, "native", "en")).not.toThrow();
    expect(historyEntryLabelText(undefined as unknown as LocalizedLabel, "native", "en")).toBe("");
  });

  it("renders an unknown terminology axis as empty", () => {
    expect(historyEntryLabelText(filled, "marketing", "en")).toBe("");
  });
});
