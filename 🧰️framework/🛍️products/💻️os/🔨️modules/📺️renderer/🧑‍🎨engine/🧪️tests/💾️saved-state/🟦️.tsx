import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { CLOSE_GUARD_LABELS_V1, closeGuardTextV1, DocumentCloseDialog, UnsavedMarker, useBeforeUnloadGuardV1 } from "../../🧱️elements/🏛️ShellHost/🚪️close-guard/🟦️.tsx";

afterEach(cleanup);

function Probe(props: Readonly<{ unsaved: boolean }>): null {
  useBeforeUnloadGuardV1(props.unsaved);
  return null;
}

describe("close guard dialog", () => {
  it("is an accessible modal alert dialog focusing the safe choice, in en and de", () => {
    for (const locale of ["en", "de"]) {
      let kept = 0, discarded = 0;
      render(<DocumentCloseDialog name="plan.json" locale={locale} failure={{ code: "folder-write-failed" }} onKeep={() => { kept += 1; }} onDiscard={() => { discarded += 1; }} />);
      const dialog = screen.getByRole("alertdialog");
      expect(dialog.getAttribute("aria-modal")).toBe("true");
      expect(dialog.textContent).toContain("plan.json");
      expect(document.activeElement?.textContent).toBe(closeGuardTextV1("keep", locale));
      fireEvent.keyDown(dialog, { key: "Escape" });
      expect(kept).toBe(1);
      fireEvent.click(screen.getByRole("button", { name: closeGuardTextV1("discard", locale) }));
      expect(discarded).toBe(1);
      cleanup();
    }
  });

  it("words every label key in en and de", () => {
    for (const key of Object.keys(CLOSE_GUARD_LABELS_V1) as (keyof typeof CLOSE_GUARD_LABELS_V1)[]) expect(closeGuardTextV1(key, "en")).not.toBe(closeGuardTextV1(key, "de"));
  });
});

describe("close dialog while saving", () => {
  it("shows a busy saving text with only a cancel that keeps the document", () => {
    let kept = 0;
    render(<DocumentCloseDialog name="plan.json" locale="de" failure={null} saving onKeep={() => { kept += 1; }} onDiscard={() => undefined} />);
    expect(screen.getByRole("alertdialog").textContent).toContain("wird vor dem Schließen gespeichert");
    expect(screen.getAllByRole("button").length).toBe(1);
    fireEvent.click(screen.getByRole("button", { name: closeGuardTextV1("cancel", "de") }));
    expect(kept).toBe(1);
  });
});

describe("unsaved marker", () => {
  it("shows only for an unsaved acknowledged state and offers retry beside a typed failure", () => {
    let retries = 0;
    const view = render(<UnsavedMarker saved={{ kind: "saved" }} locale="en" />);
    expect(view.container.querySelector("[data-semio-unsaved-marker]")).toBeNull();
    view.rerender(<UnsavedMarker saved={{ kind: "unsaved", pending: 1, failure: { code: "folder-write-failed" } }} locale="de" onRetry={() => { retries += 1; }} />);
    expect(screen.getByRole("status").textContent).toContain("Der Ordner hat das Schreiben abgelehnt.");
    fireEvent.click(screen.getByRole("button", { name: closeGuardTextV1("retry", "de") }));
    expect(retries).toBe(1);
  });
});

describe("beforeunload guard", () => {
  it("prevents unload only while unsaved and detaches afterwards", () => {
    const view = render(<Probe unsaved />);
    const event = new Event("beforeunload", { cancelable: true });
    window.dispatchEvent(event);
    expect(event.defaultPrevented).toBe(true);
    view.rerender(<Probe unsaved={false} />);
    const after = new Event("beforeunload", { cancelable: true });
    window.dispatchEvent(after);
    expect(after.defaultPrevented).toBe(false);
  });
});

import { CurrentDocumentUnsavedMarker, UnsavedMarkerContext } from "../../🧱️elements/🏛️ShellHost/🚪️close-guard/🟦️.tsx";

describe("window chrome unsaved marker", () => {
  it("reads the focused document's acknowledged state from context and is absent when saved", () => {
    const view = render(<UnsavedMarkerContext.Provider value={{ saved: { kind: "saved" }, locale: "en" }}><CurrentDocumentUnsavedMarker /></UnsavedMarkerContext.Provider>);
    expect(view.container.querySelector("[data-semio-unsaved-marker]")).toBeNull();
    view.rerender(<UnsavedMarkerContext.Provider value={{ saved: { kind: "unsaved", pending: 2, failure: null }, locale: "de" }}><CurrentDocumentUnsavedMarker /></UnsavedMarkerContext.Provider>);
    expect(screen.getByRole("status").textContent).toContain("Nicht gespeicherte Änderungen");
  });
});
