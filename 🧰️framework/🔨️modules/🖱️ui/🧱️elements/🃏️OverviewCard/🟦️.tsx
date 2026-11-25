// #region 🧲️Header
// 💻️ framework/ui/elements/🃏️OverviewCard/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { CatalogIcon as Icon } from "../🔣️Icons/🟦️.tsx";
import { WindowChrome, windowChromeTitleChipClass } from "../🗂️WindowChrome/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🃏️OverviewCard
/** 🃏️ What every overview card shows: a title chip (glyph and title) overlaid on the top of the body, the body, and optional chips overlaid on the bottom left
 * and right. `slot` prefixes the card's `data-slot`s (`<slot>-title-chip`, `<slot>-content`, `<slot>-stack`), so each
 * product keeps its own hooks for styles and tests. */
export interface OverviewCardBaseProps {
  readonly slot: string;
  readonly icon: React.ReactNode;
  readonly title: React.ReactNode;
  readonly titleAdornment?: React.ReactNode;
  readonly children?: React.ReactNode;
  readonly footerLeft?: React.ReactNode;
  readonly footerRight?: React.ReactNode;
  readonly className?: string;
  readonly bodyClassName?: string;
  readonly contentClassName?: string;
  /** 🧱️ Takes the body's padding away and makes the content a vertical shared-border group: its direct children are
   * cells (`data-shared-border-cell`, each with its own padding) whose single dividers lie between them (framed controls placed directly in it are cells too, see `data-shared-border-cells`) and whose outer
   * edges are the card's own frame, so no frame of a child lies inside the card's frame with a gap. */
  readonly flush?: boolean;
  readonly data?: Readonly<Record<`data-${string}`, string | undefined>>;
}

/** 📑️ A card that is a region of the page: a `<section>` named by its heading, whose actions are real buttons in its
 * footer chips — the shape for cards that carry more than one action. */
export interface OverviewCardSectionProps extends OverviewCardBaseProps {
  readonly as: "section";
  readonly headingId: string;
  readonly headingLevel?: 1 | 2 | 3 | 4 | 5 | 6;
  readonly headingRef?: React.Ref<HTMLHeadingElement>;
  readonly focusableHeading?: boolean;
}

/** 🔘️ A card that is itself one action (open this app): the whole card is a `<button>` whose frame responds to hover. Its body
 * holds phrasing text only, so the button keeps a valid content model. */
export interface OverviewCardButtonProps extends OverviewCardBaseProps {
  readonly as: "button";
  readonly onClick: () => void;
  readonly label?: string;
}

/** ⚙️ Props of {@link OverviewCard}. */
export type OverviewCardProps = OverviewCardSectionProps | OverviewCardButtonProps;

const CARD_STACK_DATA = { "data-overview-card-stack": "", "data-shared-border-hover": "" } as const;

/** 💳️ The overview card of semio's landing pages (play, the demonstrator, the quizzes): the design system's
 * {@link WindowChrome} at dialog level, never `active`, with the title in a glass chip on its top row and actions in glass chips on its bottom row, both inside the frame.
 * Sharp corners, glass and the 1 px frame come from the chrome: the card is one rectangle, the title chip and the footer chips are glass overlays inside it above the body, and the body keeps the chip row clearance. The stack stays transparent through
 * `data-overview-card-stack`, and its outline is emphasised while the pointer is over the card.
 *
 * @see ../🗂️WindowChrome/🟦️.tsx
 * @see https://www.w3.org/WAI/ARIA/apg/practices/landmark-regions/ — sections named by their heading */
