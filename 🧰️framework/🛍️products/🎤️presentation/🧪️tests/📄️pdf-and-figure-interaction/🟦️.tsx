/** 📄️ The presentation react deck, interaction DOM of PDF and figure dispositions: cover scale, enlarged sizing, page navigation and cropped figure enlarge. */
import { split } from "@semio-tech/presentation";
import type { Presentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { SLIDE_INTERACTIVE_ENLARGE_FRAME, mountPresentation, pdfScrollCoverScale, resetPdfCanvasPort, setPdfCanvasPort, unmountPresentation } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { globalsCssSource } from "../🧰️presentation-deck-helpers/🟦️.tsx";
import { mockClientRect, pointerClick, testPdfCanvasPort, waitForPdfCanvas } from "../🧱️interaction-dom-kit/🟦️.ts";

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

  it("scales pdf pages to cover the disposition frame", async () => {
    const deck: Presentation = {
      id: "pdf-cover",
      name: "Pdf Cover",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(frame, 96, 385, 768, 280);
    const expected = pdfScrollCoverScale(768, 280, 595, 842);
    expect(expected).not.toBeNull();
    const page = await waitForPdfCanvas(disposition, (canvas) => Math.abs(Number(canvas.dataset.scale) - (expected ?? 0)) < 0.01);
    expect(Number(page.dataset.scale)).toBeCloseTo(expected ?? 0);
  });

  it("drops nested pdf frame and uses enlarged slide sizing when toggling enlarge", async () => {
    const deck: Presentation = {
      id: "pdf-enlarge",
      name: "Pdf Enlarge",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 96, 385, 768, 280);
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.querySelector(".presentation-disposition-frame")).toBeTruthy();
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    mockClientRect(content, 48, 52, 864, 595);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
    expect(disposition.querySelector(".presentation-disposition-frame")).toBeNull();
    const pageCanvas = await waitForPdfCanvas(disposition, (canvas) => canvas.height > 400);
    expect(pageCanvas.height).toBeGreaterThan(400);
    expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition--kind-pdf\.presentation-interactive-disposition--enlarged[\s\S]*\.presentation-media-pdf-document[\s\S]*height\s*:\s*100%/s);
  });

  it("shows center-bottom pdf page nav when enlarged and switches pages", async () => {
    const deck: Presentation = {
      id: "pdf-page-nav",
      name: "Pdf Page Nav",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 96, 385, 768, 280);
    act(() => {
      pointerClick(disposition);
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(disposition.querySelector(".presentation-pdf-page-nav")).toBeTruthy();
    const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
    mockClientRect(content, 48, 52, 864, 595);
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    const nav = disposition.querySelector(".presentation-pdf-page-nav");
    expect(nav).toBeTruthy();
    const page = disposition.querySelector(".presentation-media-pdf canvas") as HTMLElement;
    expect(page.dataset.page).toBe("1");
    const nextButton = disposition.querySelector(".presentation-pdf-page-nav__button--next") as HTMLButtonElement;
    const prevButton = disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement;
    expect(prevButton.disabled).toBe(true);
    expect(nextButton.disabled).toBe(false);
    act(() => {
      nextButton.click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(disposition.querySelector(".presentation-media-pdf canvas")?.getAttribute("data-page")).toBe("2");
    expect((disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement).disabled).toBe(false);
  });

  it("navigates only within pdf pages declared on the embodiment", async () => {
    const deck: Presentation = {
      id: "pdf-page-subset",
      name: "Pdf Page Subset",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "media",
                  participants: [{ id: "thesis" }],
                  embodiments: [
                    {
                      kind: "pdf",
                      id: "thesis--doc",
                      src: "/thesis.pdf",
                      page: 1,
                      pages: [1, 12, 25],
                    },
                  ],
                  slides: [
                    {
                      arrangement: {
                        id: "media",
                        dispositions: [
                          {
                            participantId: "thesis",
                            embodimentId: "thesis--doc",
                            emphasis: "active",
                            position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
    const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(disposition, 96, 385, 768, 280);
    act(() => {
      pointerClick(disposition);
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    const nextButton = () => disposition.querySelector(".presentation-pdf-page-nav__button--next") as HTMLButtonElement;
    const prevButton = () => disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement;
    const pageNumber = () => disposition.querySelector(".presentation-media-pdf canvas")?.getAttribute("data-page");
    expect(pageNumber()).toBe("1");
    act(() => {
      const next = nextButton();
      pointerClick(next);
      next.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--gesturing")).toBe(false);
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(pageNumber()).toBe("12");
    act(() => {
      const next = nextButton();
      pointerClick(next);
      next.click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(pageNumber()).toBe("25");
    expect(nextButton().disabled).toBe(true);
    act(() => {
      prevButton().click();
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(pageNumber()).toBe("12");
  });

  it("toggles enlarge on a cropped figure tile disposition", () => {
    const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
    const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
    const deck: Presentation = {
      id: "split-enlarge",
      name: "Split Enlarge",
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
    const tile = container.querySelector("[data-disposition-id]") as HTMLElement;
    const canvas = tile.closest(".presentation-arrangement-canvas") as HTMLElement;
    act(() => {
      pointerClick(tile);
    });
    const enlargeButton = tile.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
    expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(false);
    expect(tile.style.position).toBe("absolute");
    expect(tile.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
    expect(tile.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
    expect(canvas.contains(tile)).toBe(true);
    const cropSlot = tile.querySelector(".presentation-morph-slot--figure") as HTMLElement | null;
    expect(cropSlot).toBeTruthy();
    expect(tile.querySelector(".presentation-media-figure")).toBeNull();
    expect(cropSlot?.style.backgroundImage).toContain("catalogue.png");
    const bgSize = cropSlot?.style.getPropertyValue("--presentation-figure-bg-size");
    expect(bgSize).toBeTruthy();
    expect(bgSize).not.toBe("100% 100%");
    expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition--enlarged:not\(\.presentation-interactive-disposition--offset\)[\s\S]*\.presentation-figure-crop-fill[\s\S]*width\s*:\s*100%\s*!important/s);
    act(() => {
      enlargeButton.click();
    });
    expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
  });
});
