// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { chromium } from "playwright";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createAccessibilityMirror } from "../../../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts";
import { TableHost } from "../../🟦️.tsx";
import schema from "../../🧬️schema/🔘️button-accessibility/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🔘️button-accessibility/🔣️.json" with { type: "json" };

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  document.body.replaceChildren();
});

describe("Table row button accessibility", () => {
  it("validates the neutral row contract and mounts only the actual row button with its exact action", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const onAction = vi.fn();
    const view = render(
      <TableHost
        node={{
          type: "componentScene",
          surfaceId: fixture.surfaceId,
          controllerId: fixture.controllerId,
          componentKind: "table",
          table: { columnsJson: JSON.stringify(fixture.columns), rowsJson: JSON.stringify([fixture.row]) },
        }}
        onAction={onAction}
      />,
    );
    const button = screen.getByRole("button", { name: fixture.expected.label });
    expect(button.tabIndex).toBe(0);
    button.focus();
    expect(document.activeElement).toBe(button);
    expect(view.container.querySelector(`[title='${fixture.expected.excludedMenuLabel}']`)).toBeNull();
    fireEvent.click(button);
    expect(onAction).toHaveBeenCalledOnce();
    expect(onAction).toHaveBeenCalledWith(fixture.expected.action);
  });

  it("mirrors the accepted virtual row button as one tabbable addressed activation", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: unknown[] = [];
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [{ windowId: "table-window", windowGeneration: 9, nodes: [{ nodeId: 91, key: fixture.expected.key, role: fixture.expected.role, depth: 1, label: fixture.expected.label, live: "off", focusable: fixture.expected.focusable, tabbable: fixture.expected.tabbable, actionable: true }] }] }),
      enqueueLossless: (event) => { sent.push(event); return true; },
    }, "en");
    mirror.refresh();
    await vi.runAllTimersAsync();
    const button = screen.getByRole("button", { name: fixture.expected.label });
    expect(button.tabIndex).toBe(0);
    fireEvent.click(button);
    expect(sent).toEqual([{ kind: "accessibility-activate", windowId: "table-window", windowGeneration: 9, nodeId: 91, nodeKey: fixture.expected.key }]);
    mirror.dispose();
  });

  it("renders both row-placement controls from the one-pixel neutral case with distinct actions", () => {
    const geometry = fixture.geometryCases[0];
    const onAction = vi.fn();
    const view = render(
      <div style={{ width: geometry.cellRect.width }}>
        <TableHost
          node={{
            type: "componentScene",
            surfaceId: fixture.surfaceId,
            controllerId: fixture.controllerId,
            componentKind: "table",
            table: { columnsJson: JSON.stringify([{ id: "actions", label: "Actions" }]), rowsJson: JSON.stringify([{ id: "narrow", actions: { kind: "buttons", buttons: geometry.buttons } }]) },
          }}
          onAction={onAction}
        />
      </div>,
    );
    for (const probe of geometry.probes) {
      const button = screen.getByRole("button", { name: geometry.buttons[probe.buttonIndex].label });
      fireEvent.click(button);
      expect(onAction).toHaveBeenLastCalledWith(geometry.buttons[probe.buttonIndex].action);
    }
    expect(view.container.querySelector(`[title='${geometry.buttons[2].label}']`)).toBeNull();
  });

  it("matches Chromium's fractional segment ownership and two-axis overflow clipping", async () => {
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 128, height: 128 }, deviceScaleFactor: 8 });
    try {
      for (const geometry of fixture.geometryCases) {
        await page.setContent("<div id='clip'><div id='cell'><button data-index='0'></button><button data-index='1'></button></div></div>");
        const observed = await page.evaluate(({ bodyRect, cellRect }) => {
          const clip = document.querySelector<HTMLElement>("#clip")!;
          const cell = document.querySelector<HTMLElement>("#cell")!;
          Object.assign(clip.style, { position: "absolute", overflow: "hidden", left: `${bodyRect.x}px`, top: `${bodyRect.y}px`, width: `${bodyRect.width}px`, height: `${bodyRect.height}px` });
          Object.assign(cell.style, { position: "absolute", display: "flex", left: `${cellRect.x - bodyRect.x}px`, top: `${cellRect.y - bodyRect.y}px`, width: `${cellRect.width}px`, height: `${cellRect.height}px` });
          for (const button of cell.querySelectorAll<HTMLElement>("button")) Object.assign(button.style, { boxSizing: "border-box", flex: "0 0 50%", minWidth: "0", margin: "0", border: "0", padding: "0" });
          const clipRect = clip.getBoundingClientRect();
          const visible = [...cell.querySelectorAll<HTMLElement>("button")].map(button => {
            const rect = button.getBoundingClientRect();
            const x = Math.max(rect.x, clipRect.x);
            const y = Math.max(rect.y, clipRect.y);
            return { x, y, width: Math.min(rect.right, clipRect.right) - x, height: Math.min(rect.bottom, clipRect.bottom) - y };
          });
          return visible;
        }, geometry);
        for (let index = 0; index < geometry.expectedVisibleRects.length; index += 1) {
          expect(observed[index].x).toBeCloseTo(geometry.expectedVisibleRects[index].x, 5);
          expect(observed[index].y).toBeCloseTo(geometry.expectedVisibleRects[index].y, 5);
          expect(observed[index].width).toBeCloseTo(geometry.expectedVisibleRects[index].width, 5);
          expect(observed[index].height).toBeCloseTo(geometry.expectedVisibleRects[index].height, 5);
        }
      }
    } finally {
      await browser.close();
    }
  }, 60_000);
});
