// #region 🧲️Header
// 💻️ framework/ui/elements/ShellFindDialog/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { CommandDialog, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem, CommandShortcut } from "../Command/🟦️component.tsx";
import { type ShellCommandResult } from "../ShellSearchDialog/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { useLabel, renderControlIcon, type ControlIcon } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🔎️ShellFindDialog
// Pure, prop-driven command-palette-style find-in-scene dialog built from the Command/Dialog primitives above.
// A shell supplies the query/results and owns filtering/navigation; this component only renders and forwards callbacks.

/**
 * Props interface for the ShellFindDialog component.
 **/
export interface ShellFindDialogProps {
  readonly open: boolean;
  readonly query: string;
  readonly onQueryChange: (query: string) => void;
  readonly results: readonly ShellCommandResult[];
  readonly onPick: (id: string) => void;
  readonly onClose: () => void;
}

/** @emoji 🔎️ Prop-driven command-palette find dialog a shell renders over its own find-in-scene results. */
export const ShellFindDialog: React.FC<ShellFindDialogProps> = ({ open, query, onQueryChange, results, onPick, onClose }) => {
  const titleLabel = useLabel("ui.find.title");
  const descriptionLabel = useLabel("ui.find.description");
  const placeholderLabel = useLabel("ui.find.placeholder");
  const emptyLabel = useLabel("ui.find.empty");
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

// #endregion 🔎️ShellFindDialog
