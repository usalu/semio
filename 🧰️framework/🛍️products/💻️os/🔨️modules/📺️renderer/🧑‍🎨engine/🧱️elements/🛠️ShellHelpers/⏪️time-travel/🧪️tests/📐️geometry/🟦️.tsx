/** 📐️ The history-editing band and the reprojection status in a real browser under the real stylesheet: Chromium (Playwright)
 * lays out the shell's `subfooter` row from the styling module's own CSS, which Tailwind compiles the way the product build
 * does — for every class its own source scan finds (so every theme token a chrome class references is defined), plus the
 * classes of the server-rendered `Layout`, `Navbar`, `Footer`, docked `Panel`s, `TimeTravelBand` and
 * `HistoryReprojectionStatus` — at phone, tablet and desktop widths, in English and German, for one case of every stage of the shared band corpus
 * (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`) and for a document load's status. What a DOM without layout cannot
 * state: every control is the top element at its own centre and at least 24 × 24 CSS px (WCAG 2.2, 2.5.8), every band paints
 * an opaque surface, lies over neither the navbar, the canvas region nor the footer and under no docked panel's cap, and the
 * rows stack inside the viewport (live faults F2, F5, F8, O1 and O5 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "@tailwindcss/node";
import { Scanner } from "@tailwindcss/oxide";
import { chromium, type Browser } from "playwright";
import { createElement, Fragment, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { composeControlKeybindings, Footer, Layout, Navbar, UiKeybindingsProvider } from "@semio-tech/ui-react";
import type { HistoryPatch, HistoryTimeTravel } from "@semio-tech/framework";
import { syncShellLabelLocale } from "../../../🟦️.tsx";
import { HistoryReprojectionStatus, TimeTravelBand } from "../../🟦️.tsx";

const here = dirname(fileURLToPath(import.meta.url));
const shellHelpers = join(here, "..", "..", "..");
const framework = join(here, "..", "..", "..", "..", "..", "..", "..", "..", "..", "..");
const stylesPath = join(framework, "🔨️modules", "🖱️ui", "🎨️styling", "🖌️ui", "🎨️.css");
const corpus = JSON.parse(readFileSync(join(shellHelpers, "🧫️fixtures", "🧫️time-travel-band", "🔣️.json"), "utf8")) as {
  readonly controllerId: string;
  readonly axes: { readonly terminology: string };
  readonly cases: readonly { readonly name: string; readonly session: HistoryTimeTravel; readonly controls: readonly unknown[] }[];
};
const bandsClass = /<div data-semio-bottom-bands="" className="([^"]+)">/u.exec(readFileSync(join(shellHelpers, "..", "🏛️ShellHost", "🟦️.tsx"), "utf8"))![1]!;
const LOCALES = ["en", "de"] as const;
const VIEWPORTS = [
  { name: "phone", width: 375, height: 812, mobile: true, touch: true },
  { name: "tablet", width: 768, height: 1024, mobile: false, touch: true },
  { name: "desktop", width: 1440, height: 900, mobile: false, touch: false },
] as const;
type Viewport = (typeof VIEWPORTS)[number];
type Locale = (typeof LOCALES)[number];
type Scene = { readonly name: string; readonly controls: number; readonly bands: (locale: Locale) => ReactElement };

const stages = [...new Set(corpus.cases.map((entry) => entry.session.stage))];
const widest = [...corpus.cases].sort((left, right) => right.controls.length - left.controls.length)[0]!;
const band = (session: HistoryTimeTravel, locale: Locale): ReactElement => createElement(TimeTravelBand, { session, terminology: corpus.axes.terminology, locale, controllerId: corpus.controllerId, onAction: () => undefined });
const status = (reprojection: NonNullable<HistoryPatch["reprojection"]>, locale: Locale, sessionOpen: boolean): ReactElement => createElement(HistoryReprojectionStatus, { reprojection, locale, sessionOpen, controllerId: corpus.controllerId, onAction: () => undefined });
const SCENES: readonly Scene[] = [
  ...[widest, ...stages.map((stage) => corpus.cases.find((entry) => entry.session.stage === stage)!)].map((entry) => ({ name: `${entry.session.stage} (${entry.name})`, controls: entry.controls.length, bands: (locale: Locale) => band(entry.session, locale) })),
  { name: "a document load alone", controls: 1, bands: (locale) => status({ kind: "load", done: 1, total: 4 }, locale, false) },
  { name: "a session over a paused remote change", controls: widest.controls.length, bands: (locale) => createElement(Fragment, null, band(widest.session, locale), status({ kind: "remote", done: 2, total: 9, paused: true }, locale, true)) },
];

const bindings = composeControlKeybindings(new Map(), {});
const tabs = (owner: string) => ["settings", "history"].map((name, order) => ({ kind: "leaf" as const, id: `${owner}.${name}`, icon: () => null, name, order, trees: [] }));
const docked = (anchor: "bottom-left" | "bottom-right") => ({ visible: true, tabBarHost: "chrome" as const, size: 300, tabs: tabs(anchor) });

/** 🏗️ The shell around `bands`: navbar, the canvas/panels region with both bottom docks open (the merged panel on a phone),
 * footer, and the bands in the layout's subfooter row inside the wrapper ShellHost gives them. */
