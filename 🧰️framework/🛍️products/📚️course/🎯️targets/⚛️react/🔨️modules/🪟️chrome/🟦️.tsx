/** 🪟️ The quiz's pieces of the semio card language: every screen is made of overview cards (the design system's window
 * chrome at dialog level) with the title in the cap chip and the actions as real buttons in the footer chips, plus the
 * small parts the cards share — course icons, emoji glyphs in the monochrome emoji face, fact lists, segmented choices,
 * the quiet button of card bodies, the modal dialog every question and notice opens in, the note of a failure and the
 * polite live region that announces what an action changed.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx — the shared card
 * @see https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html — why actions are at least 24 px tall
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/ — the modal dialog
 */

import { forwardRef, useCallback, useEffect, useLayoutEffect, useRef, useState, type CSSProperties, type HTMLAttributes, type ReactElement, type ReactNode, type Ref } from "react";
import { createPortal } from "react-dom";
import { CardStack, Icon, OverviewCard, SharedBorderRow, SharedBorderRowCell, OverviewCardAction, cn, overviewCardChipClass, useSharedBorderFrameRef, type IconName, type OverviewCardActionProps, sharedBorderlessBoxProps } from "@semio-tech/ui-react/chrome";
import type { Icon as QuizIcon, Motion, TaskKind } from "@semio-tech/course";
import type { CourseText } from "../🌐️i18n/🟦️.ts";

export { cn };

/** 🧷️ What lets a chip in a card's footer give way: it may shrink below its text, which then wraps inside it, so two
 * chips never lie over each other and never make their card wider than the page — whatever the translation, the
 * learner's text size or the width of the card. */
export const CARD_CHIP_WRAP = "h-auto max-h-none min-w-0 max-w-full shrink whitespace-normal text-left leading-tight";

/** 🎬️ An action in a card's footer: the design system's card action, as a chip that gives way ({@link CARD_CHIP_WRAP}). Every action carries its `icon` before its words. */
export const CardAction = forwardRef<HTMLButtonElement, OverviewCardActionProps & { readonly icon: IconName }>(({ className, icon, children, ...rest }, ref) => <OverviewCardAction ref={ref} className={cn(CARD_CHIP_WRAP, className)} {...rest}><Icon icon={icon} size="small" className="shrink-0" />{children}</OverviewCardAction>);
CardAction.displayName = "CardAction";

/** 🔗️ A link that stands where a {@link CardAction} would: a chip that gives way. `download` offers the file under that
 * name, `external` opens another site in a tab of its own that gets neither the page nor its address. */
export function CardLink(props: { readonly icon: IconName; readonly href: string; readonly download?: string; readonly external?: boolean; readonly primary?: boolean; readonly label?: string; readonly children: ReactNode }): ReactElement {
  return (
    <a
      href={props.href}
      download={props.download}
      target={props.external ? "_blank" : undefined}
      rel={props.external ? "noopener noreferrer" : undefined}
      aria-label={props.label}
      {...sharedBorderlessBoxProps}
      data-overview-card-action={props.primary ? "primary" : "secondary"}
      className={cn(overviewCardChipClass, CARD_CHIP_WRAP, "course-target cursor-pointer no-underline transition-colors", props.primary ? "text-foreground" : "text-muted-foreground hover:text-foreground")}
      style={{ minHeight: "max(1.5rem, calc(7 * var(--ui-spacing)))" }}
    >
      <Icon icon={props.icon} size="small" className="shrink-0" />
      {props.children}
    </a>
  );
}

/** 🧲️ The label of a title chip as something the pet of `topic` may try to steal; a card without a topic keeps its label. */
function stealable(topic: string | undefined, label: ReactNode): ReactNode {
  return topic === undefined ? label : <span data-pet-prop={topic} className="inline-block">{label}</span>;
}

