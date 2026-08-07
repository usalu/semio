// #region 🧲️Header
// 💻️ framework/ui/elements/🎚️Slider/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as SliderPrimitive from "@radix-ui/react-slider";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { PropertyValueColumnContext } from "../🪵Tree/🟦️component.tsx";
import { formatNumber, Input } from "../✏️Input/🟦️component.tsx";
import { loadingBorderStateClass, waitingBorderStateClass } from "../🏷️ClassNames/🟦️component.tsx";
import { useTransaction, type ElementProps } from "../🐹️ElementProps/🟦️component.tsx";
import { useLabel, useControlAccessibleLabel, Label } from "../🏷️Label/🟦️component.tsx";
import { useInteractionCommands, sliderRangeClassName, sliderReadyClassName, sliderThumbClassName, sliderValueClassName } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🏩️Slider
// Range slider built on Radix primitives.
// Consumers MUST provide min and max values.

/** @emoji 🎚️ Whether two slider value tuples match within a step-aware epsilon. */
export function sliderValuesMatch(lhs: readonly number[], rhs: readonly number[], step?: number): boolean {
  if (lhs.length !== rhs.length) return false;
  const epsilon = step != null && step > 0 ? step * 0.25 : 1e-9;
  return lhs.every((value, index) => Math.abs(value - (rhs[index] ?? value)) <= epsilon);
}

/** @emoji 🎚️ Clears a pending draft once the controlled `value` prop catches up. */
export function resolveSliderDraftClear(pending: number[] | null, external: readonly number[], step?: number): number[] | null {
  if (pending === null) return null;
  return sliderValuesMatch(pending, external, step) ? null : pending;
}

/** @emoji 🪣️ Clamps every value to `ready` (a preloaded/planned extent, e.g. a background fill plan's
 * progress) — the thumb must never be draggable past what's actually available, and `min` is the floor
 * so a `ready` of 0 still leaves the slider at its resting position rather than collapsing below `min`. */
export function clampSliderValuesToReady(values: readonly number[], ready: number | undefined, min: number): number[] {
  if (ready == null) return values.slice();
  const ceiling = Math.max(min, ready);
  return values.map((value) => Math.min(value, ceiling));
}

/**
 * Slider holds the data fields for a Slider record.
 **/
