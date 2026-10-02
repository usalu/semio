/** 🪟️ The quiz's pieces of the semio card language: every screen is made of overview cards (the design system's window
 * chrome at dialog level) with the title in the cap chip and the actions as real buttons in the footer chips, plus the
 * small parts the cards share — catalog icons, emoji glyphs in the monochrome emoji face, fact lists, segmented choices,
 * the quiet button of card bodies, the modal dialog every question and notice opens in, and the note of a failure.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx — the shared card
 * @see https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html — why actions are at least 24 px tall
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/ — the modal dialog
 */

import { forwardRef, useEffect, useLayoutEffect, useRef, useState, type ReactElement, type ReactNode, type Ref } from "react";
import { createPortal } from "react-dom";
import { Icon, OverviewCard, OverviewCardAction, cn, overviewCardChipClass, type IconName, type OverviewCardActionProps } from "@semio-tech/ui-react/chrome";
import type { Icon as QuizIcon, Motion } from "@semio-tech/quiz";
import type { QuizText } from "../🌐️i18n/🟦️.ts";

export { cn };

/** 🧷️ What lets a chip in a card's footer give way: it may shrink below its text, which then wraps inside it, so two
 * chips never lie over each other and never make their card wider than the page — whatever the translation, the
 * learner's text size or the width of the card. */
export const CARD_CHIP_WRAP = "h-auto max-h-none min-w-0 max-w-full shrink whitespace-normal text-left leading-tight";

/** 🎬️ An action in a card's footer: the design system's card action, as a chip that gives way ({@link CARD_CHIP_WRAP}). */
export const CardAction = forwardRef<HTMLButtonElement, OverviewCardActionProps>(({ className, ...rest }, ref) => <OverviewCardAction ref={ref} className={cn(CARD_CHIP_WRAP, className)} {...rest} />);
CardAction.displayName = "CardAction";

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
          <a href={props.href} className="quiz-card-link text-inherit no-underline hover:underline focus-visible:underline">
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
    <div data-page={props.page} className="quiz-page-frame relative h-full min-h-0 w-full overflow-auto bg-background p-double text-foreground">
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
  return emoji.replaceAll("️", "︎");
}

/** 😀️ An emoji shown as a glyph of the monochrome emoji face (text presentation, `font-variant-emoji: text`), playing
 * `motion` as a looping microanimation where the learner allows it. */
export function Glyph(props: { readonly emoji: string; readonly className?: string; readonly motion?: Motion; readonly order?: number }): ReactElement {
  return (
    <span aria-hidden="true" className={cn("quiz-glyph shrink-0", props.className)} data-motion={props.motion} style={props.order === undefined ? undefined : { ["--quiz-icon-order" as string]: props.order }}>
      {textPresentation(props.emoji)}
    </span>
  );
}

/** 🏷️ A label with the icon of what it names before it: an item, a category, a value card of a dimension or a task.
 * Without an icon it is the label alone. `order` is its place among its neighbours, which sets their loops apart so a
 * list of icons never moves in step. The glyph is not spoken; the label keeps naming the element. */
export function IconLabel(props: { readonly icon: QuizIcon | undefined; readonly order?: number; readonly children: ReactNode }): ReactElement {
  const { icon } = props;
  return (
    <>
      {icon === undefined ? null : <Glyph emoji={icon.emoji} motion={icon.motion} order={props.order} className="quiz-label-icon" />}
      {props.children}
    </>
  );
}

/** ✳️ A symbol that only repeats the word beside it (✓ before "Correct"): shown, never spoken. */
export function Mark(props: { readonly symbol: string }): ReactElement {
  return <span aria-hidden="true">{props.symbol} </span>;
}

/** 🕳️ A cell without a value: a dash to see and the reason to hear. */
export function Missing(props: { readonly label: string }): ReactElement {
  return (
    <>
      <span aria-hidden="true">–</span>
      <span className="sr-only">{props.label}</span>
    </>
  );
}

/** 📋️ A row of short facts in the muted colour. */
export function Facts(props: { readonly items: readonly ReactNode[] }): ReactElement {
  return (
    <ul role="list" className="m-0 flex list-none flex-wrap gap-x-double gap-y-single p-0 text-xs text-muted-foreground">
      {props.items.map((item, index) => (
        <li key={index}>{item}</li>
      ))}
    </ul>
  );
}

/** 🎚️ One choice of {@link Segments}; `short` is what it shows instead of its label below tablet width, where the label
 * stays its name for assistive technology. */
export interface Segment<T extends string> {
  readonly value: T;
  readonly label: string;
  readonly short?: string;
  readonly lang?: string;
}

/** 🎚️ A segmented choice: one pressed button per option inside a named group; a short label still gets a target 24 px
 * wide (WCAG 2.2 SC 2.5.8). */
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
          className={cn(overviewCardChipClass, "quiz-target quiz-segment min-w-[1.5rem] cursor-pointer justify-center transition-colors", option.value === value ? "bg-active-base text-active-foreground" : "text-muted-foreground hover:text-foreground")}
        >
          {option.short === undefined ? (
            option.label
          ) : (
            <>
              <span aria-hidden="true" className="md:hidden">
                {option.short}
              </span>
              <span className="max-md:sr-only">{option.label}</span>
            </>
          )}
        </button>
      ))}
    </div>
  );
}

/** 🔘️ A button inside a card body — the chip shape of the footer actions, framed so it reads as a control; its label
 * wraps instead of overflowing when a translation or the learner's text size makes it longer than its card. */
