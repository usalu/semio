import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { chromium } from "playwright";
import { expect, it } from "vitest";
import { downloadDataUrl } from "../../../🛠️ShellHelpers/🟦️.tsx";
import fixture from "../../🧫️fixtures/📥️download/🔣️.json";
import schema from "../../🧬️schema/📥️download/🔣️.json";

it("delivers the real icon data-url download through Chromium with exact UTF-8 bytes", async () => {
  expect(new Ajv().validate(schema, fixture)).toBe(true);
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ acceptDownloads: true });
    await page.route("https://download.test/**", route => route.fulfill({ contentType: "text/html", body: '<!doctype html><button id="export">Export</button>' }));
    await page.goto("https://download.test/");
    for (const sample of fixture.cases) {
      const dataUrl = `data:${sample.mime}${sample.encoding === "base64" ? ";base64" : ""},${sample.encoding === "base64" ? Buffer.from(sample.value).toString("base64") : encodeURIComponent(sample.value)}`;
      await page.evaluate(({ source, filename, dataUrl }) => {
        const download = new Function(`return (${source})`)() as (name: string, url: string) => void;
        document.querySelector<HTMLButtonElement>("#export")!.onclick = () => download(filename, dataUrl);
      }, { source: downloadDataUrl.toString(), filename: sample.filename, dataUrl });
      const received = page.waitForEvent("download");
      await page.getByRole("button", { name: "Export", exact: true }).click();
      const download = await received;
      const path = await download.path();
      expect(download.suggestedFilename()).toBe(sample.filename);
      expect(path).not.toBeNull();
      expect(readFileSync(path!)).toEqual(Buffer.from(sample.value, "utf8"));
      console.log(`[DEBUG] Chromium saved icon export ${sample.filename} with ${Buffer.byteLength(sample.value)} exact bytes`);
      await download.delete();
    }
  } finally { await browser.close(); }
}, 60_000);
