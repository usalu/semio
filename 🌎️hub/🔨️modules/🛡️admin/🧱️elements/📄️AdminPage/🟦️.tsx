// #region 🔌️Adapters
import * as React from "react";
import { SharedBorderRow, SharedBorderRowCell } from "@semio-tech/ui-react";
// #endregion 🔌️Adapters

/** 📄️ One page of the admin SPA: a flush column of bordered bands that starts exactly on the navbar's bottom line. The head band holds the title and the page actions as cells of one row.
 *
 * @see ../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow/🟦️.tsx */
export function AdminPage({ title, actions, children, "data-slot": slot = "admin-page" }: { readonly title: React.ReactNode; readonly actions?: React.ReactNode; readonly children?: React.ReactNode; readonly "data-slot"?: string }): React.ReactElement {
  return (
    <div className="flex h-full w-full flex-col overflow-auto pt-[var(--admin-clear-top,0px)] scroll-pt-[var(--admin-clear-top,0px)] [&_[data-slot=scroll-area]]:!h-auto" data-slot={slot}>
      <SharedBorderRow edges={["bottom"]} data-slot="admin-page-head">
        <SharedBorderRowCell className="flex-1">
          <h1 className="text-lg font-semibold text-emphasized">{title}</h1>
        </SharedBorderRowCell>
        {actions}
      </SharedBorderRow>
      {children}
    </div>
  );
}

/** 📄️ A headed run of bands inside {@link AdminPage}: the heading is a band of its own and everything after it follows flush. */
export function AdminSection({ id, title, children }: { readonly id: string; readonly title: React.ReactNode; readonly children?: React.ReactNode }): React.ReactElement {
  return (
    <section aria-labelledby={id} className="flex flex-col">
      <SharedBorderRow edges={["bottom"]}>
        <SharedBorderRowCell>
          <h2 id={id} className="font-semibold">
            {title}
          </h2>
        </SharedBorderRowCell>
      </SharedBorderRow>
      {children}
    </section>
  );
}

/** 📄️ A line of text as one band of {@link AdminPage}. */
export function AdminNote({ children, role, className, ...props }: React.ComponentProps<"p">): React.ReactElement {
  return (
    <SharedBorderRow edges={["bottom"]}>
      <SharedBorderRowCell className="flex-1">
        <p role={role} className={className ?? "text-sm text-muted-foreground"} {...props}>
          {children}
        </p>
      </SharedBorderRowCell>
    </SharedBorderRow>
  );
}
