import { Button, SharedBorderRow, SharedBorderRowCell, cn, type ButtonProps } from "@semio-tech/ui-react";
import type { ComponentProps, ReactElement, ReactNode } from "react";

/** 🧱️ A floating stack of shell notices: one shared frame whose notices are cells separated by single hairlines, never a box per notice or a gap between them. `className` positions the stack. */
export function NoticeStack({ className, children, ...props }: Omit<ComponentProps<"div">, "children"> & { readonly children: ReactNode }): ReactElement {
  return (
    <SharedBorderRow orientation="vertical" className={cn("pointer-events-auto absolute z-50 bg-base text-sm shadow-sm", className)} {...props}>
      {children}
    </SharedBorderRow>
  );
}

/** 🧱️ One notice of a {@link NoticeStack}: a plain cell with the standard notice padding. */
export function NoticeCell({ className, ...props }: ComponentProps<"div">): ReactElement {
  return <SharedBorderRowCell className={cn("flex-col items-stretch px-double py-single", className)} {...props} />;
}

/** 🧱️ A floating pane (workspace, administration, service panel): the frame is owned by the shared-border compositor on the fixed outer box, the scroller sits inside so the hairline never scrolls away. */
export function FloatingPane({ className, scrollClassName, children, ...props }: Omit<ComponentProps<"div">, "children"> & { readonly scrollClassName?: string; readonly children: ReactNode }): ReactElement {
  return (
    <SharedBorderRow orientation="vertical" className={cn("pointer-events-auto absolute z-50 bg-base shadow-sm", className)} {...props}>
      <div className={cn("min-h-0 w-full min-w-0 overflow-auto", scrollClassName)}>{children}</div>
    </SharedBorderRow>
  );
}

/** 🧱️ One notice with its action: a horizontal shared row of the notice text as a cell and one framework button as the next cell, no gap and no native border. Without an `action` it is the text cell alone. `actionProps` carries the button's id and data attributes. */
export function NoticeActionRow({ text, action, icon = "x", actionProps, children, ...props }: Omit<ComponentProps<"div">, "children"> & { readonly text: ReactNode; readonly action: { readonly label: string; readonly onPress: () => void } | null; readonly icon?: ButtonProps["icon"]; readonly actionProps?: Readonly<Record<`data-${string}` | "id", string>>; readonly children?: ReactNode }): ReactElement {
  return (
    <SharedBorderRow {...props}>
      <NoticeCell className="flex-1 flex-row items-center">{text}{children}</NoticeCell>
      {action === null ? null : <Button icon={icon} type="button" variant="outline" {...actionProps} onClick={action.onPress}>{action.label}</Button>}
    </SharedBorderRow>
  );
}