function shell(viewport: Viewport, bands: ReactElement): string {
  return renderToStaticMarkup(
    createElement(UiKeybindingsProvider, {
      bindings,
      children: createElement(Layout, {
        mobile: viewport.mobile,
        navbar: createElement(Navbar, { label: "Shell", items: [{ key: "title", content: "Navbar" }] }),
        footer: createElement(Footer, { items: [{ key: "status", content: "Footer status text" }] }),
        subfooter: createElement("div", { "data-semio-bottom-bands": "", className: bandsClass }, bands),
        panels: viewport.mobile ? undefined : { "bottom-left": docked("bottom-left"), "bottom-right": docked("bottom-right") },
        mobilePanel: viewport.mobile ? { visible: true, tabs: tabs("mobile") } : undefined,
        canvas: createElement("div", { "data-probe": "canvas", style: { height: "100%", overflow: "hidden" } }, "Canvas text ".repeat(400)),
      }),
    }),
  );
}

type Box = { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number; readonly width: number; readonly height: number };
const apart = (left: Box, right: Box): boolean => left.right <= right.left + 0.5 || right.right <= left.left + 0.5 || left.bottom <= right.top + 0.5 || right.bottom <= left.top + 0.5;
/** 🎨️ The opacity a computed colour paints with: its alpha, 1 when it states none. */
const alpha = (color: string): number => {
  const modern = /\/\s*([0-9.]+)(%?)\s*\)$/u.exec(color);
  if (modern) return Number(modern[1]) / (modern[2] === "%" ? 100 : 1);
  const legacy = /^rgba\(.*,\s*([0-9.]+)\)$/u.exec(color);
  return legacy ? Number(legacy[1]) : 1;
};

