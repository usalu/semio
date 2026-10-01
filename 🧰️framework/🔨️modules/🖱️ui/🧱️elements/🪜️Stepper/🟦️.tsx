// #region 🧲️Header
// 💻️ framework/ui/elements/🪜️Stepper/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { PropertyValueColumnContext } from "../🌳️Tree/🟦️.tsx";
import { formatNumber } from "../✏️Input/🟦️.tsx";
import { roundUiNumber } from "../../🧬️contract/🔢️number-format/🟦️.ts";
import { uiNumberCrossedBound, uiNumberDisplayText, uiNumberKeyValue, uiNumberTypedValue } from "../../🧬️contract/🧩️component/🟦️.ts";
import type { UiNumberLimits } from "@semio-tech/framework";
import { borderNormalClass } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { uiFormControlBrowserDefaultProps } from "../../🔨️modules/📝️form-control-presentation/🟦️.ts";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️.ts";
import { useLabel, Label } from "../🏷️Label/🟦️.tsx";
import { useInteractionCommands, RemoveIcon, AddIcon } from "../../🎯️targets/⚛️react/🟦️";
// #endregion 🔌️Adapters

// #region 🏬️Stepper
// Numeric stepper with increment/decrement and drag adjustment.
// Consumers MUST provide min and max bounds.

/**
 * StepperProps holds the data fields for a StepperProps record.
 **/
interface StepperProps extends ElementProps {
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  /** 🔀️ Mixed-selection state: shows a blank/placeholder value instead of {@link value}, mirroring {@link Input}'s `mixed` prop. */
  mixed?: boolean;
  /** 🎯️ Fraction digits shown and committed (`NumberStepperProps.precision`); every value is rounded by the shared law. */
  precision?: number;
  onChange?: (value: number) => void;
  /** ➕️➖️ Relative-delta path for increment/decrement (click, drag, arrow keys); falls back to computing an absolute {@link onChange} when omitted. */
  onDelta?: (delta: number) => void;
  onPointerDown?: () => void;
  onPointerUp?: () => void;
  onPointerCancel?: () => void;
  interactionId?: string;
  showLabel?: boolean;
  /** 🏷️ The element naming the value field when an enclosing form owns the label (a staged dialog field). */
  "aria-labelledby"?: string;
  /** 🏷️ The value field's own name when no label element names it (an interpreted node's `accessibility.label`). */
  "aria-label"?: string;
  disabled?: boolean;
  /** 📍️ Detents (`NumberStepperProps.snaps`): the page keys stop on the first they reach (the shared keyboard law). */
  snapValues?: readonly number[];
  /** 🔁️ The field shows and reads `stored × displayFactor` (`NumberStepperProps.displayFactor`); a typed value is divided back by the shared law. */
  displayFactor?: number | null;
  /** 📐️ The unit shown beside the value (`NumberStepperProps.displayUnit`, else `unit`). */
  unit?: string | null;
  /** 🚧️ The hard range a typed value must keep (`NumberStepperProps.limits`; `min`/`max` themselves when absent): a crossing value is refused visibly with the bound's refusal and never dispatched. */
  limits?: UiNumberLimits | null;
  /** 🗣️ The spoken value (`aria-valuetext`) — the display text with its unit. */
  "aria-valuetext"?: string;
}

/**
 * Numeric stepper with increment, decrement, and drag-to-adjust.
 **/
