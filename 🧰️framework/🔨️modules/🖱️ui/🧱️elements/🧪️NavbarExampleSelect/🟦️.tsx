// #region 🧲️Header
// 💻️ framework/ui/elements/NavbarExampleSelectcomponent.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../🔽️Select/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { Label, useLabel } from "../🏷️Label/🟦️.tsx";
import { Icon, type IconName } from "../🔣️Icons/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🧪️NavbarExampleSelect
/** @emoji ∅ Sentinel id for the navbar “No example” row. */
const NAVBAR_NO_EXAMPLE_ID = "__none__";

/** @emoji 🧹️ Maps navbar sentinel / legacy empty ids to the canonical blank example id (`""`). */
function normalizePlaygroundExampleId(exampleId: string): string {
  return !exampleId || exampleId === NAVBAR_NO_EXAMPLE_ID || exampleId === "empty" ? "" : exampleId;
}

/** @emoji 🧪️ One selectable example row for {@link NavbarExampleSelect}. */
export interface NavbarExampleOption {
  readonly id: string;
  readonly label: string;
  readonly icon: IconName;
}

/** @emoji 🧪️ Props for {@link NavbarExampleSelect}. */
export interface NavbarExampleSelectProps {
  readonly id: string;
  readonly label?: UiLabel;
  readonly value: string;
  readonly options: readonly NavbarExampleOption[];
  readonly onValueChange: (exampleId: string) => void;
  readonly className?: string;
  readonly includeNoExample?: boolean;
}

/** @emoji 🧪️ Center-navbar dropdown for switching playground examples (kits, graphs, shape sources). */
function NavbarExampleSelect({ id, label, value, options, onValueChange, className, includeNoExample = true }: NavbarExampleSelectProps) {
  const exampleLabel = useLabel("ui.common.example");
  const noExampleLabel = useLabel("ui.common.noExample");
  const resolvedLabel = label ?? exampleLabel;
  const resolvedOptions = reactHostPort.useMemo(() => {
    const withoutSentinels = options.filter((row) => row.id !== NAVBAR_NO_EXAMPLE_ID && row.id !== "empty");
    if (!includeNoExample) return withoutSentinels;
    return [{ id: NAVBAR_NO_EXAMPLE_ID, label: noExampleLabel, icon: "eye-off" as IconName }, ...withoutSentinels];
  }, [includeNoExample, options, noExampleLabel]);
  if (resolvedOptions.length === 0) return null;
  const resolvedValue = !value || value === NAVBAR_NO_EXAMPLE_ID ? NAVBAR_NO_EXAMPLE_ID : value;
  const selectedOption = resolvedOptions.find((row) => row.id === resolvedValue);
  return (
    <div className={cn("flex min-w-0 max-w-md flex-1 items-center justify-center px-single", className)}>
      <Label id={`${id}.label`} label={resolvedLabel} className="sr-only" />
      <Select id={`${id}.select`} value={resolvedValue} onValueChange={(next) => onValueChange(normalizePlaygroundExampleId(next))}>
        <SelectTrigger className="h-medium w-full min-w-[12rem] max-w-md" id={`${id}.trigger`} size="sm">
          <span className="flex min-w-0 flex-1 items-center gap-single">
            {selectedOption ? <span data-slot="navbar-example-icon" className="inline-flex shrink-0"><Icon icon={selectedOption.icon} size="small" /></span> : null}
            <SelectValue placeholder={resolvedLabel} />
          </span>
        </SelectTrigger>
        <SelectContent>
          {resolvedOptions.map((row) => (
            <SelectItem key={row.id} value={row.id} icon={row.icon}>
              <span className="truncate">{row.label}</span>
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}

export { NavbarExampleSelect };
// #endregion 🧪️NavbarExampleSelect
