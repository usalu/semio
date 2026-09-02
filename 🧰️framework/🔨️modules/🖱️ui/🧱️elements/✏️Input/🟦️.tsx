// #region 🧲️Header
// 💻️ framework/ui/elements/✏️Input/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { type UiLabel, uiDataLabel } from "../🏷️UiLabel/🟦️.tsx";
import { PropertyValueColumnContext } from "../🪵️Tree/🟦️.tsx";
import { borderElementClass } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { formControlFocusBorderClass, uiFormControlBrowserDefaultProps } from "../../🔨️modules/📝️form-control-presentation/🟦️.ts";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️.ts";
import { useIdLabel, useLabel, Label } from "../🏷️Label/🟦️.tsx";
import { useInteractionCommands } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx";
import { ChevronDownIcon } from "../🔣️Icons/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🩺️Input
// Text input field with label, validation, and clear support.
// Consumers MUST provide an id for accessibility.

// #region 📨️Input Collapse Helpers

const COLLAPSED_FIELD_ELLIPSIS = "...";
const collapsedFieldOverflowEpsilonPx = 0.5;
const collapsedFieldWhitespacePattern = /\s+/g;
const nonCollapsibleInputTypes = new Set(["button", "checkbox", "color", "file", "hidden", "image", "password", "radio", "range", "reset", "submit"]);
const stackedOverflowInputTypes = new Set(["email", "search", "tel", "text", "url"]);

interface FitCollapsedFieldTextOptions {
  value: string;
  maxWidth: number;
  ellipsis?: string;
  appendEllipsis?: boolean;
  measureText: (value: string) => number;
}

function normalizeCollapsedFieldText(value: string) {
  return value.replace(collapsedFieldWhitespacePattern, " ").trim();
}

function getCollapsedFieldGraphemes(value: string) {
  if (typeof Intl !== "undefined" && "Segmenter" in Intl) {
    return Array.from(new Intl.Segmenter(undefined, { granularity: "grapheme" }).segment(value), (segment) => segment.segment);
  }
  return Array.from(value);
}

