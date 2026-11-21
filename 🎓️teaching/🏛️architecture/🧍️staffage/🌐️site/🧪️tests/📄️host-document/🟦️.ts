/** 📄️ The site's document assumes no language: it names itself in English then German and carries no `lang` of its own.
 * @see ../../🏗️builder/🌐️vite/🟦️.ts */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { JSDOM } from "jsdom";
import { expect, it } from "vitest";
import { STAFFAGE_SITE_HEADERS, STAFFAGE_SITE_HOST, staffageContentSecurityPolicy, staffageHostDocument, staffageReleaseHtml } from "../../🏗️builder/🌐️vite/🟦️.ts";
import { STALE_CHUNK_BOOT_SCRIPT, STALE_CHUNK_ENTRY_ID } from "../../🛟️recovery/🟦️.ts";

it("states title, loading text and noscript in English first, then German", () => {
  for (const texts of [staffageHostDocument.title, staffageHostDocument.loading?.title, staffageHostDocument.noscript]) {
    expect(Array.isArray(texts)).toBe(true);
    expect((texts as readonly { readonly lang: string }[]).map((entry) => entry.lang)).toEqual(["en", "de"]);
  }
});

it("publishes under staffage.semio-tech.com", () => {
  expect(STAFFAGE_SITE_HOST).toBe("staffage.semio-tech.com");
  expect(staffageHostDocument.cnameHost).toBe(STAFFAGE_SITE_HOST);
  expect(STAFFAGE_SITE_HEADERS).toContain("Cache-Control: no-cache");
  const html = `<meta charset="UTF-8" />\n<title>t</title>\n<script>boot()</script>`;
  expect(staffageContentSecurityPolicy(html)).toContain("script-src 'self' 'wasm-unsafe-eval' 'sha256-");
  expect(staffageContentSecurityPolicy(html)).toContain("connect-src 'self'");
});

it("lets the DWG encoder compile WebAssembly without allowing eval", () => {
  const scripts = staffageContentSecurityPolicy("<title>t</title>").split("; ").find((directive) => directive.startsWith("script-src "))!;
  expect(scripts.split(" ")).toContain("'wasm-unsafe-eval'");
  expect(scripts.split(" ")).not.toContain("'unsafe-eval'");
});

it("ships a source document without a lang attribute", () => {
  expect(readFileSync(resolve(import.meta.dirname, "../../🌐️.html"), "utf8")).not.toMatch(/<html[^>]*\slang=/u);
});

it("installs recovery before the entry and authorizes the exact inline script in CSP", () => {
  const html = staffageReleaseHtml(`<head><meta charset="UTF-8" /><title>t</title><script type="module" crossorigin src="/assets/entry.js"></script></head>`);
  const { document } = new JSDOM(html).window;
  const policy = document.querySelector('meta[http-equiv="Content-Security-Policy"]')!;
  const entry = document.getElementById(STALE_CHUNK_ENTRY_ID)!;
  const boot = [...document.scripts].find((script) => script.text === STALE_CHUNK_BOOT_SCRIPT)!;
  expect(entry.getAttribute("src")).toBe("/assets/entry.js");
  expect(policy.nextElementSibling).toBe(boot);
  expect(boot.compareDocumentPosition(entry)).toBe(document.DOCUMENT_POSITION_FOLLOWING);
  expect(policy.getAttribute("content")).toContain(`'sha256-${createHash("sha256").update(boot.text).digest("base64")}'`);
});