export const Stepper: React.FC<StepperProps> = ({ value, defaultValue = 0, min, max, step = 1, mixed, precision, onChange, onDelta, onPointerDown, onPointerUp, onPointerCancel, interactionId, id, showLabel, "aria-labelledby": labelledBy, "aria-label": ariaLabel, "aria-valuetext": ariaValueText, disabled = false, snapValues, displayFactor = null, unit = null, limits = null }) => {
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const mixedLabel = useLabel("ui.common.mixedValues");
  const decrementLabel = useLabel("ui.tableStepper.decrement");
  const incrementLabel = useLabel("ui.tableStepper.increment");
  const borderClass = borderNormalClass;
  const [internalValue, setInternalValue] = reactHostPort.useState(value ?? defaultValue);
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [draft, setDraft] = reactHostPort.useState("");
  const [refusal, setRefusal] = reactHostPort.useState<{ readonly message: string | null } | null>(null);
  const snaps = reactHostPort.useMemo(() => snapValues ?? [], [snapValues]);
  const shownText = reactHostPort.useCallback((shown: number): string => (precision === undefined && displayFactor == null ? formatNumber(shown) : uiNumberDisplayText(shown, displayFactor, precision)), [displayFactor, precision]);
  const [hasBeenEdited, setHasBeenEdited] = reactHostPort.useState(false);
  const intervalRef = reactHostPort.useRef<NodeJS.Timeout | null>(null);
  const timeoutRef = reactHostPort.useRef<NodeJS.Timeout | null>(null);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;

  reactHostPort.useEffect(() => {
    if (value !== undefined) {
      setInternalValue(value);
    }
  }, [value]);

  const clampValue = reactHostPort.useCallback(
    (val: number): number => {
      let clampedValue = val;
      if (min !== undefined) clampedValue = Math.max(clampedValue, min);
      if (max !== undefined) clampedValue = Math.min(clampedValue, max);
      if (precision === undefined) return clampedValue;
      return displayFactor == null ? roundUiNumber(clampedValue, precision) : roundUiNumber(clampedValue * displayFactor, precision) / displayFactor;
    },
    [displayFactor, min, max, precision],
  );

  const updateValue = reactHostPort.useCallback(
    (newValue: number) => {
      const clampedValue = clampValue(newValue);
      setInternalValue(clampedValue);
      onChange?.(clampedValue);
    },
    [clampValue, onChange],
  );

  /** ➕️➖️ Increment/decrement path: reports a relative delta via {@link onDelta} when provided (e.g. mixed-selection nudging), otherwise falls back to an absolute {@link onChange}. */
  const applyDelta = reactHostPort.useCallback(
    (increment: number) => {
      const clampedValue = clampValue(internalValue + increment);
      setInternalValue(clampedValue);
      if (onDelta) onDelta(increment);
      else onChange?.(clampedValue);
    },
    [internalValue, clampValue, onDelta, onChange],
  );

  const startContinuousChange = reactHostPort.useCallback(
    (increment: number) => {
      if (intervalRef.current) clearInterval(intervalRef.current);
      if (timeoutRef.current) clearTimeout(timeoutRef.current);

      timeoutRef.current = setTimeout(() => {
        intervalRef.current = setInterval(() => {
          setInternalValue((prev) => {
            const newValue = clampValue(prev + increment);
            return newValue;
          });
        }, 100);
      }, 500);
    },
    [clampValue, onChange],
  );

  const stopContinuousChange = reactHostPort.useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  }, []);

  reactHostPort.useEffect(() => {
    return () => {
      stopContinuousChange();
    };
  }, [stopContinuousChange]);

  /** ⌨️ A typed value reads in display units: unreadable text or a value crossing a hard bound is refused (the draft kept, the
   * bound's refusal shown, nothing dispatched); an admitted one is dispatched exactly, never clamped. */
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const text = e.target.value;
    setDraft(text);
    const typed = Number(text.trim());
    if (text.trim() === "" || !Number.isFinite(typed)) {
      setRefusal({ message: null });
      return;
    }
    const stored = uiNumberTypedValue(typed, displayFactor, precision, [internalValue, ...snaps]);
    const crossed = uiNumberCrossedBound(stored, min, max, limits);
    if (crossed) {
      setRefusal({ message: crossed.refusal ?? null });
      return;
    }
    setRefusal(null);
    setInternalValue(stored);
    onChange?.(stored);
  };

  /** 🎹️ Arrows and page keys through the shared keyboard law (page keys stop on the first detent they reach); a relative stepper
   * reports the move as a delta. */
  const handleKey = (key: "increment" | "decrement" | "pageUp" | "pageDown", large: boolean) => {
    const next = uiNumberKeyValue(internalValue, min ?? null, max ?? null, step, snaps, key, large);
    if (next === internalValue) return;
    setRefusal(null);
    setDraft(shownText(next));
    setInternalValue(next);
    if (onDelta) onDelta(next - internalValue);
    else onChange?.(next);
  };

  const handleStepUp = () => {
    applyDelta(step);
  };

  const handleStepDown = () => {
    applyDelta(-step);
  };

  const handleMouseDown = (increment: number) => {
    return () => {
      if (!hasBeenEdited) setHasBeenEdited(true);
      if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
      if (!isEditing) {
        setIsEditing(true);
      }
      onPointerDown?.();
      if (increment > 0) {
        handleStepUp();
      } else {
        handleStepDown();
      }
      startContinuousChange(increment);
    };
  };

  const handleMouseUp = () => {
    stopContinuousChange();
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isEditing) {
      setIsEditing(false);
    }
    onPointerUp?.();
  };

  const handleMouseLeave = () => {
    stopContinuousChange();
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isEditing) {
      setIsEditing(false);
    }
    onPointerCancel?.();
  };

  const canStepDown = !disabled && (min === undefined || internalValue > min);
  const canStepUp = !disabled && (max === undefined || internalValue < max);
  const displayedValue = Number.isFinite(internalValue) ? internalValue : defaultValue;
  const refusalId = id ? `${id.split(".").join("-")}-refusal` : undefined;

  const labelElementId = id ? `${id.split(".").join("-")}-label` : undefined;

  const stepperEmptyOpacity = isInPropertyValueColumn && value === undefined && !hasBeenEdited ? 0.6 : 1;

  const stepperGroup = (
    <div
      data-slot="stepper-group"
      data-detail-panel-control="fill"
      className={cn("flex h-medium w-full min-w-0 items-stretch overflow-hidden rounded-sm border transition-[border-color] focus-within:border-accent", borderClass)}
      style={{ opacity: stepperEmptyOpacity, transition: "opacity 150ms" }}
    >
      <button
        data-slot="stepper-minus"
        aria-label={decrementLabel}
        type="button"
        onMouseDown={handleMouseDown(-step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(-step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepDown}
        className={cn("flex h-medium w-medium shrink-0 cursor-pointer items-center justify-center border-e hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted", borderClass)}
      >
        <RemoveIcon className="size-tiny" />
      </button>
      <input
        type="number"
        data-slot="input"
        data-stepper-input="true"
        data-mixed={mixed ? "true" : undefined}
        placeholder={mixed && !hasBeenEdited ? mixedLabel || "—" : undefined}
        value={mixed && !hasBeenEdited ? "" : isEditing || refusal ? draft : shownText(displayedValue)}
        onChange={handleInputChange}
        onFocus={() => {
          if (!hasBeenEdited) setHasBeenEdited(true);
          if (!isEditing) {
            setIsEditing(true);
            if (!refusal) setDraft(shownText(displayedValue));
          }
          onPointerDown?.();
        }}
        onBlur={() => {
          if (isEditing) {
            setIsEditing(false);
          }
          onPointerUp?.();
        }}
        onKeyDown={(e) => {
          if (e.key === "ArrowUp" || e.key === "ArrowDown" || e.key === "PageUp" || e.key === "PageDown") {
            e.preventDefault();
            if (!isEditing) {
              setIsEditing(true);
            }
            handleKey(e.key === "ArrowUp" ? "increment" : e.key === "ArrowDown" ? "decrement" : e.key === "PageUp" ? "pageUp" : "pageDown", e.shiftKey);
          } else if (e.key === "Escape") {
            setRefusal(null);
            if (isEditing) {
              setIsEditing(false);
              setInternalValue(value ?? defaultValue);
              (e.target as HTMLInputElement).blur();
            }
          } else if (e.key === "Enter") {
            if (isEditing) {
              setIsEditing(false);
              (e.target as HTMLInputElement).blur();
            }
          }
        }}
        className="file:text-element placeholder:text-muted-foreground text-element flex h-medium min-w-0 flex-1 border-0 bg-transparent px-double text-center text-base transition-[color,border-color] outline-none file:inline-flex file:h-medium file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 focus-visible:border-0 md:text-sm [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]"
        step={displayFactor == null ? step : step * displayFactor}
        min={min === undefined ? undefined : displayFactor == null ? min : min * displayFactor}
        max={max === undefined ? undefined : displayFactor == null ? max : max * displayFactor}
        disabled={disabled}
        aria-label={ariaLabel}
        aria-valuetext={ariaValueText}
        aria-invalid={refusal ? true : undefined}
        aria-describedby={refusal?.message ? refusalId : undefined}
        aria-labelledby={labelledBy ?? (ariaLabel === undefined ? labelElementId : undefined)}
        id={id}
        inputMode="decimal"
        {...uiFormControlBrowserDefaultProps}
      />
      {unit ? (
        <span data-slot="stepper-unit" aria-hidden="true" className="text-muted-foreground flex shrink-0 items-center pe-double text-xs">
          {unit}
        </span>
      ) : null}
      <button
        data-slot="stepper-plus"
        aria-label={incrementLabel}
        type="button"
        onMouseDown={handleMouseDown(step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepUp}
        className={cn("flex h-medium w-medium shrink-0 cursor-pointer items-center justify-center border-s hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted", borderClass)}
      >
        <AddIcon className="size-tiny" />
      </button>
    </div>
  );
  const stepperElement = refusal?.message ? (
    <div data-slot="stepper-field" className="flex w-full min-w-0 flex-col">
      {stepperGroup}
      <span id={refusalId} role="alert" data-slot="stepper-refusal" className="text-destructive text-xs leading-tight">
        {refusal.message}
      </span>
    </div>
  ) : (
    stepperGroup
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={labelElementId}>
        {stepperElement}
      </Label>
    );
  }

  return stepperElement;
};

// #endregion 🏬️Stepper
