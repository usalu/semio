// #region 🧲️Header
// 💻️ framework/ui/elements/ShellSearchDialog/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { CommandDialog, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem, CommandShortcut } from "../Command/🟦️component.tsx";
// 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
// 🧱️elements/<Element>/ or 🧱️elements/🫀️core/ dirs) — W3 rewires this import per-symbol as each
// dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf file
// without the same marker; grep for `🚧️W3-interim` must be empty before W6 closes.
import { useLabel, renderControlIcon, type ControlIcon } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🔎️ShellSearchDialog
// Pure, prop-driven command-palette-style search dialog built from the Command/Dialog primitives above.
// A shell supplies the query/results and owns filtering/navigation; this component only renders and forwards callbacks.

/**
 * A single pickable result row shared by {@link ShellSearchDialog} and {@link ShellFindDialog}.
 **/
export interface ShellCommandResult {
  readonly id: string;
  readonly label: string;
  readonly icon?: string;
  readonly hotkey?: string;
}

/**
 * Props interface for the ShellSearchDialog component.
 **/
export interface ShellSearchDialogProps {
  readonly open: boolean;
  readonly query: string;
  readonly onQueryChange: (query: string) => void;
  readonly results: readonly ShellCommandResult[];
  readonly onPick: (id: string) => void;
  readonly onClose: () => void;
}

/** @emoji 🔎️ Prop-driven command-palette search dialog a shell renders over its own search results. */
export const ShellSearchDialog: React.FC<ShellSearchDialogProps> = ({ open, query, onQueryChange, results, onPick, onClose }) => {
  const titleLabel = useLabel("ui.search.title");
  const descriptionLabel = useLabel("ui.search.description");
  const placeholderLabel = useLabel("ui.search.placeholder");
  const emptyLabel = useLabel("ui.search.empty");
  return (
    <CommandDialog open={open} onOpenChange={(next) => !next && onClose()} shouldFilter={false} title={titleLabel} description={descriptionLabel}>
      <CommandInput value={query} onValueChange={onQueryChange} placeholder={placeholderLabel} />
      <CommandList>
        <CommandEmpty>{emptyLabel}</CommandEmpty>
        <CommandGroup>
          {results.map((result) => (
            <CommandItem key={result.id} value={result.id} onSelect={() => onPick(result.id)}>
              {renderControlIcon(result.icon as ControlIcon | undefined, "tiny")}
              <span>{result.label}</span>
              {result.hotkey ? <CommandShortcut>{result.hotkey}</CommandShortcut> : null}
            </CommandItem>
          ))}
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
};

// #endregion 🔎️ShellSearchDialog
