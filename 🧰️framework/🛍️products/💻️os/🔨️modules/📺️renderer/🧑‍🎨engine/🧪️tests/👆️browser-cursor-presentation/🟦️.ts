import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { chromium, type Browser, type Page } from "playwright";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import fixture from "../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/👆️cursor/🧫️fixtures/🔣️.json";
import { BrowserFrameTransport, type BrowserFrameUiMessage, type BrowserFrameWorkerMessage, type BrowserFrameWorkerPort } from "../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { resolveWgpuBootDescriptor } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";

type CursorCase = { readonly cursor: string; readonly themeDark: boolean; readonly css: string };

class FakeWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];

  postMessage(message: BrowserFrameUiMessage): void {
    this.messages.push(message);
  }

  terminate(): void {}

  reply(message: BrowserFrameWorkerMessage): void {
    this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>);
  }
}

describe("👆️ browser cursor presentation", () => {
  const cases = fixture.cases as readonly CursorCase[];

  it("validates the complete cursor/theme vocabulary", () => {
    expect(new Set(cases.map(testCase => `${testCase.cursor}:${testCase.themeDark}`)).size).toBe(30);
  });

  it("retains every canonical CSS cursor through the production frame transport and page hook", () => {
    const worker = new FakeWorker();
    const received: string[] = [];
    const transport = new BrowserFrameTransport({
      worker,
      boot: {
        bindingsModuleUrl: "renderer.js",
        bindingsWasmUrl: "renderer_bg.wasm",
        canvas: {} as OffscreenCanvas,
        width: 800,
        height: 600,
        dpr: 2,
        locale: "en",
        descriptor: resolveWgpuBootDescriptor({ defaultVariant: "s" }),
        appearance: { preference: "", systemDark: false },
        platform: "MacIntel",
        storage: {},
      },
      setTimer: () => 1,
      clearTimer: () => {},
      onDirectives: directives => received.push(directives.cursor),
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    cases.forEach((testCase, index) => {
      const sequence = index + 1;
      transport.requestFrame();
      expect(transport.flush(sequence)).toBe(true);
      const batch = worker.messages.at(-1);
      expect(batch?.kind).toBe("batch");
      worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: sequence, generation: 0 });
      worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: sequence, generation: 0, cursor: testCase.css, fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
    });
    expect(received).toEqual(cases.map(testCase => testCase.css));
    const root = dirname(fileURLToPath(import.meta.url));
    const pageSource = (readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8") + "\n" + readFileSync(join(root, "../../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts"), "utf8"));
    expect(pageSource).toContain("canvas.style.cursor = cursor");
  });

  describe("Chromium CSS oracle", () => {
    let browser: Browser;
    let page: Page;

    beforeAll(async () => {
      browser = await chromium.launch({ headless: true });
      page = await browser.newPage();
      await page.setContent('<canvas></canvas><div id="react-reference"></div>');
      const styles = readFileSync(new URL("../../../../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css", import.meta.url), "utf8");
      await page.addStyleTag({ content: styles.replace(/^@import[^;]*;/gmu, "") });
    }, 60_000);

    afterAll(async () => {
      await browser?.close();
    });

    it("matches React Flow's connection-indicator handle to the distinct centered crosshair", async () => {
      const centered = cases.filter(testCase => testCase.cursor === "CrosshairCentered");
      expect(centered).toHaveLength(2);
      for (const testCase of centered) {
        const observed = await page.evaluate(({ css, themeDark }) => {
          document.documentElement.classList.toggle("dark", themeDark);
          const handle = document.createElement("div");
          handle.className = "react-flow__handle connectionindicator";
          document.body.append(handle);
          const canvas = document.querySelector("canvas")!;
          canvas.style.cursor = css;
          const computed = getComputedStyle(handle).cursor;
          const expected = getComputedStyle(canvas).cursor;
          handle.remove();
          return { computed, expected };
        }, testCase);
        expect(observed.computed).toBe(observed.expected);
      }
    });

    for (const testCase of cases) {
      it(`${testCase.cursor}/${testCase.themeDark ? "dark" : "light"}`, async () => {
        const observed = await page.evaluate(({ css, cursor, themeDark }) => {
          document.documentElement.classList.toggle("dark", themeDark);
          const canvas = document.querySelector("canvas")!;
          const reference = document.querySelector<HTMLElement>("#react-reference")!;
          const variable = "--cursor-" + cursor.replace(/([a-z])([A-Z])/gu, "$1-$2").toLowerCase();
          reference.style.cursor = `var(${variable})`;
          canvas.style.cursor = css;
          return { inline: canvas.style.cursor, computed: getComputedStyle(canvas).cursor, reference: getComputedStyle(reference).cursor, declared: getComputedStyle(reference).getPropertyValue(variable).trim() };
        }, testCase);
        const fallback = testCase.css.slice(testCase.css.lastIndexOf(",") + 1).trim();
        expect(observed.inline).not.toBe("");
        expect(observed.computed.endsWith(fallback)).toBe(true);
        expect(observed.declared).not.toBe("");
        expect(observed.computed).toBe(observed.reference);
      });
    }
  });
});
