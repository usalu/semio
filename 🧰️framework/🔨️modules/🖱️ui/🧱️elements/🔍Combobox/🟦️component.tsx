// #region 🧲️Header
// 💻️ framework/ui/elements/🔍Combobox/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { useTranslation } from "react-i18next";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { PropertyValueColumnContext } from "../🪵Tree/🟦️component.tsx";
import { Popover, PopoverTrigger, PopoverContent } from "../🗨️Popover/🟦️component.tsx";
import { Command, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem } from "../⌨️Command/🟦️component.tsx";
import { useTransaction, type ElementProps } from "../🐹️ElementProps/🟦️component.tsx";
import { useIdLabel, useLabel, Label } from "../🏷️Label/🟦️component.tsx";
import { CheckIcon } from "../🔣Icons/🟦️component.tsx";
import { ButtonGroup, ButtonGroupItem } from "../🎛️ButtonGroup/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 📧️Combobox
// Searchable dropdown with popover options list.
// Consumers MUST provide options and onValueChange handler.

/**
 * ComboboxOption holds the data fields for a ComboboxOption record.
 **/
interface ComboboxOption {
  value: string;
  label: string;
}

/**
 * ComboboxProps holds the data fields for a ComboboxProps record.
 **/
interface ComboboxProps extends ElementProps {
  options: ComboboxOption[];
  value?: string;
  placeholder?: UiLabel;
  placeholderId?: string;
  emptyMessage?: UiLabel;
  onValueChange?: (value: string) => void;
  className?: string;
  allowClear?: boolean;
  showLabel?: boolean;
}

/**
 * Searchable combobox dropdown with autocomplete filtering.
 **/
export const Combobox: React.FC<ComboboxProps> = ({ options, value = "", placeholder, placeholderId, emptyMessage, onValueChange, className, allowClear = false, showLabel, id }) => {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [open, setOpen] = reactHostPort.useState(false);
  const { t } = useTranslation();
  const placeholderIdLabel = useIdLabel(placeholderId);
  const selectOptionLabel = useLabel("ui.common.selectOption");
  const searchLabel = useLabel("ui.common.select");
  const noOptionsFoundLabel = useLabel("ui.common.noOptionsFound");
  const clearSelectionLabel = useLabel("ui.contextMenu.clearSelection");
  const computedPlaceholder = placeholderId ? placeholderIdLabel : (placeholder ?? selectOptionLabel);
  const resolvedEmptyMessage = emptyMessage ?? noOptionsFoundLabel;

  const selectedOption = options.find((option) => option.value === value);

  const handleOpenChange = (isOpen: boolean) => {
    setOpen(isOpen);
    if (isOpen) {
      transaction?.start?.();
    } else {
      transaction?.finalize?.();
    }
  };

  const handleSelect = (optionValue: string) => {
    if (allowClear && optionValue === value) {
      onValueChange?.("");
    } else {
      onValueChange?.(optionValue);
    }
    setOpen(false);
    transaction?.finalize?.();
  };

  const comboboxEmptyOpacity = isInPropertyValueColumn && !selectedOption && !open ? 0.6 : 1;

  const comboboxElement = (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <ButtonGroup detailPanelWidthMode="fill" style={{ opacity: comboboxEmptyOpacity, transition: "opacity 150ms" }}>
          <ButtonGroupItem id={id} role="combobox" aria-expanded={open} className="w-full min-w-0 justify-between" icon="chevrons-up-down" text={selectedOption ? selectedOption.label : computedPlaceholder} />
        </ButtonGroup>
      </PopoverTrigger>
      <PopoverContent className="w-full" align="start">
        <Command>
          <CommandInput placeholder={searchLabel} />
          <CommandList>
            <CommandEmpty>{resolvedEmptyMessage}</CommandEmpty>
            <CommandGroup>
              {allowClear && value && (
                <CommandItem value="" onSelect={() => handleSelect("")}>
                  <div className="me-2 size-tiny" />
                  <span className="text-muted-foreground italic">{clearSelectionLabel}</span>
                </CommandItem>
              )}
              {options.map((option) => (
                <CommandItem key={option.value} value={option.value} onSelect={() => handleSelect(option.value)}>
                  <CheckIcon className={cn("me-2 size-small", value === option.value ? "opacity-100" : "opacity-0")} />
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className={cn("h-medium", className)}>
        {comboboxElement}
      </Label>
    );
  }

  return comboboxElement;
};

// #endregion 📧️Combobox
