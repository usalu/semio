// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import sliderPresentationFixture from "../../../../🧫️fixtures/🎚️slider-presentation/🔣️.json";
import numberControlsFixture from "../../../../🧬️contract/🧫️fixtures/🧫️number-controls/🔣️.json";

import { Slider, clampSliderValuesToReady, normalizeSliderRange, normalizeSliderValues, resolveSliderDraftClear, sliderValuesMatch } from "../../🟦️.tsx";
import { sliderAxisPosition } from "../../../../🧬️contract/🧩️component/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎚️SliderMatrix
describe("Slider", () => {
  it("matches the neutral track/readout geometry and track-only pointer contract", () => {
    const row = sliderPresentationFixture.cases.find(candidate => candidate.id === "outer-rtl-inner-ltr")!;
    const changes = vi.fn();
    const { container } = render(
      <div dir="rtl">
        <Slider id="slider.presentation" value={[row.value]} min={row.min} max={row.max} step={0.1} aria-valuetext="2.5 millimetres" onValueChange={changes} />
      </div>,
    );
    const slider = container.querySelector<HTMLElement>('[data-slot="slider"]')!;
    const track = container.querySelector<HTMLElement>('[data-slot="slider-track"]')!;
    const trackCell = container.querySelector<HTMLElement>('[data-slot="slider-track-cell"]')!;
    const valueCell = container.querySelector<HTMLElement>('[data-slot="slider-value"]')!;
    const thumb = container.querySelector<HTMLElement>('[data-slot="slider-thumb"]')!;
    expect(slider.getAttribute("dir")).toBe("ltr");
    expect(trackCell.nextElementSibling).toBe(valueCell);
    expect(track.className).toContain("h-single");
    expect(thumb.className).toContain("size-small");
    expect(thumb.getAttribute("aria-valuetext")).toBe("2.5 millimetres");
    expect(valueCell.textContent).toBe(row.formatted);
    track.getBoundingClientRect = () => ({ left: row.trackCell[0], right: row.trackCell[0] + row.trackCell[2], top: row.trackCell[1], bottom: row.trackCell[1] + row.trackCell[3], width: row.trackCell[2], height: row.trackCell[3] }) as DOMRect;
    fireEvent.pointerDown(valueCell, { pointerId: 1, clientX: row.valueCell[0] + 1, clientY: row.valueCell[1] + 1 });
    expect(changes).not.toHaveBeenCalled();
    const trackPointer = sliderPresentationFixture.pointer.find(pointer => pointer.case === row.id)!;
    fireEvent.pointerDown(track, { pointerId: 2, clientX: trackPointer.point[0], clientY: trackPointer.point[1] });
    expect(changes).toHaveBeenLastCalledWith([trackPointer.emits]);
  });

  it("normalizes invalid ranges, steps, tuple values, and ready clamps", () => {
    expect(normalizeSliderRange(Number.NaN, Number.POSITIVE_INFINITY, 0)).toEqual({ min: 0, max: 0, step: 1 });
    expect(normalizeSliderRange(10, 5, -2)).toEqual({ min: 10, max: 10, step: 1 });
    expect(normalizeSliderValues([8, Number.NaN, -1, 4.49], { min: 0, max: 5, step: 0.5 })).toEqual([0, 0, 4.5, 5]);
    expect(clampSliderValuesToReady([10, 80], 40, 0)).toEqual([10, 40]);
    expect(resolveSliderDraftClear([42], [10], 1)).toEqual([42]);
    expect(resolveSliderDraftClear([42], [42], 1)).toBeNull();
    expect(sliderValuesMatch([0.5], [0.51], 0.1)).toBe(true);
  });

  it("owns default state and commits one repeated keyboard gesture exactly once", () => {
    const change = vi.fn();
    const commit = vi.fn();
    const { getByRole } = render(<Slider id="slider.keyboard" defaultValue={[10]} min={0} max={100} step={2} onValueChange={change} onValueCommit={commit} />);
    const thumb = getByRole("slider");
    expect(thumb.getAttribute("aria-valuenow")).toBe("10");
    fireEvent.keyDown(thumb, { key: "ArrowRight" });
    fireEvent.keyDown(thumb, { key: "ArrowRight", repeat: true });
    expect(change.mock.calls).toEqual([[[12]], [[14]]]);
    expect(thumb.getAttribute("aria-valuenow")).toBe("14");
    expect(commit).not.toHaveBeenCalled();
    fireEvent.keyUp(thumb, { key: "ArrowRight" });
    expect(commit).toHaveBeenCalledTimes(1);
    expect(commit).toHaveBeenCalledWith([14]);
  });

  it("keeps a controlled draft visible until the external tuple catches up", () => {
    const change = vi.fn();
    const { getByRole, rerender } = render(<Slider id="slider.controlled" value={[10]} min={0} max={100} step={1} onValueChange={change} />);
    const thumb = getByRole("slider");
    fireEvent.keyDown(thumb, { key: "ArrowRight" });
    expect(change).toHaveBeenCalledWith([11]);
    expect(thumb.getAttribute("aria-valuenow")).toBe("11");
    rerender(<Slider id="slider.controlled" value={[10]} min={0} max={100} step={1} onValueChange={change} />);
    expect(thumb.getAttribute("aria-valuenow")).toBe("11");
    rerender(<Slider id="slider.controlled" value={[11]} min={0} max={100} step={1} onValueChange={change} />);
    expect(thumb.getAttribute("aria-valuenow")).toBe("11");
  });

  it("preserves logical thumb identity and focus through keyboard and pointer crossings", () => {
    const change = vi.fn();
    const commit = vi.fn();
    const { container, getAllByRole } = render(<Slider id="slider.multiple" defaultValue={[20, 80]} min={0} max={100} step={10} onValueChange={change} onValueCommit={commit} />);
    const root = container.querySelector('[data-slot="slider"]') as HTMLDivElement;
    const track = container.querySelector('[data-slot="slider-track"]') as HTMLDivElement;
    track.getBoundingClientRect = () => ({ left: 0, right: 100, top: 0, bottom: 10, width: 100, height: 10 }) as DOMRect;
    const logicalThumb = getAllByRole("slider")[0]!;
    const thumbId = logicalThumb.dataset.sliderThumbId;
    logicalThumb.focus();
    fireEvent.keyDown(logicalThumb, { key: "End" });
    fireEvent.keyUp(logicalThumb, { key: "End" });
    expect(change).toHaveBeenLastCalledWith([80, 100]);
    expect(document.activeElement).toBe(logicalThumb);
    expect(logicalThumb.dataset.sliderThumbId).toBe(thumbId);
    expect(logicalThumb.getAttribute("aria-valuenow")).toBe("100");

    fireEvent.keyDown(logicalThumb, { key: "ArrowLeft" });
    fireEvent.keyUp(logicalThumb, { key: "ArrowLeft" });
    expect(change).toHaveBeenLastCalledWith([80, 90]);
    expect(document.activeElement).toBe(logicalThumb);
    expect(logicalThumb.getAttribute("aria-valuenow")).toBe("90");

    fireEvent.pointerDown(logicalThumb, { pointerId: 9, clientX: 90, clientY: 5 });
    fireEvent.pointerMove(root, { pointerId: 9, clientX: 70, clientY: 5 });
    expect(change).toHaveBeenLastCalledWith([70, 80]);
    expect(document.activeElement).toBe(logicalThumb);
    expect(logicalThumb.getAttribute("aria-valuenow")).toBe("70");
    fireEvent.pointerUp(root, { pointerId: 9, clientX: 70, clientY: 5 });
    expect(commit).toHaveBeenCalledTimes(3);
  });

  it("enforces optional minimum thumb steps", () => {
    const change = vi.fn();
    const { getAllByRole } = render(<Slider id="slider.multiple-gap" value={[20, 30]} min={0} max={100} step={10} minStepsBetweenThumbs={2} onValueChange={change} />);
    change.mockClear();
    fireEvent.keyDown(getAllByRole("slider")[0]!, { key: "ArrowRight" });
    expect(change).not.toHaveBeenCalled();
  });

  it("suppresses exact min/max keyboard and pointer no-op changes and commits", () => {
    const change = vi.fn();
    const commit = vi.fn();
    const { container, getByRole, rerender } = render(<Slider id="slider.noop" value={[0]} min={0} max={100} step={10} onValueChange={change} onValueCommit={commit} />);
    const thumb = getByRole("slider");
    fireEvent.keyDown(thumb, { key: "ArrowLeft" });
    fireEvent.keyUp(thumb, { key: "ArrowLeft" });
    fireEvent.keyDown(thumb, { key: "Home" });
    fireEvent.keyUp(thumb, { key: "Home" });
    expect(change).not.toHaveBeenCalled();
    expect(commit).not.toHaveBeenCalled();

    rerender(<Slider id="slider.noop" value={[100]} min={0} max={100} step={10} onValueChange={change} onValueCommit={commit} />);
    const root = container.querySelector('[data-slot="slider"]') as HTMLDivElement;
    const track = container.querySelector('[data-slot="slider-track"]') as HTMLDivElement;
    track.getBoundingClientRect = () => ({ left: 0, right: 100, top: 0, bottom: 10, width: 100, height: 10 }) as DOMRect;
    const maxThumb = getByRole("slider");
    fireEvent.keyDown(maxThumb, { key: "ArrowRight" });
    fireEvent.keyUp(maxThumb, { key: "ArrowRight" });
    fireEvent.pointerDown(maxThumb, { pointerId: 10, clientX: 100, clientY: 5 });
    fireEvent.pointerMove(root, { pointerId: 10, clientX: 150, clientY: 5 });
    fireEvent.pointerUp(root, { pointerId: 10, clientX: 150, clientY: 5 });
    expect(change).not.toHaveBeenCalled();
    expect(commit).not.toHaveBeenCalled();
  });

  it("captures pointer movement, changes during drag, and commits once on release", () => {
    const change = vi.fn();
    const commit = vi.fn();
    const pointerDown = vi.fn();
    const pointerUp = vi.fn();
    const { container } = render(<Slider id="slider.pointer" defaultValue={[10]} min={0} max={100} step={1} onValueChange={change} onValueCommit={commit} onPointerDown={pointerDown} onPointerUp={pointerUp} />);
    const root = container.querySelector('[data-slot="slider"]') as HTMLDivElement;
    const track = container.querySelector('[data-slot="slider-track"]') as HTMLDivElement;
    track.getBoundingClientRect = () => ({ left: 0, right: 100, top: 0, bottom: 10, width: 100, height: 10 }) as DOMRect;
    root.setPointerCapture = vi.fn();
    root.releasePointerCapture = vi.fn();
    fireEvent.pointerDown(track, { pointerId: 7, clientX: 25, clientY: 5 });
    fireEvent.pointerMove(root, { pointerId: 7, clientX: 60, clientY: 5 });
    expect(change).toHaveBeenNthCalledWith(1, [25]);
    expect(change).toHaveBeenNthCalledWith(2, [60]);
    expect(commit).not.toHaveBeenCalled();
    fireEvent.pointerUp(root, { pointerId: 7, clientX: 60, clientY: 5 });
    expect(commit).toHaveBeenCalledTimes(1);
    expect(commit).toHaveBeenCalledWith([60]);
    expect(pointerDown).toHaveBeenCalledTimes(1);
    expect(pointerUp).toHaveBeenCalledTimes(1);
  });

  it("rolls a cancelled pointer gesture back without committing", () => {
    const change = vi.fn();
    const commit = vi.fn();
    const cancel = vi.fn();
    const { container, getByRole } = render(<Slider id="slider.cancel" defaultValue={[20]} min={0} max={100} onValueChange={change} onValueCommit={commit} onPointerCancel={cancel} />);
    const root = container.querySelector('[data-slot="slider"]') as HTMLDivElement;
    const track = container.querySelector('[data-slot="slider-track"]') as HTMLDivElement;
    track.getBoundingClientRect = () => ({ left: 0, right: 100, top: 0, bottom: 10, width: 100, height: 10 }) as DOMRect;
    fireEvent.pointerDown(track, { pointerId: 4, clientX: 70, clientY: 5 });
    fireEvent.pointerCancel(root, { pointerId: 4 });
    expect(change).toHaveBeenLastCalledWith([20]);
    expect(getByRole("slider").getAttribute("aria-valuenow")).toBe("20");
    expect(commit).not.toHaveBeenCalled();
    expect(cancel).toHaveBeenCalledTimes(1);
  });

  it("maps arrows, pages, Home, and End through RTL and vertical orientation", () => {
    const rtlChange = vi.fn();
    const verticalChange = vi.fn();
    const { getByRole, unmount } = render(<Slider id="slider.rtl" value={[50]} min={0} max={100} dir="rtl" onValueChange={rtlChange} />);
    fireEvent.keyDown(getByRole("slider"), { key: "ArrowRight" });
    expect(rtlChange).toHaveBeenCalledWith([49]);
    unmount();

    const vertical = render(<Slider id="slider.vertical" defaultValue={[50]} min={0} max={100} orientation="vertical" onValueChange={verticalChange} />);
    const thumb = vertical.getByRole("slider");
    expect(thumb.getAttribute("aria-orientation")).toBe("vertical");
    fireEvent.keyDown(thumb, { key: "ArrowUp" });
    fireEvent.keyDown(thumb, { key: "PageUp" });
    fireEvent.keyDown(thumb, { key: "Home" });
    fireEvent.keyDown(thumb, { key: "End" });
    expect(verticalChange.mock.calls.map(([tuple]) => tuple)).toEqual([[51], [61], [0], [100]]);
  });

  it("exposes complete thumb ARIA and suppresses disabled and read-only interaction", () => {
    const disabledChange = vi.fn();
    const { getByRole, unmount } = render(<Slider id="slider.disabled" value={[5]} min={0} max={10} disabled aria-label="Volume" onValueChange={disabledChange} />);
    const disabledThumb = getByRole("slider");
    expect(disabledThumb.getAttribute("aria-label")).toBe("Volume");
    expect(disabledThumb.getAttribute("aria-valuemin")).toBe("0");
    expect(disabledThumb.getAttribute("aria-valuemax")).toBe("10");
    expect(disabledThumb.getAttribute("aria-valuenow")).toBe("5");
    expect(disabledThumb.tabIndex).toBe(-1);
    fireEvent.keyDown(disabledThumb, { key: "ArrowRight" });
    expect(disabledChange).not.toHaveBeenCalled();
    unmount();

    const readOnlyChange = vi.fn();
    const readOnly = render(<Slider id="slider.readonly" value={[5]} min={0} max={10} readOnly onValueChange={readOnlyChange} />);
    const readOnlyThumb = readOnly.getByRole("slider");
    expect(readOnlyThumb.getAttribute("aria-readonly")).toBe("true");
    expect(readOnlyThumb.tabIndex).toBe(0);
    fireEvent.keyDown(readOnlyThumb, { key: "ArrowRight" });
    expect(readOnlyChange).not.toHaveBeenCalled();
  });

  it("applies thumbClassName instead of the default thumb extent token", () => {
    const { container } = render(<Slider id="slider.compact-thumb" value={[4]} min={0} max={10} thumbClassName="size-tiny" />);
    const thumb = container.querySelector('[data-slot="slider-thumb"]') as HTMLElement;
    expect(thumb.className).toContain("size-tiny");
    expect(thumb.className).not.toContain("size-small");
  });

  it("keeps ready extent presentation and hard ready clamping distinct", () => {
    const change = vi.fn();
    const { container, getByRole } = render(<Slider id="slider.ready" value={[20]} min={0} max={100} ready={55} clampToReady onValueChange={change} />);
    const ready = container.querySelector('[data-slot="slider-ready"]') as HTMLElement;
    expect(ready.style.left).toBe("20%");
    expect(ready.style.width).toBe("35%");
    fireEvent.keyDown(getByRole("slider"), { key: "End" });
    expect(change).toHaveBeenCalledWith([55]);
    expect(getByRole("slider").getAttribute("aria-valuenow")).toBe("55");
  });

  it("lands pointer gestures on the shared detents, jumps page keys between them and paints one tick each", () => {
    
    
    for (const row of numberControlsFixture.pointer) {
      const change = vi.fn();
      const start = row.expected === row.min ? row.max : row.min;
      const scale = "scale" in row && row.scale === "log" ? "log" : "linear";
      const { container, unmount } = render(<Slider id={`slider.${row.case}`} value={[start]} min={row.min} max={row.max} step={row.step || undefined} snapValues={row.snaps} scale={scale} onValueChange={change} />);
      const track = container.querySelector<HTMLElement>('[data-slot="slider-track"]')!;
      track.getBoundingClientRect = () => ({ left: 0, right: 1000, top: 0, bottom: 4, width: 1000, height: 4 }) as DOMRect;
      expect(container.querySelectorAll('[data-slot="slider-tick"]').length).toBe(row.snaps.length);
      fireEvent.pointerDown(track, { pointerId: 1, clientX: sliderAxisPosition(row.value, row.min, row.max, scale) * 1000, clientY: 2 });
      if (row.step > 0) expect(change, row.case).toHaveBeenLastCalledWith([row.expected]);
      else expect(change.mock.lastCall?.[0]?.[0], row.case).toBe(row.expected);
      unmount();
    }
    for (const row of numberControlsFixture.adjacent) {
      const change = vi.fn();
      const { getByRole, unmount } = render(<Slider id={`slider.${row.case}`} value={[row.current]} min={0} max={10} step={0.5} snapValues={row.snaps} onValueChange={change} />);
      fireEvent.keyDown(getByRole("slider"), { key: row.forward ? "PageUp" : "PageDown" });
      const fallback = Math.min(10, Math.max(0, row.current + (row.forward ? 5 : -5)));
      const landed = row.expected ?? fallback;
      if (landed === row.current) expect(change, row.case).not.toHaveBeenCalled();
      else expect(change, row.case).toHaveBeenLastCalledWith([landed]);
      unmount();
    }
  });

  it("keeps an off-step detent exactly while arrows walk the next rung beyond it", () => {
    const change = vi.fn();
    const { getByRole } = render(<Slider id="slider.off-step-detent" value={[3.3]} min={0} max={10} step={0.5} snapValues={[3.3]} onValueChange={change} />);
    const thumb = getByRole("slider");
    expect(thumb.getAttribute("aria-valuenow")).toBe("3.3");
    fireEvent.keyDown(thumb, { key: "ArrowRight" });
    expect(change).toHaveBeenLastCalledWith([3.5]);
  });

  it("answers every bounded shared keyboard-law row through the physical keys", () => {
    const physical = { increment: "ArrowRight", decrement: "ArrowLeft", pageUp: "PageUp", pageDown: "PageDown", home: "Home", end: "End" } as const;
    for (const row of numberControlsFixture.keys) {
      if (row.min == null || row.max == null || !(row.step > 0)) continue;
      const rung = row.step > 0 ? row.step : 1;
      const settled = row.snaps.includes(row.current) || Math.abs((row.current - row.min) / rung - Math.round((row.current - row.min) / rung)) < 1e-9;
      if (!settled) continue;
      const change = vi.fn();
      const precision = "precision" in row ? row.precision : null;
      const factor = "factor" in row ? row.factor : null;
      const { getByRole, unmount } = render(<Slider id={`slider.${row.case}`} value={[row.current]} min={row.min} max={row.max} step={row.step || undefined} snapValues={row.snaps} precision={precision} displayFactor={factor} onValueChange={change} />);
      fireEvent.keyDown(getByRole("slider"), { key: physical[row.key as keyof typeof physical], shiftKey: row.large });
      if (row.expected === row.current) expect(change, row.case).not.toHaveBeenCalled();
      else expect(change, row.case).toHaveBeenLastCalledWith([row.expected]);
      unmount();
    }
  });

  it("publishes a pointer landing on a detent within a quarter step of the current ladder value", () => {
    const change = vi.fn();
    const { container } = render(<Slider id="slider.quarter-step-detent" defaultValue={[5]} min={0} max={10} step={0.5} snapValues={[5.1]} onValueChange={change} />);
    const track = container.querySelector<HTMLElement>('[data-slot="slider-track"]')!;
    track.getBoundingClientRect = () => ({ left: 0, right: 1000, top: 0, bottom: 4, width: 1000, height: 4 }) as DOMRect;
    fireEvent.pointerDown(track, { pointerId: 1, clientX: 505, clientY: 2 });
    expect(change).toHaveBeenLastCalledWith([5.1]);
    expect(sliderValuesMatch([5.1], [5], 0.5, [5.1])).toBe(false);
    expect(resolveSliderDraftClear([5.1], [5], 0.5, [5.1])).toEqual([5.1]);
  });

  it("draws a degree dial: one tick per detent, display-unit ARIA numbers, a pointer angle snapping onto 90° and one-degree arrows", () => {
    const degrees = 180 / Math.PI;
    const snaps = [-Math.PI, -Math.PI / 2, 0, Math.PI / 2, Math.PI];
    const change = vi.fn();
    const { container, getByRole } = render(<Slider id="slider.dial" appearance="dial" value={[0]} min={-Math.PI} max={Math.PI} step={Math.PI / 180} snapValues={snaps} displayFactor={degrees} aria-valuetext="0 °" onValueChange={change} />);
    expect(container.querySelector('[data-slot="slider-dial"]')).not.toBeNull();
    expect(container.querySelectorAll('[data-slot="slider-tick"]').length).toBe(5);
    const thumb = getByRole("slider");
    expect([thumb.getAttribute("aria-valuemin"), thumb.getAttribute("aria-valuemax"), thumb.getAttribute("aria-valuenow"), thumb.getAttribute("aria-valuetext")]).toEqual(["-180", "180", "0", "0 °"]);
    expect(container.querySelector('[data-slot="slider-value"]')!.textContent).toBe("0");
    const dial = container.querySelector<HTMLElement>('[data-slot="slider-dial"]')!;
    dial.getBoundingClientRect = () => ({ left: 0, right: 40, top: 0, bottom: 40, width: 40, height: 40 }) as DOMRect;
    const towards = (angle: number) => ({ pointerId: 1, clientX: 20 + Math.cos(angle) * 18, clientY: 20 - Math.sin(angle) * 18 });
    fireEvent.pointerDown(dial, towards((88 * Math.PI) / 180));
    expect(change).toHaveBeenLastCalledWith([Math.PI / 2]);
    fireEvent.pointerUp(dial, { pointerId: 1 });
    fireEvent.keyDown(thumb, { key: "ArrowRight" });
    expect(change.mock.lastCall?.[0]?.[0] * degrees).toBeCloseTo(91, 9);
  });

  it("places log-axis ticks at their log position and lands a pointer on the nearest log detent", () => {
    const change = vi.fn();
    const { container } = render(<Slider id="slider.log" scale="log" value={[1]} min={0.1} max={10} step={0.01} precision={2} snapValues={[0.25, 0.5, 1, 2, 4]} onValueChange={change} />);
    const ticks = Array.from(container.querySelectorAll<HTMLElement>('[data-slot="slider-tick"]')).map((tick) => Number.parseFloat(tick.style.left));
    [0.25, 0.5, 1, 2, 4].forEach((snap, index) => expect(ticks[index]).toBeCloseTo(sliderAxisPosition(snap, 0.1, 10, "log") * 100, 9));
    expect(ticks[2]).toBeCloseTo(50, 9);
    expect(container.querySelector('[data-slot="slider-value"]')!.textContent).toBe("1.00");
    const track = container.querySelector<HTMLElement>('[data-slot="slider-track"]')!;
    track.getBoundingClientRect = () => ({ left: 0, right: 1000, top: 0, bottom: 4, width: 1000, height: 4 }) as DOMRect;
    fireEvent.pointerDown(track, { pointerId: 1, clientX: sliderAxisPosition(3.7, 0.1, 10, "log") * 1000, clientY: 2 });
    expect(change).toHaveBeenLastCalledWith([4]);
  });

  it("refuses a typed value beyond a hard bound visibly, keeping the draft, and commits one beyond the soft travel exactly", () => {
    const change = vi.fn();
    const commit = vi.fn();
    const limits = { min: { value: 0, exclusive: true, refusal: "Must be greater than 0" }, max: null };
    const { container, getByRole, queryByRole } = render(<Slider id="slider.factor" scale="log" value={[1]} min={0.1} max={10} step={0.01} precision={2} limits={limits} onValueChange={change} onValueCommit={commit} />);
    fireEvent.doubleClick(container.querySelector('[data-slot="slider-value"]')!);
    const field = container.querySelector<HTMLInputElement>('input[type="number"]')!;
    fireEvent.change(field, { target: { value: "-5" } });
    fireEvent.keyDown(field, { key: "Enter" });
    expect(getByRole("alert").textContent).toBe("Must be greater than 0");
    expect(field.getAttribute("aria-invalid")).toBe("true");
    expect(field.getAttribute("aria-describedby")).toBe("slider.factor-refusal");
    expect(container.querySelector<HTMLInputElement>('input[type="number"]')!.value).toBe("-5");
    expect(change).not.toHaveBeenCalled();
    expect(commit).not.toHaveBeenCalled();
    fireEvent.change(field, { target: { value: "20" } });
    expect(queryByRole("alert")).toBeNull();
    fireEvent.keyDown(field, { key: "Enter" });
    expect(commit).toHaveBeenLastCalledWith([20]);
    expect(container.querySelector('input[type="number"]')).toBeNull();
  });
});
// #endregion 🎚️SliderMatrix

