// #region 🧲️Header
// 💻️ framework/ui/elements/🃏️OverviewCard/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { Icon } from "../🔣️Icons/🟦️.tsx";
import { WindowChrome, windowChromeTitleChipClass } from "../🗂️WindowChrome/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🃏️OverviewCard
/** 🃏️ What every overview card shows: a title chip (glyph and title), a body, and optional chips at the bottom left
 * and right. `slot` prefixes the card's `data-slot`s (`<slot>-title-chip`, `<slot>-content`, `<slot>-stack`), so each
 * product keeps its own hooks for styles and tests. */
export interface OverviewCardBaseProps {
  readonly slot: string;
  readonly icon: React.ReactNode;
  readonly title: React.ReactNode;
  readonly children?: React.ReactNode;
  readonly footerLeft?: React.ReactNode;
  readonly footerRight?: React.ReactNode;
  readonly className?: string;
  readonly bodyClassName?: string;
  readonly contentClassName?: string;
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

/** 🔘️ A card that is itself one action (open this app): the whole card is a `<button>` that lifts on hover. Its body
 * holds phrasing text only, so the button keeps a valid content model. */
export interface OverviewCardButtonProps extends OverviewCardBaseProps {
  readonly as: "button";
  readonly onClick: () => void;
  readonly label?: string;
  readonly lifted?: boolean;
}

/** 🃏️ Props of {@link OverviewCard}. */
export type OverviewCardProps = OverviewCardSectionProps | OverviewCardButtonProps;

const CARD_STACK_DATA = { "data-overview-card-stack": "" } as const;

/** 🃏️ The overview card of semio's landing pages (play, the demonstrator, the quizzes): the design system's
 * {@link WindowChrome} at dialog level, never `active`, with the title in its cap chip and actions in its footer chips.
 * Sharp corners, glass and the 1 px silhouette come from the chrome; the stack stays transparent through
 * `data-overview-card-stack`, and its outline is emphasised while the pointer is over the card.
 *
 * @see ../🗂️WindowChrome/🟦️.tsx
 * @see https://www.w3.org/WAI/ARIA/apg/practices/landmark-regions/ — sections named by their heading */
export function OverviewCard(props: OverviewCardProps): React.ReactElement {
  const { slot, icon, title, children, footerLeft, footerRight, className, bodyClassName, contentClassName, data } = props;
  const Heading = props.as === "section" ? (`h${props.headingLevel ?? 2}` as const) : "span";
  const chrome = (
    <WindowChrome
      level="dialog"
      active={false}
      stackSlot={`${slot}-stack`}
      stackDataAttrs={CARD_STACK_DATA}
      stackClassName={cn("w-full min-w-0", props.as === "section" && "h-full")}
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
        </div>
      }
      body={
        <div data-slot={`${slot}-content`} className={cn("w-full min-w-0", contentClassName)}>
          {children}
        </div>
      }
      footerLeftChips={footerLeft}
      footerRightChips={footerRight}
      bodyClassName={bodyClassName ?? "p-double"}
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
        "pointer-events-auto group w-full min-w-0 cursor-pointer border-0 bg-transparent p-0 text-left outline-none",
        "transition-transform duration-200 motion-reduce:transition-none",
        "hover:-translate-y-0.5 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
        props.lifted && "-translate-y-0.5",
        className,
      )}
    >
      {chrome}
    </button>
  );
}
// #endregion 🃏️OverviewCard

// #region 🔖️OverviewCardChips
/** 🔖️ The chip frame every footer chip shares — the chrome's own title-chip shape with the card's text size. */
export const overviewCardChipClass = cn(windowChromeTitleChipClass, "inline-flex items-center gap-single px-single text-xs font-medium");

/** 🏷️ A footer chip that only labels the card's own action (the whole card is the button): muted text and a chevron
 * that follow the card's hover. */
export function OverviewCardOpenChip({ slot, children }: { readonly slot: string; readonly children: React.ReactNode }): React.ReactElement {
  return (
    <div data-slot={`${slot}-open-chip`} className={windowChromeTitleChipClass}>
      <span className="inline-flex items-center gap-single px-single text-xs font-medium text-muted-foreground transition-colors group-hover:text-foreground">
        {children}
        <Icon icon="chevron-right" size="small" className="transition-transform group-hover:translate-x-0.5 motion-reduce:transition-none" />
      </span>
    </div>
  );
}

/** 🎬️ Props of {@link OverviewCardAction}: a native button, `primary` for the card's main action. */
export interface OverviewCardActionProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  readonly primary?: boolean;
}

/** 🎬️ A real button in a card's footer chip, at least 24 px tall (WCAG 2.2 target size): the primary action reads in the
 * foreground colour with a chevron that nudges on hover, a secondary one reads muted until hovered. */
export const OverviewCardAction = React.forwardRef<HTMLButtonElement, OverviewCardActionProps>(({ primary = false, className, children, style, type, ...rest }, ref) => (
  <button
    ref={ref}
    type={type ?? "button"}
    data-overview-card-action={primary ? "primary" : "secondary"}
    className={cn(
      overviewCardChipClass,
      "group/action cursor-pointer transition-colors disabled:cursor-not-allowed disabled:opacity-50",
      primary ? "text-foreground" : "text-muted-foreground hover:text-foreground",
      className,
    )}
    style={{ minHeight: "max(1.5rem, calc(7 * var(--ui-spacing)))", ...style }}
    {...rest}
  >
    {children}
    {primary ? <Icon icon="chevron-right" size="small" className="transition-transform group-hover/action:translate-x-0.5 motion-reduce:transition-none" /> : null}
  </button>
));
OverviewCardAction.displayName = "OverviewCardAction";
// #endregion 🔖️OverviewCardChips
