/** 📄️ The site's document assumes no language: it names itself in English then German and carries no `lang` of its own.
 * @see ../../🏗️builder/🌐️vite/🟦️.ts */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";
import { STAFFAGE_SITE_HEADERS, STAFFAGE_SITE_HOST, staffageContentSecurityPolicy, staffageHostDocument } from "../../🏗️builder/🌐️vite/🟦️.ts";

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
  expect(staffageContentSecurityPolicy(html)).toContain("script-src 'self' 'sha256-");
  expect(staffageContentSecurityPolicy(html)).toContain("connect-src 'self'");
});

it("ships a source document without a lang attribute", () => {
  expect(readFileSync(resolve(import.meta.dirname, "../../🌐️.html"), "utf8")).not.toMatch(/<html[^>]*\slang=/u);
});
