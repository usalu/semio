/** 🧱️ Shared fixtures and pointer helpers of the interaction DOM suites: the test PDF port, the positioned and two-slide decks, click, drag, rect mocking and the PDF canvas wait. */
import type { PdfCanvasDocument, PdfCanvasPort, Presentation } from "@semio-tech/presentation-react";
import { act } from "@semio-tech/ui-react/test";

export const testPdfCanvasPort: PdfCanvasPort = {
  load() {
    const document: PdfCanvasDocument = {
      numPages: 3,
      getPage: async () => ({
        getViewport: ({ scale }) => ({ width: 595 * scale, height: 842 * scale }),
        render: () => ({ promise: Promise.resolve(), cancel: () => undefined }),
        cleanup: () => undefined,
      }),
      destroy: () => undefined,
    };
    return { promise: Promise.resolve(document), destroy: () => undefined };
  },
};

export const positionedDeck: Presentation = {
  id: "interactive-dom",
  name: "Interactive DOM",
  chapters: [
    {
      id: "main",
      sequences: [
        {
          id: "main",
          thoughts: [
            {
              id: "placed",
              participants: [{ id: "box" }],
              embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
              slides: [
                {
                  arrangement: {
                    id: "placed",
                    dispositions: [
                      {
                        participantId: "box",
                        embodimentId: "box--main",
                        emphasis: "active",
                        position: { x: 0.2, y: 0.3, width: 0.4, height: 0.2 },
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

export const twoSlideDeck: Presentation = {
  id: "interactive-two-slide",
  name: "Interactive Two Slide",
  chapters: [
    {
      id: "main",
      sequences: [
        {
          id: "main",
          thoughts: [
            {
              id: "pair",
              participants: [{ id: "box" }],
              embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
              slides: [
                {
                  arrangement: {
                    id: "alpha",
                    dispositions: [
                      {
                        participantId: "box",
                        embodimentId: "box--main",
                        emphasis: "active",
                        position: { x: 0.2, y: 0.3, width: 0.4, height: 0.2 },
                      },
                    ],
                  },
                },
                {
                  arrangement: {
                    id: "beta",
                    dispositions: [
                      {
                        participantId: "box",
                        embodimentId: "box--main",
                        emphasis: "active",
                        position: { x: 0.55, y: 0.55, width: 0.3, height: 0.15 },
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

export const pointerClick = (target: Element, clientX = 20, clientY = 20): void => {
  target.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, cancelable: true, button: 0, clientX, clientY }));
  target.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, cancelable: true, button: 0, clientX, clientY }));
};

export const mockClientRect = (element: Element, left: number, top: number, width: number, height: number): void => {
  const rect = {
    left,
    top,
    width,
    height,
    right: left + width,
    bottom: top + height,
    x: left,
    y: top,
    toJSON: () => ({}),
  };
  element.getBoundingClientRect = () => rect as DOMRect;
};

export const waitForPdfCanvas = async (root: Element, predicate: (canvas: HTMLCanvasElement) => boolean): Promise<HTMLCanvasElement> => {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const canvas = root.querySelector<HTMLCanvasElement>(".presentation-media-pdf canvas");
    if (canvas && predicate(canvas)) {
      return canvas;
    }
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
  }
  throw new Error("PDF canvas did not reach the expected state.");
};

export const pointerDrag = (target: Element, fromX: number, fromY: number, toX: number, toY: number, pointerId = 1): void => {
  target.dispatchEvent(
    new PointerEvent("pointerdown", {
      bubbles: true,
      cancelable: true,
      button: 0,
      clientX: fromX,
      clientY: fromY,
      pointerId,
    }),
  );
  window.dispatchEvent(
    new PointerEvent("pointermove", {
      bubbles: true,
      cancelable: true,
      clientX: toX,
      clientY: toY,
      pointerId,
    }),
  );
  window.dispatchEvent(
    new PointerEvent("pointerup", {
      bubbles: true,
      cancelable: true,
      clientX: toX,
      clientY: toY,
      pointerId,
    }),
  );
};
