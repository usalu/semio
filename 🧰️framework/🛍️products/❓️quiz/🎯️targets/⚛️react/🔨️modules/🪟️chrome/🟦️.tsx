/** 🪟️ The quiz's pieces of the semio card language: every screen is made of overview cards (the design system's window
 * chrome at dialog level) with the title in the cap chip and the actions as real buttons in the footer chips, plus the
 * small parts the cards share — catalog icons, emoji glyphs in the monochrome emoji face, fact lists, segmented choices
 * and the quiet button of card bodies.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx — the shared card
 * @see https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html — why actions are at least 24 px tall
 */

import type { ReactElement, ReactNode, Ref } from "react";
import { Icon, OverviewCard, OverviewCardAction, cn, overviewCardChipClass, type IconName } from "@semio-tech/ui-react/chrome";

export { OverviewCardAction as CardAction, cn };

/** 🃏️ One quiz card: a `<section>` named by its title chip's heading, `card` naming its place for styles and tests,
 * `anchor` the presence landmark cursors are relative to. A card of the overview opens its page: its heading is a
 * link to `href` (the keyboard and assistive-technology way) and a pointer click anywhere but on a control calls
 * `onOpen`; it takes the pointer although the card layer above the glass lets it pass, and `revealed` lifts it while
 * its page shows clear behind it. */
export function QuizCard(props: {
  readonly id: string;
  readonly card: string;
  readonly anchor?: string;
  readonly href?: string;
  readonly onOpen?: () => void;
  readonly revealed?: boolean;
  readonly icon: ReactNode;
  readonly title: ReactNode;
  readonly headingLevel?: 1 | 2 | 3;
  readonly headingRef?: Ref<HTMLHeadingElement>;
  readonly focusableHeading?: boolean;
  readonly footerLeft?: ReactNode;
  readonly footerRight?: ReactNode;
  readonly className?: string;
  readonly bodyClassName?: string;
  readonly children?: ReactNode;
}): ReactElement {
  const { onOpen } = props;
  const card = (
    <OverviewCard
      as="section"
      slot="quiz-card"
      headingId={props.id}
      headingLevel={props.headingLevel ?? 2}
      headingRef={props.headingRef}
      focusableHeading={props.focusableHeading}
      data={{ "data-card": props.card, "data-presence-anchor": props.anchor, "data-revealed": props.revealed ? "" : undefined }}
      icon={props.icon}
      title={
        props.href === undefined ? (
          props.title
        ) : (
          <a href={props.href} className="text-inherit no-underline outline-none hover:underline focus-visible:underline">
            {props.title}
          </a>
        )
      }
      footerLeft={props.footerLeft}
      footerRight={props.footerRight}
      className={cn(onOpen !== undefined && "pointer-events-auto cursor-pointer transition-transform duration-200 motion-reduce:transition-none", props.revealed && "-translate-y-0.5 motion-reduce:translate-y-0", props.className)}
      bodyClassName={props.bodyClassName}
      contentClassName="flex h-full min-h-0 flex-col gap-double"
    >
      {props.children}
    </OverviewCard>
  );
  if (onOpen === undefined) return card;
  return (
    <div
      className="contents"
      onClick={(event) => {
        if (!(event.target instanceof Element) || event.target.closest(CONTROLS) === null) onOpen();
      }}
    >
      {card}
    </div>
  );
}

const CONTROLS = "a, button, summary, input, select, textarea, label";

/** 👁️ How a page behind a card of the overview is shown: opened full size, or revealed clear while its card is
 * hovered or focused (else blurred behind the glass, inert). */
export interface PaneView {
  readonly opened: boolean;
  readonly revealed: boolean;
}

/** 📄️ A page behind a card of the overview: it fills its pane, scrolls on its own and lays its cards out in a centred
 * column laid out for the pane, not the window; `overlay` lies over that column and scrolls with it (the others on
 * this page). */
export function PageFrame(props: { readonly page: string; readonly wide?: boolean; readonly overlay?: ReactNode; readonly children: ReactNode }): ReactElement {
  return (
    <div data-page={props.page} className="quiz-page-frame h-full min-h-0 w-full overflow-auto bg-background p-double text-foreground">
      <div className={cn("relative mx-auto flex w-full flex-col gap-double", props.wide ? "max-w-6xl" : "max-w-4xl")}>
        {props.children}
        {props.overlay}
      </div>
    </div>
  );
}

/** 🔣️ A catalog icon in a title chip. */
export function CardIcon(props: { readonly icon: IconName }): ReactElement {
  return <Icon icon={props.icon} size="small" className="shrink-0 text-muted-foreground" />;
}

/** 🔡️ `emoji` asking for text presentation: every emoji variation selector (VS16) becomes the text one (VS15), so a
 * sequence such as ❄️ renders from the monochrome emoji face like 🔥 and 🧲 instead of falling back to a colour font. */
export function textPresentation(emoji: string): string {
  return emoji.replaceAll("\uFE0F", "\uFE0E");
}

/** 😀️ An emoji shown as a glyph of the monochrome emoji face (text presentation, `font-variant-emoji: text`). */
export function Glyph(props: { readonly emoji: string; readonly className?: string }): ReactElement {
  return (
    <span aria-hidden="true" className={cn("quiz-glyph shrink-0", props.className)}>
      {textPresentation(props.emoji)}
    </span>
  );
}

/** 📋️ A row of short facts in the muted colour. */
export function Facts(props: { readonly items: readonly ReactNode[] }): ReactElement {
  return (
    <ul className="m-0 flex list-none flex-wrap gap-x-double gap-y-single p-0 text-xs text-muted-foreground">
      {props.items.map((item, index) => (
        <li key={index}>{item}</li>
      ))}
    </ul>
  );
}

/** 🎚️ One choice of {@link Segments}. */
export interface Segment<T extends string> {
  readonly value: T;
  readonly label: string;
  readonly lang?: string;
}

/** 🎚️ A segmented choice: one pressed button per option inside a named group. */
export function Segments<T extends string>(props: { readonly label: string; readonly options: readonly Segment<T>[]; readonly value: T | undefined; readonly onChange: (value: T) => void }): ReactElement {
  const { label, options, value, onChange } = props;
  return (
    <div role="group" aria-label={label} className="flex w-fit max-w-full flex-wrap border border-normal">
      {options.map((option) => (
        <button
          key={option.value}
          type="button"
          lang={option.lang}
          aria-pressed={option.value === value}
          onClick={() => onChange(option.value)}
          className={cn(overviewCardChipClass, "quiz-target cursor-pointer transition-colors", option.value === value ? "bg-active-base text-active-foreground" : "text-muted-foreground hover:text-foreground")}
        >
          {option.label}
        </button>
      ))}
    </div>
  );
}

/** 🔘️ A button inside a card body — the chip shape of the footer actions, framed so it reads as a control. */
export function BodyButton(props: { readonly onClick: () => void; readonly disabled?: boolean; readonly primary?: boolean; readonly type?: "button" | "submit"; readonly describedBy?: string; readonly children: ReactNode }): ReactElement {
  return (
    <button
      type={props.type ?? "button"}
      disabled={props.disabled}
      aria-describedby={props.describedBy}
      onClick={props.onClick}
      className={cn(
        overviewCardChipClass,
        "quiz-target w-fit cursor-pointer border border-normal transition-colors disabled:cursor-not-allowed disabled:opacity-50",
        props.primary ? "bg-active-base text-active-foreground" : "text-foreground hover:bg-hover-interactive-fill",
      )}
    >
      {props.children}
    </button>
  );
}