import exactEntry from "../../🧫️fixtures/⌨️exact-entry/🔣️.json";
import { computeAccessibleName } from "dom-accessibility-api";


for (const key of exactEntry.keys) it(`opens slider exact entry with ${key} and restores keyboard focus`, () => {
  const commit = vi.fn();
  const view = render(<Slider id="slider.exact-entry" aria-label={exactEntry.label} defaultValue={[exactEntry.initial]} min={0} max={1} step={0.01} onValueCommit={commit} />);
  const readout = view.container.querySelector<HTMLElement>('[data-slot="slider-value"]')!;
  const thumb = view.getByRole("slider");
  expect(readout.tabIndex).toBe(0);
  expect(thumb.className).toContain("focus-visible:ring-2");
  readout.focus();
  fireEvent.keyDown(readout, { key });
  const editor = view.getByRole("spinbutton");
  expect(computeAccessibleName(editor)).toBe(exactEntry.label);
  fireEvent.change(editor, { target: { value: exactEntry.typed } });
  fireEvent.keyDown(editor, { key: "Enter" });
  expect(commit).toHaveBeenCalledExactlyOnceWith([exactEntry.expected]);
  expect(document.activeElement).toBe(view.container.querySelector('[data-slot="slider-value"]'));
});

for (const blocked of exactEntry.blocked) it(`keeps ${blocked} slider exact entry unavailable`, () => {
  const view = render(<Slider id="slider.exact-disabled" defaultValue={[exactEntry.initial]} min={0} max={1} {...{ [blocked]: true }} />);
  const readout = view.container.querySelector<HTMLElement>('[data-slot="slider-value"]')!;
  expect(readout.tabIndex).toBe(-1);
  for (const key of exactEntry.keys) fireEvent.keyDown(readout, { key });
  expect(view.queryByRole("spinbutton")).toBeNull();
});
