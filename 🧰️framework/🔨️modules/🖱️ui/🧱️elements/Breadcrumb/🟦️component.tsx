// #region 🧲️Header
// 💻️ framework/ui/elements/Breadcrumb/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as DropdownMenuPrimitive from "@radix-ui/react-dropdown-menu";
// 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
// 🧱️elements/<Element>/ or 🧱️elements/🫀️core/ dirs) — W3 rewires this import per-symbol as each
// dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf file
// without the same marker; grep for `🚧️W3-interim` must be empty before W6 closes.
import { reactHostPort, borderNormalClass, cn, interactiveHoverClass, interactiveControlTransitionClass, ChromeControlHint, ChevronDownIcon, ChevronRightIcon, glassClass, SurfaceScope } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 💡️Breadcrumb
// Breadcrumb trail for hierarchical page navigation.
// Consumers MUST provide BreadcrumbItemData entries.

/**
 * Data interface for a single breadcrumb entry.
 **/
export interface BreadcrumbItemData {
  id?: string;
  content: React.ReactNode;
  options?: { label: React.ReactNode; href: string; id?: string }[];
  onNavigate?: (href: string) => void;
}

/**
 * BreadcrumbProps holds the data fields for a BreadcrumbProps record.
 **/
interface BreadcrumbProps extends Omit<React.ComponentProps<"nav">, "children"> {
  items: BreadcrumbItemData[];
}

/** Breadcrumb holds the data fields for a Breadcrumb record.
 **/
/**
 **/
function Breadcrumb({ className, items, ...props }: BreadcrumbProps) {
  const [openIndex, setOpenIndex] = reactHostPort.useState<number | null>(null);
  const borderClass = borderNormalClass;

  return (
    <nav aria-label="breadcrumb" data-slot="breadcrumb" className={cn("flex h-medium items-stretch border", borderClass, className)} {...props}>
      <ol data-slot="breadcrumb-list" className="flex flex-nowrap items-stretch text-xs break-words overflow-hidden h-full min-w-0">
        {items.map((item, index) => {
          const hasOptions = !!(item.options && item.options.length > 0);
          const isOpen = openIndex === index;

          return (
            <React.Fragment key={index}>
              <BreadcrumbItem {...item} />
              <BreadcrumbSeparatorItem hasOptions={hasOptions} isOpen={isOpen} onOpenChange={(open) => setOpenIndex(open ? index : null)} id={item.id} options={item.options} onNavigate={item.onNavigate} />
            </React.Fragment>
          );
        })}
      </ol>
    </nav>
  );
}

/**
 * BreadcrumbItemProps holds the data fields for a BreadcrumbItemProps record.
 **/
interface BreadcrumbItemProps extends Omit<React.ComponentProps<"li">, "content"> {
  id?: string;
  content?: React.ReactNode;
  onNavigate?: (href: string) => void;
  options?: { label: React.ReactNode; href: string; id?: string }[];
}

/**
 * BreadcrumbItem holds the data fields for a BreadcrumbItem record.
 **/
function BreadcrumbItem({ className, id, content, children, onNavigate, options, ...props }: BreadcrumbItemProps) {
  const hoverClass = interactiveHoverClass;
  const itemContent = content ?? children;
  const interactiveContent = reactHostPort.useMemo(() => {
    if (itemContent == null || typeof itemContent === "boolean") return null;
    if (React.isValidElement(itemContent)) {
      if (itemContent.type === React.Fragment) {
        return (
          <span data-slot="breadcrumb-link" className={cn("cursor-selectable flex h-full min-w-0 items-center px-single text-element", interactiveControlTransitionClass, hoverClass)}>
            {itemContent}
          </span>
        );
      }
      const elementProps = itemContent.props as { className?: string; ["data-slot"]?: string };
      return React.cloneElement(itemContent as React.ReactElement<any>, {
        className: cn("cursor-selectable h-full min-w-0 px-single text-element", interactiveControlTransitionClass, hoverClass, elementProps?.className),
        "data-slot": elementProps?.["data-slot"] ?? "breadcrumb-link",
      });
    }
    return (
      <span data-slot="breadcrumb-link" className={cn("cursor-selectable flex h-full min-w-0 items-center px-single text-element", interactiveControlTransitionClass, hoverClass)}>
        {itemContent}
      </span>
    );
  }, [hoverClass, itemContent]);

  const itemElement = (
    <li data-slot="breadcrumb-item" id={id} className={cn("flex h-full min-w-0 items-stretch cursor-selectable overflow-hidden", className)} {...props}>
      {interactiveContent}
    </li>
  );

  if (id) {
    return <ChromeControlHint id={id}>{itemElement}</ChromeControlHint>;
  }

  return itemElement;
}

