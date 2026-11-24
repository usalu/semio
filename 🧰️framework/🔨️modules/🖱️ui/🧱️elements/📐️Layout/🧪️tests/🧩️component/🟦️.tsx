// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { compile } from "@tailwindcss/node";
import { fireEvent, render, waitFor } from "@testing-library/react";
import * as React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
// 🔁️ Through the package barrel, not the element file: `📐️Layout` → `🖼️Panel` → `🎯️targets/⚛️react`
// is a real runtime cycle, and entering it at the element leaves `PanelTabButton`'s own imports
// undefined in jsdom ("Element type is invalid … Check the render method of `PanelTabButton`").
import { Footer, Layout, Navbar, Toggle } from "@semio-tech/ui-react";
import reserveFixture from "../../../../🧪️tests/🧫️fixtures/🛟️panel-window-reservation/🔣️.json";
import scrollFixture from "./scroll-canvas.cases.json";
import bandsFixture from "./shell-borders.cases.json";
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

describe("Shell band borders", () => {
  it.each(bandsFixture.bands)("keeps the $slot seam in place on hover over a toggle", async ({ slot, edge, top }) => {
    const { width, height, toggleWidth } = bandsFixture;
    const groupRect = new DOMRect(0, top, toggleWidth, height);
    const bandRect = new DOMRect(0, top, width, height);
    const measure = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      if (this.dataset.slot === slot || this.hasAttribute("data-shared-border-group")) return bandRect;
      if (this.dataset.slot === "navbar-item" || this.dataset.slot?.startsWith("toggle-group")) return groupRect;
      return new DOMRect();
    });
    const items = [{ content: <Toggle id="test.shell.toggle" icon="x" /> }];
    const { container, getByRole, unmount } = render(slot === "navbar" ? <Navbar items={items} showFullscreenToggle={false} /> : <Footer items={items} />);
    try {
      const band = container.querySelector<HTMLElement>(`[data-slot="${slot}"]`)!;
      const toggle = getByRole("button");
      const seam = () => [...band.querySelectorAll(":scope > [data-shared-border-overlay] > [data-shared-border-segment]")];
      const control = () => band.querySelectorAll('[data-slot="toggle-group"] [data-shared-border-state="hover"]');
      expect(band.getAttribute("data-shared-border-edges")).toBe(edge);
      expect(band.hasAttribute("data-shared-border-hover")).toBe(true);
      await waitFor(() => expect(seam().some(path => path.getAttribute("data-shared-border-state") === "normal")).toBe(true));
      const centerline = () => new Set(seam().flatMap(path => [...(path.getAttribute("d") ?? "").matchAll(/[ML] ([\d.+-]+) ([\d.+-]+)/g)].map(point => Number(point[2]))));
      const normal = centerline();
      expect(normal.size).toBe(1);
      fireEvent.pointerOver(toggle);
      await waitFor(() => {
        expect(seam().filter(path => path.getAttribute("data-shared-border-state") === "hover").length).toBeGreaterThan(0);
        expect(control().length).toBeGreaterThan(0);
        expect(centerline()).toEqual(normal);
      });
      fireEvent.pointerOut(toggle);
      await waitFor(() => {
        expect(seam().every(path => path.getAttribute("data-shared-border-state") === "normal")).toBe(true);
        expect(centerline()).toEqual(normal);
      });
    } finally {
      unmount();
      measure.mockRestore();
    }
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

describe("Layout scroll canvas", () => {
  it("keeps tinted bars in reach as the canvas scrolls underneath them", async () => {
    const markup = scrollFixture.cases.map(({ canvasHeight, mobile, auxiliaryRows }) =>
      renderToStaticMarkup(
        <Layout
          scrollCanvas
          mobile={mobile}
          navbar={<Navbar items={[{ content: <button type="button">Navigation</button> }]} showFullscreenToggle={false} />}
          subnavbar={auxiliaryRows && <div data-test-subnavbar style={{ height: 22 }} />}
          footer={<Footer items={[{ content: <button type="button">Footer action</button> }]} />}
          subfooter={auxiliaryRows && <div style={{ height: 26 }}>Status</div>}
          canvas={<div data-test-canvas style={{ position: "relative", zIndex: 999, height: canvasHeight, backgroundColor: "red" }} />}
        />,
      ),
    );
    const ui = resolve(import.meta.dirname, "../../../..");
    const cssPath = resolve(ui, "🎨️styling/🖌️ui/🎨️.css");
    const compiler = await compile(readFileSync(cssPath, "utf8"), { base: dirname(cssPath), onDependency: () => {} });
    const classes = [...markup.join("").matchAll(/class="([^"]*)"/g)].flatMap((match) => match[1].split(" "));
    const css = compiler.build(classes) + readFileSync(resolve(ui, "🌐️globals/🎨️.css"), "utf8");
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      for (const [index, row] of scrollFixture.cases.entries()) {
        await page.setViewportSize({ width: row.width, height: row.height });
        await page.setContent(`<!doctype html><html class="${row.dark ? "dark" : ""}" lang="en"><head><style>${css}html,body{height:100%;margin:0}</style></head><body>${markup[index]}</body></html>`);
        const scrollport = page.locator('[data-slot="layout-scrollport"]');
        expect(await scrollport.count(), row.id).toBe(1);
        const read = () => scrollport.evaluate((port) => {
          const rect = (selector: string) => document.querySelector(selector)?.getBoundingClientRect().toJSON() as DOMRect | undefined;
          const navbar = rect('[data-slot="navbar"]')!;
          const footer = rect('[data-slot="footer"]')!;
          const aboveCanvas = (band: DOMRect, slot: string) => {
            const stack = document.elementsFromPoint(band.left + band.width / 2, band.top + band.height / 2);
            return Boolean(stack[0]?.closest(`[data-slot="${slot}"]`)) && stack.some((element) => element.hasAttribute("data-test-canvas"));
          };
          return {
            top: port.scrollTop,
            range: port.scrollHeight - port.clientHeight,
            port: port.getBoundingClientRect().toJSON() as DOMRect,
            navbar,
            subnavbar: rect("[data-test-subnavbar]"),
            footer,
            subfooter: rect('[data-slot="layout-subfooter"]'),
            canvas: rect("[data-test-canvas]")!,
            navbarAboveCanvas: aboveCanvas(navbar, "navbar"),
            footerAboveCanvas: aboveCanvas(footer, "footer"),
          };
        });
        const initial = await read();
        expect(initial.top, row.id).toBe(0);
        expect(initial.range > 0, row.id).toBe(row.scrolls);
        expect(Boolean(initial.subnavbar), row.id).toBe(row.auxiliaryRows);
        expect(Boolean(initial.subfooter), row.id).toBe(row.auxiliaryRows);
        expect(initial.navbar.top, row.id).toBeCloseTo(initial.port.top);
        expect(initial.canvas.top, row.id).toBeGreaterThanOrEqual((initial.subnavbar ?? initial.navbar).bottom);
        expect(initial.footer.bottom, row.id).toBeCloseTo(initial.port.bottom);
        expect(initial.subfooter?.top ?? row.height, row.id).toBeCloseTo(initial.footer.bottom);
        if (!row.scrolls) {
          expect(initial.canvas.bottom, row.id).toBeLessThanOrEqual(initial.footer.top);
          continue;
        }
        await scrollport.focus();
        expect(await scrollport.evaluate((port) => document.activeElement === port && port.tabIndex === 0), row.id).toBe(true);
        await page.keyboard.press("PageDown");
        await expect.poll(async () => (await read()).top, { message: row.id }).toBeGreaterThan(0);
        await page.keyboard.press("Home");
        await expect.poll(async () => (await read()).top, { message: row.id }).toBe(0);

        await page.mouse.move(row.width / 2, row.height / 2);
        await page.mouse.wheel(0, Math.floor(initial.range / 2));
        await expect.poll(async () => (await read()).top, { message: row.id }).toBeGreaterThan(0);
        const middle = await read();
        expect(middle.navbar.top, row.id).toBeCloseTo(initial.navbar.top);
        expect(middle.footer.bottom, row.id).toBeCloseTo(initial.footer.bottom);
        expect(middle.navbarAboveCanvas, row.id).toBe(true);
        expect(middle.footerAboveCanvas, row.id).toBe(true);

        const pixels = async () => {
          const png = await page.screenshot();
          return page.evaluate(async ({ source, points }) => {
            const image = await createImageBitmap(await (await fetch("data:image/png;base64," + source)).blob());
            const canvas = document.createElement("canvas");
            canvas.width = image.width;
            canvas.height = image.height;
            const context = canvas.getContext("2d")!;
            context.drawImage(image, 0, 0);
            return points.map(({ x, y }) => Array.from(context.getImageData(x, y, 1, 1).data));
          }, {
            source: png.toString("base64"),
            points: [middle.navbar, middle.footer].map(({ left, width, top, height }) => ({ x: Math.floor(left + width / 2), y: Math.floor(top + height / 2) })),
          });
        };
        const redTinted = await pixels();
        await page.locator("[data-test-canvas]").evaluate((canvas) => { canvas.style.backgroundColor = "blue"; });
        const blueTinted = await pixels();
        for (let band = 0; band < 2; band++) {
          expect(redTinted[band], row.id).not.toEqual([255, 0, 0, 255]);
          expect(blueTinted[band], row.id).not.toEqual([0, 0, 255, 255]);
          expect(redTinted[band][0], row.id).toBeGreaterThan(blueTinted[band][0]);
          expect(blueTinted[band][2], row.id).toBeGreaterThan(redTinted[band][2]);
        }

        await scrollport.evaluate((port) => { port.scrollTop = port.scrollHeight; });
        const end = await read();
        expect(end.top, row.id).toBe(end.range);
        expect(end.canvas.bottom, row.id).toBeLessThanOrEqual(end.footer.top + 1);
        expect(end.subfooter?.top ?? row.height, row.id).toBeCloseTo(end.footer.bottom);
      }
    } finally {
      await browser.close();
    }
  }, 30_000);
});

describe("Shared native panel overlay contract", () => {
  it("keeps the window canvas full width under every open panel", async () => {
    const { chromium } = await import("playwright");
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
it("shared UI examples have one neutral owner and no corpus contracts", async () => {
  const [{ existsSync }, { join }, { findWorkspaceRoot }, { default: corpus }] = await Promise.all([import("node:fs"), import("node:path"), import("../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"), import("../../../../🧪️tests/🧫️fixtures/🪪️fixture-ownership/🔣️.json")]);
  const root = findWorkspaceRoot(process.cwd());
  for (const row of corpus.cases) {
    expect(existsSync(join(root, row.fixture)), row.fixture).toBe(true);
    for (const removed of row.removed) expect(existsSync(join(root, removed)), removed).toBe(false);
  }
});
