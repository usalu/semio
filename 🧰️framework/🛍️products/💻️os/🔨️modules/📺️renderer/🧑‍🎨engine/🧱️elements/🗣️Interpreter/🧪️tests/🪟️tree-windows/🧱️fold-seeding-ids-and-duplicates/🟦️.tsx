/** 🧱️ The below the fold seeding, inline control ids, duplicate key and range pick tests. */
import { captureErrors } from "../🧰️tree-windows-test-kit/🟦️.tsx";
import type { AnyRecord, TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** 🗝 Registers the below the fold seeding, inline control ids, same key windows, duplicate key warning, body row cap and range pick tests. */
export function registerFoldSeedingIdsAndDuplicateTests(kit: TreeWindowsTestKit): void {
  const { SURFACE, Scrollable, TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_PATH_SEPARATOR, createElement, expect, frame, it, mount, mountScrollable, node, nodeCost, renderWindowedTree, rowHeightPx, scrollTo, stubRect, stubRows, treeItem, treePickIntentInputV1, treePickTargetsV1, treeWindowBodyRequestsV1, windowedByKey } = kit;
  /** 🧯️ The cad lane's `structure-classic`: a default-open container below the fold at a 440 px viewport,
   * answered `rows: 0` on first paint. `rows: 0` renders as an open container with a full-height spacer and
   * no rows — to a reader, indistinguishable from an empty one, wearing the pending ring. It must be seeded
   * with one row while it is off screen, and it must materialise its real window within ONE refresh of
   * being scrolled into view. */
  it("seeds a never-materialised container below the fold, then gives it a real window when it scrolls in", () => {
    const rows = (offset: number, length: number, firstTopRows: number) => Array.from({ length }, (_, index) => ({ index: offset + index, top: (firstTopRows + index) * rowHeightPx }));
    const onScreen = { key: "shape", rowExtent: "standard", total: 200, offset: 0, length: 20, top: 0, height: 200 * rowHeightPx, rows: rows(0, 20, 0) };
    const belowFold = { key: "structure-classic", rowExtent: "standard", total: 11, offset: 0, length: 0, top: 200 * rowHeightPx, height: 11 * rowHeightPx };

    const first = treeWindowBodyRequestsV1([onScreen, belowFold], 20 * rowHeightPx);
    const seeded = first.find((request: AnyRecord) => request.key === "structure-classic")!;
    expect(seeded).toEqual({ key: "structure-classic", offset: 0, rows: 1 });
    expect(first.find((request: AnyRecord) => request.key === "shape")!.rows).toBeGreaterThan(0);

    const scrolledIn = { ...belowFold, length: 1, top: -2 * rowHeightPx, rows: rows(0, 1, -2) };
    const after = treeWindowBodyRequestsV1([{ ...onScreen, top: -202 * rowHeightPx }, scrolledIn], 20 * rowHeightPx);
    const materialised = after.find((request: AnyRecord) => request.key === "structure-classic")!;

    expect(materialised.rows).toBeGreaterThan(1);
    expect(materialised).toEqual({ key: "structure-classic", offset: 0, rows: 11 });
    expect(after.find((request: AnyRecord) => request.key === "shape")!.rows).toBe(0);
  });
  /** 🔑️ `📐️cad` builds the same `object.id` under four pane sections and `🏗️fem` builds `case.id` and
   * `combination.id` in one body — and that key is also the pick target id, so it cannot be namespaced
   * (📓️f2-sdk-body-node-ledger.md §10). Under key identity the two containers would share one open state,
   * one window and each other's measurements; under PATH identity they are two windows that happen to name
   * the same entity, and the pick still dispatches the bare key. */
  it("gives every row's inline control its own DOM id — a node key is unique only among its siblings", () => {
    const commit = { trigger: "commit", action: { scope: "s.stdio.json@rfc8259/*#editor", name: "set-node", version: 1 }, args: { nodeId: "$", revision: "r" }, capability: null };
    const edit = (id: number, value: string) => node(id, "edit", { type: "input", kind: "text", value, commit: "blur" }, [], [commit]);
    const view = mount(
      [
        node(1, "framework.window.tree", { type: "tree", interactionDomain: "outliner.objects" }, [2]),
        node(2, "framework.window.tree-root", { type: "treeSection", label: "", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 2, offset: 0 } }, [3, 4]),
        treeItem(3, "i=0", "first: 1", {}, [5]),
        treeItem(4, "i=1", "second: 2", {}, [6]),
        edit(5, "1"),
        edit(6, "2"),
      ],
      1,
      () => {},
    );
    const inputs = Array.from(view.container.querySelectorAll<HTMLInputElement>("input"));
    expect(inputs).toHaveLength(2);
    const ids = inputs.map((input) => input.id);
    expect(new Set(ids).size).toBe(2);
    for (const id of ids) {
      expect(id.endsWith("/edit")).toBe(true);
      expect(id).not.toBe(`${SURFACE}/edit`);
    }
  });
  it("gives the same node key under two different parents two independent windows", async () => {
    const reports: { requests: readonly AnyRecord[]; viewportRows: number }[] = [];
    const shared = (parent: string, id: number) => [
      treeItem(id, "shared", "Shared", { defaultOpen: true, window: { rowExtent: "standard", total: 40, offset: 0 } }, [id + 1]),
      treeItem(id + 1, `${parent}.shared.0`, "Child 0"),
    ];
    const nodes: AnyRecord[] = [
      node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2, 3]),
      node(2, "left", { type: "treeSection", label: "Left", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 4, offset: 0 } }, [10]),
      node(3, "right", { type: "treeSection", label: "Right", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 4, offset: 0 } }, [20]),
      ...shared("left", 10),
      ...shared("right", 20),
    ];
    const rendered = renderWindowedTree(nodes, (requests, viewportRows) => reports.push({ requests, viewportRows }), (tree) => createElement(Scrollable, null, tree));
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    const windowed = windowedByKey(rendered.container);
    const leftShared = `left${TREE_WINDOW_PATH_SEPARATOR}shared`;
    const rightShared = `right${TREE_WINDOW_PATH_SEPARATOR}shared`;
    expect([...windowed.keys()]).toEqual(["left", leftShared, "right", rightShared]);
    expect(windowed.get(leftShared)!.getAttribute("data-tree-window-key")).toBe("shared");
    expect(windowed.get(rightShared)!.getAttribute("data-tree-window-key")).toBe("shared");

    scrollTo(area, 0, 20);
    stubRect(windowed.get("left")!, 0, 42);
    stubRows(windowed.get("left")!, 0);
    stubRect(windowed.get(leftShared)!, 1, 40);
    stubRows(windowed.get(leftShared)!, 1);
    stubRect(windowed.get("right")!, 60, 42);
    stubRows(windowed.get("right")!, 60);
    stubRect(windowed.get(rightShared)!, 61, 40);
    stubRows(windowed.get(rightShared)!, 61);

    const shouted = await captureErrors(async () => {
      area.dispatchEvent(new Event("scroll"));
      await frame();
    });

    const last=reports.at(-1);if(!last)throw Error("Actual tree window report required");
    expect(last.requests.map((request) => request.nodeKey)).toEqual(["left", leftShared, "right", rightShared]);
    expect(last.requests.find((request) => request.nodeKey === leftShared)!.rows).toBeGreaterThan(0);
    expect(last.requests.find((request) => request.nodeKey === rightShared)).toEqual({ nodeKey: rightShared, offset: 0, rows: 0 });
    expect(shouted.filter((message) => message.includes("[tree-window] duplicate key"))).toEqual([]);
  });
  /** 🔑️ Two containers under one key share one open state and one window on both sides of the wire, and
   * each other's measurement silently overwrites the other's. The host cannot repair it — it says so. */
  it("says so, once, when two windowed containers in one body share a node key", async () => {
    const errors: string[] = [];
    const original = console.error;
    console.error = (...args: unknown[]) => void errors.push(String(args[0]));
    try {
      const rendered = mountScrollable([{ key: "objects", total: 20 }, { key: "shadow", total: 20 }], () => {});
      const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
      const windowed = windowedByKey(rendered.container);
      windowed.get("shadow")!.setAttribute("data-tree-window-path", "objects");
      scrollTo(area, 0, 20);
      stubRect(windowed.get("objects")!, 0, 20);
      stubRect(windowed.get("shadow")!, 20, 20);

      area.dispatchEvent(new Event("scroll"));
      await frame();
      area.dispatchEvent(new Event("scroll"));
      await frame();

      expect(errors.filter((message) => message.includes("[tree-window] duplicate key")).length).toBe(1);
      expect(errors.some((message) => message.includes('"objects"'))).toBe(true);
    } finally {
      console.error = original;
    }
  });
  it("never asks one body for more rows than the guest can present, however many containers are on screen", async () => {
    const reports: { requests: readonly AnyRecord[]; viewportRows: number }[] = [];
    const containers = Array.from({ length: 12 }, (_, index) => ({ key: `c${index}`, total: 12 }));
    const rendered = mountScrollable(containers, (requests, viewportRows) => reports.push({ requests, viewportRows }));
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    scrollTo(area, 0, 144);
    const elements = Array.from(rendered.container.querySelectorAll("[data-tree-window-key]")) as HTMLElement[];
    expect(elements.length).toBe(12);
    elements.forEach((element, index) => {
      stubRect(element, index * 12, 12);
      stubRows(element, index * 12);
    });

    area.dispatchEvent(new Event("scroll"));
    await frame();

    const last=reports.at(-1);if(!last)throw Error("Actual tree window report required");
    expect(last.requests.length).toBe(12);
    expect(nodeCost(last.requests)).toBe(TREE_WINDOW_BODY_NODE_BUDGET);
    for (const request of last.requests) expect(request.offset + request.rows).toBeLessThanOrEqual(12);
  });
  it("resolves a range pick into the whole selection under replace, deduplicated per (granularity, id)", () => {
    const walk = { pickTargets: new Map([[`${SURFACE}/a`, { key: "a", granularity: "piece" }], [`${SURFACE}/b`, { key: "b", granularity: "port" }]]) };
    const targets = treePickTargetsV1(walk, "b", "port", "range", [`${SURFACE}/a`, `${SURFACE}/b`, `${SURFACE}/unknown`]);
    expect(targets).toEqual([{ granularity: "piece", id: "a" }, { granularity: "port", id: "b" }]);
    expect(treePickIntentInputV1("range", targets)).toEqual({ merge: "replace", method: "pick", targets: JSON.stringify([{ granularity: "piece", id: "a" }, { granularity: "port", id: "b" }]) });
    expect(treePickIntentInputV1("invertive", treePickTargetsV1(walk, "a", "piece", "invertive", [])).merge).toBe("invertive");
    expect(treePickTargetsV1(walk, "a", "piece", "invertive", [`${SURFACE}/b`])).toEqual([{ granularity: "piece", id: "a" }]);
  });
}