/**
 * BreadcrumbSeparatorItemProps holds the data fields for a BreadcrumbSeparatorItemProps record.
 **/
interface BreadcrumbSeparatorItemProps {
  hasOptions: boolean;
  isOpen: boolean;
  onOpenChange?: (open: boolean) => void;
  id?: string;
  options?: { label: React.ReactNode; href: string; id?: string }[];
  onNavigate?: (href: string) => void;
}

/** BreadcrumbSeparatorItem holds the data fields for a BreadcrumbSeparatorItem record.
 **/
/**
 **/
function BreadcrumbSeparatorItem({ hasOptions, isOpen, onOpenChange, id, options, onNavigate }: BreadcrumbSeparatorItemProps) {
  const hoverClass = interactiveHoverClass;
  const icon = isOpen ? <ChevronDownIcon className="cursor-foldable" /> : <ChevronRightIcon className="cursor-foldable" />;

  const handleSelect = (href: string) => {
    onOpenChange?.(false);
    onNavigate?.(href);
  };

  const separatorControlClassName = cn(
    "text-element inline-flex h-full aspect-square items-center justify-center shrink-0 p-single cursor-selectable overflow-hidden outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[length:var(--stroke-focus)] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive rounded-none [&_svg]:pointer-events-none [&_svg]:size-tiny [&_svg]:shrink-0",
    interactiveControlTransitionClass,
    hoverClass,
  );

  if (!hasOptions || !options?.length) {
    return (
      <li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" className="flex h-full items-stretch">
        <div data-slot="breadcrumb-separator-control" className={cn(separatorControlClassName, "pointer-events-none")}>
          {icon}
        </div>
      </li>
    );
  }
  return (
    <li data-slot="breadcrumb-separator" role="presentation" className="flex h-full items-stretch">
      <DropdownMenuPrimitive.Root open={isOpen} onOpenChange={onOpenChange}>
        <DropdownMenuPrimitive.Trigger asChild>
          <button type="button" id={id && !isOpen ? id : undefined} data-slot="breadcrumb-separator-control" className={separatorControlClassName}>
            {icon}
          </button>
        </DropdownMenuPrimitive.Trigger>
        <DropdownMenuPrimitive.Portal>
          <DropdownMenuPrimitive.Content data-level="menu" align="center" sideOffset={8} className={cn("w-auto overflow-hidden border p-single z-menu", borderNormalClass, glassClass)}>
            <SurfaceScope level="menu" fill="glass">
              {options.map((item, index) => {
                const menuItem = (
                  <DropdownMenuPrimitive.Item
                    key={index}
                    className="text-element hover:bg-hover-interactive-fill hover:text-emphasized focus:bg-hover-interactive-fill focus:text-emphasized relative flex items-center p-single text-sm outline-none whitespace-nowrap"
                    onClick={() => handleSelect(item.href)}
                    role="button"
                  >
                    {item.label}
                  </DropdownMenuPrimitive.Item>
                );

                const wrappedItem = item.id ? <ChromeControlHint id={item.id}>{menuItem}</ChromeControlHint> : menuItem;

                return (
                  <React.Fragment key={index}>
                    {wrappedItem}
                    {index < options.length - 1 && <DropdownMenuPrimitive.Separator className="h-px bg-border my-single" />}
                  </React.Fragment>
                );
              })}
            </SurfaceScope>
          </DropdownMenuPrimitive.Content>
        </DropdownMenuPrimitive.Portal>
      </DropdownMenuPrimitive.Root>
    </li>
  );
}

export { Breadcrumb, BreadcrumbItem };

// #endregion 💡️Breadcrumb