function Slider({
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
  step,
  disabled,
  clampToReady = false,
  ...props
}: React.ComponentProps<typeof SliderPrimitive.Root> &
  ElementProps & {
    /** @emoji 🎚️ Absolute value along the fixed `[min, max]` range that is already preloaded/ready; drawn as a highlight to the right of the knob. */
    ready?: number;
    /** @emoji 🪣️ When true, the thumb cannot be dragged/keyed/typed past `ready` — for measures where
     * `ready` is a hard availability limit (e.g. a background-planned count), not merely a preload hint.
     * Most `ready` consumers want the highlight without the hard limit, so this defaults to `false`. */
    clampToReady?: boolean;
    /** @emoji 🌀️ Spinning loading ring around the track only — never the row, so the knob and hover chrome stay visible. */
    loading?: boolean;
    /** @emoji 🌀️ Dashed, slow-spinning waiting ring around the track only — never the row, so the knob and hover chrome stay visible. */
    waiting?: boolean;
    showLabel?: boolean;
    /** @emoji 🔢️ When false, only the track+thumb render (graph overlays that already paint the value elsewhere). */
    showValue?: boolean;
    /** @emoji 🔢️ Optional readout formatter — defaults to {@link formatNumber}. */
    formatDisplayValue?: (value: number) => string;
    /** @emoji 🪣️ Fires once per gesture on release (pointer-up, arrow-key-up, or Enter in the text edit) — snapped and clamped-to-`ready` like `onValueChange`, but never fired mid-drag. Use for callers that must not round-trip on every drag value. */
    onValueCommit?: (values: number[]) => void;
    onPointerDown?: () => void;
    onPointerUp?: () => void;
    onPointerCancel?: () => void;
    interactionId?: string;
    snapValues?: number[];
  }) {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [isSliding, setIsSliding] = reactHostPort.useState(false);
  const [isDragging, setIsDragging] = reactHostPort.useState(false);
  const pointerActiveRef = reactHostPort.useRef(false);
  const [editValue, setEditValue] = reactHostPort.useState("");
  const [hasBeenEdited, setHasBeenEdited] = reactHostPort.useState(false);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const doubleClickToEditLabel = useLabel("ui.common.doubleClickToEdit");
  const externalValues = reactHostPort.useMemo(() => (Array.isArray(value) ? value : Array.isArray(defaultValue) ? defaultValue : [min, max]), [value, defaultValue, min, max]);
  // 🖱️ While actively sliding, track the value locally instead of re-reading the (possibly-controlled)
  // `value` prop on every render — a slow or stale round-trip back to `value` would otherwise fight the
  // drag and snap the thumb back to its pre-drag position mid-gesture.
  const [pendingDraftValues, setPendingDraftValues] = reactHostPort.useState<number[] | null>(null);
  const draftValues = resolveSliderDraftClear(pendingDraftValues, externalValues, step ?? undefined);
  reactHostPort.useEffect(() => {
    setPendingDraftValues((pending) => resolveSliderDraftClear(pending, externalValues, step ?? undefined));
  }, [externalValues, step]);
  const _values = draftValues ?? externalValues;

  const displayValue = _values[0] ?? min;
  const formatReadout = formatDisplayValue ?? formatNumber;
  const span = max - min;
  const readyExtent = ready == null || span <= 0 ? null : Math.min(max, Math.max(min, ready));
  const readyStartPct = span <= 0 ? 0 : ((displayValue - min) / span) * 100;
  const readyWidthPct = readyExtent == null || readyExtent <= displayValue || span <= 0 ? 0 : ((readyExtent - displayValue) / span) * 100;

  const findNearestSnapValue = reactHostPort.useCallback(
    (val: number): number => {
      if (!snapValues || snapValues.length === 0) return val;
      let nearest = snapValues[0];
      let minDistance = Math.abs(val - nearest);
      for (const snapValue of snapValues) {
        const distance = Math.abs(val - snapValue);
        if (distance < minDistance) {
          minDistance = distance;
          nearest = snapValue;
        }
      }
      return nearest;
    },
    [snapValues],
  );

  const handleValueChange = reactHostPort.useCallback(
    (values: number[]) => {
      const snapped = snapValues && snapValues.length > 0 ? values.map(findNearestSnapValue) : values;
      const nextValues = clampToReady ? clampSliderValuesToReady(snapped, ready, min) : snapped;
      setPendingDraftValues(nextValues);
      onValueChange?.(nextValues);
    },
    [snapValues, findNearestSnapValue, onValueChange, clampToReady, ready, min],
  );

  const handleValueCommit = reactHostPort.useCallback(
    (values: number[]) => {
      const snapped = snapValues && snapValues.length > 0 ? values.map(findNearestSnapValue) : values;
      const nextValues = clampToReady ? clampSliderValuesToReady(snapped, ready, min) : snapped;
      onValueCommit?.(nextValues);
    },
    [snapValues, findNearestSnapValue, onValueCommit, clampToReady, ready, min],
  );

  const handleValueClick = () => {
    if (disabled) return;
    if (!hasBeenEdited) setHasBeenEdited(true);
    setEditValue(formatReadout(displayValue));
    setIsEditing(true);
    transaction?.start?.();
  };

  const handleEditKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      const newValue = parseFloat(editValue);
      if (!isNaN(newValue) && newValue >= min && newValue <= max) {
        handleValueChange([newValue]);
        handleValueCommit([newValue]);
      }
      setIsEditing(false);
      transaction?.finalize?.();
    } else if (e.key === "Escape") {
      setIsEditing(false);
      transaction?.abort?.();
    }
  };

  const handleEditBlur = () => {
    setIsEditing(false);
    transaction?.finalize?.();
  };

  const handlePointerDown = (e: React.PointerEvent) => {
    pointerActiveRef.current = true;
    if (!hasBeenEdited) setHasBeenEdited(true);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    if (!isSliding) {
      setPendingDraftValues(externalValues);
      setIsSliding(true);
      transaction?.start?.();
    }
    onPointerDown?.();
  };

  const handlePointerMove = () => {
    if (pointerActiveRef.current) setIsDragging(true);
  };

  const handlePointerUp = (e: React.PointerEvent) => {
    pointerActiveRef.current = false;
    setIsDragging(false);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isSliding) {
      setIsSliding(false);
      transaction?.finalize?.();
    }
    onPointerUp?.();
  };

  const handlePointerCancel = (e: React.PointerEvent) => {
    pointerActiveRef.current = false;
    setIsDragging(false);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isSliding) {
      setIsSliding(false);
      setPendingDraftValues(null);
      transaction?.abort?.();
    }
    onPointerCancel?.();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowLeft" || e.key === "ArrowRight" || e.key === "ArrowUp" || e.key === "ArrowDown") {
      if (!isSliding) {
        setPendingDraftValues(externalValues);
        setIsSliding(true);
        transaction?.start?.();
      }
    }
  };

  const handleKeyUp = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowLeft" || e.key === "ArrowRight" || e.key === "ArrowUp" || e.key === "ArrowDown") {
      if (isSliding) {
        setIsSliding(false);
        transaction?.finalize?.();
      }
    } else if (e.key === "Escape") {
      if (isSliding) {
        setIsSliding(false);
        setPendingDraftValues(null);
        transaction?.abort?.();
        onPointerCancel?.();
      }
    }
  };

  const sliderTitle = useControlAccessibleLabel(id);
  const sliderElement = (
    <SliderPrimitive.Root
      data-slot="slider"
      id={id}
      title={sliderTitle}
      defaultValue={defaultValue}
      value={_values}
      min={min}
      max={max}
      step={step}
      onValueChange={handleValueChange}
      onValueCommit={handleValueCommit}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerCancel}
      onKeyDown={handleKeyDown}
      onKeyUp={handleKeyUp}
      disabled={disabled}
      className={cn(
        "group relative flex h-full w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
        "has-[[data-slot=slider-thumb]:hover]:[&_[data-slot=slider-range]]:bg-emphasized",
        "has-[[data-slot=slider-thumb][data-dragging=true]]:[&_[data-slot=slider-range]]:bg-active-base",
      )}
      {...props}
    >
      <div
        data-slot="slider-track-wrap"
        data-loading={loading ? "true" : undefined}
        data-waiting={waiting ? "true" : undefined}
        className={cn("relative flex h-full min-w-0 grow items-center", loadingBorderStateClass(loading) || waitingBorderStateClass(waiting))}
      >
        <SliderPrimitive.Track data-slot="slider-track" className={cn("bg-muted relative w-full overflow-hidden rounded-[9999px] data-[orientation=horizontal]:h-single data-[orientation=vertical]:h-full data-[orientation=vertical]:w-single")}>
          <SliderPrimitive.Range data-slot="slider-range" data-dragging={isDragging ? "true" : undefined} className={cn(sliderRangeClassName)} />
          {readyWidthPct > 0 ? <div data-slot="slider-ready" data-orientation="horizontal" className={sliderReadyClassName} style={{ left: `${readyStartPct}%`, width: `${readyWidthPct}%` }} /> : null}
        </SliderPrimitive.Track>
      </div>
      {Array.from({ length: _values.length }, (_, index) => (
        <SliderPrimitive.Thumb data-slot="slider-thumb" data-dragging={isDragging ? "true" : undefined} key={index} className={sliderThumbClassName} />
      ))}
    </SliderPrimitive.Root>
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
            value={editValue}
            onChange={(e) => setEditValue(e.target.value)}
            onKeyDown={handleEditKeyDown}
            onBlur={handleEditBlur}
            className="w-large min-w-large border-0 px-0 text-end text-xs"
            min={min}
            max={max}
            autoFocus
            id={id}
          />
        ) : (
          <span data-slot="slider-value" className={sliderValueClassName} role="button" onDoubleClick={handleValueClick} title={doubleClickToEditLabel}>
            {formatReadout(displayValue)}
          </span>
        )}
      </div>
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
}

export { Slider };

// #endregion 🏩️Slider
