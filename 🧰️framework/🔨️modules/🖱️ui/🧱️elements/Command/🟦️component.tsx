// #region 🧲️Header
// 💻️ framework/ui/elements/Command/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Command as CommandPrimitive } from "cmdk";
// 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
// 🧱️elements/<Element>/ or 🧱️elements/🫀️core/ dirs) — W3 rewires this import per-symbol as each
// dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf file
// without the same marker; grep for `🚧️W3-interim` must be empty before W6 closes.
import { cn, useLabel, type UiLabel, borderNormalBottomClass, uiFormControlBrowserDefaultProps, menuListItemClassName, SearchIcon, Dialog, DialogHeader, DialogTitle, DialogDescription, DialogContent } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🪆️Command
// Command palette UI built on cmdk primitives.
// Consumers MUST use CommandInput for search functionality.

/**
 * Command holds the data fields for a Command record.
 **/
function Command({ className, ...props }: React.ComponentProps<typeof CommandPrimitive>) {
  // 🎨️ Transparent — every consumer (CommandDialog's DialogContent, Popover-based combobox/search menus) already
  // renders the level's glass; Command painting its own would be a second glass layer on top of the host's.
  return <CommandPrimitive data-slot="command" className={cn("bg-transparent text-popover-foreground flex h-full w-full flex-col overflow-hidden", className)} {...props} />;
}

/**
 * CommandDialog holds the data fields for a CommandDialog record.
 **/
function CommandDialog({
  title,
  description,
  children,
  className,
  showCloseButton = true,
  shouldFilter,
  ...props
}: React.ComponentProps<typeof Dialog> & {
  title?: UiLabel;
  description?: string;
  className?: string;
  showCloseButton?: boolean;
  /** @emoji 🔍️ When false, host filters items (e.g. Fuse) and cmdk must not re-filter. */
  shouldFilter?: boolean;
}) {
  const commandPaletteLabel = useLabel("ui.common.commandPalette");
  const searchForCommandLabel = useLabel("ui.common.searchForCommand");
  const resolvedTitle = title ?? commandPaletteLabel;
  const resolvedDescription = description ?? searchForCommandLabel;
  return (
    <Dialog {...props}>
      <DialogHeader className="sr-only">
        <DialogTitle>{resolvedTitle}</DialogTitle>
        <DialogDescription>{resolvedDescription}</DialogDescription>
      </DialogHeader>
      <DialogContent className={cn("overflow-hidden p-0", className)} showCloseButton={showCloseButton}>
        <Command
          shouldFilter={shouldFilter}
          className="[&_[cmdk-group-heading]]:text-muted-foreground **:data-[slot=command-input-wrapper]:h-large [&_[cmdk-group-heading]]:px-single [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group]:px-single [&_[cmdk-group]:not([hidden])_~[cmdk-group]]:pt-0 [&_[cmdk-input-wrapper]_svg]:h-small [&_[cmdk-input-wrapper]_svg]:w-small [&_[cmdk-input]]:h-large [&_[cmdk-item]]:px-single [&_[cmdk-item]]:py-tiny [&_[cmdk-item]_svg]:h-small [&_[cmdk-item]_svg]:w-small"
        >
          {children}
        </Command>
      </DialogContent>
    </Dialog>
  );
}

/**
 * CommandInput holds the data fields for a CommandInput record.
 **/
function CommandInput({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Input>) {
  return (
    <div data-slot="command-input-wrapper" className={cn("flex h-medium items-center gap-single px-tiny", borderNormalBottomClass)}>
      <SearchIcon className="size-small shrink-0 opacity-50" />
      <CommandPrimitive.Input
        data-slot="command-input"
        className={cn("placeholder:text-muted-foreground flex h-medium w-full bg-transparent text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50", className)}
        {...uiFormControlBrowserDefaultProps}
        {...props}
      />
    </div>
  );
}

/**
 * CommandList holds the data fields for a CommandList record.
 **/
function CommandList({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.List>) {
  return <CommandPrimitive.List data-slot="command-list" className={cn("max-h-layout-command scroll-py-single overflow-x-hidden overflow-y-auto", className)} {...props} />;
}

/**
 * CommandEmpty holds the data fields for a CommandEmpty record.
 **/
function CommandEmpty({ ...props }: React.ComponentProps<typeof CommandPrimitive.Empty>) {
  return <CommandPrimitive.Empty data-slot="command-empty" className="py-medium text-center text-sm" {...props} />;
}

/**
 * CommandGroup holds the data fields for a CommandGroup record.
 **/
function CommandGroup({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Group>) {
  return (
    <CommandPrimitive.Group
      data-slot="command-group"
      className={cn(
        "text-element [&_[cmdk-group-heading]]:text-muted-foreground overflow-hidden p-single [&_[cmdk-group-heading]]:px-single [&_[cmdk-group-heading]]:py-single [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium",
        className,
      )}
      {...props}
    />
  );
}

function CommandSeparator({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Separator>) {
  return <CommandPrimitive.Separator data-slot="command-separator" className={cn("bg-border -mx-single h-px", className)} {...props} />;
}

/**
 * CommandItem holds the data fields for a CommandItem record.
 **/
function CommandItem({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Item>) {
  return (
    <CommandPrimitive.Item
      data-slot="command-item"
      className={cn(
        "[&_svg:not([class*='text-'])]:text-muted-foreground relative flex items-center gap-single p-single text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny cursor-selectable",
        menuListItemClassName,
        className,
      )}
      {...props}
    />
  );
}

/**
 * CommandShortcut holds the data fields for a CommandShortcut record.
 **/
function CommandShortcut({ className, ...props }: React.ComponentProps<"span">) {
  return <span data-slot="command-shortcut" className={cn("text-muted-foreground ms-auto text-xs tracking-widest", className)} {...props} />;
}

export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };

// #endregion 🪆️Command
