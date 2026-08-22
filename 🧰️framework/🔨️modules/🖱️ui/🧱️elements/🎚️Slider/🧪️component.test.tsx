// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Slider, clampSliderValuesToReady, normalizeSliderRange, normalizeSliderValues, resolveSliderDraftClear, sliderValuesMatch } from "./🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🎚️SliderMatrix
describe("Slider", () => {
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
});
// #endregion 🎚️SliderMatrix
