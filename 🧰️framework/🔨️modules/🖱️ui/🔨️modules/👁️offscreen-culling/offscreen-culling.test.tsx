/** 👁️ Off-screen culling: what lies a viewport or more away from the view is marked `data-offscreen` by one shared observer per root, so the stylesheet can drop its blur, its layers and its heavy shadows until it comes near again. */
import { render } from "@testing-library/react";
import * as React from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { OFFSCREEN_ATTRIBUTE, OFFSCREEN_MARGIN, observeOffscreen, useOffscreen } from "./🟦️.ts";

interface Seen {
  readonly callback: IntersectionObserverCallback;
  readonly options: IntersectionObserverInit | undefined;
  readonly watched: Set<Element>;
  disconnected: boolean;
}

const made: Seen[] = [];
const stops: (() => void)[] = [];

function watch(...args: Parameters<typeof observeOffscreen>): () => void {
  const stop = observeOffscreen(...args);
  stops.push(stop);
  return stop;
}

class FakeObserver {
  readonly seen: Seen;
  constructor(callback: IntersectionObserverCallback, options?: IntersectionObserverInit) {
    this.seen = { callback, options, watched: new Set(), disconnected: false };
    made.push(this.seen);
  }
  observe(element: Element): void {
    this.seen.watched.add(element);
  }
  unobserve(element: Element): void {
    this.seen.watched.delete(element);
  }
  disconnect(): void {
    this.seen.disconnected = true;
    this.seen.watched.clear();
  }
}

const report = (seen: Seen, element: Element, isIntersecting: boolean): void => seen.callback([{ target: element, isIntersecting } as IntersectionObserverEntry], seen as unknown as IntersectionObserver);

beforeEach(() => {
  made.length = 0;
  vi.stubGlobal("IntersectionObserver", FakeObserver);
});

afterEach(() => {
  for (const stop of stops.splice(0)) stop();
  vi.unstubAllGlobals();
  document.body.replaceChildren();
});

describe("observeOffscreen", () => {
  it("shares one observer among every element of a root, with a margin of a viewport on every side", () => {
    const [one, two, three] = [document.createElement("div"), document.createElement("div"), document.createElement("div")] as const;
    document.body.append(one, two, three);
    watch(one);
    watch(two);
    watch(three);
    expect(made).toHaveLength(1);
    expect(made[0]!.options).toEqual({ root: null, rootMargin: OFFSCREEN_MARGIN, threshold: 0 });
    expect(OFFSCREEN_MARGIN).toBe("100% 100% 100% 100%");
    expect([...made[0]!.watched]).toEqual([one, two, three]);
  });

  it("marks nothing until the observer has spoken, then marks what is away and unmarks what comes near", () => {
    const element = document.createElement("div");
    document.body.append(element);
    watch(element);
    expect(element.hasAttribute(OFFSCREEN_ATTRIBUTE)).toBe(false);
    report(made[0]!, element, false);
    expect(element.getAttribute(OFFSCREEN_ATTRIBUTE)).toBe("");
    report(made[0]!, element, true);
    expect(element.hasAttribute(OFFSCREEN_ATTRIBUTE)).toBe(false);
  });

  it("keeps one observer per root, because a root's margin is the only margin its clipped children get", () => {
    const rootA = document.createElement("section");
    const rootB = document.createElement("section");
    const inA = document.createElement("div");
    const inB = document.createElement("div");
    document.body.append(rootA, rootB, inA, inB);
    watch(inA, rootA);
    watch(inB, rootB);
    watch(document.createElement("div"), rootA);
    expect(made).toHaveLength(2);
    expect(made.map((seen) => seen.options?.root)).toEqual([rootA, rootB]);
  });

  it("leaves nothing behind: the mark goes, the element is no longer watched, and the last one out disconnects the observer", () => {
    const [one, two] = [document.createElement("div"), document.createElement("div")] as const;
    document.body.append(one, two);
    const stopOne = watch(one);
    const stopTwo = watch(two);
    report(made[0]!, one, false);
    stopOne();
    expect(one.hasAttribute(OFFSCREEN_ATTRIBUTE)).toBe(false);
    expect(made[0]!.watched.has(one)).toBe(false);
    expect(made[0]!.disconnected).toBe(false);
    stopTwo();
    expect(made[0]!.disconnected).toBe(true);
    watch(one);
    expect(made).toHaveLength(2);
  });

  it("counts an element watched twice, so the first to stop does not blind the second", () => {
    const element = document.createElement("div");
    document.body.append(element);
    const first = watch(element);
    const second = watch(element);
    first();
    expect(made[0]!.watched.has(element)).toBe(true);
    second();
    expect(made[0]!.watched.has(element)).toBe(false);
  });

  it("does nothing where there is no observer, and shows everything", () => {
    vi.stubGlobal("IntersectionObserver", undefined);
    const element = document.createElement("div");
    const stop = watch(element);
    expect(element.hasAttribute(OFFSCREEN_ATTRIBUTE)).toBe(false);
    expect(() => stop()).not.toThrow();
  });
});

const overviewOf = (element: HTMLElement): Element | null => element.closest("[data-overview]");

function Page(): React.ReactElement {
  const ref = useOffscreen(overviewOf);
  return <div ref={ref} data-page="" />;
}

describe("useOffscreen", () => {
  it("watches a mounted element against the root its caller names, and lets go when it unmounts", () => {
    const { container, unmount } = render(
      <section data-overview="">
        <Page />
        <Page />
      </section>,
    );
    const root = container.querySelector("[data-overview]")!;
    expect(made).toHaveLength(1);
    expect(made[0]!.options?.root).toBe(root);
    const pages = [...container.querySelectorAll("[data-page]")];
    report(made[0]!, pages[0]!, false);
    expect(pages[0]!.hasAttribute(OFFSCREEN_ATTRIBUTE)).toBe(true);
    expect(pages[1]!.hasAttribute(OFFSCREEN_ATTRIBUTE)).toBe(false);
    unmount();
    expect(made[0]!.disconnected).toBe(true);
  });
});
