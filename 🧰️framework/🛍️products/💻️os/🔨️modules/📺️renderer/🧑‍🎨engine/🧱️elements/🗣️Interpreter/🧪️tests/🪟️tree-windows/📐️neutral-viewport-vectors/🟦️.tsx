/** 📐️ The neutral viewport vectors both hosts answer. */
import type { AnyRecord, TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** 🧮 Registers the neutral viewport vector tests. */
export async function registerNeutralViewportVectorTests(kit: TreeWindowsTestKit): Promise<void> {
  const { TREE_WINDOW_BODY_NODE_BUDGET, describe, dirname, expect, fileURLToPath, it, join, readFileSync, source, treeWindowBodyRequestsV1 } = kit;
  const requestsFixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧫️fixtures/🪟️window-requests/🔣️.json"), "utf8"));
  const { TREE_WINDOW_OVERSCAN_ROWS, treeWindowRequestsForViewport, treeWindowVisibleRowsForViewport } = await import("@semio-tech/ui-react");
  describe("🪟️ the neutral viewport vectors both hosts answer (WG11 P5)", () => {
    it("the vectors satisfy their schema and were written against this host's body budget", () => {
      expect(requestsFixture.budget).toBe(TREE_WINDOW_BODY_NODE_BUDGET);
      expect(requestsFixture.overscan).toBe(TREE_WINDOW_OVERSCAN_ROWS);
    });
    for (const law of requestsFixture.cases) {
      it(law.name, () => {
        const containers = law.containers.map((container: AnyRecord) => ({ ...container, rows: container.rows.map(([index, top]: [number, number]) => ({ index, top })) }));
        expect([...treeWindowVisibleRowsForViewport(containers, 0, law.viewportHeight).entries()].map(([key, rows]: [string, AnyRecord]) => ({ key, ...rows }))).toEqual(law.visible);
        expect(treeWindowRequestsForViewport(containers, 0, law.viewportHeight, TREE_WINDOW_OVERSCAN_ROWS)).toEqual(law.requests);
        expect(treeWindowBodyRequestsV1(containers, law.viewportHeight)).toEqual(law.body);
      });
    }
  });
}
