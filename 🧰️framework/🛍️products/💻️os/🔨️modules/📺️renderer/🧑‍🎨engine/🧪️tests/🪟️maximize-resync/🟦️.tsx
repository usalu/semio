// @vitest-environment jsdom

import React from "react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { Mode, uiDataLabel, type WindowLayoutNode } from "@semio-tech/ui-react";
import { resolveFrameworkLayoutSeed, retitleWindowLayoutNode } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import fixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/🪟️maximize-resync/🔣️.json";

beforeEach(() => {
  vi.stubGlobal("ResizeObserver", class { observe() {} unobserve() {} disconnect() {} });
});

it("React locale retitling preserves authored extra-window titles and maximize", () => {
  const row = fixture.localeRetitle;
  const kinds = fixture.windows.map(id => ({ id, label: id === "a" ? "A" : row.resolvedTitle }));
  const extra = [{ id: row.extraId, windowKindId: row.kindId, title: row.bakedTitle }];
  const initial: WindowLayoutNode = { kind: "row", children: [
    { kind: "stack", size: 0.5, children: [{ kind: "window", id: "a", title: uiDataLabel("A") }] },
    { kind: "stack", size: 0.5, children: [{ kind: "window", id: "b", title: uiDataLabel(row.resolvedTitle) }, { kind: "window", id: row.extraId, title: uiDataLabel(row.bakedTitle) }] },
  ] };
  const mode = (layout: WindowLayoutNode) => <Mode windows={[...kinds, { id: row.extraId, label: row.bakedTitle }].map(entry => ({ id: entry.id, title: uiDataLabel(entry.label), iconId: "app-window", children: <div>{entry.id}</div> }))} layout={layout} activeWindowId="b" onActiveWindowChange={() => {}} />;
  const { container, rerender } = render(mode(initial));
  const maximize = () => fireEvent.click(container.querySelector("[data-stack-path='1'] [data-slot='mode-dock-tab-focus']")!);
  maximize();
  expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe("1");
  const retitled = retitleWindowLayoutNode(initial, kinds, extra, "native", row.locale, {});
  rerender(mode(retitled));
  expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe(String(row.maximized));
  expect(retitled.kind !== "window" && retitled.children[1]?.kind === "stack" && retitled.children[1].children[1]?.title).toBe(row.retainedTitle);
  rerender(mode(retitleWindowLayoutNode(retitled, kinds, extra, "native", row.locale, {})));
  expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe("1");
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

for (const row of fixture.ingressCases) it(`React Mode reconciles ${row.id}`, () => {
  const kinds = fixture.windows.map(id => ({ id, label: id.toUpperCase() }));
  const labels = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} };
  const resolveSeed = (stacks: typeof row.stacks) => resolveFrameworkLayoutSeed({ root: { kind: "row", children: stacks.map(children => ({ kind: "stack", children: children.map(leaf => ({ kind: "window", ...leaf })) })) } }, kinds, labels, "native", "en");
  const seed = resolveSeed(row.stacks);
  expect(seed.extraInstances.map(instance => instance.id)).toEqual(row.extras);
  expect(Object.fromEntries(seed.pendingProjections.map(entry => [entry.windowId, entry.templateId]))).toEqual(row.templates);
  const mode = (seed: ReturnType<typeof resolveSeed>) => <Mode windows={[...kinds, ...seed.extraInstances.map(instance => ({ id: instance.id, label: instance.title }))].map(window => ({ id: window.id, title: uiDataLabel(window.label), iconId: "app-window", children: <div>{window.id} Body</div> }))} layout={seed.modeLayout} activeWindowId={null} onActiveWindowChange={() => {}} />;
  const { container, rerender } = render(mode("previousStacks" in row ? resolveSeed(row.previousStacks!) : seed));
  rerender(mode(seed));
  expect([...container.querySelectorAll('[data-slot="mode-dock-tab"]')].map(tab => tab.getAttribute("data-window-id"))).toEqual(row.windows);
  expect([...container.querySelectorAll('[data-stack-path]')].map(stack => stack.getAttribute("data-stack-path"))).toEqual(row.stackPaths);
});

it("validates the shared maximize resynchronization contract", () => {
});

for (const row of fixture.cases) it(`React Mode replays ${row.id}`, () => {
  const extraId = "extraId" in row ? row.extraId : undefined;
  const templateId = "templateId" in row ? row.templateId : undefined;
  const labels = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} };
  const mode = (ids: readonly string[], weights: readonly number[], label: string, layoutLabel: string, updated = false) => (
    <Mode
      windows={ids.map((id) => ({ id, title: uiDataLabel(id === "a" ? label : "B"), iconId: "app-window", children: <div>{id} Body</div> }))}
      activeWindowId="b"
      onActiveWindowChange={() => {}}
      layout={resolveFrameworkLayoutSeed({ root: {
        kind: "row",
        children: fixture.windows.map((id, index) => ({ kind: "stack" as const, size: weights[index], children: [
          { kind: "window" as const, windowKindId: id, ...(updated && templateId && id === "b" ? { templateId } : {}) },
          ...(updated && extraId && id === "b" ? [{ kind: "window" as const, windowKindId: "b", instanceId: extraId }] : []),
        ] })),
      } }, fixture.windows.map(id => ({ id, label: id === "a" ? layoutLabel : "B" })), labels, "native", "en").modeLayout}
    />
  );
  const { container, rerender } = render(mode(fixture.windows, fixture.initialWeights, "A", "A"));
  fireEvent.click(container.querySelector(`[data-stack-path='${fixture.maximized}'] [data-slot='mode-dock-tab-focus']`)!);
  expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe(String(fixture.maximized));
  rerender(mode(row.windows, row.weights, row.label, row.layoutLabel, true));
  expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path") ?? null).toBe(row.maximized === null ? null : String(row.maximized));
  if (row.maximized === null) fireEvent.click(container.querySelector(`[data-stack-path='${fixture.maximized}'] [data-slot='mode-dock-tab-focus']`)!);
  rerender(mode(row.windows, row.weights, row.label, row.layoutLabel, true));
  expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe(String(fixture.maximized));
});
