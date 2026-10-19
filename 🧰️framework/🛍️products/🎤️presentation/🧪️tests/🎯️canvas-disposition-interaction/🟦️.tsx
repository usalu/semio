/** 🎯️ The presentation react deck, interaction DOM of canvas dispositions: dragging, drag previews, resizing, chrome alignment and enlarging on the arrangement canvas. */
import { split } from "@semio-tech/presentation";
import type { Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { SLIDE_INTERACTIVE_ENLARGE_FRAME, mountPresentation, resetPdfCanvasPort, setPdfCanvasPort, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { mockClientRect, pointerClick, pointerDrag, positionedDeck, testPdfCanvasPort } from "../🧱️interaction-dom-kit/🟦️.ts";

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

  it("drags positioned disposition", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    expect(parseFloat(disposition.style.left)).toBeCloseTo(38.333, 1);
    expect(parseFloat(disposition.style.top)).toBeCloseTo(45.714, 1);
    expect(content.style.transform).toBe("");
  });

  it("keeps other canvas dispositions on their declared frames while one is dragged", () => {
    const twoBoxDeck: Presentation = {
      id: "interactive-dom-two",
      name: "Interactive DOM Two",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "placed",
                  participants: [{ id: "left" }, { id: "right" }],
                  embodiments: [
                    { kind: "text", id: "left--main", lines: ["Left"], level: "body" },
                    { kind: "text", id: "right--main", lines: ["Right"], level: "body" },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "placed",
                        dispositions: [
                          {
                            participantId: "left",
                            embodimentId: "left--main",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
                          },
                          {
                            participantId: "right",
                            embodimentId: "right--main",
                            emphasis: "active",
                            position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
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
      mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
    expect(dispositions).toHaveLength(2);
    const section = dispositions[0]!.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(dispositions[0]!, 96, 210, 288, 140);
    mockClientRect(dispositions[1]!, 576, 210, 288, 140);
    const peerBefore = dispositions[1]!.style.left;
    act(() => {
      pointerDrag(dispositions[0]!, 240, 280, 400, 320);
    });
    expect(parseFloat(dispositions[0]!.style.left)).toBeCloseTo(26.667, 1);
    expect(dispositions[1]!.style.left).toBe(peerBefore);
    expect(dispositions[0]!.querySelector<HTMLElement>(".presentation-interactive-disposition__content")?.style.transform).toBe("");
  });

  it("click-drags only the newly targeted disposition when another stays selected", () => {
    const twoBoxDeck: Presentation = {
      id: "interactive-dom-click-drag",
      name: "Interactive DOM Click Drag",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "placed",
                  participants: [{ id: "left" }, { id: "right" }],
                  embodiments: [
                    { kind: "text", id: "left--main", lines: ["Left"], level: "body" },
                    { kind: "text", id: "right--main", lines: ["Right"], level: "body" },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "placed",
                        dispositions: [
                          {
                            participantId: "left",
                            embodimentId: "left--main",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
                          },
                          {
                            participantId: "right",
                            embodimentId: "right--main",
                            emphasis: "active",
                            position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
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
      mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
    const section = dispositions[0]!.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(dispositions[0]!, 96, 210, 288, 140);
    mockClientRect(dispositions[1]!, 576, 210, 288, 140);
    act(() => {
      pointerClick(dispositions[0]!, 240, 280);
    });
    const leftBefore = dispositions[0]!.style.left;
    const rightBefore = dispositions[1]!.style.left;
    act(() => {
      pointerDrag(dispositions[1]!, 720, 280, 880, 320);
    });
    expect(dispositions[0]!.style.left).toBe(leftBefore);
    expect(dispositions[1]!.style.left).not.toBe(rightBefore);
  });

  it("shows tile disposition drag preview outside the declared frame without clipping", () => {
    const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
    const splitDeck: Presentation = {
      id: "interactive-dom-split-tiles",
      name: "Interactive DOM Split Tiles",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "tiles",
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
      mountPresentation(container, splitDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const tile = container.querySelector(".presentation-arrangement-canvas > .presentation-interactive-disposition[data-disposition-id]") as HTMLElement;
    expect(tile).toBeTruthy();
    const section = tile.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(tile, 96, 70, 384, 210);
    act(() => {
      pointerDrag(tile, 200, 140, 360, 260);
    });
    expect(parseFloat(tile.style.left)).toBeCloseTo(26.667, 1);
    expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
    expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    expect(getComputedStyle(tile).overflow).not.toBe("hidden");
    expect(tile.querySelector<HTMLElement>(".presentation-interactive-disposition__content")?.style.transform).toBe("");
  });

  it("resizes positioned disposition from se handle", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    act(() => {
      pointerClick(disposition);
    });
    const handle = disposition.querySelector(".presentation-interaction-handle--se") as HTMLElement;
    act(() => {
      pointerDrag(handle, 560, 340, 640, 400);
    });
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    expect(parseFloat(disposition.style.width)).toBeCloseTo(48.333, 1);
    expect(parseFloat(disposition.style.height)).toBeCloseTo(28.571, 1);
    expect(content.style.width).toBe("");
    expect(content.style.height).toBe("");
    const chrome = disposition.querySelector(".presentation-interactive-disposition__chrome") as HTMLElement;
    expect(chrome.isConnected).toBe(true);
    expect(chrome.querySelectorAll(".presentation-interaction-handle").length).toBe(8);
  });

  it("aligns canvas-framed chrome with the disposition wrapper on the arrangement canvas", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
    mockClientRect(section, 0, 0, 1200, 900);
    mockClientRect(canvas, 120, 100, 960, 700);
    const frameBox = { left: 408, top: 380, width: 384, height: 140 };
    mockClientRect(disposition, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
    mockClientRect(frame, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
    const chrome = disposition.querySelector(".presentation-interactive-disposition__chrome") as HTMLElement;
    mockClientRect(chrome, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
    const chromeRect = chrome.getBoundingClientRect();
    const wrapperRect = disposition.getBoundingClientRect();
    expect(chromeRect.left).toBeCloseTo(wrapperRect.left, 0);
    expect(chromeRect.top).toBeCloseTo(wrapperRect.top, 0);
    expect(chromeRect.width).toBeCloseTo(wrapperRect.width, 0);
    expect(chromeRect.height).toBeCloseTo(wrapperRect.height, 0);
  });

  it("toggles enlarge on a canvas-framed disposition", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
    const enlargeOn = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeOn.getAttribute("aria-pressed")).toBe("true");
    act(() => {
      enlargeOn.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    const enlargeOff = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeOff.getAttribute("aria-pressed")).toBe("false");
  });

  it("restores pre-enlarge frame after exit enlarge following drag", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    act(() => {
      pointerClick(disposition);
    });
    const declaredLeft = parseFloat(disposition.style.left);
    act(() => {
      pointerDrag(disposition, 300, 280, 400, 320);
    });
    const draggedLeft = parseFloat(disposition.style.left);
    expect(draggedLeft).not.toBeCloseTo(declaredLeft, 1);
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(parseFloat(disposition.style.left)).toBeCloseTo(draggedLeft, 1);
    expect(parseFloat(disposition.style.left)).not.toBeCloseTo(declaredLeft, 1);
  });
});
