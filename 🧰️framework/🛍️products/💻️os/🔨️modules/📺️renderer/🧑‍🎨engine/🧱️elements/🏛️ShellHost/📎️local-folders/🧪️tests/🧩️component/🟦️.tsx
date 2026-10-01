// #region 🧲️Header
/** 📎️ The offer to reconnect a remembered folder is accessible in both shell languages: one polite status region named
 * "Folder of this document" / "Ordner dieses Dokuments" that names the folder, a "Reconnect folder" button that is the
 * person's own gesture and a "Forget folder" button; while reconnecting both are disabled and the reconnect is busy.
 * Accessible names are computed by `dom-accessibility-api` (third party), clicks are driven by `@testing-library/user-event`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createRequire } from "node:module";
import * as React from "react";
import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterAll, describe, expect, it, vi } from "vitest";
import type * as DomAccessibilityApi from "dom-accessibility-api" with { "resolution-mode": "require" };
import { shellLabel, syncShellLabelLocale } from "../../../../🛠️ShellHelpers/🟦️.tsx";
import { LocalFolderReconnectBand, localFolderNameV1 } from "../../🟦️.tsx";

const { computeAccessibleName } = createRequire(import.meta.url)("dom-accessibility-api") as typeof DomAccessibilityApi;
// #endregion 🔌️Adapters

//#region 🧪️Laws
const EXPECTED = {
  en: { label: "Folder of this document", message: "This document was attached to the folder “puzzles”.", reconnect: "Reconnect folder", forget: "Forget folder" },
  de: { label: "Ordner dieses Dokuments", message: "Dieses Dokument war mit dem Ordner „puzzles“ verbunden.", reconnect: "Ordner wieder verbinden", forget: "Ordner vergessen" },
} as const;

function band(busy: boolean, onReconnect: () => void, onForget: () => void): React.ReactElement {
  return (
    <LocalFolderReconnectBand
      label={String(shellLabel("ui.sync.reconnect.label"))}
      message={String(shellLabel("ui.sync.reconnect.message", { folder: localFolderNameV1("/Users/ada/Documents/puzzles/") }))}
      reconnect={String(shellLabel("ui.sync.reconnect.attach"))}
      forget={String(shellLabel("ui.sync.reconnect.forget"))}
      busy={busy}
      onReconnect={onReconnect}
      onForget={onForget}
    />
  );
}

describe("local folder reconnect band", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("names the remembered folder in a polite status region and offers reconnect and forget, in both languages", async () => {
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      const reconnect = vi.fn();
      const forget = vi.fn();
      const view = render(band(false, reconnect, forget));
      const region = view.getByRole("status");
      expect(region.getAttribute("aria-live")).toBe("polite");
      expect(computeAccessibleName(region)).toBe(EXPECTED[locale].label);
      expect(view.container.querySelector("#s-folder-reconnect-message")?.textContent).toBe(EXPECTED[locale].message);
      const reconnectButton = view.container.querySelector<HTMLButtonElement>("#s-folder-reconnect")!;
      const forgetButton = view.container.querySelector<HTMLButtonElement>("#s-folder-forget")!;
      expect([computeAccessibleName(reconnectButton), computeAccessibleName(forgetButton)]).toEqual([EXPECTED[locale].reconnect, EXPECTED[locale].forget]);
      const user = userEvent.setup();
      await user.click(reconnectButton);
      await user.click(forgetButton);
      expect([reconnect.mock.calls.length, forget.mock.calls.length]).toEqual([1, 1]);
      view.unmount();
    }
  });

  it("disables both actions and marks the reconnect busy while the folder reconnects", async () => {
    syncShellLabelLocale("en");
    const reconnect = vi.fn();
    const view = render(band(true, reconnect, vi.fn()));
    const reconnectButton = view.container.querySelector<HTMLButtonElement>("#s-folder-reconnect")!;
    expect([reconnectButton.disabled, reconnectButton.getAttribute("aria-busy"), view.container.querySelector<HTMLButtonElement>("#s-folder-forget")!.disabled]).toEqual([true, "true", true]);
    await userEvent.setup().click(reconnectButton);
    expect(reconnect).not.toHaveBeenCalled();
    view.unmount();
  });

  it("names a folder by its last path segment", () => {
    expect([localFolderNameV1("/Users/ada/Documents/puzzles"), localFolderNameV1("C:\\Users\\ada\\drawings\\"), localFolderNameV1("/")]).toEqual(["puzzles", "drawings", "/"]);
  });
});
//#endregion 🧪️Laws
