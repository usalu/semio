import policy from "../🧬️schema/📏️policy.json" with { type: "json" };
import { isAbsolute } from "node:path";
import type { Page } from "playwright";
import type { OpenedUiV1 } from "../🟦️.ts";

/** 🧬 Reads only the real same-origin Blob URL's response header; revoked or other URLs remain unknown. */
export async function openedBlobMimeV1(page: Page, url: string): Promise<string | null> {
  try {
    return await page.evaluate(async value => {
      const parsed = new URL(value);
      if (parsed.protocol !== "blob:" || parsed.origin !== location.origin) return null;
      const controller = new AbortController(), timer = setTimeout(() => controller.abort(), 1000);
      try {
        const response = await fetch(value, { signal: controller.signal });
        const mime = response.headers.get("content-type");
        await response.body?.cancel();
        return mime !== null && mime.length <= 256 ? mime : null;
      } catch { return null; }
      finally { clearTimeout(timer); }
    }, url);
  } catch { return null; }
}

/** 🧭 Adapts one real browser session to the owned human-interaction port. */
export function openedUiOwnerV1(page: Page, signal: AbortSignal): { ui: OpenedUiV1; finish(): Promise<void> } {
  let revoked = false;
  const active = new Set<Promise<unknown>>();
  const check = () => { if (revoked) throw Error("UI test port is closed"); signal.throwIfAborted(); };
  const one = async (selector: string) => {
    const locator = page.locator(selector);
    await locator.first().waitFor({ state: "visible", timeout: policy.selectorTimeoutMs });
    if (await locator.count() !== 1) throw Error("Interaction requires exactly one element");
    return locator;
  };
  const operations: OpenedUiV1 = {
    async observe(selector, attributes, readText = true) {
      check();
      const locator = page.locator(selector), count = await locator.count();
      if (count > policy.maxElements || attributes.length > policy.maxAttributes) throw Error("Observation exceeds its bound");
      const result = [];
      let total = 0;
      for (let index = 0; index < count; index++) {
        check();
        const element = locator.nth(index);
        const observed = await element.evaluate((node, { names, max, read }) => {
          const values: Record<string, string | null> = {};
          for (const name of names) {
            const value = node.getAttribute(name);
            if (value !== null && value.length > max) throw Error("Attribute exceeds its bound");
            values[name] = value;
          }
          const walker = read ? node.ownerDocument.createTreeWalker(node, NodeFilter.SHOW_TEXT) : null;
          let text = "", current: Node | null;
          while (walker !== null && (current = walker.nextNode()) !== null) {
            text += (current.nodeValue ?? "").slice(0, max + 1 - text.length);
            if (text.length > max) throw Error("Text exceeds its bound");
          }
          return { text, attributes: values };
        }, { names: [...attributes], max: policy.maxValueChars, read: readText }, { timeout: policy.selectorTimeoutMs });
        total += observed.text.length + Object.values(observed.attributes).reduce((sum, value) => sum + (value?.length ?? 0), 0);
        if (total > policy.maxObservationChars) throw Error("Observation exceeds aggregate bound");
        result.push({ ...observed, visible: await element.isVisible() });
      }
      check();
      return result;
    },
    async canvas(selector) {
      check();
      const target = await one(selector);
      return target.evaluate(node => {
        if (!(node instanceof HTMLCanvasElement) || node.width === 0 || node.height === 0) return null;
        const scratch = node.ownerDocument.createElement("canvas");
        scratch.width = 16; scratch.height = 16;
        const context = scratch.getContext("2d");
        if (context === null) return null;
        context.drawImage(node, 0, 0, 16, 16);
        const data = context.getImageData(0, 0, 16, 16).data, colors = new Set<string>();
        let visiblePixels = 0;
        for (let offset = 0; offset < data.length; offset += 4) {
          if (data[offset + 3] !== 0) { visiblePixels++; colors.add(data[offset] + "," + data[offset + 1] + "," + data[offset + 2] + "," + data[offset + 3]); }
        }
        return { width: node.width, height: node.height, visiblePixels, colors: colors.size };
      }, undefined, { timeout: policy.selectorTimeoutMs });
    },
    async click(selector) { check(); await (await one(selector)).click({ timeout: policy.selectorTimeoutMs }); check(); },
    async press(selector, key) { check(); await (await one(selector)).press(key, { timeout: policy.selectorTimeoutMs }); check(); },
    async chooseFiles(selector, paths) {
      check();
      if (paths.length === 0 || paths.length > policy.maxFiles || paths.some(path => !path || path.length > policy.maxPathChars || !isAbsolute(path))) throw Error("File selection exceeds its bound");
      const target = await one(selector);
      const [chooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: policy.selectorTimeoutMs }), target.click({ timeout: policy.selectorTimeoutMs })]);
      if (paths.length > 1 && !chooser.isMultiple()) throw Error("File chooser does not accept multiple files");
      await chooser.setFiles([...paths], { timeout: policy.selectorTimeoutMs });
      check();
    },
    async download(selector) {
      check();
      const target = await one(selector);
      const [download] = await Promise.all([page.waitForEvent("download", { timeout: policy.operationTimeoutMs }), target.click({ timeout: policy.selectorTimeoutMs })]);
      try {
      const mime = await openedBlobMimeV1(page, download.url());
      const stream = await download.createReadStream();
      if (stream === null) throw Error("Download has no readable content");
      const chunks: Uint8Array[] = [];
      let size = 0;
      for await (const chunk of stream) {
        check();
        if (chunk.byteLength > policy.maxReturnedBytes - size) { stream.destroy(); throw Error("Download exceeds its bound"); }
        const bytes = new Uint8Array(chunk);
        size += bytes.length;
        chunks.push(bytes);
      }
      const bytes = new Uint8Array(size);
      let offset = 0;
      for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.length; }
      check();
      return { name: download.suggestedFilename(), mime, bytes };
      } finally { await download.cancel().catch(() => undefined); await download.delete().catch(() => undefined); }
    },
  };
  const guarded = <T>(work: () => Promise<T>): Promise<T> => {
    check();
    return new Promise<T>((resolve, reject) => {
      let finished = false;
      const finish = (error?: unknown, value?: T) => {
        if (finished) return;
        finished = true;
        clearTimeout(timeout);
        signal.removeEventListener("abort", abort);
        if (error !== undefined) reject(error); else resolve(value as T);
      };
      let stopping = false;
      const operation = Promise.resolve().then(work);
      const stop = (reason: unknown) => {
        if (stopping) return;
        stopping = true;
        void (async () => {
          await page.close().catch(() => undefined);
          await operation.catch(() => undefined);
          finish(reason);
        })();
      };
      const abort = () => stop(signal.reason ?? Error("Cancelled"));
      const timeout = setTimeout(() => stop(Error("Interaction deadline exceeded")), policy.operationTimeoutMs);
      signal.addEventListener("abort", abort, { once: true });
      if (signal.aborted) abort();
      operation.then(value => { if (!stopping) finish(undefined, value); }, error => { if (!stopping) finish(error); });
    });
  };
  const track = <T>(work: () => Promise<T>): Promise<T> => {
    const pending = guarded(work);
    active.add(pending);
    void pending.then(() => active.delete(pending), () => active.delete(pending));
    return pending;
  };
  const ui: OpenedUiV1 = {
    observe: (selector, attributes, readText) => track(() => operations.observe(selector, attributes, readText)),
    canvas: selector => track(() => operations.canvas(selector)),
    click: selector => track(() => operations.click(selector)),
    press: (selector, key) => track(() => operations.press(selector, key)),
    chooseFiles: (selector, paths) => track(() => operations.chooseFiles(selector, paths)),
    download: selector => track(() => operations.download(selector)),
  };
  return { ui, async finish() { revoked = true; if (active.size) await page.close().catch(() => undefined); await Promise.allSettled([...active]); } };
}
