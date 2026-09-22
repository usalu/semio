// #region 🔌️Adapters
import { render } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it } from "vitest";
// 🔁️ Through the package barrel, not the element file: `📐️Layout` → `🖼️Panel` → `🎯️targets/⚛️react`
// is a real runtime cycle, and entering it at the element leaves `PanelTabButton`'s own imports
// undefined in jsdom ("Element type is invalid … Check the render method of `PanelTabButton`").
import { Layout } from "@semio-tech/ui-react";
import reserveFixture from "../../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🛟️panel-window-reservation/🔣️.json";
import reserveSchema from "../../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🛟️panel-window-reservation/🧬️schema/🔣️.json";
// #endregion 🔌️Adapters

describe("Layout panel overlay", () => {
  it("keeps the window column full width so panels can float over the windows", () => {
    const { container } = render(<Layout canvas={<div data-testid="canvas" />} />);
    const column = container.querySelector('[data-slot="layout-canvas-column"]') as HTMLElement;

    expect(column).not.toBeNull();
    expect(column.contains(container.querySelector('[data-testid="canvas"]'))).toBe(true);
    expect(column.style.paddingLeft).toBe("");
    expect(column.style.paddingRight).toBe("");
    expect(column.getAttribute("style")).toBeNull();
  });
});

describe("Shared native panel overlay contract", () => {
  it("keeps the window canvas full width under every open panel", async () => {
    const [{ chromium }, { default: Ajv }, { renderToStaticMarkup }] = await Promise.all([import("playwright"), import("ajv/dist/2020"), import("react-dom/server")]);
    const validate = new Ajv({ strict: true }).compile(reserveSchema);
    expect(validate(reserveFixture), JSON.stringify(validate.errors)).toBe(true);
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      for (const row of reserveFixture.cases) {
        const markup = renderToStaticMarkup(<div id="column" style={{ boxSizing: "border-box", width: row.width }}><div id="canvas" style={{ height: 400 }} /></div>);
        await page.setContent(`<style>body{margin:0;--spacing-single:${reserveFixture.spacing}px}</style>${markup}`);
        const rect = await page.locator("#canvas").boundingBox();
        expect(rect, row.id).not.toBeNull();
        expect(rect!.x, row.id).toBeCloseTo(row.reserved[0]!, 1);
        expect(rect!.width, row.id).toBeCloseTo(row.width - row.reserved[0]! - row.reserved[1]!, 1);
      }
      await page.setContent('<canvas id="scene" width="900" height="600"></canvas>');
      await page.evaluate(() => {
        const scene = document.querySelector("#scene")!;
        scene.addEventListener("pointerdown", () => document.body.dataset.owner = "surface");
        scene.addEventListener("wheel", () => document.body.dataset.wheel = "true");
      });
      await page.mouse.click(400, 300);
      await page.mouse.wheel(0, 120);
      await expect.poll(() => page.evaluate(() => document.body.dataset.wheel === "true")).toBe(reserveFixture.retainedWorld.wheel);
      expect(await page.evaluate(() => document.body.dataset.owner)).toBe(reserveFixture.retainedWorld.owner);
      for (const row of reserveFixture.pointerCases) {
        await page.setContent(`<div id="window" style="position:absolute;inset:0"></div>${row.visible && row.tabs ? '<div id="panel" style="position:absolute;left:100px;top:100px;width:300px;height:300px;z-index:30"></div>' : ""}`);
        await page.evaluate(() => {
          document.querySelector("#window")!.addEventListener("pointerdown", () => document.body.dataset.activated = "true", { capture: true });
        });
        await page.mouse.click(250, 250);
        expect(await page.evaluate(() => document.body.dataset.activated === "true"), JSON.stringify(row)).toBe(row.activates);
      }
    } finally {
      await browser.close();
    }
  }, 30000);
});
