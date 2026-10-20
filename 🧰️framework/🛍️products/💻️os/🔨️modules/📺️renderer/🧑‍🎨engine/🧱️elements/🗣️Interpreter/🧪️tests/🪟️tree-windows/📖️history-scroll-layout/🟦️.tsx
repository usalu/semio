/** 📖️ The authored history scroll layout tree viewport test. */
import { stubExtent } from "../🧰️tree-windows-test-kit/🟦️.tsx";
import type { TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** 🗂 Registers the authored history scroll layout as the Tree viewport test. */
export async function registerHistoryScrollLayoutTests(kit: TreeWindowsTestKit): Promise<void> {
  const { expect, it, mount, node, treeItem, treeWindowScrollViewport } = kit;
  const { default: historyViewportFixture } = await import("@semio-tech/framework-plugin/testing/fixtures/history-panel-command-window.json");
  it("uses the authored history scroll layout as its Tree viewport", () => {
    const root = node(0, "history", { type: "tree" }, [1]);
    root.layout = historyViewportFixture.rootLayout;
    const mounted = mount([root, node(1, "commands", { type: "treeSection", label: "History", defaultOpen: true }, [2]), treeItem(2, "entry", "Move")], 0, () => {});
    const viewport = mounted.container.querySelector<HTMLElement>('[data-ui-node-id="0"]');
    expect(viewport).not.toBeNull();
    const expected = historyViewportFixture.rootCss;
    for (const [property, value] of Object.entries(expected)) expect(viewport!.style[property as "overflowX"]).toBe(value);
    stubExtent(viewport!, 240, 2400);
    expect(treeWindowScrollViewport(viewport!)).toBe(viewport);
  });
}
