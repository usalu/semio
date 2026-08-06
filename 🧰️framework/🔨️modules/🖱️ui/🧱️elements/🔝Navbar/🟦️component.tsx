// #region 🧲️Header
// 💻️ framework/ui/elements/🔝Navbar/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../🧱️elements/☑️Select/🟦️component.tsx";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { reactHostPort } from "../🫀️core/🔌Ports/🟦️component.tsx";
import { type UiLabel } from "../🫀️core/🏷️UiLabel/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { shellFloorPaints, shellFloorFillClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { useSurface, SurfaceScope, NavbarTrailingFullscreenSlot, Label, Icon, useLabel, type IconName } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🩺️Navbar
// Top navigation bar with icon items.
// Consumers MUST provide NavbarItem entries.

/**
 * Configuration interface for a single navbar item.
 **/
export interface NavbarItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
  /** @emoji 🎯️ When true, positions the item absolutely so it is centered relative to the full navbar width, independent of sibling item widths. */
  centered?: boolean;
}

/**
 * Props interface for the Navbar component.
 **/
export interface NavbarProps {
  items: NavbarItem[];
  className?: string;
  showFullscreenToggle?: boolean;
}

/**
 * Navbar holds the data fields for a Navbar record.
 **/
function Navbar({ items, className, showFullscreenToggle = true }: NavbarProps) {
  const parent = useSurface();
  const paints = shellFloorPaints(parent);
  const bgClass = shellFloorFillClass(parent);
  const normalItems = items.filter((item) => !item.centered);
  const centeredItems = items.filter((item) => item.centered);
  const body = (
    <>
      <div className="p-single flex gap-single items-center min-w-0 h-full">
        {normalItems.map((item, index) => (
          <div key={item.key ?? index} className={cn("h-medium flex shrink-0 items-center min-w-0", item.className)}>
            {item.content}
          </div>
        ))}
        {showFullscreenToggle ? <NavbarTrailingFullscreenSlot /> : null}
      </div>
      {centeredItems.map((item, index) => (
        <div key={item.key ?? index} className="pointer-events-none absolute inset-0 flex items-center justify-center">
          <div className={cn("pointer-events-auto h-medium flex items-center", item.className)}>{item.content}</div>
        </div>
      ))}
    </>
  );
  return (
    <nav id="ui.navbar" data-slot="navbar" data-level="base" data-ui-reveal-region="navbar" data-elevation-root="" className={cn("relative h-large z-navbar", bgClass, className)}>
      {paints ? <SurfaceScope level="base" fill="surface">{body}</SurfaceScope> : body}
    </nav>
  );
}

export { Navbar };

//#region 🩺️SemioLogo
/** @emoji 🎨️ Round dark semio emblem for navbar and chrome. */
export function SemioLogo({ className, style }: { className?: string; style?: React.CSSProperties }) {
  return (
    <svg viewBox="0 0 350 350" className={className} style={style} xmlns="http://www.w3.org/2000/svg">
      <path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117" />
      <path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" strokeWidth="2.5" strokeMiterlimit="5" />
      <g fill="#ff344f" stroke="#f7f3e3" strokeWidth="2.5" strokeMiterlimit="5">
        <path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z" />
      </g>
      <g fill="#34d1bf" stroke="#f7f3e3" strokeWidth="2.5" strokeMiterlimit="5">
        <path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z" />
      </g>
    </svg>
  );
}
//#endregion 🩺️SemioLogo

//#region 🏷️ShellBrandLogo
/** @emoji 🏷️ Renders a shell brand's raw inline-SVG mark in navbar chrome (first-party repo content authored in `framework/os/dev/brand`, injected as markup). */
export function ShellBrandLogo({ svg, className, style }: { svg: string; className?: string; style?: React.CSSProperties }) {
  return <span className={cn("inline-flex items-center [&>svg]:h-full [&>svg]:w-auto", className)} style={style} dangerouslySetInnerHTML={{ __html: svg }} />;
}
//#endregion 🏷️ShellBrandLogo

/** @emoji ↔ Flex grow class that pushes trailing navbar chrome to the right edge. */
export const navbarFillClassName = "flex-1 min-w-0";

/** @emoji ↔ Invisible navbar filler; use before trailing toggles when no center slot consumes the flex region. */
export function navbarFillItem(key = "navbarFill"): NavbarItem {
  return { key, className: navbarFillClassName, content: null };
}

/** @emoji ↔ Navbar trailing control footprint (fullscreen toggle slot + flex gap). */
export const shellNavbarTrailingEndReserveCss = "calc(var(--size-medium) + var(--spacing-single))";

/** @emoji ∅ Sentinel id for the navbar “No example” row (matches {@link PLAYGROUND_NO_EXAMPLE_ID}). */
export const NAVBAR_NO_EXAMPLE_ID = "__none__";

/** @emoji 🧹️ Maps navbar sentinel / legacy empty ids to the canonical blank example id (`""`). */
export function normalizePlaygroundExampleId(exampleId: string): string {
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

// #endregion 🩺️Navbar
