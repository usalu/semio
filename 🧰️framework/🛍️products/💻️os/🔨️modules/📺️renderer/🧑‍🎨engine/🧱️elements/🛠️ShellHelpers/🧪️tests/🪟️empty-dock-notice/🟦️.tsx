// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "@tailwindcss/node";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium } from "playwright";
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { COMPOSE_WINDOW_TEMPLATE_MIME, Mode, uiI18n } from "@semio-tech/ui-react";
import { afterEach, expect, it, vi } from "vitest";
import schema from "../../🧬️schema/🪟️empty-dock-notice/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🪟️empty-dock-notice/🔣️.json" with { type: "json" };

afterEach(async () => {
  cleanup();
  vi.unstubAllGlobals();
  await uiI18n.changeLanguage("en");
});

it("validates the bilingual empty dock contract", () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

it("centers the actual localized Mode notice and wraps it without adding a window or control", async () => {
  vi.stubGlobal("ResizeObserver", class { observe(): void {} unobserve(): void {} disconnect(): void {} });
  vi.stubGlobal("MutationObserver", class { observe(): void {} disconnect(): void {} takeRecords(): MutationRecord[] { return []; } });
  const stylesPath = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css");
  const compiler = await compile(readFileSync(stylesPath, "utf8"), { base: dirname(stylesPath), from: stylesPath, onDependency: () => {} });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    for (const locale of fixture.locales) {
      if (locale.id !== "en" && locale.id !== "de") throw new Error("Invalid fixture locale");
      await uiI18n.changeLanguage(locale.id);
      const onTemplateDrop = vi.fn();
      const view = render(<Mode windows={[]} layout={{ kind: "stack", children: [] }} activeWindowId={null} onTemplateDrop={onTemplateDrop} />);
      const mode = view.container.querySelector<HTMLElement>('[data-slot="mode"]')!;
      expect(mode.querySelector('[data-slot="mode-empty"]')?.textContent).toBe(locale.text);
      expect(mode.querySelector('[role="tab"], [data-slot="window-body"], button')).toBeNull();
      const classes = [...new Set([mode, ...mode.querySelectorAll("[class]")].flatMap(node => [...node.classList]))];
      for (const viewport of fixture.viewports) {
        await page.setContent('<div class="semio-scope" style="width:' + viewport.width + 'px;height:' + viewport.height + 'px">' + mode.outerHTML + "</div>");
        await page.addStyleTag({ content: compiler.build(classes) });
        const observed = await page.locator('[data-slot="mode-empty"]').evaluate(element => {
          const style = getComputedStyle(element);
          const bounds = element.getBoundingClientRect();
          const range = document.createRange();
          range.selectNodeContents(element);
          const lines = [...range.getClientRects()].filter(rect => rect.width > 0);
          const first = lines[0]!;
          const last = lines.at(-1)!;
          return {
            text: element.textContent, fontSize: Number.parseFloat(style.fontSize), lineHeight: Number.parseFloat(style.lineHeight),
            padding: Number.parseFloat(style.paddingLeft), lines: lines.length, align: style.textAlign,
            horizontalError: Math.max(...lines.map(line => Math.abs(line.x + line.width / 2 - bounds.x - bounds.width / 2))),
            verticalError: Math.abs((first.y + last.bottom) / 2 - bounds.y - bounds.height / 2),
          };
        });
        expect(observed.text).toBe(locale.text);
        expect(observed.fontSize, JSON.stringify(observed)).toBe(fixture.fontSize);
        expect(observed.lineHeight).toBe(fixture.lineHeight);
        expect(observed.padding).toBe(fixture.padding);
        expect(observed.align).toBe("center");
        expect(observed.lines > 1).toBe(viewport.wraps);
        expect(observed.horizontalError).toBeLessThanOrEqual(fixture.centerTolerance);
        expect(observed.verticalError).toBeLessThanOrEqual(fixture.centerTolerance);
      }
      const body = mode.querySelector<HTMLElement>('[data-slot="mode-body"]')!;
      vi.spyOn(body, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 1280, 640));
      const payload = { windowKindId: "notice-test-window", templateId: "top" };
      const drop = new MouseEvent("drop", { bubbles: true, clientX: 640, clientY: 320 });
      Object.defineProperty(drop, "dataTransfer", { value: { getData: (type: string) => type === COMPOSE_WINDOW_TEMPLATE_MIME ? JSON.stringify(payload) : "" } });
      act(() => { mode.querySelector('[data-slot="mode-empty"]')!.dispatchEvent(drop); });
      expect(onTemplateDrop).toHaveBeenCalledTimes(1);
      expect(onTemplateDrop.mock.calls[0]?.[0]).toEqual(payload);
      cleanup();
    }
  } finally {
    await browser.close();
  }
}, 90_000);