function fitCollapsedFieldText({ value, maxWidth, ellipsis = COLLAPSED_FIELD_ELLIPSIS, appendEllipsis = true, measureText }: FitCollapsedFieldTextOptions) {
  const normalizedValue = normalizeCollapsedFieldText(value);
  if (!normalizedValue || maxWidth <= 0) {
    return normalizedValue;
  }
  if (measureText(normalizedValue) <= maxWidth) {
    return normalizedValue;
  }

  if (measureText(ellipsis) >= maxWidth) {
    return ellipsis;
  }

  const words = normalizedValue.split(" ");
  if (words.length > 1) {
    let low = 1;
    let high = words.length;
    let bestWordCount = 0;

    while (low <= high) {
      const mid = Math.floor((low + high) / 2);
      const prefix = words.slice(0, mid).join(" ");
      const candidate = appendEllipsis ? `${prefix}${ellipsis}` : prefix;
      if (measureText(candidate) <= maxWidth) {
        bestWordCount = mid;
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }

    if (bestWordCount > 0 && bestWordCount < words.length) {
      const prefix = words.slice(0, bestWordCount).join(" ");
      return appendEllipsis ? `${prefix}${ellipsis}` : prefix;
    }
  }

  const graphemes = getCollapsedFieldGraphemes(normalizedValue);
  let low = 1;
  let high = graphemes.length;
  let bestCharacterCount = 0;

  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    const prefix = graphemes.slice(0, mid).join("").trimEnd();
    const candidate = appendEllipsis ? `${prefix}${ellipsis}` : prefix;
    if (measureText(candidate) <= maxWidth) {
      bestCharacterCount = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  if (bestCharacterCount <= 0) {
    return appendEllipsis ? ellipsis : (graphemes[0] ?? "");
  }

  const prefix = graphemes.slice(0, bestCharacterCount).join("").trimEnd();
  return appendEllipsis ? `${prefix}${ellipsis}` : prefix;
}

function isCollapsibleInputType(type?: string) {
  return !type || !nonCollapsibleInputTypes.has(type);
}

function isStackedOverflowInputType(type?: string) {
  return !type || stackedOverflowInputTypes.has(type);
}

interface ResolveCollapsedFieldDisplayStateOptions {
  allowStackedOverflow?: boolean;
  value: string;
  maxWidth: number;
  measureText: (value: string) => number;
}

interface CollapsedFieldDisplayState {
  value: string;
  normalizedValue: string;
  isOverflowing: boolean;
  layoutKind: "single-line" | "stacked-overflow";
}

function resolveCollapsedFieldDisplayState({ allowStackedOverflow = false, value, maxWidth, measureText }: ResolveCollapsedFieldDisplayStateOptions): CollapsedFieldDisplayState {
  const normalizedValue = normalizeCollapsedFieldText(value);
  if (!normalizedValue || maxWidth <= 0) {
    return {
      value: normalizedValue,
      normalizedValue,
      isOverflowing: false,
      layoutKind: "single-line",
    };
  }

  const measuredValueWidth = measureText(normalizedValue);
  const isOverflowing = measuredValueWidth > maxWidth + collapsedFieldOverflowEpsilonPx;
  if (!isOverflowing) {
    return {
      value: normalizedValue,
      normalizedValue,
      isOverflowing: false,
      layoutKind: "single-line",
    };
  }

  const collapsedValue = fitCollapsedFieldText({ value: normalizedValue, maxWidth, appendEllipsis: !allowStackedOverflow, measureText });

  return {
    value: collapsedValue,
    normalizedValue,
    isOverflowing,
    layoutKind: allowStackedOverflow && isOverflowing ? "stacked-overflow" : "single-line",
  };
}

interface CollapsedFieldDisplayProps {
  allowStackedOverflow?: boolean;
  className?: string;
  disabled?: boolean;
  id?: string;
  mixed?: boolean;
  onActivate: () => void;
  placeholder?: UiLabel;
  slot: "input" | "textarea";
  value: string;
}

function CollapsedFieldDisplay({ allowStackedOverflow = false, className, disabled, id, mixed, onActivate, placeholder, slot, value }: CollapsedFieldDisplayProps) {
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const displayRef = reactHostPort.useRef<HTMLDivElement>(null);
  const lineRef = reactHostPort.useRef<HTMLSpanElement>(null);
  const normalizedValue = reactHostPort.useMemo(() => normalizeCollapsedFieldText(value), [value]);
  const stackedOverflowEnabled = isInPropertyValueColumn && allowStackedOverflow;
  const [displayState, setDisplayState] = reactHostPort.useState<CollapsedFieldDisplayState>({
    value: normalizedValue,
    normalizedValue,
    isOverflowing: false,
    layoutKind: "single-line",
  });

  const updateCollapsedValue = reactHostPort.useCallback(() => {
    const element = displayRef.current;
    const lineElement = lineRef.current;
    if (!element || !lineElement) {
      return;
    }
    if (!normalizedValue) {
      setDisplayState({
        value: "",
        normalizedValue,
        isOverflowing: false,
        layoutKind: "single-line",
      });
      return;
    }

    const computedStyle = window.getComputedStyle(element);
    const maxWidth = lineElement.clientWidth;
    if (maxWidth <= 0) {
      setDisplayState({
        value: normalizedValue,
        normalizedValue,
        isOverflowing: false,
        layoutKind: "single-line",
      });
      return;
    }

    const measurementElement = document.createElement("span");
    measurementElement.style.position = "absolute";
    measurementElement.style.visibility = "hidden";
    measurementElement.style.pointerEvents = "none";
    measurementElement.style.whiteSpace = "nowrap";
    measurementElement.style.font = computedStyle.font || `${computedStyle.fontStyle} ${computedStyle.fontVariant} ${computedStyle.fontWeight} ${computedStyle.fontSize} / ${computedStyle.lineHeight} ${computedStyle.fontFamily}`;
    measurementElement.style.letterSpacing = computedStyle.letterSpacing;
    measurementElement.style.textTransform = computedStyle.textTransform;
    measurementElement.style.textRendering = computedStyle.textRendering;
    document.body.appendChild(measurementElement);

    const measureText = (candidate: string) => {
      measurementElement.textContent = candidate;
      return measurementElement.getBoundingClientRect().width;
    };

    const nextState = resolveCollapsedFieldDisplayState({ allowStackedOverflow: stackedOverflowEnabled, value: normalizedValue, maxWidth, measureText });
    measurementElement.remove();

    setDisplayState((previousState) =>
      previousState.value === nextState.value && previousState.normalizedValue === nextState.normalizedValue && previousState.isOverflowing === nextState.isOverflowing && previousState.layoutKind === nextState.layoutKind ? previousState : nextState,
    );
  }, [normalizedValue, stackedOverflowEnabled]);

  reactHostPort.useEffect(() => {
    updateCollapsedValue();
  }, [updateCollapsedValue]);

  reactHostPort.useEffect(() => {
    const fontSet = document.fonts;
    if (!fontSet?.ready) {
      return;
    }

    let isCancelled = false;
    void fontSet.ready.then(() => {
      if (!isCancelled) {
        updateCollapsedValue();
      }
    });

    return () => {
      isCancelled = true;
    };
  }, [updateCollapsedValue]);

  reactHostPort.useEffect(() => {
    const element = displayRef.current;
    if (!element || typeof ResizeObserver === "undefined") {
      return;
    }
    const resizeObserver = new ResizeObserver(() => updateCollapsedValue());
    resizeObserver.observe(element);
    return () => resizeObserver.disconnect();
  }, [updateCollapsedValue]);

  const activate = () => {
    if (!disabled) {
      onActivate();
    }
  };

  const showStackedOverflow = stackedOverflowEnabled && displayState.layoutKind === "stacked-overflow";

  return (
    <div
      ref={displayRef}
      data-slot={slot}
      data-collapsed="true"
      data-overflowing={displayState.isOverflowing ? "true" : undefined}
      data-overflow-layout={showStackedOverflow ? "stacked" : "single-line"}
      id={id}
      className={cn(
        "text-element flex w-full min-w-0 overflow-hidden border bg-transparent text-base transition-[color,border-color] outline-none md:text-sm",
        showStackedOverflow ? "h-auto min-h-0 flex-col px-single" : "h-medium items-center px-single whitespace-nowrap",
        "aria-invalid:border-destructive flex-1 cursor-text",
        disabled && "cursor-not-allowed opacity-50",
        mixed && !displayState.value && "italic text-muted-foreground/70",
        className,
      )}
      tabIndex={disabled ? -1 : 0}
      role="textbox"
      aria-readonly="true"
      aria-disabled={disabled ? "true" : undefined}
      onClick={activate}
      onFocus={activate}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          activate();
        }
      }}
    >
      <span ref={lineRef} data-slot="collapsed-field-line" className={cn("flex min-w-0 overflow-hidden whitespace-nowrap", showStackedOverflow ? "h-medium w-full items-center" : "w-full items-center")}>
        {displayState.value ? (
          <span className={cn("block min-w-0 overflow-hidden whitespace-nowrap", !showStackedOverflow && "text-ellipsis")}>{displayState.value}</span>
        ) : (
          <span className={cn("block min-w-0 truncate", mixed ? "italic text-muted-foreground/70" : "text-muted-foreground")}>{placeholder}</span>
        )}
      </span>
      {showStackedOverflow ? (
        <span data-slot="collapsed-field-overflow" aria-hidden="true" className="flex h-tiny min-w-0 items-center justify-center overflow-hidden leading-none">
          <span data-slot="collapsed-field-indicator" className="inline-flex items-center justify-center text-muted-foreground/75 leading-none">
            <ChevronDownIcon data-slot="collapsed-field-indicator-chevron" className="size-tiny shrink-0 stroke-[2.5]" />
          </span>
        </span>
      ) : null}
    </div>
  );
}

