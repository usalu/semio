/** @emoji 🃏️ Compact window-silhouette overview card for one play pane — icon title chip and tagline. */

import { cn, Icon, WindowChrome, windowChromeTitleChipClass } from "@semio-tech/ui-react";
import type { PlayPaneSpec } from "./🪧️brand.ts";

export function PlayCard({
  pane,
  lifted,
  onClick,
  onMouseEnter,
  onMouseLeave,
  onFocus,
  className,
}: {
  readonly pane: PlayPaneSpec;
  /** @emoji 🎈️ Pointer-hover lift only — never maps to window `active` (that paints the primary silhouette stroke). */
  readonly lifted?: boolean;
  readonly onClick: () => void;
  readonly onMouseEnter?: () => void;
  readonly onMouseLeave?: () => void;
  readonly onFocus?: () => void;
  readonly className?: string;
}) {
  return (
    <button
      type="button"
      data-play-pane-card=""
      data-pane-id={pane.id}
      data-hover-scope=""
      title={pane.description}
      aria-label={`Open ${pane.label}: ${pane.tagline}`}
      onClick={onClick}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      onFocus={onFocus}
      className={cn(
        "pointer-events-auto group w-full min-w-0 max-w-xs cursor-pointer border-0 bg-transparent p-0 text-left outline-none",
        "transition-transform duration-200",
        "hover:-translate-y-0.5 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
        lifted && "-translate-y-0.5",
        className,
      )}
    >
      <WindowChrome
        level="dialog"
        active={false}
        stackSlot="play-pane-card-stack"
        stackClassName="w-full min-w-0"
        titleChips={
          <div data-slot="play-pane-card-title-chip" className={cn(windowChromeTitleChipClass, "flex min-w-0 items-center gap-single px-single")}>
            <Icon icon={pane.icon} size="small" className="shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />
            <span className="truncate text-sm font-medium text-foreground">{pane.label}</span>
          </div>
        }
        body={
          <p data-slot="play-pane-card-tagline" className="truncate text-xs leading-normal text-muted-foreground transition-colors group-hover:text-foreground">
            {pane.tagline}
          </p>
        }
        bodyClassName="px-double py-single"
      />
    </button>
  );
}
