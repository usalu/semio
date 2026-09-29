/** 🃏️ Compact overview card for one play pane — the shared {@link OverviewCard} as a whole-card button: icon title chip,
 * tagline and open chip. */

import { cn, Icon, OverviewCard, OverviewCardOpenChip, registerUiTranslationBundles, uiDataLabel, useLabel } from "@semio-tech/ui-react";
import type { PlayPaneSpec } from "./🪧️brand.ts";

//#region 🌐️PlayCardLabels
/** 🌐️ The card's own chrome strings — registered for English AND German so the overview never
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
  className,
}: {
  readonly pane: PlayPaneSpec;
  /** 🎈️ Lifted while its app is revealed — never maps to window `active` (that paints the primary silhouette stroke). */
  readonly lifted?: boolean;
  readonly onClick: () => void;
  readonly className?: string;
}) {
  const openLabel = useLabel(playCardUiLabel("play.card.open"));
  const openAppLabel = useLabel(playCardUiLabel("play.card.openApp"), { label: pane.label, tagline: pane.tagline });

  return (
    <OverviewCard
      as="button"
      slot="play-pane-card"
      label={openAppLabel}
      lifted={lifted}
      onClick={onClick}
      className={cn("max-w-xs", className)}
      icon={<Icon icon={pane.icon} size="small" title={uiDataLabel(pane.label)} className="shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />}
      title={pane.label}
      footerRight={<OverviewCardOpenChip slot="play-pane-card">{openLabel}</OverviewCardOpenChip>}
      bodyClassName="px-double py-single"
    >
      <span data-slot="play-pane-card-tagline" className="block truncate text-xs leading-normal text-muted-foreground">
        {pane.tagline}
      </span>
    </OverviewCard>
  );
}
