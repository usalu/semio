/** 🔗️ The presentation react deck, bookmarks and preview: syncing the slide url and rendering the JSON preview tree.
 *
 * @see ../🧰️presentation-deck-helpers/🟦️.tsx — the shared source locator and helpers
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { collectPresentationSlides, intro } from "@semio-tech/presentation";
import { describe, expect, it } from "vitest";
import { jsonPreview, readPresentationSlideIndicesFromUrl, renderJsonTree, syncPresentationSlideUrl } from "../../🎯️targets/⚛️react/🟦️.tsx";

describe("syncPresentationSlideUrl", () => {
  const sampleDeck = intro({
    language: "de",
    title: { full: ["A"], short: "Short" },
    description: { full: ["D"], short: "D short" },
    goal: ["G"],
    authors: { lines: [[{ name: "Alice" }]] },
    affiliations: {
      steps: [
        [{ mark: "a", name: "Faculty" }],
        [
          { mark: "a", name: "Faculty" },
          { mark: "1", name: "Uni" },
        ],
        [
          { mark: "a", name: "Faculty" },
          { mark: "1", name: "Uni" },
        ],
      ],
    },
  });

  it("uses German bookmark labels for de intro decks", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A"], short: "Short" },
      description: { full: ["D"], short: "D short" },
      goal: ["G"],
      authors: { lines: [[{ name: "Alice" }]] },
      affiliations: {
        steps: [
          [{ mark: "a", name: "Faculty" }],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
        ],
      },
    });
    history.replaceState(null, "", "/deck");
    for (const slide of collectPresentationSlides(deck)) {
      if (slide.slide === "Ziel") {
        syncPresentationSlideUrl(deck, { h: slide.h, v: slide.v });
        break;
      }
    }
    const params = new URLSearchParams(new URL(window.location.href).hash.split("?")[1] ?? "");
    expect(params.get("kapitel")).toBe("Hauptteil");
    expect(params.get("sequenz")).toBe("Einführung");
    expect(params.get("gedanke")).toBe("Einleitung");
    expect(params.get("folie")).toBe("Ziel");
    history.replaceState(null, "", "/deck");
  });

  it("writes chapter, sequence, thought, and slide bookmark params after the hash path", () => {
    history.replaceState(null, "", "/deck");
    const goalSlide = collectPresentationSlides(sampleDeck).find((slide) => slide.slide === "Ziel");
    syncPresentationSlideUrl(sampleDeck, { h: goalSlide!.h, v: goalSlide!.v });
    const url = new URL(window.location.href);
    expect(url.search).toBe("");
    expect(url.hash).toContain("folie=Ziel");
    history.replaceState(null, "", "/deck");
  });

  it("readPresentationSlideIndicesFromUrl ignores bookmark query params", () => {
    expect(readPresentationSlideIndicesFromUrl("#/1/3")).toEqual({ h: 1, v: 3 });
    expect(readPresentationSlideIndicesFromUrl("")).toEqual({ h: 0, v: 0 });
  });
});

describe("renderJsonTree", () => {
  it("renders nested object keys in preview metadata", () => {
    expect(jsonPreview({ item: { item_id: "x" }, tags: ["a", "b"] })).toBe("Object(2)");
    expect(jsonPreview(["alpha", "beta"])).toBe("Array(2)");
    expect(jsonPreview(null)).toBe("null");
  });

  it("accepts nested null and undefined property values", () => {
    expect(() =>
      renderJsonTree({
        price_amount: null,
        currency: undefined,
        nested: { value: null },
      }),
    ).not.toThrow();
  });
});
