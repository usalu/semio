/** 🔗️ One shared-border frame for a row of adjacent chips: the frame owns the outline, each chip is a cell and every seam is a single hairline.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}; mechanism: `@semio-tech/ui/elements/WindowChrome` `shared-border.ts`.
 */

import { type FC, type ReactNode } from "@semio-tech/ui-react";
import { useSharedBorderFrameRef } from "@semio-tech/ui-react/chrome";

/** 🧩️ Spread onto every chip inside a {@link SharedBorderChipGroup}. */
export const SHARED_BORDER_CELL = { "data-shared-border-cell": "" } as const;

/** 🔗️ Horizontal frame whose direct children are shared-border cells; never combine with a flex gap. */
export const SharedBorderChipGroup: FC<{ readonly className: string; readonly role?: string; readonly "aria-label"?: string; readonly children: ReactNode }> = ({ className, children, ...rest }) => {
  const ref = useSharedBorderFrameRef<HTMLDivElement>();
  return (
    <div ref={ref} data-shared-border-frame="" data-shared-border-edges="top right bottom left" data-orientation="horizontal" className={className} {...rest}>
      {children}
    </div>
  );
};