/** 🃏️ One quiz card: a `<section>` named by its title chip's heading, `card` naming its place for styles and tests,
 * `anchor` the presence landmark cursors are relative to. A card of the overview opens its page: its heading is a
 * link to `href` (the keyboard and assistive-technology way). An ordinary primary click on that link opens the page
 * through `onOpen` and writes the link's hash: the card's own click ignores controls, and a hash navigation can finish
 * without a `hashchange` the address is still listening for, while `onOpen` can be dropped when the overview is not
 * taking openings. A pointer click anywhere but on a control also calls `onOpen`. The card takes the
 * pointer although the card layer above the glass lets it pass, and `revealed` lifts it while
 * its page shows clear behind it. `topic` names what the card is about for the pets (`data-pet-topic`: a key among the
 * grounds of their menagerie that they may match) and makes the label of its title chip a thing the pet of that topic may
 * try to steal (`data-pet-prop`): a copy of the label is tugged out of the chip and put back, the card itself is never lifted. A `flush` card has no body padding: its
 * children are {@link CardSection}s, {@link RowList}s and other cells whose single dividers lie between them and whose
 * outer edges are the card's own. */
export function CourseCard(props: {
  readonly id: string;
  readonly card: string;
  readonly anchor?: string;
  readonly topic?: string;
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
  readonly flush?: boolean;
  readonly children?: ReactNode;
}): ReactElement {
  const { onOpen } = props;
  const card = (
    <OverviewCard
      as="section"
      slot="course-card"
      headingId={props.id}
      headingLevel={props.headingLevel ?? 2}
      headingRef={props.headingRef}
      focusableHeading={props.focusableHeading}
      data={{ "data-card": props.card, "data-presence-anchor": props.anchor, "data-pet-topic": props.topic, "data-revealed": props.revealed ? "" : undefined }}
      icon={props.icon}
      title={stealable(props.topic,
        props.href === undefined ? (
          props.title
        ) : (
          <a
            href={props.href}
            className="course-card-link text-inherit no-underline outline-none hover:underline"
            onClick={(event) => {
              if (onOpen === undefined || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
              onOpen();
            }}
          >
            {props.title}
          </a>
        ))
      }
      footerLeft={props.footerLeft}
      footerRight={props.footerRight}
      className={cn("pointer-events-auto", onOpen !== undefined && "cursor-pointer", props.className)}
      bodyClassName={props.bodyClassName}
      contentClassName="flex h-full min-h-0 flex-col"
      flush
    >
      {props.flush ? props.children : <CardSection className="min-h-0 flex-1">{props.children}</CardSection>}
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

/** 🧩️ The attributes that make an element a cell of a shared-border group (its single divider lies after it, none
 * after the last) and the group whose cells follow each other in a column; every divider is the one the design system's
 * `shared-border` rules draw, never a border of the cell itself.
 * @see ../../../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css — the shared-border rules */
export const CELL = { "data-shared-border-cell": "" } as const;
export const GROUP = { "data-shared-border-group": "", "data-orientation": "vertical" } as const;
export const FRAME = { "data-shared-border-frame": "" } as const;
export const ROW = { "data-shared-border-group": "", "data-shared-border-cell": "" } as const;

/** 🔀️ A setting that is on or off as a toggle cell: the whole cell is the switch, with its icon and its label, filled with the active colour while it is on. Its tick box stays the real control (the keyboard, its name is the label) and is not painted. */
export function ToggleCell(props: { readonly icon: IconName; readonly checked: boolean; readonly disabled?: boolean; readonly describedBy?: string; readonly onChange: (checked: boolean) => void; readonly className?: string; readonly data?: Readonly<Record<`data-${string}`, string>>; readonly children: ReactNode }): ReactElement {
  return (
    <label {...CELL} {...props.data} data-state={props.checked ? "on" : "off"} className={cn("course-choice course-target flex min-w-0 items-center gap-single px-double py-single", props.disabled ? "cursor-not-allowed opacity-50" : "cursor-pointer", props.className)}>
      <input type="checkbox" className="course-check sr-only" checked={props.checked} disabled={props.disabled} aria-describedby={props.describedBy} onChange={(event) => props.onChange(event.target.checked)} />
      <Icon icon={props.icon} size="small" className="shrink-0" />
      <span className="min-w-0">{props.children}</span>
    </label>
  );
}

/** 🔘️ One choice of a radio group as a toggle: the whole cell is the choice, filled with the active colour while it is the chosen one like the task of a run, so the group shows at a glance that one of its cells is on and which. Its radio button stays the real control (arrow keys, the name and the describing line for assistive technology) and is not painted. */
export function RadioChoice(props: { readonly id: string; readonly icon: IconName; readonly name: string; readonly value: string; readonly checked: boolean; readonly onChange: () => void; readonly label: ReactNode; readonly hint: ReactNode }): ReactElement {
  return (
    <label {...CELL} data-state={props.checked ? "on" : "off"} className="course-choice course-target block min-w-0 cursor-pointer px-double py-single">
      <input id={props.id} type="radio" name={props.name} value={props.value} checked={props.checked} onChange={props.onChange} aria-labelledby={`${props.id}-label`} aria-describedby={`${props.id}-hint`} className="course-radio sr-only" />
      <span className="flex items-center gap-single text-sm font-semibold"><Icon icon={props.icon} size="small" className="shrink-0" /><span id={`${props.id}-label`}>{props.label}</span></span>
      <span id={`${props.id}-hint`} className="course-choice-hint block text-xs leading-normal">{props.hint}</span>
    </label>
  );
}

/** 🧱️ A padded section of a flush card (or of a zone): a cell with its own padding and column of children. */
export function CardSection(props: { readonly className?: string; readonly children: ReactNode } & Pick<HTMLAttributes<HTMLDivElement>, "id" | "role" | "aria-live">): ReactElement {
  const { className, children, ...rest } = props;
  return (
    <div {...rest} {...CELL} className={cn("flex min-w-0 flex-col gap-double p-double", className)}>
      {children}
    </div>
  );
}

/** 📚️ A list whose rows share their dividers: single hairlines between the rows, none after the last. `framed` gives the
 * list its own outline, shared with whatever lies against it (the design system's compositor attaches it); an unframed
 * list lies flush in a frame that already has one, such as a flush card, whose edges its outer rows coincide with, and is
 * a cell of that frame's group itself.
 * Its rows are {@link CELL}s (`<li>`) that carry their own padding. */
export function RowList(props: { readonly as?: "ul" | "ol"; readonly framed?: boolean; readonly className?: string; readonly children: ReactNode } & Pick<HTMLAttributes<HTMLElement>, "id" | "tabIndex" | "aria-label" | "aria-labelledby" | "aria-busy" | "onBlur">): ReactElement {
  const { as: Tag = "ul", framed = false, className, children, ...rest } = props;
  const ref = useSharedBorderFrameRef<HTMLElement>();
  return (
    <Tag
      {...rest}
      role="list"
      ref={framed ? ref : undefined}
      {...(framed ? { "data-shared-border-frame": "", "data-shared-border-edges": "top right bottom left", "data-orientation": "vertical" } : { ...GROUP, ...CELL })}
      className={cn("m-0 flex min-w-0 list-none flex-col p-0", className)}
    >
      {children}
    </Tag>
  );
}

const CONTROLS = "a, button, summary, input, select, textarea, label";

/** 👁️ How a page behind a card of the overview is shown: opened full size, or revealed clear while its card is
 * hovered or focused (else blurred behind the glass, inert). */
export interface PaneView {
  readonly opened: boolean;
  readonly revealed: boolean;
}

/** 📄️ A page behind a card of the overview: it fills its pane, scrolls on its own and lays its cards out in a
 * column as wide as the pane ({@link CardStack}: no margin, no gap, the cards abut and their edges are the pane's); `overlay` lies over that column and scrolls with it (the others on
 * this page). */
export function PageFrame(props: { readonly page: string; readonly overlay?: ReactNode; readonly children: ReactNode }): ReactElement {
  return (
    <div data-page={props.page} className="course-page-frame ui-scrollbar-hidden relative h-full min-h-0 w-full overflow-auto bg-background text-foreground">
      <CardStack>
        {props.children}
        {props.overlay}
      </CardStack>
    </div>
  );
}

/** 🔣️ A course icon in a title chip. */
export function CardIcon(props: { readonly icon: IconName }): ReactElement {
  return <Icon icon={props.icon} size="small" className="shrink-0 text-muted-foreground" />;
}

/** 🔡️ `emoji` asking for text presentation: every emoji variation selector (VS16) becomes the text one (VS15), so a
 * sequence such as ❄️ renders from the monochrome emoji face like 🔥 and 🧲 instead of falling back to a colour font. */
export function textPresentation(emoji: string): string {
  return emoji.replaceAll("️", "︎");
}

/** 😀️ An emoji shown as a glyph of the monochrome emoji face (text presentation, `font-variant-emoji: text`), playing
 * `motion` as a microanimation when its host is hovered or tapped and the learner allows it ({@link ICON_HOSTS}). */
export function Glyph(props: { readonly emoji: string; readonly className?: string; readonly motion?: Motion }): ReactElement {
  return (
    <span aria-hidden="true" className={cn("course-glyph shrink-0", props.className)} data-motion={props.motion}>
      {textPresentation(props.emoji)}
    </span>
  );
}

/** 🖥️ What an icon plays for: its host — the element it stands in (marked `data-icon-host`: a chip, a row, a value
 * card, a bin's heading, a task button) or the title chip of its card. Hosts never nest. A pointer that hovers a host
 * (desktop) or a keyboard focus on or inside it loops its icons; a tap on it (tablet, phone) plays them once. */
export const ICON_HOSTS = '[data-icon-host], [data-slot="course-card-title-chip"]';

/** ⏱️ One loop of an icon's microanimation, the length of a tap's play. */
export const ICON_LOOP_MS = 3200;

const tapped = new WeakMap<Element, ReturnType<typeof setTimeout>>();

/** 👆️ Plays the icons of the host a touch or pen pointer went down on once, from the start even when they still play;
 * a mouse plays them by hovering instead. */
export function playTappedIcons(target: EventTarget | null, pointerType: string): void {
  if (pointerType === "mouse" || !(target instanceof Element)) return;
  const host = target.closest(ICON_HOSTS);
  if (host === null || host.querySelector(".course-glyph[data-motion]") === null) return;
  clearTimeout(tapped.get(host));
  host.removeAttribute("data-icon-tapped");
  void host.getBoundingClientRect();
  host.setAttribute("data-icon-tapped", "");
  tapped.set(
    host,
    setTimeout(() => host.removeAttribute("data-icon-tapped"), ICON_LOOP_MS),
  );
}

/** 🗂️ The icon of each task kind in its card's title chip. */
export const TASK_KIND_ICONS: { readonly [K in TaskKind]: IconName } = { classification: "layout-grid", sorting: "list", matching: "link" };

/** 🖼️ The icon of a task in its title chip: the task's own emoji playing its microanimation, else the icon of its kind. */
export function TaskGlyph(props: { readonly task: { readonly kind: TaskKind; readonly icon?: QuizIcon } }): ReactElement {
  const { kind, icon } = props.task;
  return icon === undefined ? <CardIcon icon={TASK_KIND_ICONS[kind]} /> : <Glyph emoji={icon.emoji} motion={icon.motion} className="text-sm" />;
}

/** 🔈️ What a polite live region says: the message and how many were said before it, so a repeated message is a new one. */
export interface Announcement {
  readonly message: string;
  readonly serial: number;
}

/** 🗨️ The message of a polite live region, and the function that replaces it — after `delayMs` without a newer one, so
 * a burst of changes is announced once, by its last message. */
export function useAnnouncement(delayMs = 0): { readonly announcement: Announcement; readonly announce: (message: string) => void } {
  const [announcement, setAnnouncement] = useState<Announcement>({ message: "", serial: 0 });
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  useEffect(() => () => clearTimeout(timer.current), []);
  const announce = useCallback(
    (message: string) => {
      const say = (): void => setAnnouncement((said) => ({ message, serial: said.serial + 1 }));
      clearTimeout(timer.current);
      if (delayMs <= 0) say();
      else timer.current = setTimeout(say, delayMs);
    },
    [delayMs],
  );
  return { announcement, announce };
}

/** 📢️ The visually hidden polite live region announcing what a keyboard or pointer action changed; every announcement
 * is a new node, so the same sentence said twice is spoken twice. */
export function LiveRegion(props: { readonly announcement: Announcement }): ReactElement {
  const { message, serial } = props.announcement;
  return (
    <p className="sr-only" role="status" aria-live="polite" aria-atomic="true">
      {message === "" ? null : <span key={serial}>{message}</span>}
    </p>
  );
}

/** 🏷️ A label with the icon of what it names before it: an item, a category, a value card of a dimension or a task.
 * Without an icon it is the label alone. The element it stands in is its host ({@link ICON_HOSTS}). The glyph is not
 * spoken; the label keeps naming the element. */
export function IconLabel(props: { readonly icon: QuizIcon | undefined; readonly children: ReactNode }): ReactElement {
  const { icon } = props;
  return (
    <>
      {icon === undefined ? null : <Glyph emoji={icon.emoji} motion={icon.motion} className="course-label-icon" />}
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

/** 🗃️ The table role of every part of a table that folds into records where its card is too narrow for its columns
 * (`.course-records` around `.course-fold` or `.course-plot`): a folded part is no table box any more, and with its role
 * spelled out it stays a table, a row, a heading or a cell for assistive technology in every browser.
 * @see https://www.w3.org/TR/html-aria/#docconformance — the roles the elements have by themselves */
export const TABLE = {
  table: { role: "table" },
  group: { role: "rowgroup" },
  row: { role: "row" },
  column: { role: "columnheader", scope: "col" },
  name: { role: "rowheader", scope: "row" },
  cell: { role: "cell" },
} as const;

/** 🗄️ Retains one semantic table and folds it below its reading tier or whenever its actual columns exceed their frame. A table embedded in a figure uses the figure's surrounding cell. */
export function Records(props: { readonly fold: number; readonly children: ReactNode; readonly cell?: boolean } & Pick<HTMLAttributes<HTMLDivElement>, "className" | "role" | "tabIndex" | "aria-labelledby" | "aria-describedby">): ReactElement {
  const { fold, children, className, cell = true, ...rest } = props;
  const host = useRef<HTMLDivElement>(null);
  useLayoutEffect(() => {
    const frame = host.current;
    const view = frame?.ownerDocument.defaultView;
    if (!frame || !view) return;
    let disposed = false;
    let scheduled: number | undefined;
    let table: HTMLTableElement | null = null;
    let resize: ResizeObserver | undefined;
    const measure = (): void => {
      scheduled = undefined;
      if (disposed) return;
      const next = frame.querySelector<HTMLTableElement>(":scope > table.course-fold");
      if (next !== table) {
        if (table) resize?.unobserve(table);
        table = next;
        if (table) resize?.observe(table);
      }
      frame.removeAttribute("data-folded");
      if (!table) return;
      const minimum = view.getComputedStyle(table).getPropertyValue("--course-records-minimum-fold").trim() === "1";
      frame.toggleAttribute("data-folded", minimum || table.scrollWidth > frame.clientWidth);
    };
    const queue = (): void => {
      if (!disposed && scheduled === undefined) scheduled = view.requestAnimationFrame(measure);
    };
    resize = typeof ResizeObserver === "undefined" ? undefined : new ResizeObserver(queue);
    resize?.observe(frame);
    const content = new MutationObserver(queue);
    content.observe(frame, { childList: true, subtree: true, characterData: true, attributes: true, attributeFilter: ["class", "style", "hidden"] });
    const appearance = new MutationObserver(queue);
    for (const element of new Set([frame.ownerDocument.documentElement, frame.ownerDocument.body, frame.closest(".course-app")])) if (element) appearance.observe(element, { attributes: true, attributeFilter: ["class", "style"] });
    const fonts = frame.ownerDocument.fonts;
    fonts?.addEventListener("loadingdone", queue);
    view.addEventListener("resize", queue);
    measure();
    return () => {
      disposed = true;
      if (scheduled !== undefined) view.cancelAnimationFrame(scheduled);
      resize?.disconnect();
      content.disconnect();
      appearance.disconnect();
      fonts?.removeEventListener("loadingdone", queue);
      view.removeEventListener("resize", queue);
    };
  }, [fold]);
  return (
    <div {...rest} {...(cell ? CELL : undefined)} ref={host} className={cn("course-records", className)} style={{ ["--course-fold" as string]: fold } as CSSProperties}>
      {children}
    </div>
  );
}

/** 🎚️ One choice of {@link Segments}; `short` is what it shows instead of its label below tablet width, where the label
 * stays its name for assistive technology. */
export interface Segment<T extends string> {
  readonly value: T;
  readonly label: string;
  readonly short?: string;
  readonly lang?: string;
  readonly icon?: IconName;
  readonly glyph?: string;
}

/** 🍱️ A segmented choice: one pressed button per option inside a named group. Every option shows an icon and its
 * label — a course icon, or the emoji a quiz already carries. A short label still gets a target 24 px wide
 * (WCAG 2.2 SC 2.5.8). A `flush` choice is a whole row of a flush card: no outline of its own, its cells as wide as the card
 * and sharing the card's edges, with one divider below it. */
export function Segments<T extends string>(props: { readonly label: string; readonly options: readonly Segment<T>[]; readonly value: T | undefined; readonly onChange: (value: T) => void; readonly flush?: boolean }): ReactElement {
  const { label, options, value, onChange, flush = false } = props;
  return (
    <SharedBorderRow
      wrap
      framed={!flush}
      role="group"
      aria-label={label}
      {...(flush ? CELL : undefined)}
      className={cn("max-w-full", flush ? "w-full" : "w-fit")}
    >
      {options.map((option) => (
        <button
          key={option.value}
          type="button"
          lang={option.lang}
          aria-pressed={option.value === value}
          onClick={() => onChange(option.value)}
          {...CELL}
          className={cn(overviewCardChipClass, "course-target course-segment shrink min-w-[1.5rem] cursor-pointer justify-center whitespace-normal transition-colors", option.value === value ? "bg-active-base text-active-foreground" : "text-muted-foreground hover:text-foreground")}
        >
          {option.icon !== undefined ? <Icon icon={option.icon} size="small" /> : option.glyph !== undefined ? <Glyph emoji={option.glyph} /> : null}
          {option.glyph !== undefined ? " " : null}
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
    </SharedBorderRow>
  );
}

/** 🔘️ A button inside a card body — the chip shape of the footer actions, framed so it reads as a control; its label
 * wraps instead of overflowing when a translation or the learner's text size makes it longer than its card. A `row`
 * button is a whole row of a flush card: a cell as wide as the card whose dividers are the card's single ones and
 * which has no outline of its own, since its outer edges are the card's. A `cell` button is a cell of a row (a notice band's
 * actions) with no outline of its own either. */
export function BodyButton(props: { readonly icon: IconName; readonly onClick: () => void; readonly disabled?: boolean; readonly primary?: boolean; readonly row?: boolean; readonly cell?: boolean; readonly type?: "button" | "submit"; readonly describedBy?: string; readonly children: ReactNode }): ReactElement {
  return (
    <button
      type={props.type ?? "button"}
      disabled={props.disabled}
      aria-describedby={props.describedBy}
      onClick={props.onClick}
      {...(props.row || props.cell ? CELL : FRAME)}
      className={cn(
        overviewCardChipClass,
        "course-target h-auto max-h-none cursor-pointer whitespace-normal text-left leading-tight transition-colors disabled:cursor-not-allowed disabled:opacity-50",
        props.row ? "w-full max-w-none justify-start p-double" : props.cell ? "w-fit max-w-full justify-center px-double" : "w-fit max-w-full border border-normal",
        props.primary ? "bg-active-base text-active-foreground" : "text-foreground hover:bg-hover-interactive-fill",
      )}
    >
      <Icon icon={props.icon} size="small" className="shrink-0" />
      {props.children}
    </button>
  );
}

/** 🔗️ A link that is a whole row of a flush card, like a `row` {@link BodyButton}: a cell as wide as the card whose
 * dividers are the card's single ones and which has no outline of its own. `download` offers the file under that name,
 * `external` opens another site in a tab of its own that gets neither the page nor its address. */
export function RowLink(props: { readonly icon: IconName; readonly href: string; readonly download?: string; readonly external?: boolean; readonly primary?: boolean; readonly label?: string; readonly children: ReactNode }): ReactElement {
  return (
    <a
      href={props.href}
      download={props.download}
      target={props.external ? "_blank" : undefined}
      rel={props.external ? "noopener noreferrer" : undefined}
      aria-label={props.label}
      {...CELL}
      className={cn(overviewCardChipClass, "course-target h-auto max-h-none w-full max-w-none cursor-pointer justify-start whitespace-normal p-double text-left leading-tight no-underline transition-colors hover:bg-hover-interactive-fill", props.primary ? "bg-active-base text-active-foreground" : "text-foreground")}
    >
      <Icon icon={props.icon} size="small" className="shrink-0" />
      {props.children}
    </a>
  );
}

/** 🚫️ What went wrong: the sentence in the learner's language and, apart, the proctor's own words (English, from the
 * wire) for whoever has to look into it. */
export interface Problem {
  readonly message: string;
  readonly detail?: string;
}

/** 📓️ A {@link Problem} as an alert: the sentence, the technical detail collapsed and marked as English, and what the
 * learner can do about it (`children`). A `cell` note is a row of a flush card: no outline of its own, only the card's
 * single dividers. A `band` note lies across the page under the navbar with no margin: its top is the navbar's bottom line,
 * its bottom line the top of whatever follows, and its actions (`BodyButton cell`) are cells of its row. */
export function ProblemNote(props: { readonly problem: Problem; readonly text: CourseText; readonly id?: string; readonly className?: string; readonly cell?: boolean; readonly band?: boolean; readonly children?: ReactNode }): ReactElement {
  const { problem, text } = props;
  if (props.band) {
    return (
      <SharedBorderRow id={props.id} role="alert" orientation="vertical" edges={["bottom"]} className={cn("course-alert text-sm", props.className)}>
        <SharedBorderRow orientation="horizontal" framed={false} className="flex-wrap">
          <SharedBorderRowCell className="min-w-0 flex-1 px-double py-single font-semibold">{problem.message}</SharedBorderRowCell>
          {props.children}
        </SharedBorderRow>
        {problem.detail === undefined ? null : (
          <SharedBorderRowCell className="px-double text-xs text-muted-foreground">
            <details className="w-full">
              <summary {...sharedBorderlessBoxProps} className="course-target flex cursor-pointer items-center">{text("course.app.details")}</summary>
              <code lang="en" className="course-name block">
                {problem.detail}
              </code>
            </details>
          </SharedBorderRowCell>
        )}
      </SharedBorderRow>
    );
  }
  return (
    <div id={props.id} role="alert" {...(props.cell ? CELL : undefined)} className={cn("course-alert flex flex-wrap items-center gap-x-double gap-y-single px-double py-single text-sm", !props.cell && "border border-normal", props.className)}>
      <p className="m-0 min-w-0 flex-1 font-semibold">{problem.message}</p>
      {props.children}
      {problem.detail === undefined ? null : (
        <details className="w-full text-xs text-muted-foreground">
          <summary {...sharedBorderlessBoxProps} className="course-target flex cursor-pointer items-center">{text("course.app.details")}</summary>
          <code lang="en" className="course-name block">
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

/** 🎯 Focus `el` and leave every scroll offset alone. The browser's default scrolls the focused control into view,
 * which is what slides the page up under a window and leaves it on empty space once the window closes.
 * @see https://html.spec.whatwg.org/multipage/interaction.html#dom-focusoptions-preventscroll */
function focusHeld(el: HTMLElement | null | undefined): void {
  el?.focus({ preventScroll: true });
}

/** 📌 The scroll offsets of the window and of the page under a dialog, put back after a focus change. The dialog's own
 * glass is left free to scroll when the window is taller than the screen. */
function holdPageScroll(host: HTMLElement, backdrop: HTMLElement | null): () => void {
  const nodes: HTMLElement[] = [];
  const seen = new Set<HTMLElement>();
  const add = (el: HTMLElement | null): void => {
    if (el === null || seen.has(el) || (backdrop !== null && (el === backdrop || backdrop.contains(el)))) return;
    seen.add(el);
    nodes.push(el);
  };
  add(document.scrollingElement instanceof HTMLElement ? document.scrollingElement : null);
  add(document.documentElement);
  let node: HTMLElement | null = host;
  while (node !== null) {
    add(node);
    node = node.parentElement;
  }
  for (const el of host.querySelectorAll<HTMLElement>("*")) add(el);
  const snap = nodes.map((el) => [el, el.scrollTop, el.scrollLeft] as const);
  return () => {
    for (const [el, top, left] of snap) {
      if (el.scrollTop !== top) el.scrollTop = top;
      if (el.scrollLeft !== left) el.scrollLeft = left;
    }
  };
}

/** 💬️ A modal card over the whole client, a band from the left edge of the client to the right one whose content alone is centred: a question that needs an answer (`alertdialog`) or a notice to read
 * (`dialog`). It is rendered as the last child of the client's root, so the root's language and styles apply and
 * everything else of the client is inert while it shows; focus goes to its `data-autofocus` control (again whenever
 * `focusKey` changes), Tab stays inside from wherever focus is, Escape calls `onEscape`, and closing returns focus to
 * the control that opened it. The page keeps its scroll: the glass blurs what is already on screen and the window
 * stays centred on it. A click inside it never reaches the card it was opened from.
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/ */
export function Dialog(props: {
  readonly id: string;
  readonly role: "dialog" | "alertdialog";
  readonly icon: ReactNode;
  readonly title: string;
  readonly describedBy: string;
  readonly onEscape: () => void;
  readonly focusKey?: string;
  readonly observed?: boolean;
  readonly locale?: string;
  readonly wide?: boolean;
  readonly flush?: boolean;
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
  const host = origin === null ? null : (origin.closest<HTMLElement>(".course-app") ?? origin.ownerDocument.body);

  useLayoutEffect(() => {
    if (host === null || props.observed) return;
    const restore = holdPageScroll(host, backdrop.current);
    const silenced = [...host.children].filter((sibling) => sibling !== backdrop.current && !sibling.hasAttribute("inert"));
    focusHeld(dialog.current?.querySelector<HTMLElement>("[data-autofocus]"));
    for (const sibling of silenced) sibling.setAttribute("inert", "");
    restore();
    let holding = false;
    const onScroll = (): void => {
      if (holding) return;
      holding = true;
      restore();
      holding = false;
    };
    document.addEventListener("scroll", onScroll, true);
    return () => {
      document.removeEventListener("scroll", onScroll, true);
      const putBack = holdPageScroll(host, backdrop.current);
      for (const sibling of silenced) sibling.removeAttribute("inert");
      if (opener instanceof HTMLElement && opener.isConnected) focusHeld(opener);
      putBack();
    };
  }, [host, opener, props.observed]);

  useLayoutEffect(() => {
    if (host === null || props.observed) return;
    const restore = holdPageScroll(host, backdrop.current);
    focusHeld(dialog.current?.querySelector<HTMLElement>("[data-autofocus]"));
    restore();
  }, [host, props.focusKey, props.observed]);

  useEffect(() => {
    if (host === null || props.observed) return;
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
        focusHeld(last);
      } else if (!event.shiftKey && (outside || active === last)) {
        event.preventDefault();
        focusHeld(first);
      }
    };
    document.addEventListener("keydown", onKey, true);
    return () => document.removeEventListener("keydown", onKey, true);
  }, [host, props.observed]);

  return (
    <>
      <span ref={setOrigin} hidden />
      {host === null
        ? null
        : createPortal(
            <div ref={backdrop} data-course-dialog="" data-observed-dialog={props.observed ? "" : undefined} className={cn("course-backdrop fixed inset-0 z-50 overflow-y-auto", props.observed && "pointer-events-none")} onClick={(event) => event.stopPropagation()}>
              <div className="flex min-h-full items-center">
                <div ref={dialog} role={props.role} lang={props.locale ?? origin?.closest<HTMLElement>("[lang]")?.lang} aria-modal={props.observed ? undefined : true} aria-labelledby={props.id} aria-describedby={props.describedBy} className={cn("w-full", props.observed && "pointer-events-auto")}>
                  <CourseCard flush={props.flush} id={props.id} card="dialog" icon={props.icon} title={props.title} footerLeft={props.footerLeft} footerRight={props.footerRight}>
                    {props.flush ? props.children : <div className={cn("mx-auto w-full", props.wide ? "max-w-2xl" : "max-w-md")}>{props.children}</div>}
                  </CourseCard>
                </div>
              </div>
            </div>,
            host,
          )}
    </>
  );
}