// #endregion 📨️Input Collapse Helpers

//#region Number formatting

/** 🔢️ Strip IEEE-754 float artifacts for display without losing real precision. */
export function formatNumber(value: number | string): string {
  const n = typeof value === "string" ? Number(value) : value;
  if (typeof value === "string" && !Number.isFinite(n)) return value;
  if (!Number.isFinite(n)) return "";
  return Number.parseFloat(n.toPrecision(12)).toString();
}

//#endregion Number formatting

/**
 * InputProps holds the data fields for a InputProps record.
 **/
interface InputProps extends Omit<React.ComponentProps<"input">, "value" | "onChange" | "id">, ElementProps {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onLazyChange?: (value: string) => void;
  interactionId?: string;
  placeholderId?: string;
  showLabel?: boolean;
  mixed?: boolean;
}

/**
 * Input holds the data fields for a Input record.
 **/
function Input({ className, type, lazy, value: externalValue, onChange, onLazyChange, interactionId, id, placeholderId, placeholder, showLabel, mixed, ...props }: InputProps) {
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const scalarInputValue = (value: string | number | readonly string[] | undefined): string | number => {
    if (typeof value === "number") return value;
    if (typeof value === "string") return value;
    if (Array.isArray(value)) return (value as readonly string[]).join(", ");
    return "";
  };
  const [localValue, setLocalValue] = reactHostPort.useState(type === "number" ? formatNumber(scalarInputValue(externalValue)) : scalarInputValue(externalValue).toString() || "");
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [isFocused, setIsFocused] = reactHostPort.useState(false);
  const inputRef = reactHostPort.useRef<HTMLInputElement>(null);
  /** @emoji 🧾️ Enter key already runs {@link onLazyChange} + blur; skip duplicate commit on the subsequent blur event. */
  const skipLazyBlurCommitRef = reactHostPort.useRef(false);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const placeholderLabel = useIdLabel(placeholderId);
  const mixedLabel = useLabel("ui.common.mixedValues");
  const computedPlaceholder: UiLabel | undefined = mixed
    ? mixedLabel || uiDataLabel("—")
    : placeholderId
      ? placeholderLabel
      : placeholder !== undefined
        ? uiDataLabel(placeholder)
        : undefined;

  reactHostPort.useEffect(() => {
    if (!isEditing) setLocalValue(type === "number" ? formatNumber(scalarInputValue(externalValue)) : scalarInputValue(externalValue).toString() || "");
  }, [externalValue, isEditing, type]);

  reactHostPort.useEffect(() => {
    if (isFocused && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isFocused]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLInputElement>) => {
    setIsFocused(true);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    if (lazy) {
      setIsEditing(true);
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    setIsFocused(false);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (lazy) {
      setIsEditing(false);
      if (skipLazyBlurCommitRef.current) {
        skipLazyBlurCommitRef.current = false;
        props.onBlur?.(e);
        return;
      }
      onLazyChange?.(localValue);
    }
    props.onBlur?.(e);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (lazy) {
      if (e.key === "Enter") {
        if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
        setIsEditing(false);
        skipLazyBlurCommitRef.current = true;
        onLazyChange?.(localValue);
        (e.target as HTMLInputElement).blur();
      } else if (e.key === "Escape") {
        if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
        setIsEditing(false);
        setLocalValue(type === "number" ? formatNumber(scalarInputValue(externalValue)) : scalarInputValue(externalValue).toString() || "");
        (e.target as HTMLInputElement).blur();
      }
    }
    props.onKeyDown?.(e);
  };

  const inputValue = lazy ? localValue : scalarInputValue(externalValue);

  const inputDisplayValue = type === "number" && !isFocused ? formatNumber(inputValue) : inputValue.toString() || "";
  const showCollapsedDisplay = !!showLabel && !isFocused && isCollapsibleInputType(type);
  const allowStackedOverflow = isStackedOverflowInputType(type);

  const inputEmptyOpacity = isInPropertyValueColumn && !inputDisplayValue && !isFocused ? 0.6 : 1;

  const inputElement = (
    <div data-slot="input-root" data-detail-panel-control="fill" data-dim className="flex min-w-0 w-full flex-1 items-stretch" style={{ opacity: inputEmptyOpacity, transition: "opacity 150ms" }}>
      {showCollapsedDisplay ? (
        <CollapsedFieldDisplay
          allowStackedOverflow={allowStackedOverflow}
          className={className}
          disabled={props.disabled}
          id={id}
          mixed={mixed}
          onActivate={() => setIsFocused(true)}
          placeholder={computedPlaceholder}
          slot="input"
          value={mixed && !inputDisplayValue ? "" : inputDisplayValue}
        />
      ) : (
        <input
          ref={inputRef}
          type={type}
          data-slot="input"
          data-mixed={mixed ? "true" : undefined}
          id={id}
          className={cn(
            `file:text-element placeholder:text-muted-foreground text-element flex h-medium w-full min-w-0 border bg-transparent p-single text-base ${borderElementClass} file:inline-flex file:h-medium file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 md:text-sm`,
            formControlFocusBorderClass,
            "aria-invalid:ring-destructive/20 aria-invalid:border-destructive flex-1",
            mixed && "placeholder:italic placeholder:text-muted-foreground/70",
            type === "number" && "[&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]",
            className,
          )}
          value={mixed && !isFocused && !inputValue ? "" : inputValue}
          onChange={handleChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          placeholder={computedPlaceholder}
          {...uiFormControlBrowserDefaultProps}
          {...props}
        />
      )}
    </div>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {inputElement}
      </Label>
    );
  }

  return inputElement;
}

export { Input, CollapsedFieldDisplay, fitCollapsedFieldText, resolveCollapsedFieldDisplayState, COLLAPSED_FIELD_ELLIPSIS };

// #endregion 🩺️Input
