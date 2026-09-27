// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createAccessibilityMirror } from "../../../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts";
import { GraphTimelineHost } from "../../🟦️.tsx";
import schema from "../../🧬️schema/🎯️checkpoint-hit/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🎯️checkpoint-hit/🔣️.json" with { type: "json" };

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  document.body.replaceChildren();
});

describe("GraphTimeline checkpoint hit regions", () => {
  it("validates the neutral checkpoint hit contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it.each(fixture.cases)("matches the actual React checkpoint action for $id", (sample) => {
    const onAction = vi.fn();
    const view = render(
      <GraphTimelineHost
        node={{
          type: "componentScene",
          surfaceId: fixture.surfaceId,
          controllerId: fixture.controllerId,
          componentKind: "graphTimeline",
          graphTimeline: { columnsJson: JSON.stringify(fixture.columns) },
        }}
        onAction={onAction}
      />,
    );
    const column = fixture.columns[sample.row]!;
    const label = view.getByText(column.labels[0]!);
    const checkpoint = label.closest<HTMLElement>("[style*='grid-column: 1 / 3']");
    expect(checkpoint).not.toBeNull();
    if (!checkpoint) throw new Error("React checkpoint area is missing");
    const target = sample.region === "labels"
      ? label
      : sample.region === "graph"
        ? checkpoint.lastElementChild!
        : sample.region === "description"
          ? view.getByText(column.description)
          : view.container;
    fireEvent.click(target);
    expect(onAction.mock.calls).toEqual(sample.action === null ? [] : [[sample.action]]);
  });

  it("exposes only the selectable labels and graph shell as one keyboard checkpoint button", () => {
    const onAction = vi.fn();
    const view = render(
      <GraphTimelineHost
        node={{
          type: "componentScene",
          surfaceId: fixture.surfaceId,
          controllerId: fixture.controllerId,
          componentKind: "graphTimeline",
          graphTimeline: { columnsJson: JSON.stringify(fixture.columns) },
        }}
        onAction={onAction}
      />,
    );
    const button = view.getByRole(fixture.accessibility.role, { name: fixture.accessibility.accessibleName });
    expect(button.getAttribute("tabindex")).toBe("0");
    expect(view.getByText("Original state").closest("[role='button']")).toBeNull();
    fireEvent.keyDown(button, { key: "Enter" });
    fireEvent.keyDown(button, { key: " " });
    expect(onAction.mock.calls).toEqual([[fixture.accessibility.action], [fixture.accessibility.action]]);
  });

  it("mirrors the accepted checkpoint as one tabbable addressed browser activation", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: unknown[] = [];
    const mirror = createAccessibilityMirror(
      root,
      {
        introspect: async () =>
          JSON.stringify({
            windows: [
              {
                windowId: "history-window",
                windowGeneration: 11,
                nodes: [
                  {
                    nodeId: 111,
                    key: fixture.accessibility.key,
                    role: fixture.accessibility.role,
                    depth: 1,
                    label: fixture.accessibility.accessibleName,
                    live: "off",
                    focusable: true,
                    tabbable: true,
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
      },
      "en",
    );
    mirror.refresh();
    await vi.runAllTimersAsync();
    const button = screen.getByRole("button", { name: fixture.accessibility.accessibleName });
    expect(button.tabIndex).toBe(0);
    fireEvent.click(button);
    expect(sent).toEqual([{ kind: "accessibility-activate", windowId: "history-window", windowGeneration: 11, nodeId: 111, nodeKey: fixture.accessibility.key }]);
    mirror.dispose();
  });
});
