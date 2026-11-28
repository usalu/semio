import * as React from "react";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render } from "@testing-library/react";
import { PanelTabBar, singleTreeLeaf, type PanelTabNode } from "./🟦️.tsx";

HTMLElement.prototype.scrollIntoView = () => {};
afterEach(cleanup);

const Stub = (): null => null;
const tabs: PanelTabNode[] = ["a", "b"].map((id) => singleTreeLeaf({ id: `tab-${id}`, icon: Stub, name: `Tab ${id}`, tree: { sections: [] } }));

describe("🔘️ PanelTabBar aria-pressed", () => {
  it("presses only the showing active tab", () => {
    const { container } = render(<div className="semio-scope"><PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} /></div>);
    const state = [...container.querySelectorAll<HTMLElement>("button[data-tab-id]")].map((b) => [b.dataset.tabId, b.getAttribute("aria-pressed")]);
    expect(state).toEqual([["tab-a", "true"], ["tab-b", "false"]]);
  });

  it("presses no tab while the panel is folded and nothing is showing", () => {
    const { container } = render(<div className="semio-scope"><PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} showActiveColor={false} /></div>);
    const state = [...container.querySelectorAll<HTMLElement>("button[data-tab-id]")].map((b) => b.getAttribute("aria-pressed"));
    expect(state).toEqual(["false", "false"]);
  });
});