describe("📐️ the history-editing bands in a real browser under the real stylesheet", () => {
  let browser: Browser;
  let stylesheet: Awaited<ReturnType<typeof compile>>;
  beforeAll(async () => {
    stylesheet = await compile(readFileSync(stylesPath, "utf8"), { base: dirname(stylesPath), from: stylesPath, onDependency: () => {} });
    stylesheet.build(new Scanner({ sources: stylesheet.sources }).scan());
    browser = await chromium.launch({ headless: true });
  }, 120_000);
  afterAll(async () => {
    await browser?.close();
    syncShellLabelLocale("en");
  });

  for (const viewport of VIEWPORTS) {
    it(`lays every band out in its own row at ${viewport.name} width (${viewport.width} × ${viewport.height}): on top at every control, opaque, over no other row and under no docked panel`, async () => {
      const page = await browser.newPage({ viewport: { width: viewport.width, height: viewport.height } });
      try {
        for (const locale of LOCALES) {
          syncShellLabelLocale(locale);
          for (const scene of SCENES) {
            const markup = shell(viewport, scene.bands(locale));
            const classes = [...new Set(["semio-scope", "touch", ...[...markup.matchAll(/class="([^"]*)"/gu)].flatMap((found) => found[1]!.split(/\s+/u))])].filter((name) => name !== "");
            await page.setContent(`<style>${stylesheet.build(classes)}</style><div class="semio-scope${viewport.touch ? " touch" : ""}" style="width:100vw;height:100vh">${markup}</div>`);
            const measured = await page.evaluate(() => {
              const box = (element: Element) => {
                const rect = element.getBoundingClientRect();
                return { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom, width: rect.width, height: rect.height };
              };
              const onTop = (element: Element) => {
                const rect = element.getBoundingClientRect();
                const hit = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
                return hit !== null && element.contains(hit);
              };
              const rows = [...document.querySelector('[data-slot="layout"]')!.children];
              const slot = (row: Element) => row.getAttribute("data-slot") ?? row.querySelector(':scope > [data-slot="navbar"], :scope > [data-slot="footer"]')?.getAttribute("data-slot") ?? "region";
              return {
                slots: rows.map(slot),
                rows: rows.map(box),
                bands: [...document.querySelectorAll("[data-semio-time-travel], [data-semio-history-reprojection]")].map((surface) => ({ ...box(surface), background: getComputedStyle(surface).backgroundColor })),
                panels: [...document.querySelectorAll('[data-slot="panel"][data-panel-visible="true"]')].map((panel) => ({ anchor: panel.getAttribute("data-anchor") ?? panel.getAttribute("data-panel"), ...box(panel) })),
                controls: [...document.querySelectorAll("[data-semio-time-travel-control], [data-semio-history-reprojection-control]")].map((control) => ({ name: control.getAttribute("data-semio-time-travel-control") ?? control.getAttribute("data-semio-history-reprojection-control"), onTop: onTop(control), ...box(control) })),
                spacing: getComputedStyle(document.querySelector("[data-semio-bottom-bands]")!).getPropertyValue("--ui-spacing").trim(),
                viewport: { width: innerWidth, height: innerHeight },
              };
            });
            const where = `${viewport.name} ${locale}, ${scene.name}`;
            const [navbar, region, footer, row] = measured.rows as [Box, Box, Box, Box];
            expect(measured.slots, `${where}: navbar, region, footer, bands — one row each`).toEqual(["navbar", "region", "footer", "layout-subfooter"]);
            expect([navbar.bottom <= region.top + 0.5, region.bottom <= footer.top + 0.5, footer.bottom <= row.top + 0.5, row.bottom <= measured.viewport.height + 0.5, region.height > 0], `${where}: the rows stack top to bottom inside the viewport and the region keeps a height`).toEqual([true, true, true, true, true]);
            expect(measured.panels.map((panel) => panel.anchor), `${where}: the docks that are open`).toEqual(viewport.mobile ? ["mobilePanel"] : ["bottom-right", "bottom-left"]);
            for (const panel of measured.panels.filter((entry) => entry.anchor !== "mobilePanel")) expect([panel.bottom > region.bottom + 1, panel.bottom <= footer.bottom + 0.5], `${where}: the open ${panel.anchor} dock pulls its cap below the region, into the footer band`).toEqual([true, true]);
            expect(measured.bands.length, `${where}: its bands`).toBeGreaterThanOrEqual(1);
            for (const [index, surface] of measured.bands.entries()) {
              expect([surface.left >= 0, surface.right <= measured.viewport.width + 0.5, surface.top >= row.top - 0.5, surface.bottom <= row.bottom + 0.5], `${where}: band ${index} lies inside its row and the viewport`).toEqual([true, true, true, true]);
              expect([apart(surface, navbar), apart(surface, region), apart(surface, footer), ...measured.panels.map((panel) => apart(surface, panel))], `${where}: band ${index} lies over neither navbar, region nor footer and under no dock`).toEqual([true, true, true, ...measured.panels.map(() => true)]);
              expect([surface.background, alpha(surface.background)], `${where}: band ${index} paints an opaque surface`).toEqual([surface.background, 1]);
              for (const other of measured.bands.slice(index + 1)) expect(apart(surface, other), `${where}: bands ${index} and ${index + 1} do not overlap`).toBe(true);
            }
            expect(measured.controls.length, `${where}: its controls`).toBe(scene.controls);
            expect(measured.controls.filter((control) => !control.onTop).map((control) => control.name), `${where}: every control is the top element at its own centre`).toEqual([]);
            expect(measured.controls.filter((control) => control.width < 24 || control.height < 24).map((control) => [control.name, control.width, control.height]), `${where}: every control is at least 24 × 24 (spacing ${measured.spacing})`).toEqual([]);
            for (const [index, control] of measured.controls.entries()) for (const other of measured.controls.slice(index + 1)) expect(apart(control, other), `${where}: ${control.name} and ${other.name} do not overlap`).toBe(true);
          }
        }
      } finally {
        await page.close();
        syncShellLabelLocale("en");
      }
    }, 180_000);
  }
});
