/** 🧷️ The overlay scrollbar thumb: its geometry against the shared vectors, and the thumb of a mounted scroller that appears with an overflow, follows the scrolling and takes no room.
 *
 * @see ../../🧫️fixtures/🧷️scroll-thumb/🔣️.json
 */

import { act, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ScrollThumb, scrollThumbGeometry, scrollTopForThumb, type ScrollThumbInput } from "../../🎯️targets/⚛️react/🔨️modules/🧷️scroll-thumb/🟦️.tsx";
import fixture from "../../🧫️fixtures/🧷️scroll-thumb/🔣️.json";

interface Vector {
  readonly name: string;
  readonly input: ScrollThumbInput;
  readonly expected: { readonly size: number; readonly offset: number } | null;
}

describe("🧷️ scroll thumb geometry", () => {
  for (const vector of fixture.geometry as readonly Vector[]) {
    it(vector.name, () => {
      const geometry = scrollThumbGeometry(vector.input);
      if (vector.expected === null) return expect(geometry).toBeNull();
      expect(geometry?.size).toBeCloseTo(vector.expected.size, 3);
      expect(geometry?.offset).toBeCloseTo(vector.expected.offset, 3);
    });
  }
  for (const vector of fixture.thumbToScroll as readonly { readonly name: string; readonly input: ScrollThumbInput; readonly offset: number; readonly expected: number }[]) {
    it(`thumb to scroll: ${vector.name}`, () => expect(scrollTopForThumb(vector.input, vector.offset)).toBeCloseTo(vector.expected, 3));
  }
  it("round-trips: the scroll position of a thumb offset places the thumb there again", () => {
    const base: ScrollThumbInput = { scrollTop: 0, scrollHeight: 3000, clientHeight: 700, top: 30, bottom: 25, minimum: 24 };
    for (const scrollTop of [0, 100, 777, 2300]) {
      const placed = scrollThumbGeometry({ ...base, scrollTop });
      expect(scrollTopForThumb(base, placed!.offset)).toBeCloseTo(scrollTop, 3);
    }
  });
});

describe("🧷️ scroll thumb in a scroller", () => {
  const frame = (): Promise<void> => new Promise((resolve) => requestAnimationFrame(() => resolve()));
  it("shows only while the content overflows, follows the scrolling, is a glass chip that takes no room and is hidden from assistive technology", async () => {
    const { container } = render(
      <div data-testid="scroller" style={{ overflow: "auto" }}>
        <ScrollThumb />
        <p>content</p>
      </div>,
    );
    const scroller = container.firstElementChild as HTMLElement;
    const thumb = scroller.querySelector<HTMLElement>("[data-course-scroll-thumb]")!;
    const row = scroller.querySelector<HTMLElement>("[data-course-scroll-thumb-row]")!;
    expect(row.getAttribute("aria-hidden")).toBe("true");
    expect(row.style.height).toMatch(/^0(px)?$/u);
    expect(thumb.hasAttribute("data-glass-chip")).toBe(true);
    expect(thumb.style.borderRadius).toMatch(/^0(px)?$/u);
    expect(thumb.style.display).toBe("none");
    Object.defineProperty(scroller, "clientHeight", { value: 600, configurable: true });
    Object.defineProperty(scroller, "scrollHeight", { value: 1200, configurable: true });
    await act(async () => {
      scroller.dispatchEvent(new Event("scroll"));
      await frame();
    });
    expect(thumb.style.display).toBe("block");
    expect(thumb.style.height).toBe("300px");
    expect(thumb.style.transform).toBe("translateY(0px)");
    scroller.scrollTop = 600;
    await act(async () => {
      scroller.dispatchEvent(new Event("scroll"));
      await frame();
    });
    expect(thumb.style.transform).toBe("translateY(300px)");
    Object.defineProperty(scroller, "scrollHeight", { value: 600, configurable: true });
    await act(async () => {
      scroller.dispatchEvent(new Event("scroll"));
      await frame();
    });
    expect(thumb.style.display).toBe("none");
  });
});
