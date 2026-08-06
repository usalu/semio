// #region 🧲️Header
// 💻️ framework/ui/elements/Textarea/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../🫀️core/ClassNames/🟦️component.tsx";
import { reactHostPort } from "../🫀️core/Ports/🟦️component.tsx";
import { PropertyValueColumnContext } from "../Tree/🟦️component.tsx";
import { type UiLabel, uiDataLabel } from "../🫀️core/UiLabel/🟦️component.tsx";
import { CollapsedFieldDisplay } from "../Input/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { borderElementClass, formControlFocusBorderClass, uiFormControlBrowserDefaultProps } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { useTransaction, type ElementProps, useIdLabel, useLabel, Label } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🎏️Textarea
// Multi-line text input with label and validation.
// Consumers MUST provide an id for the field.

/**
 * TextareaProps holds the data fields for a TextareaProps record.
 **/
interface TextareaProps extends Omit<React.ComponentProps<"textarea">, "value" | "onChange" | "id">, ElementProps {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onLazyChange?: (value: string) => void;
  showLabel?: boolean;
  placeholderId?: string;
  readOnly?: boolean;
  mixed?: boolean;
}

/**
 **/
function Textarea({ className, lazy, value: externalValue, onChange, onLazyChange, id, showLabel, placeholderId, placeholder, mixed, rows, ...props }: TextareaProps) {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [localValue, setLocalValue] = reactHostPort.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [isFocused, setIsFocused] = reactHostPort.useState(false);
  const textareaRef = reactHostPort.useRef<HTMLTextAreaElement>(null);
  const placeholderIdLabel = useIdLabel(placeholderId);
  const computedPlaceholder: UiLabel | undefined = placeholderId ? placeholderIdLabel : placeholder !== undefined ? uiDataLabel(placeholder) : undefined;
  const mixedLabel = useLabel("ui.common.mixedValues");
  const effectivePlaceholder: UiLabel | undefined = mixed ? mixedLabel || uiDataLabel("—") : computedPlaceholder;

  reactHostPort.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  reactHostPort.useEffect(() => {
    if (isFocused && textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [isFocused]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLTextAreaElement>) => {
    setIsFocused(true);
    if (lazy) {
      setIsEditing(true);
      transaction?.start?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLTextAreaElement>) => {
    setIsFocused(false);
    if (lazy) {
      setIsEditing(false);
      onLazyChange?.(localValue);
      transaction?.finalize?.();
    }
    props.onBlur?.(e);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      if (e.key === "Escape") {
        setIsEditing(false);
        setLocalValue(externalValue?.toString() || "");
        transaction?.abort?.();
        (e.target as HTMLTextAreaElement).blur();
      }
    }
    props.onKeyDown?.(e);
  };

  const textareaValue = lazy ? localValue : externalValue;
  const displayValue = textareaValue?.toString() || "";
  const showCollapsedDisplay = !!showLabel && !isFocused;
  const useSingleRowPropertyEditor = isInPropertyValueColumn && !!showLabel;

  const textareaEmptyOpacity = isInPropertyValueColumn && !displayValue && !isFocused ? 0.6 : 1;

  const textareaElement = (
    <div data-slot="textarea-root" data-detail-panel-control="fill" className="flex min-w-0 w-full flex-1 items-stretch" style={{ opacity: textareaEmptyOpacity, transition: "opacity 150ms" }}>
      {!showCollapsedDisplay ? (
        <textarea
          ref={textareaRef}
          data-slot="textarea"
          data-mixed={mixed ? "true" : undefined}
          id={id}
          className={cn(
            `placeholder:text-muted-foreground text-element flex w-full border bg-transparent text-base ${borderElementClass} disabled:cursor-not-allowed disabled:opacity-50 md:text-sm`,
            formControlFocusBorderClass,
            "flex-1",
            useSingleRowPropertyEditor ? "h-medium min-h-medium max-h-medium resize-none overflow-y-auto px-single py-single leading-normal" : "field-sizing-content min-h-huge px-tiny py-single",
            className,
          )}
          rows={useSingleRowPropertyEditor ? 1 : rows}
          value={textareaValue}
          onChange={handleChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          placeholder={effectivePlaceholder}
          {...uiFormControlBrowserDefaultProps}
          {...props}
        />
      ) : (
        <CollapsedFieldDisplay
          allowStackedOverflow={true}
          className={className}
          disabled={props.disabled}
          id={id}
          mixed={mixed}
          onActivate={() => setIsFocused(true)}
          placeholder={effectivePlaceholder}
          slot="textarea"
          value={mixed && !displayValue ? "" : displayValue}
        />
      )}
    </div>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className="items-start">
        {textareaElement}
      </Label>
    );
  }

  return textareaElement;
}

export { Textarea };

// #endregion 🎏️Textarea
