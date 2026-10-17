/** 🖱️ The presentation react deck, interaction DOM of selection: the interactive wrappers and selecting on click on a mounted deck.
 *
 * @see ../🧱️interaction-dom-kit/🟦️.ts — the shared fixtures and pointer helpers
 * @see ../🌊️flow-disposition-interaction/🟦️.tsx — intro flow dispositions
 * @see ../🎯️canvas-disposition-interaction/🟦️.tsx — canvas dispositions
 * @see ../♻️enlarge-and-reset-interaction/🟦️.tsx — leaving enlarge and resetting
 * @see ../📄️pdf-and-figure-interaction/🟦️.tsx — PDF and figure dispositions
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { split } from "@semio-tech/presentation";
import type { Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { mountPresentation, resetPdfCanvasPort, setPdfCanvasPort, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { pointerClick, positionedDeck, testPdfCanvasPort } from "../🧱️interaction-dom-kit/🟦️.ts";

describe("presentation interaction dom", () => {
  let container: HTMLDivElement;

  beforeEach(() => {
    container = document.createElement("div");
    document.body.appendChild(container);
    setPdfCanvasPort(testPdfCanvasPort);
  });

  afterEach(() => {
    unmountPresentation();
    resetPdfCanvasPort();
    container.remove();
  });

  it("renders data-disposition-id on every disposition", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    expect(container.querySelectorAll("[data-disposition-id]").length).toBe(1);
  });

  it("renders one interactive wrapper per tile disposition", () => {
    const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
    const deck: Presentation = {
      id: "split-interactive",
      name: "Split Interactive",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "split",
                  participants: grid.participants,
                  embodiments: grid.embodiments,
                  slides: [
                    {
                      arrangement: {
                        id: "tiles",
                        dispositions: grid.dispositions,
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
    expect(container.querySelectorAll("[data-disposition-id]").length).toBe(4);
    expect(container.querySelectorAll(".presentation-interactive-row-band").length).toBe(0);
  });

  it("selects on click and deselects on empty slide click", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
    act(() => {
      pointerClick(layer);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    act(() => {
      pointerClick(canvas, 8, 8);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    const reveal = container.querySelector(".reveal") as HTMLElement;
    const background = document.createElement("div");
    background.className = "slide-background present";
    reveal.appendChild(background);
    act(() => {
      pointerClick(background, 8, 8);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
  });

  it("keeps selection when empty slide click started in reveal overview", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const reveal = container.querySelector(".reveal") as HTMLElement;
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    reveal.classList.add("overview");
    act(() => {
      pointerClick(layer);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    reveal.classList.remove("overview");
    act(() => {
      pointerClick(layer);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
  });
});
