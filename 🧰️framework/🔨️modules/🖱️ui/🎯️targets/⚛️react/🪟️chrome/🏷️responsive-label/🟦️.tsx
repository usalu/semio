import * as React from "react";

/** 📱 A shell label that shows its short form below the tablet breakpoint and its full form from tablet width up. Both forms stay available to assistive technology. Lives beside the slim chrome so a page that imports `@semio-tech/ui-react/chrome` does not load the React target barrel.
 * @see ../🟦️.ts — the slim chrome that re-exports this
 * @see ../../🟦️.tsx — the full React target, which re-exports it unchanged
 */
export const shellChromeTitleClassName = "truncate text-sm font-medium text-element";

/** 📱 The short form below the tablet breakpoint, the full form from tablet width up. */
export function ResponsiveLabel(props: { readonly full: string; readonly short?: string; readonly className?: string; readonly title?: string }): React.ReactElement {
  const short = props.short ?? props.full;
  if (short === props.full) return <span className={props.className} title={props.title ?? props.full}>{props.full}</span>;
  return (
    <span className={props.className} title={props.title ?? props.full}>
      <span aria-hidden="true" className="md:hidden">{short}</span>
      <span className="max-md:sr-only">{props.full}</span>
    </span>
  );
}
