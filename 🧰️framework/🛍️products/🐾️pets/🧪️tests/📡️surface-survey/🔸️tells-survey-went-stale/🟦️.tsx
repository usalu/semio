/** 🔸️ The tells survey went stale tests of the 📡️ surface survey suite. */
import { expect, it, vi } from "vitest";
import { SURVEY_ATTRIBUTES, watchSurvey } from "@semio-tech/pets-react";
import { hostWith, ledger, viewport } from "../🟣️harness/🟦️.tsx";

/** 🔹 Registers the tells survey went stale tests of the 📡️ surface survey suite. */
export function registerTellsSurveyWentStaleTests(): void {
  it("tells when a survey went stale, at once for what moves and soon for what changed, and leaves nothing behind", async () => {
    const live = { resize: 0, mutation: 0 };
    const resized: (() => void)[] = [];
    const observed: unknown[] = [];
    class Resizes {
      constructor(callback: () => void) {
        resized.push(callback);
      }
      observe(target: Element): void {
        live.resize += 1;
        observed.push(target);
      }
      unobserve(): void {}
      disconnect(): void {
        live.resize -= 1;
      }
    }
    class Mutations extends MutationObserver {
      override observe(target: Node, options?: MutationObserverInit): void {
        live.mutation += 1;
        observed.push(options);
        super.observe(target, options);
      }
      override disconnect(): void {
        live.mutation -= 1;
        super.disconnect();
      }
    }
    const pane = document.createElement("div");
    const layer = document.createElement("div");
    const button = document.createElement("button");
    document.body.append(pane, layer, button);
    const page = ledger(document);
    const view = ledger(window);
    const host = hostWith({ ResizeObserver: Resizes as unknown as typeof ResizeObserver, MutationObserver: Mutations });
    const stale = vi.fn<(urgent: boolean) => void>();
    const stop = watchSurvey(host, document, stale, { ignore: layer });
    expect(live).toEqual({ resize: 1, mutation: 1 });
    expect(observed).toEqual([document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: [...SURVEY_ATTRIBUTES] }]);
    expect(SURVEY_ATTRIBUTES).toEqual(["class", "hidden", "inert", "open", "data-concealed"]);
    expect(page.open()).toEqual(["focusin", "focusout", "scroll", "transitionend"]);
    expect(view.open()).toEqual(["resize"]);
    for (const options of [...page.options(), ...view.options()]) expect(options).toEqual({ capture: true, passive: true });
    page.close();
    view.close();
    stop();
    expect(live).toEqual({ resize: 0, mutation: 0 });

    const end = watchSurvey(host, document, stale, { ignore: layer });
    const scroll = new Event("scroll", { cancelable: true });
    pane.dispatchEvent(scroll);
    expect(stale.mock.calls).toEqual([[true]]);
    expect(scroll.defaultPrevented).toBe(false);
    window.dispatchEvent(new Event("resize"));
    resized.at(-1)!();
    expect(stale.mock.calls).toEqual([[true], [true], [true]]);
    stale.mockClear();
    button.focus();
    expect(stale.mock.calls.length).toBeGreaterThanOrEqual(1);
    expect(stale.mock.calls.every(([urgent]) => urgent === false)).toBe(true);
    stale.mockClear();
    button.blur();
    pane.dispatchEvent(new Event("transitionend", { bubbles: true }));
    expect(stale.mock.calls).toEqual([[false], [false]]);
    stale.mockClear();
    const pet = layer.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "g"));
    pet.dispatchEvent(new Event("transitionend", { bubbles: true }));
    layer.dispatchEvent(new Event("transitionend", { bubbles: true }));
    expect(stale, "the end of a transition inside the layer moves no surface").not.toHaveBeenCalled();
    document.body.dispatchEvent(new Event("transitionend", { bubbles: true }));
    expect(stale.mock.calls).toEqual([[false]]);
    pet.remove();
    await new Promise((resolve) => setTimeout(resolve, 0));
    stale.mockClear();

    const settle = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));
    pane.append(document.createElement("p"));
    await settle();
    expect(stale.mock.calls).toEqual([[false]]);
    stale.mockClear();
    for (const [name, value] of [["class", "open"], ["hidden", ""], ["inert", ""], ["open", ""], ["data-concealed", ""]] as const) {
      pane.setAttribute(name, value);
      await settle();
      expect(stale.mock.calls, name).toEqual([[false]]);
      stale.mockClear();
    }
    pane.setAttribute("title", "nothing that moves a box");
    pane.setAttribute("style", "color: red");
    layer.append(document.createElementNS("http://www.w3.org/2000/svg", "svg"));
    layer.firstElementChild!.setAttribute("class", "pet");
    await settle();
    expect(stale).not.toHaveBeenCalled();
    layer.append(document.createElement("i"));
    pane.append(document.createElement("i"));
    await settle();
    expect(stale.mock.calls).toEqual([[false]]);
    stale.mockClear();
    layer.setAttribute("inert", "");
    await settle();
    expect(stale.mock.calls).toEqual([[false]]);
    layer.removeAttribute("inert");
    await settle();
    stale.mockClear();

    end();
    pane.dispatchEvent(new Event("scroll"));
    window.dispatchEvent(new Event("resize"));
    button.focus();
    pane.append(document.createElement("p"));
    await settle();
    expect(stale).not.toHaveBeenCalled();
    expect(live).toEqual({ resize: 0, mutation: 0 });
  });
  it("passes over the greeting of a resize observer about a root that already has a size", () => {
    viewport(1000, 700);
    const notify: (() => void)[] = [];
    class Resizes {
      constructor(callback: () => void) {
        notify.push(callback);
      }
      observe(): void {}
      unobserve(): void {}
      disconnect(): void {}
    }
    const stale = vi.fn<(urgent: boolean) => void>();
    const stop = watchSurvey(hostWith({ ResizeObserver: Resizes as unknown as typeof ResizeObserver }), document, stale);
    notify[0]!();
    expect(stale).not.toHaveBeenCalled();
    notify[0]!();
    notify[0]!();
    expect(stale.mock.calls).toEqual([[true], [true]]);
    stop();
  });
  it("watches a part of the page by its own element and works without observers", () => {
    const part = document.createElement("section");
    document.body.append(part);
    const stale = vi.fn<(urgent: boolean) => void>();
    const stop = watchSurvey(hostWith(), part, stale);
    window.dispatchEvent(new Event("resize"));
    expect(stale.mock.calls).toEqual([[true]]);
    stop();
  });
}
