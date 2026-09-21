// #region 🔌️Adapters
import { render } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it } from "vitest";
// 🔁️ Through the package barrel, not the element file: `📐️Layout` → `🖼️Panel` → `🎯️targets/⚛️react`
// is a real runtime cycle, and entering it at the element leaves `PanelTabButton`'s own imports
// undefined in jsdom ("Element type is invalid … Check the render method of `PanelTabButton`").
import { Layout, layoutPanelReserveStyle } from "@semio-tech/ui-react";
import { PANEL_DEFAULT_SIZE_PX } from "../../../🖼️Panel/🟦️.tsx";
import reserveFixture from "../../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🛟️panel-window-reservation/🔣️.json";
import reserveSchema from "../../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🛟️panel-window-reservation/🧬️schema/🔣️.json";
// #endregion 🔌️Adapters

type LayoutPanels = Parameters<typeof layoutPanelReserveStyle>[0];

// #region 🛟️PanelRailReserve
// 📐️generation3d at 1600×1000 (ticket 26/09/18, PB1 §4 / PB3 §4): the right-docked History panel
// spans x 1297–1597 at `z-index: 30` and the window's Actions rail x 1094–1394 at `z-index: 20`, so
// the rail's last 97 px — the centre of `addWidget`'s `kind` combobox at x 1311 among them — could
// not be clicked at all. A full-height rail cannot yield (`chromePanelSafeArea` answers clear for
// it, since neither axis clears the panel inside the window), so the LAYOUT reserves the band.
const leaf = (id: string) => ({ kind: "leaf" as const, id, label: id });

describe("Layout panel reserve", () => {
  it("reserves an open panel's own width on the side it is docked to", () => {
    expect(layoutPanelReserveStyle({ "bottom-right": { visible: true, size: 300, tabs: [leaf("framework.panel.history")] } })).toEqual({
      paddingLeft: undefined,
      paddingRight: "calc(300px + 2 * var(--spacing-single))",
    });
    expect(layoutPanelReserveStyle({ "left-middle": { visible: true, size: 240, tabs: [leaf("framework.panel.artifact")] } })).toEqual({
      paddingLeft: "calc(240px + 2 * var(--spacing-single))",
      paddingRight: undefined,
    });
  });

  it("reserves nothing for a folded panel, an empty anchor or a middle anchor with no edge of its own", () => {
    const cases: LayoutPanels[] = [
      {},
      { "bottom-right": { visible: false, size: 300, tabs: [leaf("framework.panel.history")] } },
      { "bottom-right": { visible: true, size: 300, tabs: [] } },
      { "top-middle": { visible: true, size: 300, tabs: [leaf("framework.panel.inspection")] } },
      { "bottom-middle": { visible: true, size: 300, tabs: [leaf("framework.panel.inspection")] } },
    ];
    for (const panels of cases) {
      expect(layoutPanelReserveStyle(panels)).toEqual({ paddingLeft: undefined, paddingRight: undefined });
    }
  });

  it("takes the widest open panel per side and defaults to the panel element's own size", () => {
    expect(
      layoutPanelReserveStyle({
        "top-right": { visible: true, size: 200, tabs: [leaf("a")] },
        "bottom-right": { visible: true, tabs: [leaf("b")] },
        "top-left": { visible: true, size: 420, tabs: [leaf("c")] },
        "bottom-left": { visible: true, size: 180, tabs: [leaf("d")] },
      }),
    ).toEqual({
      paddingLeft: "calc(420px + 2 * var(--spacing-single))",
      paddingRight: `calc(${PANEL_DEFAULT_SIZE_PX}px + 2 * var(--spacing-single))`,
    });
  });

  // 🪟️ The reserve is carried by the canvas column that hosts every window, and a shell with no open
  // panel keeps its authored layout byte-for-byte. Only the no-panel shell is rendered here: mounting
  // a folded `Panel` in jsdom trips `PanelTabButton` on the element/barrel import cycle, which is not
  // this law's subject — the applied-reserve half is `layoutPanelReserveStyle` above, and it is
  // measured live on 📐️generation3d (`🗑️generated/pb3-generation3d-*`).
  it("carries the reserve on the canvas column that hosts the windows, and reserves nothing without a panel", () => {
    const { container } = render(<Layout canvas={<div data-testid="canvas" />} />);
    const column = container.querySelector('[data-slot="layout-canvas-column"]') as HTMLElement;

    expect(column).not.toBeNull();
    expect(column.contains(container.querySelector('[data-testid="canvas"]'))).toBe(true);
    expect(column.style.paddingLeft).toBe("");
    expect(column.style.paddingRight).toBe("");
  });
});
// #endregion 🛟️PanelRailReserve

describe("Shared native panel reservation contract", () => {
  it("matches Chromium canvas geometry and overlay pointer ownership", async () => {
    const [{ chromium }, { default: Ajv }, { renderToStaticMarkup }] = await Promise.all([import("playwright"), import("ajv/dist/2020"), import("react-dom/server")]);
    const validate = new Ajv({ strict: true }).compile(reserveSchema);
    expect(validate(reserveFixture), JSON.stringify(validate.errors)).toBe(true);
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      for (const row of reserveFixture.cases) {
        const panels = Object.fromEntries(row.panels.map((panel) => [panel.anchor, { visible: panel.visible, size: panel.size, tabs: panel.tabs ? [leaf(panel.anchor)] : [] }])) as LayoutPanels;
        const style = row.width <= 767 ? {} : layoutPanelReserveStyle(panels);
        const markup = renderToStaticMarkup(<div id="column" style={{ boxSizing: "border-box", width: row.width, ...style }}><div id="canvas" style={{ height: 400 }} /></div>);
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
