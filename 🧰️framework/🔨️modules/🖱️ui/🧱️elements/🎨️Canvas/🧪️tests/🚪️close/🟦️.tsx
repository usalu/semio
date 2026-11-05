import * as React from "react";
import { act, cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Mode } from "@semio-tech/ui-react";
import fixture from "../🧫️fixtures/🚪️close/🔣️.json";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

describe("rendered window close admission", () => {
  for (const row of fixture.cases) it(`${row.decision} preserves the dock until the decision settles`, async () => {
    let resolve!: (accepted: boolean) => void, reject!: (error: Error) => void;
    const decision = new Promise<boolean>((accept, refuse) => { resolve = accept; reject = refuse; });
    const request = vi.fn(() => decision);
    vi.spyOn(console, "error").mockImplementation(() => undefined);
    const windows = [{ id: "a", title: "A", children: <div>First document</div> }, { id: "b", title: "B", children: <div>Second document</div> }];
    const view = render(<Mode windows={windows} activeWindowId="a" onWindowClose={request} layout={{ kind: "stack", children: windows.map(({ id }) => ({ kind: "window", id })) }} />);
    const ids = () => [...view.container.querySelectorAll('[data-slot="mode-dock-tab"]')].map((tab) => tab.getAttribute("data-window-id")).sort();
    const close = view.container.querySelector<HTMLElement>('[data-slot="mode-dock-tab"][data-window-id="a"] [data-slot="mode-dock-tab-close"]')!;
    expect(close).not.toBeNull();
    fireEvent.click(close);
    fireEvent.click(close);
    await waitFor(() => expect(request).toHaveBeenCalledTimes(1));
    expect(ids()).toEqual(["a", "b"]);
    await act(async () => {
      if (row.decision === "reject") reject(new Error("save unavailable"));
      else resolve(row.decision === "accept");
      await decision.catch(() => undefined);
    });
    await waitFor(() => expect(ids()).toEqual(row.remaining));
  });
});
