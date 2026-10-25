/** ♻️ The presentation react deck, interaction DOM of leaving enlarge and resetting: the corner control, the icon hit target, kept enlarge, navigation and the reset controls. */
import { collectPresentationSlides } from "@semio-tech/presentation";
import Reveal from "reveal.js";
import { describe, expect, it, beforeEach, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { mountPresentation, resetPdfCanvasPort, setPdfCanvasPort, unmountPresentation } from "@semio-tech/presentation-react";
import { globalsCssSource } from "../🧰️presentation-deck-helpers/🟦️.tsx";
import { mockClientRect, pointerClick, pointerDrag, positionedDeck, testPdfCanvasPort, twoSlideDeck } from "../🧱️interaction-dom-kit/🟦️.ts";

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

  it("exits enlarge through the corner control while slide reset host is active", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(section.querySelector(".presentation-interaction-slide-reset-host")).toBeTruthy();
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
  });

  it("enlarges when pointerdown lands on the svg icon inside the enlarge button", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    const icon = enlargeButton.querySelector("svg");
    expect(icon).toBeTruthy();
    act(() => {
      pointerClick(icon!);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
  });

  it("keeps enlarge when empty slide click clears selection", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    act(() => {
      pointerClick(disposition);
    });
    const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    act(() => {
      enlargeButton.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    act(() => {
      pointerClick(canvas, 8, 8);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    const enlargeWhileDeselected = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
    expect(enlargeWhileDeselected).toBeTruthy();
    act(() => {
      enlargeWhileDeselected.click();
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
  });

  it("clears drag and selection after navigating away and back", async () => {
    const alphaRef = collectPresentationSlides(twoSlideDeck)[0];
    const betaRef = collectPresentationSlides(twoSlideDeck)[1];
    expect(alphaRef?.slide).toBe("alpha");
    expect(betaRef?.slide).toBe("beta");
    let revealApi: Reveal.Api | undefined;
    act(() => {
      mountPresentation(container, twoSlideDeck, {
        hash: false,
        slideNumber: false,
        surfaceChrome: false,
        onRevealReady: (api) => {
          revealApi = api;
        },
      });
    });
    await new Promise<void>((resolve) => {
      const start = Date.now();
      const wait = (): void => {
        if (revealApi) {
          resolve();
          return;
        }
        if (Date.now() - start > 5000) {
          throw new Error("reveal.js did not become ready.");
        }
        setTimeout(wait, 50);
      };
      wait();
    });
    const alphaSlide = container.querySelector('section[title="alpha"]') as HTMLElement;
    const disposition = alphaSlide.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    const originLeft = disposition.style.left;
    act(() => {
      pointerClick(disposition);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    const modifiedLeft = disposition.style.left;
    expect(modifiedLeft).not.toBe(originLeft);
    await revealApi!.slide(betaRef!.h, betaRef!.v);
    await new Promise((resolve) => setTimeout(resolve, 50));
    await revealApi!.slide(alphaRef!.h, alphaRef!.v);
    await new Promise((resolve) => setTimeout(resolve, 50));
    const alphaSlideAgain = container.querySelector('section[title="alpha"]') as HTMLElement;
    const dispositionAgain = alphaSlideAgain.querySelector("[data-disposition-id]") as HTMLElement;
    expect(dispositionAgain.style.left).toBe(originLeft);
    expect(dispositionAgain.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
  });

  it("resets the whole slide from the proximity control in the top-right corner", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    expect(globalsCssSource).toMatch(/\.presentation-interaction-slide-reset-host\s*\{[^}]*pointer-events:\s*none/s);
    expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition\s*>\s*\.presentation-interaction-actions\s*\{[^}]*z-index:\s*90/s);
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    const originLeft = disposition.style.left;
    expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    expect(disposition.style.left).not.toBe(originLeft);
    const slideResetHost = section.querySelector(".presentation-interaction-slide-reset-host") as HTMLElement;
    const slideReset = slideResetHost.querySelector(".presentation-interaction-slide-reset") as HTMLButtonElement;
    expect(slideResetHost).toBeTruthy();
    expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(false);
    act(() => {
      window.dispatchEvent(new PointerEvent("pointermove", { bubbles: true, clientX: 920, clientY: 20, pointerId: 2 }));
    });
    expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(true);
    act(() => {
      slideReset.click();
    });
    expect(disposition.style.left).toBe(originLeft);
    expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
  });

  it("resets a dragged canvas-framed disposition to its declared position", () => {
    act(() => {
      mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
    });
    const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
    const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
    const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
    mockClientRect(section, 0, 0, 960, 700);
    mockClientRect(canvas, 0, 0, 960, 700);
    mockClientRect(disposition, 192, 210, 384, 140);
    const originLeft = disposition.style.left;
    const originTop = disposition.style.top;
    act(() => {
      pointerDrag(disposition, 300, 280, 380, 320);
    });
    expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
    expect(disposition.style.left).not.toBe(originLeft);
    expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
    const reset = disposition.querySelector(".presentation-interaction-reset") as HTMLButtonElement;
    expect(reset).toBeTruthy();
    const actions = disposition.querySelector(".presentation-interaction-actions")!;
    const buttons = [...actions.querySelectorAll("button")];
    expect(buttons[0]?.classList.contains("presentation-interaction-reset")).toBe(true);
    expect(buttons[1]?.classList.contains("presentation-interaction-enlarge")).toBe(true);
    act(() => {
      reset.click();
    });
    expect(disposition.style.left).toBe(originLeft);
    expect(disposition.style.top).toBe(originTop);
    expect(disposition.querySelector(".presentation-interaction-reset")).toBeNull();
  });
});