export function OverviewCard(props: OverviewCardProps): React.ReactElement {
  const { slot, icon, title, titleAdornment, children, footerLeft, footerRight, className, bodyClassName, contentClassName, flush = false, data } = props;
  const Heading = props.as === "section" ? (`h${props.headingLevel ?? 2}` as const) : "span";
  const chrome = (
    <WindowChrome
      level="dialog"
      active={false}
      stackSlot={`${slot}-stack`}
      stackDataAttrs={CARD_STACK_DATA}
      stackClassName="h-full w-full min-w-0"
      titleChips={
        <div data-slot={`${slot}-title-chip`} className={cn(windowChromeTitleChipClass, "flex min-w-0 max-w-full items-center gap-single px-single")}>
          {icon}
          <Heading
            id={props.as === "section" ? props.headingId : undefined}
            ref={props.as === "section" ? props.headingRef : undefined}
            tabIndex={props.as === "section" && props.focusableHeading ? -1 : undefined}
            className="m-0 truncate text-sm font-medium text-foreground outline-none"
          >
            {title}
          </Heading>
          {titleAdornment}
        </div>
      }
      body={
        <div data-slot={`${slot}-content`} data-shared-border-group={flush ? "" : undefined} data-shared-border-cells={flush ? "" : undefined} data-orientation={flush ? "vertical" : undefined} className={cn("w-full min-w-0", contentClassName)}>
          {children}
        </div>
      }
      footerLeftChips={footerLeft}
      footerRightChips={footerRight}
      bodyClassName={bodyClassName ?? (flush ? "p-0" : "p-double")}
    />
  );
  if (props.as === "section") {
    return (
      <section {...data} data-overview-card="" aria-labelledby={props.headingId} className={cn("group min-h-0 min-w-0", className)}>
        {chrome}
      </section>
    );
  }
  return (
    <button
      type="button"
      {...data}
      data-overview-card=""
      data-hover-scope=""
      aria-label={props.label}
      onClick={props.onClick}
      className={cn(
        "pointer-events-auto group h-full w-full min-w-0 cursor-pointer border-0 bg-transparent p-0 text-left outline-none",
        className,
      )}
    >
      <div data-overview-card-paint="" className="h-full">{chrome}</div>
    </button>
  );
}
// #endregion 🃏️OverviewCard

// #region 🔖️OverviewCardChips
/** 🔖️ The chip frame every footer chip shares — the chrome's own title-chip shape with the card's text size. */
export const overviewCardChipClass = cn(windowChromeTitleChipClass, "inline-flex items-center gap-single px-single text-xs font-medium");

/** 🏷️ A footer chip that only labels the card's own action (the whole card is the button): muted text and an optional icon
 * that follow the card's hover. Hovering this chip only centers the level on the card; pressing it turns the page behind the card clear. */
export function OverviewCardOpenChip({ slot, children, icon }: { readonly slot: string; readonly children: React.ReactNode; readonly icon?: React.ReactNode }): React.ReactElement {
  return (
    <div data-slot={`${slot}-open-chip`} data-layered-zoom="" className={windowChromeTitleChipClass}>
      <span className="inline-flex items-center gap-single px-single text-xs font-medium text-muted-foreground transition-colors group-hover:text-foreground">
        {children}
        {icon === undefined ? <Icon icon="chevron-right" size="small" className="transition-transform group-hover:translate-x-0.5 motion-reduce:transition-none" /> : icon}
      </span>
    </div>
  );
}

/** 🎬️ Props of {@link OverviewCardAction}: a native button, `primary` for the card's main action. */
export interface OverviewCardActionProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  readonly primary?: boolean;
}

/** 🖱️ A real button in a card's footer chip, at least 24 px tall (WCAG 2.2 target size): the primary action reads in the
 * foreground colour with a chevron that nudges on hover, a secondary one reads muted until hovered. Hovering the primary action
 * only centers the level on the card; pressing it turns the page behind the card clear. */
export const OverviewCardAction = React.forwardRef<HTMLButtonElement, OverviewCardActionProps>(({ primary = false, className, children, style, type, ...rest }, ref) => (
  <button
    ref={ref}
    type={type ?? "button"}
    data-overview-card-action={primary ? "primary" : "secondary"}
    data-layered-zoom={primary ? "" : undefined}
    className={cn(
      overviewCardChipClass,
      "group/action cursor-pointer transition-colors disabled:cursor-not-allowed disabled:opacity-50",
      primary ? "text-foreground" : "text-muted-foreground hover:text-foreground",
      className,
    )}
    style={{ minHeight: "var(--chip-row-height)", ...style }}
    {...rest}
  >
    {children}
    {primary ? <Icon icon="chevron-right" size="small" className="transition-transform group-hover/action:translate-x-0.5 motion-reduce:transition-none" /> : null}
  </button>
));
OverviewCardAction.displayName = "OverviewCardAction";
// #endregion 🔖️OverviewCardChips