export function BodyButton(props: { readonly onClick: () => void; readonly disabled?: boolean; readonly primary?: boolean; readonly type?: "button" | "submit"; readonly describedBy?: string; readonly children: ReactNode }): ReactElement {
  return (
    <button
      type={props.type ?? "button"}
      disabled={props.disabled}
      aria-describedby={props.describedBy}
      onClick={props.onClick}
      className={cn(
        overviewCardChipClass,
        "quiz-target h-auto max-h-none w-fit max-w-full cursor-pointer whitespace-normal border border-normal text-left leading-tight transition-colors disabled:cursor-not-allowed disabled:opacity-50",
        props.primary ? "bg-active-base text-active-foreground" : "text-foreground hover:bg-hover-interactive-fill",
      )}
    >
      {props.children}
    </button>
  );
}

/** 🚫️ What went wrong: the sentence in the learner's language and, apart, the proctor's own words (English, from the
 * wire) for whoever has to look into it. */
export interface Problem {
  readonly message: string;
  readonly detail?: string;
}

/** 🚫️ A {@link Problem} as an alert: the sentence, the technical detail collapsed and marked as English, and what the
 * learner can do about it (`children`). */
export function ProblemNote(props: { readonly problem: Problem; readonly text: QuizText; readonly id?: string; readonly className?: string; readonly children?: ReactNode }): ReactElement {
  const { problem, text } = props;
  return (
    <div id={props.id} role="alert" className={cn("quiz-alert flex flex-wrap items-center gap-x-double gap-y-single border border-normal px-double py-single text-sm", props.className)}>
      <p className="m-0 min-w-0 flex-1 font-semibold">{problem.message}</p>
      {props.children}
      {problem.detail === undefined ? null : (
        <details className="w-full text-xs text-muted-foreground">
          <summary className="quiz-target flex cursor-pointer items-center">{text("quiz.app.details")}</summary>
          <code lang="en" className="quiz-name block">
            {problem.detail}
          </code>
        </details>
      )}
    </div>
  );
}

const FOCUSABLE = "button:not([disabled]), a[href], summary, input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex='-1'])";

function focusables(dialog: HTMLElement): HTMLElement[] {
  return [...dialog.querySelectorAll<HTMLElement>(FOCUSABLE)];
}

/** 💬️ A modal card over the whole client: a question that needs an answer (`alertdialog`) or a notice to read
 * (`dialog`). It is rendered as the last child of the client's root, so the root's language and styles apply and
 * everything else of the client is inert while it shows; focus goes to its `data-autofocus` control (again whenever
 * `focusKey` changes), Tab stays inside from wherever focus is, Escape calls `onEscape`, and closing returns focus to
 * the control that opened it. A click inside it never reaches the card it was opened from. */
export function Dialog(props: {
  readonly id: string;
  readonly role: "dialog" | "alertdialog";
  readonly icon: ReactNode;
  readonly title: string;
  readonly describedBy: string;
  readonly onEscape: () => void;
  readonly focusKey?: string;
  readonly wide?: boolean;
  readonly footerLeft?: ReactNode;
  readonly footerRight?: ReactNode;
  readonly children: ReactNode;
}): ReactElement {
  const [origin, setOrigin] = useState<HTMLSpanElement | null>(null);
  const [opener] = useState<Element | null>(() => (typeof document === "undefined" ? null : document.activeElement));
  const backdrop = useRef<HTMLDivElement>(null);
  const dialog = useRef<HTMLDivElement>(null);
  const escape = useRef(props.onEscape);
  escape.current = props.onEscape;
  const host = origin === null ? null : (origin.closest<HTMLElement>(".quiz-app") ?? origin.ownerDocument.body);

  useLayoutEffect(() => {
    if (host === null) return;
    const silenced = [...host.children].filter((sibling) => sibling !== backdrop.current && !sibling.hasAttribute("inert"));
    for (const sibling of silenced) sibling.setAttribute("inert", "");
    return () => {
      for (const sibling of silenced) sibling.removeAttribute("inert");
      if (opener instanceof HTMLElement && opener.isConnected) opener.focus();
    };
  }, [host, opener]);

  useLayoutEffect(() => {
    if (host !== null) dialog.current?.querySelector<HTMLElement>("[data-autofocus]")?.focus();
  }, [host, props.focusKey]);

  useEffect(() => {
    if (host === null) return;
    const onKey = (event: KeyboardEvent): void => {
      const within = dialog.current;
      if (within === null) return;
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopPropagation();
        escape.current();
        return;
      }
      if (event.key !== "Tab") return;
      const stops = focusables(within);
      const first = stops[0];
      const last = stops[stops.length - 1];
      const active = document.activeElement;
      const outside = active === null || !within.contains(active);
      if (event.shiftKey && (outside || active === first)) {
        event.preventDefault();
        last?.focus();
      } else if (!event.shiftKey && (outside || active === last)) {
        event.preventDefault();
        first?.focus();
      }
    };
    document.addEventListener("keydown", onKey, true);
    return () => document.removeEventListener("keydown", onKey, true);
  }, [host]);

  return (
    <>
      <span ref={setOrigin} hidden />
      {host === null
        ? null
        : createPortal(
            <div ref={backdrop} data-quiz-dialog="" className="quiz-backdrop fixed inset-0 z-50 grid place-items-center overflow-auto p-double" onClick={(event) => event.stopPropagation()}>
              <div ref={dialog} role={props.role} aria-modal="true" aria-labelledby={props.id} aria-describedby={props.describedBy} className={cn("w-full", props.wide ? "max-w-2xl" : "max-w-md")}>
                <QuizCard id={props.id} card="dialog" icon={props.icon} title={props.title} footerLeft={props.footerLeft} footerRight={props.footerRight}>
                  {props.children}
                </QuizCard>
              </div>
            </div>,
            host,
          )}
    </>
  );
}
