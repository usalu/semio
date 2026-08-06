// #region 🧲️Header
// 💻️ framework/ui/elements/💬Dialog/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { veilClass, glassClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { useFlow, useLabel, CloseIconAlt, SurfaceScope } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🧸️Dialog
// Modal dialog built on Radix primitives.
// Consumers MUST use DialogTrigger to open.

/**
 * Dialog holds the data fields for a Dialog record.
 **/
function Dialog({ ...props }: React.ComponentProps<typeof DialogPrimitive.Root>) {
  return <DialogPrimitive.Root data-slot="dialog" {...props} />;
}

/**
 * DialogTrigger holds the data fields for a DialogTrigger record.
 **/
function DialogTrigger({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Trigger>) {
  return <DialogPrimitive.Trigger data-slot="dialog-trigger" className={cn(className)} {...props} />;
}

/**
 * DialogPortal holds the data fields for a DialogPortal record.
 **/
function DialogPortal({ ...props }: React.ComponentProps<typeof DialogPrimitive.Portal>) {
  return <DialogPrimitive.Portal data-slot="dialog-portal" {...props} />;
}

/**
 * DialogClose holds the data fields for a DialogClose record.
 **/
function DialogClose({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Close>) {
  return <DialogPrimitive.Close data-slot="dialog-close" className={cn(className)} {...props} />;
}

/**
 * DialogOverlay holds the data fields for a DialogOverlay record.
 **/
function DialogOverlay({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Overlay>) {
  return (
    <DialogPrimitive.Overlay
      data-slot="dialog-overlay"
      data-level="dialog"
      className={cn(veilClass, "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-dialog", className)}
      {...props}
    />
  );
}

/**
 * DialogContent holds the data fields for a DialogContent record.
 **/
function DialogContent({
  className,
  showCloseButton = true,
  children,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Content> & {
  showCloseButton?: boolean;
}) {
  const flow = useFlow();
  const closeLabel = useLabel("ui.common.close");
  return (
    <DialogPortal data-slot="dialog-portal">
      <DialogPrimitive.Content
        data-slot="dialog-content"
        data-level="dialog"
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        className={cn(
          "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-dialog grid w-full max-w-[calc(100%-2*var(--ui-spacing)*var(--medium))] translate-x-[-50%] translate-y-[-50%] gap-medium border p-medium duration-200 sm:max-w-lg",
          glassClass,
          className,
        )}
        {...props}
      >
        <SurfaceScope level="dialog" fill="glass">
          {children}
          {showCloseButton && (
            <DialogPrimitive.Close
              data-slot="dialog-close"
              className="ring-offset-background focus:ring-ring data-[state=open]:bg-accent data-[state=open]:text-muted-foreground absolute top-medium right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-small"
            >
              <CloseIconAlt />
              <span className="sr-only">{closeLabel}</span>
            </DialogPrimitive.Close>
          )}
        </SurfaceScope>
      </DialogPrimitive.Content>
    </DialogPortal>
  );
}

/**
 * DialogHeader holds the data fields for a DialogHeader record.
 **/
function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-header" className={cn("flex flex-col gap-single text-center sm:text-start", className)} {...props} />;
}

/**
 * DialogFooter holds the data fields for a DialogFooter record.
 **/
function DialogFooter({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-footer" className={cn("flex flex-col-reverse gap-single sm:flex-row sm:justify-end", className)} {...props} />;
}

/**
 * DialogTitle holds the data fields for a DialogTitle record.
 **/
function DialogTitle({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Title>) {
  return <DialogPrimitive.Title data-slot="dialog-title" className={cn("text-lg font-semibold leading-none tracking-tight", className)} {...props} />;
}

/**
 * DialogDescription holds the data fields for a DialogDescription record.
 **/
function DialogDescription({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Description>) {
  return <DialogPrimitive.Description data-slot="dialog-description" className={cn("text-muted-foreground text-sm", className)} {...props} />;
}

export { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger };

// #endregion 🧸️Dialog
