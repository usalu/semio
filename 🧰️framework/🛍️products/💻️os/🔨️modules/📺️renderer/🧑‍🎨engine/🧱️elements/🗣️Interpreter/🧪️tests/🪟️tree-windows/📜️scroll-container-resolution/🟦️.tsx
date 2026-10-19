/** 📜️ The scroll container resolution and viewport measuring tests. */
import { stubExtent } from "../🧰️tree-windows-test-kit/🟦️.tsx";
import type { TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** 📏 Registers the scroll container resolution, overflow candidate and client box measuring tests. */
export function registerScrollContainerResolutionTests(kit: TreeWindowsTestKit): void {
  const { cleanup, createElement, expect, it, mountScrollable, node, renderWindowedTree, rowHeightPx, stubRect, treeItem, treeWindowContainersUnder, treeWindowScrollViewport, treeWindowViewportMetrics, windowedByKey } = kit;
  /** 🧯️ The defect 📓️w3-browser-verification.md §5 measured in three browser lanes: the observer bound
   * to `📜️Scrollable`'s INNER content div, whose height is its content's, so `scrollTop` was
   * permanently 0 and no scroll ever produced a new window request. The numbers below are the ones
   * read off the fem3d House panel (466/3722 against 3720/3720). */
  it("resolves the scroll container to the element that actually scrolls, never the unbounded Scrollable viewport", () => {
    const rendered = mountScrollable([{ key: "objects", total: 200 }], () => {});
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    const viewport = rendered.container.querySelector('[data-slot="scroll-area-viewport"]') as HTMLElement;
    stubExtent(area, 466, 3722);
    stubExtent(viewport, 3720, 3720);
    const windowed = rendered.container.querySelector("[data-tree-window-key]") as HTMLElement;
    expect(windowed).toBeTruthy();

    const resolved = treeWindowScrollViewport(windowed) as HTMLElement;

    expect(resolved).toBe(area);
    expect(resolved.scrollHeight).toBeGreaterThan(resolved.clientHeight);
    expect(viewport.scrollHeight - viewport.clientHeight).toBe(0);
  });
  it("falls back to a plain overflow ancestor, and to the page itself, for a tree outside any Scrollable", () => {
    const nodes = [
      node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2]),
      node(2, "objects", { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 200, offset: 0 } }, [3]),
      treeItem(3, "objects.0", "Row 0"),
    ];
    const overflowing = renderWindowedTree(nodes, () => {}, (tree) => createElement("div", { "data-testid": "plain", style: { overflowY: "auto" } }, tree));
    const plain = overflowing.container.querySelector('[data-testid="plain"]') as HTMLElement;
    const contents = overflowing.container.querySelector("div.contents") as HTMLElement;
    expect(treeWindowScrollViewport(contents)).toBe(plain);
    cleanup();

    const bare = renderWindowedTree(nodes, () => {}, (tree) => tree);
    const bareContents = bare.container.querySelector("div.contents") as HTMLElement;
    expect(treeWindowScrollViewport(bareContents)).toBe(document.scrollingElement ?? document.documentElement);
  });
  it("prefers whichever candidate actually overflows, even when it is the guest tree's own root", () => {
    const rendered = mountScrollable([{ key: "objects", total: 200 }], () => {});
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    const contents = rendered.container.querySelector("div.contents") as HTMLElement;
    const treeRoot = contents.firstElementChild as HTMLElement;
    treeRoot.style.overflowY = "auto";
    stubExtent(area, 500, 500);
    stubExtent(treeRoot, 400, 3000);

    expect(treeWindowScrollViewport(contents)).toBe(treeRoot);
  });
  it("measures the viewport by its client box, never by the border box a scrollbar and a border inflate", () => {
    const rendered = mountScrollable([{ key: "objects", total: 200 }], () => {});
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    stubRect(area, 0, 10);
    Object.defineProperty(area, "clientHeight", { value: 7 * rowHeightPx, configurable: true });
    Object.defineProperty(area, "clientTop", { value: 2, configurable: true });
    const windowed = windowedByKey(rendered.container).get("objects")!;
    stubRect(windowed, 0, 200);

    expect(treeWindowViewportMetrics(area)).toEqual({ originTop: 2, height: 7 * rowHeightPx });
    expect(treeWindowContainersUnder(rendered.container, area)).toEqual([{ key: "objects", rowExtent: "standard", total: 200, offset: 0, length: 1, top: -2, height: 200 * rowHeightPx, rows: [{ index: 0, top: -2 }] }]);
  });
}
