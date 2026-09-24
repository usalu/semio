/** @emoji 🃏️ Compact window-silhouette overview card for one play pane — icon title chip, tagline and open chip. */

import { cn, Icon, registerUiTranslationBundles, uiDataLabel, useLabel, WindowChrome, windowChromeTitleChipClass } from "@semio-tech/ui-react";
import type { PlayPaneSpec } from "./🪧️brand.ts";

//#region 🌐️PlayCardLabels
/** @emoji 🌐️ The card's own chrome strings — registered for English AND German so the overview never
 * carries a default language, even though play locks its shells to English. */
const playCardUiLabel = registerUiTranslationBundles({
  en: { translation: { play: { card: { open: { label: { normal: "Open", beginner: "Open this app" } }, openApp: { label: { normal: "Open {{label}}: {{tagline}}", beginner: "Open {{label}}: {{tagline}}" } } } } } },
  de: { translation: { play: { card: { open: { label: { normal: "Öffnen", beginner: "Diese App öffnen" } }, openApp: { label: { normal: "{{label}} öffnen: {{tagline}}", beginner: "{{label}} öffnen: {{tagline}}" } } } } } },
});
//#endregion 🌐️PlayCardLabels

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
  const openLabel = useLabel(playCardUiLabel("play.card.open"));
  const openAppLabel = useLabel(playCardUiLabel("play.card.openApp"), { label: pane.label, tagline: pane.tagline });

  return (
    <button
      type="button"
      data-play-pane-card=""
      data-pane-id={pane.id}
      data-hover-scope=""
      aria-label={openAppLabel}
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
            <Icon icon={pane.icon} size="small" title={uiDataLabel(pane.label)} className="shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />
            <span className="truncate text-sm font-medium text-foreground">{pane.label}</span>
          </div>
        }
        body={
          <div data-slot="play-pane-card-content" className="w-full min-w-0">
            <p data-slot="play-pane-card-tagline" className="truncate text-xs leading-normal text-muted-foreground">
              {pane.tagline}
            </p>
          </div>
        }
        footerRightChips={
          <div data-slot="play-pane-card-open-chip" className={windowChromeTitleChipClass}>
            <span className="inline-flex items-center gap-single px-single text-xs font-medium text-muted-foreground transition-colors group-hover:text-foreground">
              {openLabel}
              <Icon icon="chevron-right" size="small" className="transition-transform group-hover:translate-x-0.5" />
            </span>
          </div>
        }
        bodyClassName="px-double py-single"
      />
    </button>
  );
}
