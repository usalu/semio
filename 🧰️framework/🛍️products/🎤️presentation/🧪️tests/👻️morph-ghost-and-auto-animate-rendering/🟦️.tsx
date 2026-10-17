/** 👻️ The presentation react deck, rendering of auto-animate targets, settled state, many-to-one ghost frames, one-to-many source ghosts and slot labels. */
import type { AutoAnimateMatcherHost, Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { PRESENTATION_MANY_TO_ONE_MORPH_CLASS, clearRevealAutoAnimateInlineLayout, finalizeRevealAutoAnimateRestState, isManyToOneMorphTransition, isRevealSlideAutoAnimating, isSectionAutoAnimating, mountPresentation, prepareArrangementBeforeAutoAnimate, presentationAutoAnimateMatcher, readPresentationSlideIndicesFromUrl, relaxHiddenPreflight, slidesShareAutoAnimateId, syncArrangementSettledState, syncManyToOneGhostMorphFramesFromDom, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { source } from "../🧰️presentation-deck-helpers/🟦️.tsx";

describe("PresentationDeck", () => {
  let container: HTMLDivElement;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
  });

  afterEach(() => {
    unmountPresentation();
    container.remove();
  });

  it("clearRevealAutoAnimateInlineLayout removes only FLIP transform from auto-animate targets", () => {
    const deckEl = document.createElement("div");
    const target = document.createElement("div");
    target.dataset.autoAnimateTarget = "0";
    target.style.left = "65%";
    target.style.transform = "translate(10px, 20px) scale(2)";
    deckEl.appendChild(target);
    clearRevealAutoAnimateInlineLayout(deckEl);
    expect(target.style.left).toBe("65%");
    expect(target.style.transform).toBe("");
  });

  it("finalizeRevealAutoAnimateRestState clears running so morph sources rest hidden", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const slide = document.createElement("section");
    slide.classList.add("present");
    slide.setAttribute("data-auto-animate", "running");
    slide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
    const morphSource = document.createElement("div");
    morphSource.className = "presentation-target-ghost";
    morphSource.dataset.autoAnimateTarget = "0";
    slide.appendChild(morphSource);
    deckEl.appendChild(slide);
    document.body.appendChild(deckEl);
    finalizeRevealAutoAnimateRestState(deckEl);
    expect(slide.getAttribute("data-auto-animate")).toBe("");
    expect(slide.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
    expect(morphSource.hasAttribute("data-auto-animate-target")).toBe(false);
    deckEl.remove();
  });

  it("matches auto-animate targets only by data-id", () => {
    const fromSlide = document.createElement("section");
    fromSlide.innerHTML = '<div data-id="catalogue-col1" class="presentation-morph-slot--figure"><h2>Rippenplatte</h2></div>';
    const toSlide = document.createElement("section");
    toSlide.innerHTML = '<div data-id="catalogue-col1" class="presentation-morph-slot--label"><h2>Rippenplatte</h2></div>';
    const host: AutoAnimateMatcherHost = {
      findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
        for (const element of fromScope.querySelectorAll<HTMLElement>(selector)) {
          const key = serializer(element);
          const toElement = toScope.querySelector<HTMLElement>(`${selector}[data-id="${element.getAttribute("data-id")}"]`);
          if (toElement) {
            pairs.push({ from: element, to: toElement });
          }
        }
      },
    };
    const pairs = presentationAutoAnimateMatcher.call(host, fromSlide, toSlide);
    expect(pairs).toHaveLength(1);
    expect(pairs[0]?.from.getAttribute("data-id")).toBe("catalogue-col1");
    expect(pairs[0]?.from.classList.contains("presentation-morph-slot--figure")).toBe(true);
  });

  it("detects reveal slide auto-animate pending or running on section or deck", () => {
    const section = document.createElement("section");
    const deck = document.createElement("div");
    deck.className = "reveal";
    deck.appendChild(section);
    expect(isRevealSlideAutoAnimating(section)).toBe(false);
    expect(isSectionAutoAnimating(section)).toBe(false);
    section.setAttribute("data-auto-animate", "pending");
    expect(isSectionAutoAnimating(section)).toBe(true);
    expect(isRevealSlideAutoAnimating(section)).toBe(true);
    section.setAttribute("data-auto-animate", "");
    const other = document.createElement("section");
    other.setAttribute("data-auto-animate", "running");
    deck.appendChild(other);
    expect(isSectionAutoAnimating(section)).toBe(false);
    expect(isRevealSlideAutoAnimating(section)).toBe(true);
  });

  it("detects consecutive slides in the same auto-animate run", () => {
    const focus = document.createElement("section");
    focus.setAttribute("data-auto-animate-id", "medien--m0");
    const labels = document.createElement("section");
    labels.setAttribute("data-auto-animate-id", "medien--m0");
    const overview = document.createElement("section");
    overview.setAttribute("data-auto-animate-id", "medien--m1");
    expect(slidesShareAutoAnimateId(focus, labels)).toBe(true);
    expect(slidesShareAutoAnimateId(focus, overview)).toBe(false);
  });

  it("clears settled state when arriving on a slide and prepares it before morph to listed targets", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const focus = document.createElement("section");
    focus.setAttribute("title", "catalogue-focus");
    focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
    focus.classList.add("presentation-arrangement--positioned");
    deckEl.appendChild(focus);
    focus.classList.add("presentation-arrangement--settled");
    syncArrangementSettledState(deckEl, focus, null);
    expect(focus.classList.contains("presentation-arrangement--settled")).toBe(false);
    const labels = document.createElement("section");
    labels.setAttribute("title", "catalogue-labels");
    deckEl.appendChild(labels);
    prepareArrangementBeforeAutoAnimate(focus, labels);
    expect(focus.classList.contains("presentation-arrangement--settled")).toBe(true);
    expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
    expect(labels.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
  });

  it("syncManyToOneGhostMorphFramesFromDom uses live source tile geometry on target ghosts", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const focus = document.createElement("section");
    focus.setAttribute("title", "catalogue-focus");
    focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
    focus.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
    const labels = document.createElement("section");
    labels.setAttribute("title", "catalogue-labels");
    labels.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
    const source = document.createElement("div");
    source.className = "presentation-interactive-disposition presentation-interactive-disposition--canvas-framed";
    source.setAttribute("data-id", "Stütze");
    Object.assign(source.style, {
      position: "absolute",
      left: "20%",
      top: "10%",
      width: "15%",
      height: "70%",
    });
    const ghost = document.createElement("div");
    ghost.className = "presentation-interactive-disposition presentation-target-ghost presentation-interactive-disposition--canvas-framed";
    ghost.setAttribute("data-id", "Stütze");
    Object.assign(ghost.style, {
      position: "absolute",
      left: "60%",
      top: "44%",
      width: "24%",
      height: "12%",
    });
    const slot = document.createElement("div");
    slot.className = "presentation-morph-slot presentation-morph-slot--figure";
    slot.dataset.presentationMorphCrop = JSON.stringify({ x: 0, y: 0, width: 0.5, height: 1 });
    ghost.append(slot);
    focus.append(source);
    labels.append(ghost);
    deckEl.append(focus, labels);
    document.body.append(deckEl);
    syncManyToOneGhostMorphFramesFromDom(focus, labels);
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-left")).toBe("20%");
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-top")).toBe("10%");
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-width")).toBe("15%");
    expect(ghost.style.getPropertyValue("--presentation-morph-frame-height")).toBe("70%");
    expect(ghost.style.getPropertyValue("--presentation-frame-left")).toBe("60%");
    deckEl.remove();
  });

  it("does not mark many-to-one morph for catalogue to focus auto-animate", () => {
    const deckEl = document.createElement("div");
    deckEl.className = "reveal";
    const catalogue = document.createElement("section");
    catalogue.setAttribute("title", "catalogue");
    const focus = document.createElement("section");
    focus.setAttribute("title", "catalogue-focus");
    focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
    deckEl.append(catalogue, focus);
    expect(isManyToOneMorphTransition(catalogue, focus)).toBe(false);
    prepareArrangementBeforeAutoAnimate(catalogue, focus);
    expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
    expect(catalogue.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
  });

  it("renders expanded source ghosts for one-to-many morphTo", () => {
    const deck: Presentation = {
      id: "source-ghost",
      name: "Source ghost",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "split",
                  participants: [{ id: "whole" }, { id: "tile-a" }],
                  embodiments: [
                    { kind: "figure", id: "whole--figure", src: "/catalogue.png" },
                    {
                      kind: "figure",
                      id: "tile-a--figure",
                      src: "/catalogue.png",
                      crop: { x: 0, y: 0, width: 0.5, height: 1 },
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "whole",
                        dispositions: [
                          {
                            participantId: "whole",
                            embodimentId: "whole--figure",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.1, width: 0.8, height: 0.8 },
                            morphTo: [
                              {
                                participantId: "tile-a",
                                position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 },
                              },
                            ],
                          },
                        ],
                      },
                      transition: { kind: "morph" },
                    },
                    {
                      arrangement: {
                        id: "tiles",
                        dispositions: [
                          {
                            participantId: "tile-a",
                            embodimentId: "tile-a--figure",
                            emphasis: "active",
                            position: { x: 0.05, y: 0.2, width: 0.4, height: 0.6 },
                          },
                        ],
                      },
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    };
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const wholeSlide = container.querySelector('section[title="whole"]') as HTMLElement;
    const sourceGhost = wholeSlide.querySelector('.presentation-interactive-disposition.presentation-source-ghost[data-id="tile-a"]');
    expect(sourceGhost).toBeTruthy();
    expect(wholeSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
  });

  it("renders positioned labels with data-id on the morph slot frame", () => {
    const deck: Presentation = {
      id: "label-slot",
      name: "Labels",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "labels",
                  participants: [{ id: "catalogue-col1" }],
                  embodiments: [
                    {
                      kind: "text",
                      id: "catalogue-col1--label",
                      lines: ["Rippenplatte"],
                      level: "heading",
                      morphRoot: "heading-line",
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "labels",
                        dispositions: [
                          {
                            participantId: "catalogue-col1",
                            embodimentId: "catalogue-col1--label",
                            emphasis: "active",
                            position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
                          },
                        ],
                      },
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    };
    act(() => {
      mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const slot = container.querySelector('[data-id="catalogue-col1"].presentation-morph-slot--label');
    expect(slot?.querySelector("h2")?.textContent).toBe("Rippenplatte");
    expect(slot?.querySelector("h2[data-id]")).toBeNull();
  });

  it("relaxes Tailwind preflight [hidden] so reveal's inline display drives slide visibility", () => {
    const style = document.createElement("style");
    style.textContent = '[hidden]:where(:not([hidden="until-found"])) { display: none; color: red; }';
    document.head.appendChild(style);
    const hiddenRule = (style.sheet as CSSStyleSheet).cssRules[0] as CSSStyleRule;
    expect(hiddenRule.style.getPropertyValue("display")).toBe("none");
    relaxHiddenPreflight();
    expect(hiddenRule.style.getPropertyValue("display")).toBe("");
    expect(hiddenRule.style.getPropertyValue("color")).toBe("red");
    style.remove();
  });

  it("does not navigate from bookmark query params", () => {
    expect(readPresentationSlideIndicesFromUrl("#/?sequence=main&thought=intro&slide=goal")).toEqual({
      h: 0,
      v: 0,
    });
    expect(readPresentationSlideIndicesFromUrl("#/0/2?sequence=main&thought=intro&slide=goal")).toEqual({
      h: 0,
      v: 2,
    });
    history.replaceState(null, "", "/presentation");
  });
});
