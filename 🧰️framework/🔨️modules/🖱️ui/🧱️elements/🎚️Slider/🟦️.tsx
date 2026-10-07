// #region 🧲️Header
// 💻️ framework/ui/elements/🎚️Slider/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { PropertyValueColumnContext } from "../🌳️Tree/🟦️.tsx";
import { formatNumber, Input } from "../✏️Input/🟦️.tsx";
import { loadingBorderStateClass, waitingBorderStateClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️.ts";
import { useLabel, useControlAccessibleLabel, Label } from "../🏷️Label/🟦️.tsx";
import { useInteractionCommands } from "../../🎯️targets/⚛️react/🟦️";
import { dialAngle, dialPosition, sliderAxisPosition, sliderAxisValue, sliderKeyValue, sliderPointerValue, uiNumberCrossedBound, uiNumberDisplayText, uiNumberTypedValue, type SliderKey } from "../../🧬️contract/🧩️component/🟦️.ts";
import { formatUiNumber } from "../../🧬️contract/🔢️number-format/🟦️.ts";
import type { SliderAppearance, UiNumberLimits, UiNumberScale } from "@semio-tech/framework";
// #endregion 🔌️Adapters

// #region 🏩️Slider
// Owned range slider.
// Consumers MUST provide min and max values.

/** 🎚️ Slider filled range presentation. */
const sliderRangeClassName = cn("bg-element absolute transition-[background-color] data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full", "group-hover:bg-emphasized", "data-[dragging=true]:bg-active-base");

/** 🎚️ Slider ready extent presentation. */
const sliderReadyClassName = cn("bg-[var(--accent-secondary)] pointer-events-none absolute data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full");

/** 🧲️ Slider detent tick presentation — one per snap, never interactive, never announced. */
const sliderTickClassName = cn("bg-element pointer-events-none absolute w-px data-[orientation=horizontal]:top-1/2 data-[orientation=horizontal]:h-small data-[orientation=horizontal]:-translate-x-1/2 data-[orientation=horizontal]:-translate-y-1/2 data-[orientation=vertical]:left-1/2 data-[orientation=vertical]:h-px data-[orientation=vertical]:w-small data-[orientation=vertical]:-translate-x-1/2 data-[orientation=vertical]:translate-y-1/2 opacity-60");

/** 🎚️ Slider thumb presentation (extent token applied per instance). */
const sliderThumbBaseClassName = cn(
  "block shrink-0 rounded-full bg-element transition-[background-color] outline-hidden",
  "hover:bg-emphasized group-hover:bg-emphasized",
  "focus-visible:bg-active-base focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2",
  "data-[dragging=true]:bg-active-base",
  "disabled:pointer-events-none disabled:opacity-50",
);

/** 🧭️ Dial knob presentation: a square that holds the rotary face, its detent ticks and needle. */
const sliderDialClassName = cn("relative aspect-square h-medium shrink-0 rounded-full", "data-[invalid=true]:ring-1 data-[invalid=true]:ring-destructive");

/** ⛔️ Typed-value refusal presentation: the producer's localized message naming the crossed bound. */
const sliderRefusalClassName = cn("text-destructive block text-xs leading-tight");

/** 🎚️ Slider numeric readout presentation. */
const sliderValueClassName = cn("text-element w-large text-end text-xs leading-none select-none transition-colors", "hover:text-emphasized group-hover:text-emphasized focus-visible:outline-2 focus-visible:outline-accent focus-visible:outline-offset-2");

// #region 📐️Contract
export type SliderOrientation = "horizontal" | "vertical";
export type SliderDirection = "ltr" | "rtl";
export type SliderValue = number[];

/** 🎚️ Owned slider root, tuple, interaction, and presentation contract. */
export interface SliderProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "id" | "defaultValue" | "dir" | "onChange" | "onPointerDown" | "onPointerUp" | "onPointerCancel" | "onKeyDown" | "onKeyUp">, ElementProps {
  defaultValue?: SliderValue;
  value?: SliderValue;
  min?: number;
  max?: number;
  step?: number;
  minStepsBetweenThumbs?: number;
  orientation?: SliderOrientation;
  dir?: SliderDirection;
  inverted?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  ready?: number;
  clampToReady?: boolean;
  loading?: boolean;
  waiting?: boolean;
  showLabel?: boolean;
  showValue?: boolean;
  formatDisplayValue?: (value: number) => string;
  onValueChange?: (values: SliderValue) => void;
  onValueCommit?: (values: SliderValue) => void;
  onPointerDown?: () => void;
  onPointerUp?: () => void;
  onPointerCancel?: () => void;
  interactionId?: string;
  /** 📍️ Detents (`SliderProps.snaps`): a pointer value within the shared radius lands on one, a page key stops on the first it reaches (the shared keyboard law), and each paints a tick at its axis position. */
  snapValues?: readonly number[];
  /** 📈️ How the travel maps onto the track (`SliderProps.scale`): evenly, or by ratio on a log axis. */
  scale?: UiNumberScale;
  /** 🎡️ How the travel is drawn (`SliderProps.appearance`): a straight track, or a rotary dial (one full counter-clockwise turn, the travel's centre at three o'clock). */
  appearance?: SliderAppearance;
  /** 🔁️ The readout, the spoken numbers and a typed value read `stored × displayFactor` (`SliderProps.displayFactor`); a typed value is divided back by the shared law. */
  displayFactor?: number | null;
  /** 🎯️ Fraction digits of the readout and a typed value, in display units (`SliderProps.precision`). */
  precision?: number | null;
  /** 🚧️ The hard range a typed value must keep (`SliderProps.limits`; the travel itself when absent): a crossing value is refused visibly with the bound's refusal and never dispatched. */
  limits?: UiNumberLimits | null;
  thumbClassName?: string;
}

export interface SliderRange {
  min: number;
  max: number;
  step: number;
}

/** 📏️ Normalizes non-finite and inverted numeric bounds to a safe, possibly degenerate range. */
export function normalizeSliderRange(min: number, max: number, step: number | undefined): SliderRange {
  const safeMin = Number.isFinite(min) ? min : 0;
  const safeMax = Number.isFinite(max) ? Math.max(safeMin, max) : safeMin;
  return { min: safeMin, max: safeMax, step: step != null && Number.isFinite(step) && step > 0 ? step : 1 };
}

function decimalPlaces(value: number): number {
  const text = value.toString().toLowerCase();
  if (text.includes("e-")) return Number(text.split("e-")[1] ?? 0);
  return text.split(".")[1]?.length ?? 0;
}

function normalizeSliderValue(value: number, range: SliderRange): number {
  const precision = Math.min(12, Math.max(decimalPlaces(range.min), decimalPlaces(range.step)));
  const finite = Number.isFinite(value) ? value : range.min;
  const snapped = Math.round((finite - range.min) / range.step) * range.step + range.min;
  return Math.min(range.max, Math.max(range.min, Number(snapped.toFixed(precision))));
}

/** 🧮️ Snaps, clamps, and sorts an owned slider tuple while preserving an empty tuple. */
export function normalizeSliderValues(values: readonly number[], range: SliderRange): SliderValue {
  return values.map((value) => normalizeSliderValue(value, range)).sort((lhs, rhs) => lhs - rhs);
}
// #endregion 📐️Contract

interface SliderGestureStart {
  values: SliderValue;
  thumbIds: string[];
}

/** 🟰️ Whether two slider value tuples match within a step-aware epsilon; a detent is its own value, so it matches only itself even within a quarter step of a ladder value. */
export function sliderValuesMatch(lhs: readonly number[], rhs: readonly number[], step?: number, snaps: readonly number[] = []): boolean {
  if (lhs.length !== rhs.length) return false;
  const epsilon = step != null && step > 0 ? step * 0.25 : 1e-9;
  return lhs.every((value, index) => {
    const other = rhs[index] ?? value;
    return Math.abs(value - other) <= (snaps.includes(value) || snaps.includes(other) ? 1e-9 : epsilon);
  });
}

/** 🧹️ Clears a pending draft once the controlled `value` prop catches up. */
export function resolveSliderDraftClear(pending: number[] | null, external: readonly number[], step?: number, snaps: readonly number[] = []): number[] | null {
  if (pending === null) return null;
  return sliderValuesMatch(pending, external, step, snaps) ? null : pending;
}

/** 🪣️ Clamps every value to `ready` (a preloaded/planned extent, e.g. a background fill plan's
 * progress) — the thumb must never be draggable past what's actually available, and `min` is the floor
 * so a `ready` of 0 still leaves the slider at its resting position rather than collapsing below `min`. */
export function clampSliderValuesToReady(values: readonly number[], ready: number | undefined, min: number): number[] {
  if (ready == null) return values.slice();
  const ceiling = Math.max(min, ready);
  return values.map((value) => Math.min(value, ceiling));
}

/** 🗝️ Maps a physical key onto the value axis: the horizontal arrows follow `dir`, and `inverted` swaps arrows and pages. */
export function sliderKeyOf(key: string, dir: SliderDirection, inverted: boolean): SliderKey | null {
  const forward = key === "ArrowUp" || key === (dir === "rtl" ? "ArrowLeft" : "ArrowRight");
  const backward = key === "ArrowDown" || key === (dir === "rtl" ? "ArrowRight" : "ArrowLeft");
  if (forward || backward) return forward !== inverted ? "increment" : "decrement";
  if (key === "PageUp" || key === "PageDown") return (key === "PageUp") !== inverted ? "pageUp" : "pageDown";
  return key === "Home" ? "home" : key === "End" ? "end" : null;
}

/** 🎚️ Renders an owned pointer- and keyboard-operable slider tuple. */
const Slider = React.forwardRef<HTMLDivElement, SliderProps>(function Slider(
  {
    className,
    defaultValue,
    value,
    min = 0,
    max = 100,
    ready,
    loading = false,
    waiting = false,
    showLabel,
    showValue = true,
    formatDisplayValue,
    onValueChange,
    onValueCommit,
    onPointerDown,
    onPointerUp,
    onPointerCancel,
    interactionId,
    id,
    snapValues,
    scale = "linear",
    appearance = "track",
    displayFactor = null,
    precision = null,
    limits = null,
    step,
    minStepsBetweenThumbs = 0,
    orientation = "horizontal",
    dir = "ltr",
    inverted = false,
    disabled = false,
    readOnly = false,
    clampToReady = false,
    thumbClassName,
    "aria-valuetext": ariaValueText,
    ...props
  },
  forwardedRef,
) {
  const thumbPresentationClassName = cn(sliderThumbBaseClassName, thumbClassName ?? "size-small");
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [isDragging, setIsDragging] = reactHostPort.useState(false);
  const pointerIdRef = reactHostPort.useRef<number | null>(null);
  const activeThumbIdRef = reactHostPort.useRef<string | null>(null);
  const gestureStartRef = reactHostPort.useRef<SliderGestureStart | null>(null);
  const gestureChangedRef = reactHostPort.useRef(false);
  const keyboardActiveRef = reactHostPort.useRef(false);
  const trackRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const readoutRef = reactHostPort.useRef<HTMLSpanElement | null>(null);
  const returnFocusRef = reactHostPort.useRef(false);
  const [editValue, setEditValue] = reactHostPort.useState("");
  const [hasBeenEdited, setHasBeenEdited] = reactHostPort.useState(false);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const editExactValueLabel = useLabel("ui.common.editExactValue");
  reactHostPort.useEffect(() => {
    if (!isEditing && returnFocusRef.current) { returnFocusRef.current = false; readoutRef.current?.focus(); }
  }, [isEditing]);
  const [refusal, setRefusal] = reactHostPort.useState<{ readonly message: string | null } | null>(null);
  const range = reactHostPort.useMemo(() => normalizeSliderRange(min, max, step), [max, min, step]);
  const snaps = reactHostPort.useMemo(() => snapValues ?? [], [snapValues]);
  const settle = reactHostPort.useCallback((value: number): number => (snaps.includes(value) ? value : precision != null ? Math.min(range.max, Math.max(range.min, value)) : normalizeSliderValue(value, range)), [precision, range, snaps]);
  const settleAll = reactHostPort.useCallback((values: readonly number[]): SliderValue => values.map(settle).sort((lhs, rhs) => lhs - rhs), [settle]);
  const controlled = Array.isArray(value);
  const [uncontrolledValues, setUncontrolledValues] = reactHostPort.useState<SliderValue>(() => settleAll(Array.isArray(defaultValue) ? defaultValue : [range.min, range.max]));
  const externalValues = reactHostPort.useMemo(() => settleAll(controlled ? value : uncontrolledValues), [controlled, settleAll, uncontrolledValues, value]);
  const [pendingDraftValues, setPendingDraftValues] = reactHostPort.useState<number[] | null>(null);
  const draftValues = controlled ? resolveSliderDraftClear(pendingDraftValues, externalValues, range.step, snaps) : null;
  reactHostPort.useEffect(() => {
    if (controlled) setPendingDraftValues((pending) => resolveSliderDraftClear(pending, externalValues, range.step, snaps));
    else
      setUncontrolledValues((current) => {
        const normalized = settleAll(current);
        return sliderValuesMatch(current, normalized, range.step, snaps) ? current : normalized;
      });
  }, [controlled, externalValues, range, settleAll, snaps]);
  const _values = draftValues ?? externalValues;
  const valuesRef = reactHostPort.useRef(_values);
  valuesRef.current = _values;
  const thumbIdBase = React.useId().replace(/[^A-Za-z0-9_-]/g, "");
  const nextThumbIdRef = reactHostPort.useRef(0);
  const thumbIdsRef = reactHostPort.useRef<string[]>([]);
  while (thumbIdsRef.current.length < _values.length) thumbIdsRef.current.push(`${thumbIdBase}-thumb-${nextThumbIdRef.current++}`);
  if (thumbIdsRef.current.length > _values.length) thumbIdsRef.current = thumbIdsRef.current.slice(0, _values.length);

  const displayValue = _values[0] ?? range.min;
  const displays = displayFactor != null || precision != null;
  const formatReadout = formatDisplayValue ?? (displays ? (value: number) => uiNumberDisplayText(value, displayFactor, precision) : formatNumber);
  const spoken = (value: number): number => (displayFactor == null ? value : Number(formatUiNumber(value * displayFactor)));
  const editorBound = (bound: { readonly value: number; readonly exclusive?: boolean } | null | undefined): number | undefined => (bound && !bound.exclusive ? spoken(bound.value) : undefined);
  const axisShare = (value: number): number => sliderAxisPosition(value, range.min, range.max, scale);
  const span = range.max - range.min;
  const readyExtent = ready == null || span <= 0 ? null : Math.min(range.max, Math.max(range.min, ready));
  const readyWidthPct = readyExtent == null || readyExtent <= displayValue || span <= 0 ? 0 : (sliderAxisPosition(readyExtent, range.min, range.max, scale) - sliderAxisPosition(displayValue, range.min, range.max, scale)) * 100;

  const publishValues = reactHostPort.useCallback(
    (rawValues: SliderValue, rawThumbIds: readonly string[] = thumbIdsRef.current): SliderValue => {
      const records = rawValues
        .map((rawValue, index) => ({ id: rawThumbIds[index] ?? `${thumbIdBase}-thumb-${nextThumbIdRef.current++}`, value: settle(rawValue) }))
        .map((record) => ({ ...record, value: clampToReady ? (clampSliderValuesToReady([record.value], ready, range.min)[0] ?? range.min) : record.value }))
        .map((record) => ({ ...record, value: settle(record.value) }))
        .sort((lhs, rhs) => lhs.value - rhs.value);
      const nextValues = records.map((record) => record.value);
      const minimumGap = Math.max(0, Number.isFinite(minStepsBetweenThumbs) ? minStepsBetweenThumbs : 0) * range.step;
      if (nextValues.some((next, index) => index > 0 && next - (nextValues[index - 1] ?? next) < minimumGap)) return valuesRef.current;
      const changed = !sliderValuesMatch(nextValues, valuesRef.current, range.step, snaps);
      if (!changed) return valuesRef.current;
      valuesRef.current = nextValues;
      thumbIdsRef.current = records.map((record) => record.id);
      gestureChangedRef.current = true;
      if (controlled) setPendingDraftValues(nextValues);
      else setUncontrolledValues(nextValues);
      onValueChange?.(nextValues.slice());
      return nextValues;
    },
    [clampToReady, controlled, minStepsBetweenThumbs, onValueChange, range, ready, settle, snaps, thumbIdBase],
  );

  const updateThumb = reactHostPort.useCallback(
    (thumbId: string, nextValue: number): SliderValue => {
      const index = thumbIdsRef.current.indexOf(thumbId);
      if (index < 0) return valuesRef.current;
      const next = valuesRef.current.slice();
      next[index] = nextValue;
      activeThumbIdRef.current = thumbId;
      return publishValues(next, thumbIdsRef.current);
    },
    [publishValues],
  );

  const beginGesture = reactHostPort.useCallback(() => {
    if (gestureStartRef.current) return;
    gestureStartRef.current = { values: valuesRef.current.slice(), thumbIds: thumbIdsRef.current.slice() };
    gestureChangedRef.current = false;
  }, []);

  const commitGesture = reactHostPort.useCallback(() => {
    if (!gestureStartRef.current) return;
    const changed = gestureChangedRef.current;
    gestureStartRef.current = null;
    gestureChangedRef.current = false;
    if (changed) onValueCommit?.(valuesRef.current.slice());
  }, [onValueCommit]);

  const cancelGesture = reactHostPort.useCallback(() => {
    const start = gestureStartRef.current;
    if (!start) return;
    const changed = gestureChangedRef.current;
    gestureStartRef.current = null;
    gestureChangedRef.current = false;
    if (changed) {
      valuesRef.current = start.values;
      thumbIdsRef.current = start.thumbIds;
      if (controlled) setPendingDraftValues(start.values);
      else setUncontrolledValues(start.values);
      onValueChange?.(start.values.slice());
    }
    onPointerCancel?.();
  }, [controlled, onPointerCancel, onValueChange]);

  const handleValueClick = () => {
    if (disabled || readOnly) return;
    if (!hasBeenEdited) setHasBeenEdited(true);
    setEditValue(formatReadout(displayValue));
    setIsEditing(true);
  };

  /** ⌨️ A typed readout reads in display units: unreadable text or a value crossing a hard bound is refused (the draft kept, the
   * bound's refusal shown); an admitted value — inside the limits, even beyond a soft travel — commits exactly, never clamped. */
  const commitTyped = (): boolean => {
    const typed = Number(editValue.trim());
    if (editValue.trim() === "" || !Number.isFinite(typed)) {
      setRefusal({ message: null });
      return false;
    }
    const stored = uiNumberTypedValue(typed, displayFactor, precision, [displayValue, ...snaps]);
    const crossed = uiNumberCrossedBound(stored, range.min, range.max, limits);
    if (crossed) {
      setRefusal({ message: crossed.refusal ?? null });
      return false;
    }
    setRefusal(null);
    if (stored >= range.min && stored <= range.max) {
      beginGesture();
      publishValues([stored]);
      commitGesture();
    } else {
      valuesRef.current = [stored];
      if (controlled) setPendingDraftValues([stored]);
      else setUncontrolledValues([stored]);
      onValueChange?.([stored]);
      onValueCommit?.([stored]);
    }
    return true;
  };

  const handleEditKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      if (commitTyped()) { returnFocusRef.current = true; setIsEditing(false); }
    } else if (e.key === "Escape") {
      returnFocusRef.current = true;
      setRefusal(null);
      setIsEditing(false);
    }
  };

  const handleEditBlur = () => {
    if (refusal === null) setIsEditing(false);
  };

  const pointerValue = reactHostPort.useCallback(
    (event: React.PointerEvent): number => {
      const rect = trackRef.current?.getBoundingClientRect();
      if (!rect || span <= 0) return range.min;
      let ratio: number;
      if (appearance === "dial") ratio = dialPosition(Math.atan2(rect.top + rect.height / 2 - event.clientY, event.clientX - (rect.left + rect.width / 2)));
      else {
        ratio = orientation === "horizontal" ? (event.clientX - rect.left) / Math.max(1, rect.width) : 1 - (event.clientY - rect.top) / Math.max(1, rect.height);
        if (orientation === "horizontal" && dir === "rtl") ratio = 1 - ratio;
        if (inverted) ratio = 1 - ratio;
      }
      return sliderPointerValue(sliderAxisValue(ratio, range.min, range.max, scale), range.min, range.max, range.step, snaps, scale);
    },
    [appearance, dir, inverted, orientation, range.max, range.min, range.step, scale, snaps, span],
  );

  const handlePointerDown = (event: React.PointerEvent<HTMLDivElement>) => {
    if (disabled || readOnly || pointerIdRef.current !== null) return;
    pointerIdRef.current = event.pointerId;
    beginGesture();
    if (!hasBeenEdited) setHasBeenEdited(true);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    const thumb = (event.target as Element).closest<HTMLElement>('[data-slot="slider-thumb"]');
    const valueAtPointer = pointerValue(event);
    const nearestIndex = valuesRef.current.reduce((best, current, index, values) => (Math.abs(current - valueAtPointer) < Math.abs((values[best] ?? current) - valueAtPointer) ? index : best), 0);
    activeThumbIdRef.current = thumb?.dataset.sliderThumbId ?? thumbIdsRef.current[nearestIndex] ?? null;
    if (!thumb && activeThumbIdRef.current) updateThumb(activeThumbIdRef.current, valueAtPointer);
    event.currentTarget.setPointerCapture?.(event.pointerId);
    onPointerDown?.();
  };

  const handlePointerMove = (event: React.PointerEvent<HTMLDivElement>) => {
    if (pointerIdRef.current !== event.pointerId) return;
    setIsDragging(true);
    if (activeThumbIdRef.current) updateThumb(activeThumbIdRef.current, pointerValue(event));
  };

  const handlePointerUp = (event: React.PointerEvent<HTMLDivElement>) => {
    if (pointerIdRef.current !== event.pointerId) return;
    pointerIdRef.current = null;
    setIsDragging(false);
    event.currentTarget.releasePointerCapture?.(event.pointerId);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    commitGesture();
    onPointerUp?.();
  };

  const handlePointerCancel = (event: React.PointerEvent<HTMLDivElement>) => {
    if (pointerIdRef.current !== event.pointerId) return;
    pointerIdRef.current = null;
    setIsDragging(false);
    event.currentTarget.releasePointerCapture?.(event.pointerId);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    cancelGesture();
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLElement>, thumbId: string) => {
    if (disabled || readOnly) return;
    if (event.key === "Escape" && keyboardActiveRef.current) {
      event.preventDefault();
      keyboardActiveRef.current = false;
      cancelGesture();
      return;
    }
    const key = sliderKeyOf(event.key, dir, inverted);
    if (!key) return;
    event.preventDefault();
    if (!keyboardActiveRef.current) {
      beginGesture();
      keyboardActiveRef.current = true;
    }
    const current = valuesRef.current[thumbIdsRef.current.indexOf(thumbId)] ?? range.min;
    updateThumb(thumbId, sliderKeyValue(current, range.min, range.max, range.step, precision, displayFactor, snaps, key, event.shiftKey));
  };

  const handleKeyUp = (event: React.KeyboardEvent<HTMLElement>) => {
    if (!keyboardActiveRef.current || !["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "PageUp", "PageDown", "Home", "End"].includes(event.key)) return;
    keyboardActiveRef.current = false;
    commitGesture();
  };

  const sliderTitle = useControlAccessibleLabel(id);
  const valuePercent = (sliderValue: number): number => (span <= 0 ? 0 : axisShare(sliderValue) * 100);
  const physicalPercent = (sliderValue: number): number => {
    const logical = valuePercent(sliderValue);
    const physical = orientation === "vertical" ? (inverted ? 100 - logical : logical) : (dir === "rtl") !== inverted ? 100 - logical : logical;
    return Number(physical.toFixed(12));
  };
  const physicalValues = _values.map(physicalPercent);
  const rangeStart = physicalValues.length === 1 ? physicalPercent(range.min) : Math.min(...physicalValues);
  const rangeEnd = physicalValues.length ? Math.max(...physicalValues) : rangeStart;
  const readyPhysicalStart = physicalPercent(displayValue);
  const readyPhysicalEnd = readyExtent == null ? readyPhysicalStart : physicalPercent(readyExtent);
  const sliderElement = (
    <div
      {...props}
      ref={forwardedRef}
      data-slot="slider"
      id={id}
      title={sliderTitle}
      dir={dir}
      aria-disabled={disabled || undefined}
      data-disabled={disabled ? "" : undefined}
      data-orientation={orientation}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerCancel}
      className={cn(
        "group relative flex h-full w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
        "has-[[data-slot=slider-thumb]:hover]:[&_[data-slot=slider-range]]:bg-emphasized",
        "has-[[data-slot=slider-thumb][data-dragging=true]]:[&_[data-slot=slider-range]]:bg-active-base",
      )}
    >
      {appearance === "dial" ? (
        <div ref={trackRef} data-slot="slider-dial" data-invalid={refusal ? "true" : undefined} className={sliderDialClassName}>
          <svg viewBox="-1 -1 2 2" aria-hidden="true" className="absolute inset-0 size-full overflow-visible">
            <circle data-slot="slider-dial-face" cx={0} cy={0} r={0.92} className="fill-muted stroke-element" strokeWidth={0.08} />
            {snaps.map((snap) => {
              const angle = dialAngle(axisShare(snap));
              return <line key={snap} data-slot="slider-tick" data-snap={snap} x1={Math.cos(angle) * 0.62} y1={-Math.sin(angle) * 0.62} x2={Math.cos(angle) * 0.92} y2={-Math.sin(angle) * 0.92} className="stroke-element" strokeWidth={0.08} opacity={0.6} />;
            })}
            {(() => {
              const angle = dialAngle(axisShare(displayValue));
              return <line data-slot="slider-dial-needle" x1={0} y1={0} x2={Math.cos(angle) * 0.82} y2={-Math.sin(angle) * 0.82} className="stroke-active-base" strokeWidth={0.14} strokeLinecap="round" />;
            })()}
          </svg>
        </div>
      ) : null}
      <div
        data-slot="slider-track-wrap"
        data-loading={loading ? "true" : undefined}
        data-waiting={waiting ? "true" : undefined}
        hidden={appearance === "dial" || undefined}
        className={cn("relative flex h-full min-w-0 grow items-center", loadingBorderStateClass(loading) || waitingBorderStateClass(waiting))}
      >
        <div
          ref={appearance === "dial" ? undefined : trackRef}
          data-slot="slider-track"
          data-orientation={orientation}
          className={cn("bg-muted relative w-full overflow-hidden rounded-full data-[orientation=horizontal]:h-single data-[orientation=vertical]:h-full data-[orientation=vertical]:w-single")}
        >
          <div
            data-slot="slider-range"
            data-orientation={orientation}
            data-dragging={isDragging ? "true" : undefined}
            className={cn(sliderRangeClassName)}
            style={orientation === "horizontal" ? { left: `${rangeStart}%`, width: `${Math.max(0, rangeEnd - rangeStart)}%` } : { bottom: `${rangeStart}%`, height: `${Math.max(0, rangeEnd - rangeStart)}%` }}
          />
          {readyWidthPct > 0 ? (
            <div
              data-slot="slider-ready"
              data-orientation={orientation}
              className={sliderReadyClassName}
              style={
                orientation === "horizontal"
                  ? { left: `${Math.min(readyPhysicalStart, readyPhysicalEnd)}%`, width: `${Math.abs(readyPhysicalEnd - readyPhysicalStart)}%` }
                  : { bottom: `${Math.min(readyPhysicalStart, readyPhysicalEnd)}%`, height: `${Math.abs(readyPhysicalEnd - readyPhysicalStart)}%` }
              }
            />
          ) : null}
        </div>
        {appearance === "dial" ? null : snaps.map((snap) => (
          <span key={snap} data-slot="slider-tick" data-snap={snap} data-orientation={orientation} aria-hidden="true" className={sliderTickClassName} style={orientation === "horizontal" ? { left: `${physicalPercent(snap)}%` } : { bottom: `${physicalPercent(snap)}%` }} />
        ))}
      </div>
      {_values.map((sliderValue, index) => {
        const thumbId = thumbIdsRef.current[index]!;
        const dialThumb = appearance === "dial" ? dialAngle(axisShare(sliderValue)) : null;
        return (
          <span
            role="slider"
            tabIndex={disabled ? -1 : 0}
            aria-disabled={disabled || undefined}
            aria-readonly={readOnly || undefined}
            aria-valuemin={spoken(range.min)}
            aria-valuemax={spoken(range.max)}
            aria-valuenow={spoken(sliderValue)}
            aria-valuetext={ariaValueText}
            aria-orientation={orientation}
            aria-label={props["aria-label"] ?? sliderTitle}
            aria-labelledby={showLabel && id ? `${id}-label` : props["aria-labelledby"]}
            data-slot="slider-thumb"
            data-slider-index={index}
            data-slider-thumb-id={thumbId}
            data-orientation={orientation}
            data-dragging={isDragging && activeThumbIdRef.current === thumbId ? "true" : undefined}
            key={thumbId}
            className={thumbPresentationClassName}
            style={
              dialThumb !== null
                ? { position: "absolute", left: `calc(var(--size-medium) * ${Number((0.5 + Math.cos(dialThumb) * 0.41).toFixed(6))})`, top: `${Number((50 - Math.sin(dialThumb) * 41).toFixed(6))}%`, transform: "translate(-50%, -50%)" }
                : orientation === "horizontal"
                  ? { position: "absolute", left: `${physicalValues[index]}%`, transform: "translateX(-50%)" }
                  : { position: "absolute", bottom: `${physicalValues[index]}%`, transform: "translateY(50%)" }
            }
            onFocus={() => {
              activeThumbIdRef.current = thumbId;
            }}
            onKeyDown={(event) => handleKeyDown(event, thumbId)}
            onKeyUp={handleKeyUp}
            onBlur={() => {
              if (!keyboardActiveRef.current) return;
              keyboardActiveRef.current = false;
              commitGesture();
            }}
          />
        );
      })}
    </div>
  );

  const contentClassName = showLabel ? undefined : className;
  const sliderContent = showValue ? (
    <div data-slot="slider-content" data-detail-panel-control="fill" data-dim style={{ opacity: isInPropertyValueColumn && !hasBeenEdited ? 0.6 : 1, transition: "opacity 150ms" }} className={cn("flex-1 min-w-0", contentClassName)}>
      <div data-slot="slider-row" className="grid h-medium grid-cols-[minmax(0,1fr)_var(--size-large)] items-center gap-x-tiny">
        <div data-slot="slider-track-cell" className="min-w-0">
          {sliderElement}
        </div>
        {isEditing ? (
          <Input
            type="number"
            aria-label={props["aria-label"] ?? sliderTitle}
            aria-labelledby={showLabel && id ? `${id}-label` : props["aria-labelledby"]}
            value={editValue}
            onChange={(e) => {
              setEditValue(e.target.value);
              setRefusal(null);
            }}
            onKeyDown={handleEditKeyDown}
            onBlur={handleEditBlur}
            min={editorBound(limits ? limits.min : { value: range.min })}
            max={editorBound(limits ? limits.max : { value: range.max })}
            className="w-large min-w-large border-0 px-0 text-end text-xs"
            aria-invalid={refusal ? true : undefined}
            aria-describedby={refusal?.message && id ? `${id}-refusal` : undefined}
            autoFocus
            id={id}
          />
        ) : (
          <span ref={readoutRef} data-slot="slider-value" className={sliderValueClassName} role="button" tabIndex={disabled || readOnly ? -1 : 0} aria-disabled={disabled || readOnly || undefined} aria-label={props["aria-label"] ?? sliderTitle} aria-labelledby={showLabel && id ? `${id}-label` : props["aria-labelledby"]} aria-keyshortcuts="Enter F2 Space" onDoubleClick={handleValueClick} onKeyDown={(event) => {
            if (disabled || readOnly || !["Enter", "F2", " "].includes(event.key)) return;
            event.preventDefault();
            event.stopPropagation();
            handleValueClick();
          }} title={editExactValueLabel}>
            {formatReadout(displayValue)}
          </span>
        )}
      </div>
      {refusal?.message ? (
        <span id={id ? `${id}-refusal` : undefined} role="alert" data-slot="slider-refusal" className={sliderRefusalClassName}>
          {refusal.message}
        </span>
      ) : null}
    </div>
  ) : (
    <div data-slot="slider-content" data-detail-panel-control="fill" data-dim className={cn("flex h-full min-w-0 flex-1 items-center", contentClassName)}>
      {sliderElement}
    </div>
  );

  if (showLabel) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className={className}>
        {sliderContent}
      </Label>
    );
  }

  return sliderContent;
});

export { Slider };

// #endregion 🏩️Slider
