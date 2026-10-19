/** 🎚️ The graph pick channel decoding and the graph slider accessibility region tests. */
import { createElement } from "react";
import { dagSliderOverlayFixture as graphSliderFixture, artifactsDagVcsSchema as dagVcsSchema } from "../../../../../♾️infinite/🧪️tests/🧫️fixtures/🟦️.ts";
import graphPickFixture from "@semio-tech/framework-os-node-graph-rs/testing/fixtures/pick-target.json";
import { fireEvent, render } from "@semio-tech/ui-react/test";
import { expect, it, vi } from "vitest";
import { nodeGraphPickChannel, parseDagSliderOverlays, dagSliderValueText, GraphSliderOverlays } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { computeAccessibleName, rendererExport } from "../../🧰️engine-contract-helpers/🟦️.ts";
import { peerExport } from "../🧰️node-graph-host-fixtures/🟦️.ts";

/** 🔘 Registers the pick channel decoding, strict slider labels, localized captions, keyboard changes, lost capture and painted readout tests. */
export function registerGraphPickAndSliderAccessibilityTests(): void {
  //#region 🎚️GraphSliderAccessibility
  it("decodes graph pick channels from the native handle grammar with strict schema parity", () => {
    const validateTarget = rendererExport("NodeGraphPickTargetV1");
    const validateChannel = rendererExport("NodeGraphInteractionChannelV1");
    for (const row of graphPickFixture) {
      const target = row.target;
      expect(validateTarget(target), JSON.stringify(validateTarget.errors)).toBe(true);
      expect(validateChannel(row.channel), JSON.stringify(validateChannel.errors)).toBe(true);
      const match = target?.domain === "handle" ? /^([^@]+)@(.+)$/u.exec(target.id) : null;
      const oracle = match ? { nodeId: match[1], portId: match[2] } : null;
      expect(oracle).toEqual(row.channel);
      expect(nodeGraphPickChannel(target)).toEqual(row.channel);
    }
    const hostile = structuredClone(graphPickFixture);
    Object.assign(hostile[1]!.target!, { portId: "invented" });
    expect(validateTarget(hostile[1]!.target)).toBe(false);
  });
  it("validates strict language-neutral graph slider labels and rejects unnamed rows", () => {
    const validate = peerExport(dagVcsSchema,"SliderOverlayRow");
    for(const item of graphSliderFixture.cases)expect(validate(item.row),JSON.stringify(validate.errors)).toBe(true);
    for (const label of ["", "   ", null, 42]) {
      const malformed = structuredClone(graphSliderFixture);
      (malformed.cases[0]!.row as Record<string, unknown>).label = label;
      expect(validate(malformed.cases[0]!.row)).toBe(false);
      expect(parseDagSliderOverlays(JSON.stringify({ sliders: [malformed.cases[0]!.row] }))).toEqual([]);
    }
    expect(parseDagSliderOverlays('{"sliders":{}}')).toEqual([]);
  });
  it("names graph slider overlays from exact localized captions and keeps scoped ids stable", () => {
    const first = graphSliderFixture.cases[0]!;
    const second = graphSliderFixture.cases[1]!;
    const props = (item: typeof first) => ({
      scopeId: item.scopeId,
      stateJson: JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [item.row] }),
      logicalW: 800,
      logicalH: 600,
      editable: true,
      onSliderChange: () => {},
    });
    const view = render(createElement("div", {}, createElement(GraphSliderOverlays, props(first)), createElement(GraphSliderOverlays, props(second))));
    const english = view.getByRole("slider", { name: first.row.label });
    const german = view.getByRole("slider", { name: second.row.label });
    expect(computeAccessibleName(english)).toBe(first.row.label);
    expect(computeAccessibleName(german)).toBe(second.row.label);
    const controlId = english.closest('[data-slot="slider"]')!.id;
    expect(controlId).toBe(`graph-slider-${encodeURIComponent(JSON.stringify([first.scopeId, first.row.widgetId]))}`);
    expect(german.closest('[data-slot="slider"]')!.id).not.toBe(controlId);
    view.rerender(createElement("div", {}, createElement(GraphSliderOverlays, { ...props(first), stateJson: JSON.stringify({ sliders: [{ ...first.row, value: 3 }] }) }), createElement(GraphSliderOverlays, props(second))));
    expect(view.getByRole("slider", { name: first.row.label }).closest('[data-slot="slider"]')!.id).toBe(controlId);
  });
  it("keeps graph slider keyboard changes exact and disabled controls inert", () => {
    const item = graphSliderFixture.cases[1]!;
    const changes = vi.fn();
    const gesture = vi.fn();
    const props = { scopeId: item.scopeId, stateJson: JSON.stringify({ sliders: [item.row] }), logicalW: 800, logicalH: 600, editable: true, onSliderChange: changes, onSliderPointerDown: gesture };
    const view = render(createElement(GraphSliderOverlays, props));
    const slider = view.getByRole("slider", { name: item.row.label });
    slider.focus();
    expect(document.activeElement).toBe(slider);
    for (const action of graphSliderFixture.keyboard) {
      fireEvent.keyDown(slider, { key: action.key });
      fireEvent.keyUp(slider, { key: action.key });
      expect(changes).toHaveBeenLastCalledWith(item.row.widgetId, action.expected);
    }
    expect(gesture).not.toHaveBeenCalled();
    changes.mockClear();
    view.rerender(createElement(GraphSliderOverlays, { ...props, editable: false }));
    const disabled = view.getByRole("slider", { name: item.row.label });
    expect(disabled.getAttribute("aria-disabled")).toBe("true");
    expect(disabled.tabIndex).toBe(-1);
    fireEvent.keyDown(disabled, { key: "ArrowRight" });
    fireEvent.keyUp(disabled, { key: "ArrowRight" });
    expect(changes).not.toHaveBeenCalled();
  });
  /** ⚖️ LAW (scrub protocol, `📓️api-scrub-machine.md`): a graph slider cancels its open press like every continuous
   * control — a lost pointer capture as `captureLost`, a blur as `blur` — so the guest drops the press with zero trace. */
  it("cancels a graph slider press on lost capture and on blur", () => {
    const item = graphSliderFixture.cases[1]!;
    const aborts = vi.fn();
    const view = render(createElement(GraphSliderOverlays, { scopeId: item.scopeId, stateJson: JSON.stringify({ sliders: [item.row] }), logicalW: 800, logicalH: 600, editable: true, onSliderChange: vi.fn(), onSliderAbort: aborts }));
    const slider = view.getByRole("slider", { name: item.row.label });
    const root = slider.closest('[data-slot="slider"]')!;
    fireEvent.pointerDown(root, { pointerId: 1, clientX: 0 });
    fireEvent.pointerCancel(root, { pointerId: 1 });
    expect(aborts).toHaveBeenLastCalledWith(item.row.widgetId, "captureLost");
    slider.focus();
    fireEvent.blur(slider);
    expect(aborts).toHaveBeenLastCalledWith(item.row.widgetId, "blur");
  });
  /** ⚖️ LAW: a knob has ONE value. The readout painted beside the track and the `aria-valuenow` a
   * screen reader announces are two renderings of the same published number, so they can never
   * disagree — the readout used to be painted on the GPU, where it moved only when a frame was
   * drawn, and a released knob read `10` to a screen reader while the canvas said `0.0`
   * (`📓️slider-reevaluation-correctness-2026-09-15.md`). */
  it("renders a graph slider's painted readout and its aria value from one published number", () => {
    const item = graphSliderFixture.cases[0]!;
    const rowAt = (value: number) => ({ ...item.row, value, fontScreenPx: 9, gapScreenPx: 4 });
    const propsAt = (value: number) => ({
      scopeId: item.scopeId,
      stateJson: JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [rowAt(value)] }),
      logicalW: 800,
      logicalH: 600,
      editable: true,
      onSliderChange: () => {},
    });
    const view = render(createElement(GraphSliderOverlays, propsAt(item.row.min)));
    for (const value of [item.row.min, (item.row.min + item.row.max) / 2, item.row.max]) {
      view.rerender(createElement(GraphSliderOverlays, propsAt(value)));
      const knob = view.getByRole("slider", { name: item.row.label });
      const readout = view.container.querySelector(`[data-graph-slider-value="${item.row.widgetId}"]`);
      expect(readout, "the overlay paints the value beside the track it belongs to").not.toBeNull();
      expect(readout!.textContent).toBe(dagSliderValueText(value));
      expect(readout!.textContent).toBe(dagSliderValueText(Number(knob.getAttribute("aria-valuenow"))));
      expect(readout!.getAttribute("aria-hidden")).toBe("true");
    }
    const parsed = parseDagSliderOverlays(JSON.stringify({ sliders: [rowAt(item.row.max)] }))[0]!;
    expect(parsed.value).toBe(item.row.max);
    expect(dagSliderValueText(parsed.value)).toBe(view.container.querySelector(`[data-graph-slider-value="${item.row.widgetId}"]`)!.textContent);
  });
  //#endregion 🎚️GraphSliderAccessibility
}
