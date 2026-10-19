/** 🛣 Workflow flow routing and ui search. */
import { createElement } from "react";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { UIFind, UIFindProvider, UISearch, type UISearchItem, useUIFind } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";
import { registerFlowFrameAndPresenceTests } from "./🖼️flow-frame-and-presence/🟦️.ts";
import { registerCatalogueDragAndSuggestionTests } from "./🧲️catalogue-drag-and-suggestions/🟦️.ts";
import { registerPanelBodyAndTreeHostingTests } from "./🌳️panel-body-and-tree-hosting/🟦️.ts";
import { registerCatalogueRowAndIntentChannelTests } from "./🔗️catalogue-row-and-intent-channel/🟦️.ts";
import { registerShellRoutesAndIdentityTests } from "./🧭️shell-routes-and-identity/🟦️.ts";
import { registerAutoCheckinAndSyncStatusTests } from "./✅️auto-checkin-and-sync-status/🟦️.ts";

describe("s workflow flow routing", () => {
  registerFlowFrameAndPresenceTests();
  registerCatalogueDragAndSuggestionTests();
  registerPanelBodyAndTreeHostingTests();
  registerCatalogueRowAndIntentChannelTests();
  registerShellRoutesAndIdentityTests();
  registerAutoCheckinAndSyncStatusTests();
});

describe("ui search/find (fuse re-export from @semio-tech/ui-react)", () => {
  afterEach(async () => {
    const { cleanup } = await import("@semio-tech/ui-react/test");
    cleanup();
  });

  it("UISearch renders all items and fuzzy-filters them through the owned ranker", async () => {
    const { render, fireEvent } = await import("@semio-tech/ui-react/test");
    const selected = vi.fn();
    const openChange = vi.fn();
    const items: UISearchItem[] = [
      { id: "a", label: "Alpha", category: "Test", onSelect: selected },
      { id: "b", label: "Bravo", category: "Test", onSelect: noopAction },
    ];
    render(createElement(UIFindProvider, null, createElement(UISearch, { items, open: true, onOpenChange: openChange })));
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).toContain("Bravo");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "alp" } });
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).not.toContain("Bravo");
    fireEvent.keyDown(input, { key: "Enter" });
    expect(openChange).toHaveBeenCalledWith(false);
    expect(selected).toHaveBeenCalledTimes(1);
  });

  it("UIFind renders and fuzzy-filters items registered on its context through the owned ranker", async () => {
    const { render, fireEvent, act } = await import("@semio-tech/ui-react/test");
    let contextValue: ReturnType<typeof useUIFind> | undefined;
    const Harness = () => {
      contextValue = useUIFind();
      return createElement(UIFind, { open: true, onOpenChange: noopAction });
    };
    render(createElement(UIFindProvider, null, createElement(Harness)));
    act(() => {
      contextValue!.setFindItems([
        { id: "1", label: "Chair", category: "Test" },
        { id: "2", label: "Table", category: "Test" },
      ]);
    });
    const selected = vi.fn();
    act(() => contextValue!.setOnFindItem(selected));
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).toContain("Table");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "cha" } });
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).not.toContain("Table");
    fireEvent.keyDown(input, { key: "Enter" });
    expect(selected).toHaveBeenCalledWith("1");
  });
});
