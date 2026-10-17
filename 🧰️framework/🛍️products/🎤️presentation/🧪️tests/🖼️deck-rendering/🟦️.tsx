/** 🖼️ The presentation react deck, rendering of intro slides: mounting `PresentationDeck`, the expanded intro slides, intro morph DOM and the auto-animate tagging of intro morph slides.
 *
 * @see ../🎥️media-and-tile-rendering/🟦️.tsx — media and tiles
 * @see ../✂️cropped-figure-and-sheet-rules/🟦️.tsx — cropped figures and auto-animate sheet rules
 * @see ../🔍️figure-scroll-and-zoom-rendering/🟦️.tsx — figure scroll and zoom math
 * @see ../👻️morph-ghost-and-auto-animate-rendering/🟦️.tsx — morph ghosts and auto-animate targets
 * @see ../🧰️presentation-deck-helpers/🟦️.tsx — the shared source locator and helpers
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { intro } from "@semio-tech/presentation";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { mountPresentation, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { globalsCssSource } from "../🧰️presentation-deck-helpers/🟦️.tsx";

describe("PresentationDeck", () => {
  let container: HTMLDivElement;

  const testAffiliationSteps = {
    steps: [
      [{ mark: "a", name: "Faculty" }],
      [
        { mark: "a", name: "Faculty" },
        { mark: "1", name: "Uni" },
      ],
      [
        { mark: "a", name: "Faculty" },
        { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
      ],
    ],
  } as const;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
  });

  afterEach(() => {
    unmountPresentation();
    container.remove();
  });

  it("renders expanded intro slides", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false });
    });
    const sections = container.querySelectorAll(".slides > section > section");
    expect(sections.length).toBe(7);
    expect(sections[0]?.hasAttribute("data-auto-animate")).toBe(true);
    const revealEl = container.querySelector(".reveal");
    expect(revealEl?.getAttribute("style")).toContain("100vw");
    expect(container.querySelector('[data-id^="title"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="description"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="goal"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="authors--"]')).toBeTruthy();
    expect(container.querySelector('[data-id^="institutions--"]')).toBeTruthy();
  });

  it("keeps intro flow slides centered while enabling interactive dispositions", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="einleitung--m0"]')) {
      expect(slide.classList.contains("presentation-arrangement--interactive")).toBe(true);
      expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
      expect(slide.classList.contains("presentation-arrangement--positioned")).toBe(false);
      expect(slide.querySelector(".presentation-arrangement-canvas")).toBeNull();
      expect(slide.querySelectorAll("[data-disposition-id]").length).toBeGreaterThan(0);
    }
    expect(globalsCssSource).toMatch(/\.presentation-arrangement--interactive\s*\{[^}]*overflow\s*:\s*visible/s);
    expect(globalsCssSource).toMatch(/\.presentation-arrangement-canvas\s*>\s*\.presentation-interactive-disposition\[data-id\][\s\S]*:not\(\s*\.presentation-interactive-disposition--gesturing\s*\)[\s\S]*overflow\s*:\s*hidden/s);
    expect(globalsCssSource).not.toMatch(/\.presentation-arrangement--interactive\s*\{[^}]*position\s*:\s*relative/s);
  });

  it("applies muted opacity on layered description slide", () => {
    const deck = intro({ language: "de", title: { full: ["A"], short: "Short" }, description: { full: ["D"], short: "D short" }, goal: ["G"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const descriptionSlide = container.querySelector('.slides > section > section[title="description"]');
    expect(descriptionSlide?.querySelector(".opacity-20")).toBeTruthy();
  });

  it("matches eg-ice-25 intro morph DOM per arrangement", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A", "B", "C"], short: "Short" },
      description: { full: ["D1", "D2"], short: "D short" },
      goal: ["G1"],
      authors: {
        lines: [
          [
            { name: "Alice Example", marks: ["a", "1", "x"] },
            { name: "Bob Beta", marks: ["a", "1", "x"] },
          ],
          [{ name: "Carol Creator" }],
        ],
      },
      affiliations: testAffiliationSteps,
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
    expect(slide("title")?.querySelectorAll('h2[data-id^="title"]').length).toBe(3);
    expect(slide("description")?.querySelector('h2[data-id="title"]')).toBeTruthy();
    expect(slide("description")?.querySelector('div[data-id="title"].presentation-disposition-frame')).toBeNull();
    expect(slide("description")?.querySelectorAll('h2[data-id^="description"]').length).toBe(2);
    expect(slide("goal")?.querySelector('h2[data-id="description"]')?.textContent).toBe("D short");
    expect(slide("goal")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
    expect(slide("description")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
    expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
    const authorLines = slide("authors")?.querySelectorAll('h4[data-id^="authors--"]');
    expect(authorLines?.length).toBe(3);
    expect(slide("authors")?.getAttribute("data-auto-animate-id")).toMatch(/^einleitung--/);
    expect(slide("authors")?.querySelector(".presentation-intro-line")?.className).toContain("gap-x-");
    expect(slide("affiliations-1")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(1);
    expect(slide("affiliations-2")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
    expect(slide("affiliations-3")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
    expect(slide("affiliations-3")?.querySelectorAll('[data-id^="institutions--"]').length).toBe(3);
    expect(slide("affiliations-2")?.querySelector('h5[data-id="institutions"]')).toBeNull();
    expect(slide("affiliations-2")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("Uni");
    expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
    expect(slide("affiliations-3")?.textContent).toContain("Chair X");
    expect(slide("affiliations-3")?.querySelector(".presentation-affiliation-morph-source")).toBeNull();
    expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.classList.contains("presentation-affiliation-row")).toBe(true);
    expect(slide("affiliations-3")?.querySelector('[data-id="institutions--x"]')?.textContent).toContain("Chair X");
    expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"] [data-id="institutions--x"]')).toBeTruthy();
    expect(slide("affiliations-1")?.querySelector('h4[data-id="authors--Alice Example"] sup')?.textContent).toBe("a");
    const marked2 = slide("affiliations-2")?.querySelector('h4[data-id="authors--Alice Example"] sup');
    expect(marked2?.textContent).toBe("a,1");
    expect(marked2?.querySelector("span:not(.opacity-20)")?.textContent).toBe("1");
    const marked3 = slide("affiliations-3")?.querySelector('h4[data-id="authors--Alice Example"] sup');
    expect(marked3?.textContent).toBe("a,1,x");
    expect(marked3?.querySelector("span:not(.opacity-20)")?.textContent).toBe("x");
    expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("A. Example");
    expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeTruthy();
    expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeNull();
    expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("Alice Example");
    const aff2 = slide("affiliations-2");
    expect(aff2?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
    expect(aff2?.querySelector('h4[data-id="institutions--1"] .opacity-20')).toBeNull();
    const aff3 = slide("affiliations-3");
    expect(aff3?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
    expect(aff3?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
    expect(aff3?.querySelector('[data-id="institutions--x"] .opacity-20')).toBeNull();
  });

  it("applies title and secondary morph text sizes on intro slides", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A", "B", "C"], short: "Short" },
      description: { full: ["D1", "D2"], short: "D short" },
      goal: ["G1"],
      authors: {
        lines: [[{ name: "Alice", marks: ["1"] }, { name: "Bob" }], [{ name: "Carol" }]],
      },
      affiliations: testAffiliationSteps,
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const expectMorphClass = (selector: string, sizeClass: string) => {
      for (const node of container.querySelectorAll(selector)) {
        expect(node.classList.contains("presentation-morph-text")).toBe(true);
        expect(node.classList.contains(sizeClass)).toBe(true);
      }
    };
    expectMorphClass('h1[data-id^="title"], h2[data-id^="title"]', "presentation-morph-text--title");
    expectMorphClass('h2[data-id^="description"], p[data-id^="description"], h2[data-id^="goal"], p[data-id^="goal"]', "presentation-morph-text--secondary");
    expect(container.querySelector('h2[data-id^="title"].presentation-morph-text--secondary')).toBeNull();
    expect(container.querySelector('h2[data-id^="goal"].presentation-morph-text--title')).toBeNull();
  });

  it("does not use reveal fit-text on intro headings", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B"], short: "Short" }, description: { full: ["D1"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    expect(container.querySelector(".r-fit-text")).toBeNull();
  });

  it("enables reveal auto-animate on intro morph slides with unmatched layering", () => {
    const deck = intro({
      language: "de",
      title: { full: ["A"], short: "Short" },
      description: { full: ["D"], short: "D short" },
      goal: ["G1", "G2"],
      authors: {
        lines: [
          [
            { name: "Alice", marks: ["a"] },
            { name: "Bob", marks: ["a"] },
          ],
          [{ name: "Carol", marks: ["a"] }],
        ],
      },
      affiliations: testAffiliationSteps,
    });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="einleitung--m0"]')) {
      expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
      expect(slide.hasAttribute("data-auto-animate")).toBe(true);
      expect(slide.querySelector(".presentation-arrangement-surface")).toBeNull();
    }
    expect(globalsCssSource).toMatch(/\.presentation-arrangement--intro:not\(\.presentation-arrangement--positioned\)[\s\S]*:is\(h1,\s*h2,\s*h3,\s*h4,\s*p\)[\s\S]*margin:\s*0/s);
    expect(globalsCssSource).not.toMatch(/data-auto-animate-id\^="intro--"/);
  });

  it("enables reveal auto-animate and tags every morph arrangement with data-auto-animate", () => {
    const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const morphSections = container.querySelectorAll('.slides > section > section[data-auto-animate][data-auto-animate-id="einleitung--m0"]');
    expect(morphSections.length).toBe(7);
    const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
    expect(slide("title")?.querySelector('h2[data-id^="title"]')).toBeTruthy();
    expect(slide("title")?.querySelector('[data-id^="description"]')).toBeNull();
    expect(slide("description")?.querySelector('h2[data-id^="description"]')).toBeTruthy();
    expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
  });
});
