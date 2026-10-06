// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createAccessibilityMirror } from "../../../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts";
import { EventFeedHost } from "../../🟦️.tsx";
import fixture from "../../🧫️fixtures/♿️accessible-entry/🔣️.json" with { type: "json" };

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.restoreAllMocks();
  document.body.replaceChildren();
});

describe("EventFeed accessible entry parity", () => {
  it("validates the neutral entry contract and mounts the actual React role, name, and action", () => {
    vi.spyOn(Date.prototype, "toLocaleTimeString").mockReturnValue(fixture.acceptedTimeLabel);

    for (const row of fixture.cases) {
      const onAction = vi.fn();
      const view = render(
        <EventFeedHost
          node={{
            type: "componentScene",
            surfaceId: fixture.surfaceId,
            controllerId: fixture.controllerId,
            componentKind: "eventFeed",
            eventFeed: { entriesJson: JSON.stringify([row.entry]), activateAction: row.activateAction ?? undefined },
          }}
          onAction={onAction}
        />,
      );
      if (row.expected.role === "button") {
        const entry = screen.getByRole("button", { name: row.expected.accessibleName! });
        expect(entry.tabIndex).toBe(0);
        fireEvent.click(entry);
        fireEvent.keyDown(entry, { key: "Enter" });
        fireEvent.keyDown(entry, { key: " " });
        expect(onAction).toHaveBeenCalledTimes(3);
        for (const call of onAction.mock.calls) expect(call).toEqual([row.expected.action]);
      } else {
        expect(view.container.querySelector("[role='button']")).toBeNull();
        const entry = view.container.querySelector<HTMLElement>("[role='paragraph']");
        expect(entry).not.toBeNull();
        if (!entry) throw new Error("passive EventFeed entry did not expose paragraph semantics");
        expect(entry.tabIndex).toBe(-1);
        expect(entry.textContent).toContain(row.entry.title);
        expect(entry.textContent).toContain(row.entry.detail);
        expect(onAction).not.toHaveBeenCalled();
      }
      view.unmount();
    }
  });

  it("turns the accepted WGPU virtual row into one tabbable addressed browser activation", async () => {
    vi.useFakeTimers();
    const row = fixture.cases[0]!;
    const root = document.createElement("div");
    document.body.append(root);
    const sent: unknown[] = [];
    const mirror = createAccessibilityMirror(root, {
      introspect: async () =>
        JSON.stringify({
          windows: [
            {
              windowId: "events-window",
              windowGeneration: 7,
              nodes: [
                {
                  nodeId: 71,
                  key: row.expected.key,
                  role: row.expected.role,
                  depth: 1,
                  label: row.expected.accessibleName,
                  live: "off",
                  focusable: row.expected.focusable,
                  tabbable: row.expected.tabbable,
                  actionable: true,
                },
              ],
            },
          ],
        }),
      enqueueLossless: (event) => {
        sent.push(event);
        return true;
      },
    }, "en");
    mirror.refresh();
    await vi.runAllTimersAsync();
    const button = screen.getByRole("button", { name: row.expected.accessibleName! });
    expect(button.tabIndex).toBe(0);
    fireEvent.click(button);
    expect(sent).toEqual([{ kind: "accessibility-activate", windowId: "events-window", windowGeneration: 7, nodeId: 71, nodeKey: row.expected.key }]);
    mirror.dispose();
  });
});
