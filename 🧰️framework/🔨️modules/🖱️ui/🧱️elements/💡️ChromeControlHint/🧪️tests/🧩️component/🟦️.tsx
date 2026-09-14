// #region 🧲️Header
// framework/ui/elements/💡️ChromeControlHint/tests/component.tsx
// #endregion 🧲️Header

import { describe, expect, it } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { DEFAULT_UI_DRIVER, UiDriverProvider } from "../../../🚗️UiDriver/🟦️.tsx";
import { ChromeControlHint } from "../../🟦️.tsx";

describe("ChromeControlHint", () => {
  it("omits native title on the trigger when tooltip text resolves", () => {
    const markup = renderToStaticMarkup(
      <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
        <ChromeControlHint id="ui.tree.drag.sort" always>
          <span data-slot="drag-handle">Grip</span>
        </ChromeControlHint>
      </UiDriverProvider>,
    );
    expect(markup).toContain('data-slot="chrome-control-hint"');
    expect(markup).not.toMatch(/data-slot="drag-handle"[^>]*\btitle=/);
  });
});
