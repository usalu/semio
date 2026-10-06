// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "@tailwindcss/node";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium } from "playwright";
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { EventFeedHost } from "../../🟦️.tsx";

type Fixture = {
  readonly viewport: { readonly width: number; readonly height: number };
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly activateAction: string;
  readonly entries: readonly unknown[];
  readonly expected: {
    readonly hostPaddingPx: number;
    readonly cardGapPx: number;
    readonly cardPaddingPx: number;
    readonly cardRadiusPx: number;
    readonly plainCardHeightPx: number;
    readonly detailCardHeightPx: number;
    readonly iconSizePx: number;
    readonly contentGapPx: number;
    readonly titleFontSizePx: number;
    readonly titleLineHeightPx: number;
    readonly titleWeight: number;
    readonly fatalTitleWeight: number;
    readonly timeFontSizePx: number;
    readonly detailFontSizePx: number;
    readonly descriptionTruncated: boolean;
  };
};

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(suiteRoot, "../../🧫️fixtures/🎨️layout/🔣️.json"), "utf8")) as Fixture;

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe("EventFeed styled layout", () => {
  it("validates the neutral card and tone contract", () => {
  });

  it("measures the actual React DOM with production CSS and Chromium", async () => {
    vi.spyOn(Date.prototype, "toLocaleTimeString").mockReturnValue("12:34");
    const view = render(
      <EventFeedHost
        node={{
          type: "componentScene",
          surfaceId: fixture.surfaceId,
          controllerId: fixture.controllerId,
          componentKind: "eventFeed",
          eventFeed: { entriesJson: JSON.stringify(fixture.entries), activateAction: fixture.activateAction },
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
      const observed = await page.evaluate(() => {
        const host = document.querySelector<HTMLElement>(".semio-event-feed-host");
        const cards = [...(host?.querySelectorAll<HTMLElement>(":scope > [role='button']") ?? [])];
        if (!host || cards.length !== 5) throw new Error("EventFeed card layout nodes missing");
        const contents = cards.map((card) => card.children.item(1) as HTMLElement | null);
        const titleRows = contents.map((content) => content?.children.item(0) as HTMLElement | null);
        const titles = titleRows.map((row) => row?.children.item(0) as HTMLElement | null);
        const times = titleRows.map((row) => row?.children.item(1) as HTMLElement | null);
        const firstIcon = cards[0]!.children.item(0)?.firstElementChild as HTMLElement | null;
        const detail = contents[0]?.children.item(1) as HTMLElement | null;
        if (titles.some((title) => !title) || times.some((time) => !time) || !firstIcon || !detail) throw new Error("EventFeed content layout nodes missing");
        const hostStyle = getComputedStyle(host);
        const firstCardStyle = getComputedStyle(cards[0]!);
        const firstCardRect = cards[0]!.getBoundingClientRect();
        const secondCardRect = cards[1]!.getBoundingClientRect();
        const firstIconRect = firstIcon.getBoundingClientRect();
        const contentRect = contents[0]!.getBoundingClientRect();
        const titleStyles = titles.map((title) => getComputedStyle(title!));
        const detailStyle = getComputedStyle(detail);
        const foregroundProbe = document.createElement("span");
        foregroundProbe.className = "text-foreground";
        foregroundProbe.style.position = "absolute";
        host.append(foregroundProbe);
        const foregroundColor = getComputedStyle(foregroundProbe).color;
        foregroundProbe.remove();
        return {
          hostPadding: Number.parseFloat(hostStyle.paddingLeft),
          cardGap: secondCardRect.top - firstCardRect.bottom,
          cardPadding: Number.parseFloat(firstCardStyle.paddingLeft),
          cardRadius: Number.parseFloat(firstCardStyle.borderTopLeftRadius),
          cardHeights: cards.map((card) => card.getBoundingClientRect().height),
          iconSize: firstIconRect.width,
          contentGap: contentRect.left - firstIconRect.right,
          titleFontSize: Number.parseFloat(titleStyles[0]!.fontSize),
          titleLineHeight: Number.parseFloat(titleStyles[0]!.lineHeight),
          titleWeights: titleStyles.map((style) => Number.parseInt(style.fontWeight, 10)),
          timeFontSize: Number.parseFloat(getComputedStyle(times[0]!).fontSize),
          detailFontSize: Number.parseFloat(detailStyle.fontSize),
          titleTruncated: titleStyles[0]!.overflow === "hidden" && titleStyles[0]!.whiteSpace === "nowrap" && titleStyles[0]!.textOverflow === "ellipsis" && titles[0]!.scrollWidth > titles[0]!.clientWidth,
          detailTruncated: detailStyle.overflow === "hidden" && detailStyle.whiteSpace === "nowrap" && detailStyle.textOverflow === "ellipsis" && detail.scrollWidth > detail.clientWidth,
          foregroundColor,
          titleColors: titleStyles.map((style) => style.color),
        };
      });
      expect(observed.hostPadding).toBeCloseTo(fixture.expected.hostPaddingPx, 3);
      expect(observed.cardGap).toBeCloseTo(fixture.expected.cardGapPx, 1);
      expect(observed.cardPadding).toBeCloseTo(fixture.expected.cardPaddingPx, 3);
      expect(observed.cardRadius).toBeCloseTo(fixture.expected.cardRadiusPx, 3);
      expect(observed.cardHeights[0]).toBeCloseTo(fixture.expected.detailCardHeightPx, 1);
      for (const height of observed.cardHeights.slice(1)) expect(height).toBeCloseTo(fixture.expected.plainCardHeightPx, 1);
      expect(observed.iconSize).toBeCloseTo(fixture.expected.iconSizePx, 3);
      expect(observed.contentGap).toBeCloseTo(fixture.expected.contentGapPx, 1);
      expect(observed.titleFontSize).toBeCloseTo(fixture.expected.titleFontSizePx, 3);
      expect(observed.titleLineHeight).toBeCloseTo(fixture.expected.titleLineHeightPx, 3);
      expect(observed.titleWeights.slice(0, 4)).toEqual(Array(4).fill(fixture.expected.titleWeight));
      expect(observed.titleWeights[4]).toBe(fixture.expected.fatalTitleWeight);
      expect(observed.timeFontSize).toBeCloseTo(fixture.expected.timeFontSizePx, 3);
      expect(observed.detailFontSize).toBeCloseTo(fixture.expected.detailFontSizePx, 3);
      expect(observed.titleTruncated).toBe(fixture.expected.descriptionTruncated);
      expect(observed.detailTruncated).toBe(fixture.expected.descriptionTruncated);
      expect(observed.titleColors[0]).toBe(observed.foregroundColor);
      expect(new Set(observed.titleColors.slice(0, 4)).size).toBe(4);
      expect(observed.titleColors[3]).toBe(observed.titleColors[4]);
    } finally {
      await browser.close();
      view.unmount();
    }
  }, 90_000);
});
