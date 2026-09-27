// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "@tailwindcss/node";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium } from "playwright";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { GraphTimelineHost } from "../../🟦️.tsx";

type Fixture = {
  readonly viewport: { readonly width: number; readonly height: number };
  readonly columns: readonly unknown[];
  readonly expected: {
    readonly hostPaddingPx: number;
    readonly rowHeightPx: number;
    readonly graphWidthPx: number;
    readonly graphColumnWidthPx: number;
    readonly labelTrackPx: number;
    readonly labelTrackTolerancePx: number;
    readonly labelTrackMaximumViewportFraction: number;
    readonly authorLeftPx: number;
    readonly avatarSizePx: number;
    readonly avatarOverlapPx: number;
    readonly mutationLevel: string;
    readonly descriptionInert: boolean;
  };
};

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../../../..");
const fixturePath = resolve(suiteRoot, "../../🧫️fixtures/🎨️layout/🔣️.json");
const schemaPath = resolve(suiteRoot, "../../🧬️schema/🎨️layout/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(schemaPath, "utf8"));

afterEach(() => cleanup());

describe("GraphTimeline styled layout", () => {
  it("validates the neutral layout contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("measures the actual React DOM with production CSS and Chromium", async () => {
    const view = render(
      <GraphTimelineHost
        node={{
          type: "componentScene",
          surfaceId: "vcs.layout.timeline",
          controllerId: "vcs-layout",
          componentKind: "graphTimeline",
          graphTimeline: { columnsJson: JSON.stringify(fixture.columns) },
        }}
        onAction={() => undefined}
      />,
    );
    const stylesPath = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css");
    const compiler = await compile(readFileSync(stylesPath, "utf8"), { base: dirname(stylesPath), from: stylesPath, onDependency: () => {} });
    const candidates = [...new Set([...view.container.querySelectorAll("[class]")].flatMap((element) => [...element.classList]))];
    const css = compiler.build(candidates).replace(/url\("\/🖼️assets\/([^"\n]+)"\)/gu, (_, path: string) => `url("data:font/woff2;base64,${readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖼️assets", path)).toString("base64")}")`);
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage({ viewport: fixture.viewport });
      await page.setContent(`<div class="semio-scope" style="width:${fixture.viewport.width}px;height:${fixture.viewport.height}px">${view.container.innerHTML}</div>`);
      await page.addStyleTag({ content: css });
      await page.evaluate(() => document.fonts.ready);
      const observed = await page.evaluate((mutationLevel) => {
        const host = document.querySelector<HTMLElement>(".semio-graph-timeline-host");
        const button = document.querySelector<HTMLElement>('[role="button"][aria-label="feature-hosts"]');
        if (!host || !button) throw new Error("layout contract host/button missing");
        const labels = button.children.item(0) as HTMLElement | null;
        const graph = button.children.item(1) as HTMLElement | null;
        const grid = button.parentElement;
        const svg = grid?.querySelector<SVGSVGElement>("svg");
        const avatars = [...button.querySelectorAll<HTMLElement>('[data-slot="avatar"]')];
        const description = [...(grid?.querySelectorAll<HTMLElement>("span") ?? [])].find((node) => node.textContent === "wire up TableHost story");
        const badge = [...(description?.parentElement?.querySelectorAll<HTMLElement>("span") ?? [])].find((node) => node.textContent === mutationLevel);
        if (!labels || !graph || !grid || !svg || avatars.length < 2 || !description || !badge) throw new Error("layout contract geometry nodes missing");
        const hostRect = host.getBoundingClientRect();
        const buttonRect = button.getBoundingClientRect();
        const labelsRect = labels.getBoundingClientRect();
        const graphRect = graph.getBoundingClientRect();
        const svgRect = svg.getBoundingClientRect();
        const firstAvatar = avatars[0]!.getBoundingClientRect();
        const secondAvatar = avatars[1]!.getBoundingClientRect();
        return {
          hostPaddingLeft: Number.parseFloat(getComputedStyle(host).paddingLeft),
          contentLeft: buttonRect.left - hostRect.left,
          rowHeight: buttonRect.height,
          labelTrackWidth: labelsRect.width,
          graphColumnWidth: graphRect.width,
          graphWidth: svgRect.width,
          selectableWidth: buttonRect.width,
          authorLeft: firstAvatar.left - graphRect.left,
          avatarSize: firstAvatar.width,
          avatarOverlap: firstAvatar.right - secondAvatar.left,
          descriptionLeft: description.parentElement!.getBoundingClientRect().left - hostRect.left,
          mutationLevel: badge.textContent,
          badgeUppercase: getComputedStyle(badge).textTransform,
          descriptionInert: description.closest('[role="button"]') === null,
        };
      }, fixture.expected.mutationLevel);
      console.info("[DEBUG] GraphTimeline styled reference", JSON.stringify(observed));
      expect(observed.hostPaddingLeft).toBeCloseTo(fixture.expected.hostPaddingPx, 3);
      expect(observed.contentLeft).toBeCloseTo(fixture.expected.hostPaddingPx, 1);
      expect(observed.rowHeight).toBeCloseTo(fixture.expected.rowHeightPx, 3);
      expect(observed.graphWidth).toBeCloseTo(fixture.expected.graphWidthPx, 3);
      expect(observed.graphColumnWidth).toBeCloseTo(fixture.expected.graphColumnWidthPx, 3);
      expect(observed.labelTrackWidth).toBeCloseTo(fixture.expected.labelTrackPx, 2);
      expect(observed.selectableWidth).toBeCloseTo(observed.labelTrackWidth + observed.graphColumnWidth, 3);
      expect(observed.labelTrackWidth).toBeLessThan(fixture.viewport.width * fixture.expected.labelTrackMaximumViewportFraction);
      expect(observed.authorLeft).toBeCloseTo(fixture.expected.authorLeftPx, 3);
      expect(observed.avatarSize).toBeCloseTo(fixture.expected.avatarSizePx, 3);
      expect(observed.avatarOverlap).toBeCloseTo(fixture.expected.avatarOverlapPx, 3);
      expect(observed.descriptionLeft).toBeCloseTo(fixture.expected.hostPaddingPx + observed.selectableWidth, 1);
      expect(observed.mutationLevel).toBe(fixture.expected.mutationLevel);
      expect(observed.badgeUppercase).toBe("uppercase");
      expect(observed.descriptionInert).toBe(fixture.expected.descriptionInert);
    } finally {
      await browser.close();
      view.unmount();
    }
  }, 90_000);
});
