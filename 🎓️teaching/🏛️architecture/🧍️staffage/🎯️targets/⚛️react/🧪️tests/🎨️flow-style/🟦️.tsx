/** 🎨️ The air flow layer takes its style as an adopted style sheet: the release policy of the site (`style-src 'self'` and hashes) blocks a `<style>` element made at run time, so the layer was unstyled there and the console reported the violation.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🌊️flow/🖥️stage/🟦️.ts — `styleFlowLayer`
 * @see ../../../../🌐️site/🏗️builder/🌐️vite/🟦️.ts — `staffageContentSecurityPolicy` */
import { afterEach, describe, expect, it, vi } from "vitest";
import { styleFlowLayer } from "../../🔨️modules/🌊️flow/🖥️stage/🟦️.ts";

class Sheet {
  text = "";
  replaceSync(text: string): void {
    this.text = text;
  }
}

function parent(): HTMLElement {
  const element = document.createElement("div");
  document.body.append(element);
  return element;
}

afterEach(() => {
  vi.unstubAllGlobals();
  Reflect.deleteProperty(document, "adoptedStyleSheets");
  document.head.replaceChildren();
  document.body.replaceChildren();
});

describe("🎨️ flow style", () => {
  it("adopts one style sheet and creates no style element where the platform has constructable sheets", () => {
    vi.stubGlobal("CSSStyleSheet", Sheet);
    Object.defineProperty(document, "adoptedStyleSheets", { value: [], writable: true, configurable: true });
    styleFlowLayer(parent());
    styleFlowLayer(parent());
    const sheets = document.adoptedStyleSheets as unknown as Sheet[];
    expect(sheets).toHaveLength(1);
    expect(sheets[0]!.text).toContain("semio-flow");
    expect(document.querySelector("style")).toBeNull();
  });

  it("falls back to one style element in the head", () => {
    styleFlowLayer(parent());
    styleFlowLayer(parent());
    expect(document.head.querySelectorAll("style#semio-flow-style")).toHaveLength(1);
  });
});
