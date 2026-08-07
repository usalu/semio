// #region 🧲️Header
// 💻️ framework/ui/elements/🪜Stepper/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { PropertyValueColumnContext } from "../🪵Tree/🟦️component.tsx";
import { formatNumber } from "../✏️Input/🟦️component.tsx";
import { borderNormalClass, uiFormControlBrowserDefaultProps } from "../🏷️ClassNames/🟦️component.tsx";
import { useTransaction, type ElementProps } from "../🐹️ElementProps/🟦️component.tsx";
import { useLabel, Label } from "../🏷️Label/🟦️component.tsx";
import { useInteractionCommands, RemoveIcon, AddIcon } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
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
  onChange?: (value: number) => void;
  /** ➕️➖️ Relative-delta path for increment/decrement (click, drag, arrow keys); falls back to computing an absolute {@link onChange} when omitted. */
  onDelta?: (delta: number) => void;
  onPointerDown?: () => void;
  onPointerUp?: () => void;
  onPointerCancel?: () => void;
  interactionId?: string;
  showLabel?: boolean;
}

/**
 * Numeric stepper with increment, decrement, and drag-to-adjust.
 **/
export const Stepper: React.FC<StepperProps> = ({ value, defaultValue = 0, min, max, step = 1, mixed, onChange, onDelta, onPointerDown, onPointerUp, onPointerCancel, interactionId, id, showLabel }) => {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const mixedLabel = useLabel("ui.common.mixedValues");
  const borderClass = borderNormalClass;
  const [internalValue, setInternalValue] = reactHostPort.useState(value ?? defaultValue);
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
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
      return clampedValue;
    },
    [min, max],
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

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = parseFloat(e.target.value);
    if (!isNaN(newValue)) {
      updateValue(newValue);
    }
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
        transaction?.start?.();
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
      transaction?.finalize?.();
    }
    onPointerUp?.();
  };

  const handleMouseLeave = () => {
    stopContinuousChange();
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isEditing) {
      setIsEditing(false);
      transaction?.finalize?.();
    }
    onPointerCancel?.();
  };

  const canStepDown = min === undefined || internalValue > min;
  const canStepUp = max === undefined || internalValue < max;
  const displayedValue = Number.isFinite(internalValue) ? internalValue : defaultValue;

  const labelElementId = id ? `${id.split(".").join("-")}-label` : undefined;

  const stepperEmptyOpacity = isInPropertyValueColumn && value === undefined && !hasBeenEdited ? 0.6 : 1;

  const stepperElement = (
    <div
      data-slot="stepper-group"
      data-detail-panel-control="fill"
      className={cn("flex h-medium w-full min-w-0 items-stretch overflow-hidden rounded-sm border transition-[border-color] focus-within:border-accent", borderClass)}
      style={{ opacity: stepperEmptyOpacity, transition: "opacity 150ms" }}
    >
      <button
        data-slot="stepper-minus"
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
        value={mixed && !hasBeenEdited ? "" : isEditing ? displayedValue : formatNumber(displayedValue)}
        onChange={handleInputChange}
        onFocus={() => {
          if (!hasBeenEdited) setHasBeenEdited(true);
          if (!isEditing) {
            setIsEditing(true);
            transaction?.start?.();
          }
          onPointerDown?.();
        }}
        onBlur={() => {
          if (isEditing) {
            setIsEditing(false);
            transaction?.finalize?.();
          }
          onPointerUp?.();
        }}
        onKeyDown={(e) => {
          if (e.key === "ArrowUp" || e.key === "ArrowDown") {
            e.preventDefault();
            if (!isEditing) {
              setIsEditing(true);
              transaction?.start?.();
            }
            if (e.key === "ArrowUp") {
              handleStepUp();
            } else {
              handleStepDown();
            }
          } else if (e.key === "Escape") {
            if (isEditing) {
              setIsEditing(false);
              setInternalValue(value ?? defaultValue);
              transaction?.abort?.();
              (e.target as HTMLInputElement).blur();
            }
          } else if (e.key === "Enter") {
            if (isEditing) {
              setIsEditing(false);
              transaction?.finalize?.();
              (e.target as HTMLInputElement).blur();
            }
          }
        }}
        className="file:text-element placeholder:text-muted-foreground text-element flex h-medium min-w-0 flex-1 border-0 bg-transparent px-double text-center text-base transition-[color,border-color] outline-none file:inline-flex file:h-medium file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 focus-visible:border-0 md:text-sm [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]"
        step={step}
        min={min}
        max={max}
        aria-labelledby={labelElementId}
        id={id}
        inputMode="decimal"
        {...uiFormControlBrowserDefaultProps}
      />
      <button
        data-slot="stepper-plus"
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
