/** 📄️ The site's document assumes no language: it names itself in English then German and carries no `lang` of its own.
 * @see ../../🏗️builder/🌐️vite/🟦️.ts */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";
import { staffageHostDocument } from "../../🏗️builder/🌐️vite/🟦️.ts";

it("states title, loading text and noscript in English first, then German", () => {
  for (const texts of [staffageHostDocument.title, staffageHostDocument.loading?.title, staffageHostDocument.noscript]) {
    expect(Array.isArray(texts)).toBe(true);
    expect((texts as readonly { readonly lang: string }[]).map((entry) => entry.lang)).toEqual(["en", "de"]);
  }
});

it("ships a source document without a lang attribute", () => {
  expect(readFileSync(resolve(import.meta.dirname, "../../🌐️.html"), "utf8")).not.toMatch(/<html[^>]*\slang=/u);
});
