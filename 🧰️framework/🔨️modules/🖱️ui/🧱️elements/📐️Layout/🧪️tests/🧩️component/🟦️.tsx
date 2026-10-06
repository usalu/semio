// #region 🔌️Adapters
import { render } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it } from "vitest";
// 🔁️ Through the package barrel, not the element file: `📐️Layout` → `🖼️Panel` → `🎯️targets/⚛️react`
// is a real runtime cycle, and entering it at the element leaves `PanelTabButton`'s own imports
// undefined in jsdom ("Element type is invalid … Check the render method of `PanelTabButton`").
import { Layout } from "@semio-tech/ui-react";
import reserveFixture from "../../../../🧫️fixtures/🛟️panel-window-reservation/🔣️.json";
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

describe("Layout subfooter row", () => {
  it("keeps the subfooter in flow as the last row, directly under the footer — the footer stays next to the region whose docked panels reach into it — and renders no row without one", () => {
    for (const mobile of [false, true]) {
      const { container, unmount } = render(<Layout mobile={mobile} navbar={<div data-testid="navbar" />} footer={<div data-testid="footer" />} subfooter={<div data-testid="band" />} canvas={<div data-testid="canvas" />} />);
      const layout = container.querySelector('[data-slot="layout"]') as HTMLElement;
      const row = container.querySelector('[data-slot="layout-subfooter"]') as HTMLElement;
      const rows = Array.from(layout.children);
      const holding = (testId: string) => rows.findIndex((child) => child.querySelector(`[data-testid="${testId}"]`) !== null);
      expect([row.parentElement === layout, holding("navbar"), holding("canvas"), holding("footer"), holding("band"), rows.length], `mobile=${mobile}: navbar, middle, footer, subfooter — one row each, in this order`).toEqual([true, 0, 1, 2, 3, 4]);
      expect([row.className, rows[holding("canvas")]!.contains(row), rows[holding("canvas")]!.nextElementSibling === rows[holding("footer")]], `mobile=${mobile}: the row is in flow, outside the region the panels and the canvas share, and never between that region and the footer`).toEqual(["flex-shrink-0", false, true]);
      unmount();
    }
    const bare = render(<Layout footer={<div data-testid="footer" />} canvas={<div data-testid="canvas" />} />);
    expect(bare.container.querySelector('[data-slot="layout-subfooter"]')).toBeNull();
  });
});

describe("Shared native panel overlay contract", () => {
  it("keeps the window canvas full width under every open panel", async () => {
    const [{ chromium }, { default: Ajv }, { renderToStaticMarkup }] = await Promise.all([import("playwright"), import("ajv/dist/2020"), import("react-dom/server")]);
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

/** 🪪️ Keeps shared React and native vectors at their physical neutral owner. */
it("shared UI fixture ownership has no product copies and preserves schema validation", async () => {
  const [{ existsSync, readFileSync }, { join }, { findWorkspaceRoot }, { default: Ajv }, { default: Ajv2020 }, { default: corpus }, { default: schema }] = await Promise.all([import("node:fs"), import("node:path"), import("../../../../../🏃️process/🧭️routing/🟦️.ts"), import("ajv"), import("ajv/dist/2020"), import("../../../../🧫️fixtures/🪪️fixture-ownership/🔣️.json"), import("../../../../🧬️schema/🪪️fixture-ownership/🔣️.json")]);
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  const root = findWorkspaceRoot(process.cwd());
  for (const row of corpus.cases) {
    const ownerSchema = JSON.parse(readFileSync(join(root, row.schema), "utf8"));
    const vectors = JSON.parse(readFileSync(join(root, row.fixture), "utf8"));
    expect(ownerSchema.$id).toBe(row.schemaId);
    const oracle = new Ajv2020({ strict: true, allErrors: true }).compile(ownerSchema);
    expect(oracle(vectors), JSON.stringify(oracle.errors)).toBe(true);
    for (const removed of row.removed) expect(existsSync(join(root, removed)), removed).toBe(false);
  }
});
