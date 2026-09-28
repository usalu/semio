// #region 🔌️Adapters
import { act, render } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it } from "vitest";
import { liveTreePanelDefinition, PanelTreeUnitsPane } from "@semio-tech/ui-react";
// #endregion 🔌️Adapters

// #region 📡️LiveTreeUnit
describe("📡️ live panel tree unit", () => {
  it("follows its source on every notification while the host that composed the tab never re-renders", () => {
    let rows = ["alpha"];
    const listeners = new Set<() => void>();
    let builds = 0;
    const source = liveTreePanelDefinition(
      (listener) => {
        listeners.add(listener);
        return () => void listeners.delete(listener);
      },
      () => rows,
      (current) => {
        builds += 1;
        return { sections: [{ id: "rows", label: "Rows", items: current.map((row) => ({ id: row, label: row })) }] };
      },
    );
    let hostRenders = 0;
    const Host = () => {
      hostRenders += 1;
      return <PanelTreeUnitsPane tabId="tab" units={[{ id: "tab.tree", tree: source }]} />;
    };
    const view = render(<Host />);
    expect(view.getByText("alpha")).toBeTruthy();
    act(() => {
      rows = ["alpha", "beta"];
      for (const listener of listeners) listener();
    });
    expect(view.getByText("beta")).toBeTruthy();
    act(() => {
      for (const listener of listeners) listener();
    });
    expect(hostRenders, "the host rendered once, at mount").toBe(1);
    expect(builds, "one config per value, however often the pane re-reads it").toBe(2);
  });
});
// #endregion 📡️LiveTreeUnit
