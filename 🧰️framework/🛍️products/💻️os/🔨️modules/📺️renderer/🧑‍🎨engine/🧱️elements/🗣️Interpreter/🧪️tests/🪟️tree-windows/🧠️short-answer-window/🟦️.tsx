/** 🧠️ The window the guest answers short tests. */
import type { AnyRecord, TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** ✂ Registers the short answered window tests. */
export function registerShortAnswerWindowTests(kit: TreeWindowsTestKit): void {
  const { describe, dirname, expect, fileURLToPath, it, join, readFileSync, rowHeightPx, source, treeWindowServedRequestsV1 } = kit;
  const servedFixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧫️fixtures/🪟️tree-window-served.json"), "utf8"));
  describe("🧠️ a window the guest answers short", () => {

    /** 📐️ One unexpanded container `total` rows tall whose materialised rows sit at `topRows + offset + i`. */
    function measureOf(container: AnyRecord): AnyRecord {
      return {
        key: container.key,
        rowExtent: "standard",
        total: container.total,
        offset: container.offset,
        length: container.length,
        top: container.topRows * rowHeightPx,
        height: container.total * rowHeightPx,
        rows: Array.from({ length: container.length }, (_, index) => ({ index: container.offset + index, top: (container.topRows + container.offset + index) * rowHeightPx })),
      };
    }

    for (const law of servedFixture.cases) {
      it(law.name, () => {
        const memory = new Map(law.memory === null ? [] : [[law.container.key, { ...law.memory, asked: { key: law.container.key, ...law.memory.asked } }]]);
        const served = treeWindowServedRequestsV1([measureOf(law.container)], law.viewportRows * rowHeightPx, memory);
        expect(served.requests).toEqual([{ key: law.container.key, ...law.expect.request }]);
        const kept = served.memory.get(law.container.key)!;
        expect({ capacity: kept.capacity, anchor: kept.anchor }).toEqual({ capacity: law.expect.capacity, anchor: law.expect.anchor });
      });
    }

    /** 🧪️ The guest's own rule (`sliced_capped` + the item budget ending a run early), stated independently: the
     * asked offset clamped into the list, then as many rows as were asked, exist and fit its capacity. */
    const answer = (asked: AnyRecord, total: number, capacity: number) => {
      const offset = Math.min(asked.offset, Math.max(0, total - Math.max(1, asked.rows)));
      return { offset, length: Math.min(asked.rows, total - offset, capacity) };
    };

    for (const [total, viewportRows, capacity] of [[42, 32, 27], [45, 32, 27], [140, 32, 27], [140, 32, 40], [60, 10, 20], [42, 32, 200]] as const) {
      it(`reaches every row of ${total} with a ${viewportRows}-row viewport and a guest serving ${capacity}, and settles silent at every scroll position`, () => {
        let shown = { offset: 0, length: Math.min(total, capacity, 48) };
        let memory: ReadonlyMap<string, any> = new Map();
        const reached = new Set<number>();
        const lastScroll = Math.max(0, total - viewportRows);
        const positions = [...Array.from({ length: lastScroll + 1 }, (_, index) => index), ...Array.from({ length: lastScroll + 1 }, (_, index) => lastScroll - index)];
        for (const scroll of positions) {
          let settled = false;
          for (let round = 0; round < 8 && !settled; round += 1) {
            const measure = measureOf({ key: "spaces", total, ...shown, topRows: -scroll });
            const served = treeWindowServedRequestsV1([measure], viewportRows * rowHeightPx, memory);
            const repeated = treeWindowServedRequestsV1([measure], viewportRows * rowHeightPx, served.memory);
            memory = served.memory;
            const next = answer(served.requests[0]!, total, capacity);
            settled = next.offset === shown.offset && next.length === shown.length && JSON.stringify(repeated.requests) === JSON.stringify(served.requests);
            shown = next;
          }
          expect(settled).toBe(true);
          const visible = Array.from({ length: Math.min(viewportRows, total) }, (_, index) => scroll + index);
          const servedVisible = visible.filter((row) => row >= shown.offset && row < shown.offset + shown.length).length;
          expect(servedVisible).toBe(Math.min(capacity, visible.length));
          if (scroll === 0) expect(shown.offset).toBe(0);
          if (scroll === lastScroll && scroll > 0) expect(shown.offset + shown.length).toBe(total);
          for (let row = shown.offset; row < shown.offset + shown.length; row += 1) reached.add(row);
        }
        expect(reached.size).toBe(total);
      });
    }
  });
}
