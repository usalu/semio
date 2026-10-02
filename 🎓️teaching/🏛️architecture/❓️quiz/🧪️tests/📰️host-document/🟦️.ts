/** 📰️ The site's document before the app runs assumes no language: no `lang` on its root, its title and loading text are
 * the catalog's own title in every language of the catalog (English, then German), a reader without scripts is told so
 * in both, and it adds no inline script or style block to the ones the shared host template ships. The footer's legal
 * links come from the deployment's `site.legal` and are unset until the owner supplies them.
 * @see ../../🏗️builder/🌐️vite/🟦️.ts — `quizHostDocument`
 * @see ../../🚀️deploy/🔣️.json — `site.legal`
 * @see https://www.w3.org/WAI/WCAG22/Understanding/language-of-parts.html */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { semioHostHtmlString } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { quizHostDocument } from "../../🏗️builder/🌐️vite/🟦️.ts";
import catalog from "../../🔣️.json" with { type: "json" };
import deployment from "../../🚀️deploy/🔣️.json" with { type: "json" };

const site = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const html = semioHostHtmlString(quizHostDocument);
const languages = Object.keys(catalog.title);
const blocks = (document: string): number => [...document.matchAll(/<(?:script|style)[\s>]/gu)].length;

describe("📰️ host document", () => {
  it("offers English first, then German, and names no language of its own", () => {
    expect(languages).toEqual(["en", "de"]);
    expect(html).toContain("<html>\n");
    expect(html).not.toMatch(/<html[^>]*\slang=/u);
    expect(readFileSync(resolve(site, "🌐️.html"), "utf8")).not.toMatch(/<html[^>]*\slang=/u);
  });

  it("is titled by the catalog in every language, before and while it loads", () => {
    const title = `${catalog.title.en} · ${catalog.title.de}`;
    expect(html).toContain(`<title>${title}</title>`);
    expect(readFileSync(resolve(site, "🌐️.html"), "utf8")).toContain(`<title>${title}</title>`);
    expect(html).toContain(`<span lang="en">${catalog.title.en}</span> · <span lang="de">${catalog.title.de}</span></div>`);
    expect(catalog.title.de).toBe("Quizze zu Architektur und Technologie");
  });

  it("tells a reader without scripts in both languages, visibly although the boot style hides the body", () => {
    const noscript = /<noscript>(.*?)<\/noscript>/su.exec(html)?.[1] ?? "";
    expect([...noscript.matchAll(/<p lang="(\w+)" style="visibility:visible;[^"]*">([^<]+)<\/p>/gu)].map((part) => [part[1], part[2]])).toEqual([
      ["en", "This site needs JavaScript."],
      ["de", "Diese Seite braucht JavaScript."],
    ]);
  });

  it("adds no inline script or style block to the ones of the shared host template", () => {
    expect(blocks(html)).toBe(blocks(semioHostHtmlString({ title: "plain", entry: "./🟦️.ts" })));
  });

  it("links only the legal pages the deployment names, which are unset until the owner supplies them", () => {
    const legal: Readonly<Record<string, unknown>> = deployment.site.legal;
    expect(Object.keys(legal).filter((key) => key !== "imprint" && key !== "privacy")).toEqual([]);
    for (const url of Object.values(legal)) expect(String(url)).toMatch(/^https:\/\/\S+$/u);
    expect(readFileSync(resolve(site, "🟦️.ts"), "utf8")).toContain("legal: site.legal");
  });
});
