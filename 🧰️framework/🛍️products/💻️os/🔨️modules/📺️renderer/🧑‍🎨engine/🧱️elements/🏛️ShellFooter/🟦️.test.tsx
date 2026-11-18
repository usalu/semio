import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { shellFooterCreditsCompact, shellFooterCreditsMenuItem, shellFooterNavbarItem } from "./🟦️.tsx";

const credit = { id: "credit", placement: "leading", caption: { en: "A project of", de: "Ein Projekt von" }, separator: { en: "and", de: "und" }, logos: [{ href: "https://a.example", src: "a.svg", alt: "A" }, { href: "https://b.example", src: "b.svg", alt: "B" }] } as never;

describe("shell footer credits", () => {
  it("shrinks to the logos on a phone and on a tablet, where the full row is wider than the viewport", () => {
    expect([shellFooterCreditsCompact(true, false), shellFooterCreditsCompact(false, true), shellFooterCreditsCompact(false, false)]).toEqual([true, true, false]);
  });

  it("drops caption and separator when compact and keeps every logo link", () => {
    const markup = (compact: boolean) => renderToStaticMarkup(shellFooterNavbarItem(credit, "en", compact).content as never);
    expect(markup(false)).toContain("A project of");
    expect(markup(false)).toContain("and");
    expect(markup(true)).not.toContain("A project of");
    expect(markup(true)).not.toContain("<span>and</span>");
    expect(markup(true).match(/<a /g)).toHaveLength(2);
  });

  it("folds every credit into one labelled footer button in both languages", () => {
    for (const [locale, label] of [["en", "A project of"], ["de", "Ein Projekt von"]] as const) {
      const item = shellFooterCreditsMenuItem([credit, credit], locale);
      const markup = renderToStaticMarkup(item.content as never);
      expect(item.key).toBe("footerCredits");
      expect(markup.match(/<button /g)).toHaveLength(1);
      expect(markup).toContain(`aria-label="${label} · ${label}"`);
    }
  });
});
