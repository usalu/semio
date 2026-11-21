/** 🪟️ The small controls and states every page of the archive is made of: buttons, segmented choices, fields, sliders, notices, progress with cancel and the archive card.
 *
 * Everything is a real native control (button, select, input range) with a visible label, at least 24 px high (WCAG 2.2 target size), styled with the semio tokens (sharp corners, hairlines, sans for interface, serif for editorial headings, mono for values).
 * Notices state the situation in words and offer the way out: nothing is conveyed by colour alone.
 *
 * @see ../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx — the card the archive card wraps
 * @see https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html — target size
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/radio/ — the radio group keyboard model
 */
import { Children, cloneElement, Fragment, isValidElement, useId, useRef, useState, type ButtonHTMLAttributes, type KeyboardEvent, type ReactElement, type ReactNode } from "react";
import type * as React from "react";
import { CardGrid, CardGridCell, Icon, OverviewCard, SharedBorderRow, SharedBorderRowCell, SharedBorderRowField, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, cn, useMediaQuery, type IconName, type SharedBorderRowProps } from "@semio-tech/ui-react/chrome";

export { CardGrid, CardGridCell, Icon, SharedBorderRow, SharedBorderRowCell, SharedBorderRowField, cn, type IconName };

/** 🪢️ A row of cells that stands up as a column where the window is narrower than `below`: wrapping cells cannot share one divider, so a narrow window stacks them instead. */
export function Bar(props: Omit<SharedBorderRowProps, "orientation"> & { readonly below?: "mobile" | "tablet" | "never" }): ReactElement {
  const { below = "mobile", ...rest } = props;
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  const stacked = below === "tablet" ? mobile || tablet : below === "mobile" && mobile;
  return <SharedBorderRow orientation={stacked ? "vertical" : "horizontal"} {...rest} />;
}

/** 🧱️ A column of cells: the plain vertical shared-border row; framed unless an enclosing owner paints its edges. */
export function Column(props: Omit<SharedBorderRowProps, "orientation">): ReactElement {
  return <SharedBorderRow orientation="vertical" {...props} />;
}

/** 🪟️ A cell of text: padded content that shares the dividers of the row it stands in. */
export function Cell(props: React.ComponentProps<typeof SharedBorderRowCell>): ReactElement {
  const { className, ...rest } = props;
  return <SharedBorderRowCell className={cn("flex-wrap gap-single px-double py-double", className)} {...rest} />;
}

const CONTROL = "inline-flex min-h-6 items-center justify-center gap-single border border-border bg-panel px-double py-single text-xs font-medium text-foreground transition-colors hover:bg-hover-panel disabled:cursor-not-allowed disabled:opacity-50 motion-reduce:transition-none";
const PRIMARY = "border-primary bg-primary text-primary-foreground hover:bg-primary/90";
const PRESSED = "bg-active-base text-active-foreground hover:bg-active-hover";

/** ☑️ A control is pressed when `aria-pressed` is the boolean or the string true. */
function pressed(value: unknown): boolean {
  return value === true || value === "true";
}

const CELL = "border-0";

/** 🧱️ The compositor box of a control: a cell of its row, a borderless box when quiet, otherwise a framed box. */
function controlBox(grouped: boolean | undefined, quiet: boolean): Record<string, string> {
  if (grouped) return { "data-shared-border-cell": "" };
  return quiet ? { "data-shared-border-frame": "", "data-shared-border-edges": "" } : { "data-shared-border-frame": "" };
}

/** 🧷️ The children of a row with every `Button` and `ButtonLink` made a cell of it, fragments included. */
export function asCells(children: ReactNode): ReactNode {
  return Children.map(children, (child) => {
    if (!isValidElement<{ readonly children?: ReactNode; readonly grouped?: boolean }>(child)) return child;
    if (child.type === Fragment) return <Fragment>{asCells(child.props.children)}</Fragment>;
    return child.type === Button || child.type === ButtonLink ? cloneElement(child, { grouped: true }) : child;
  });
}

/** 🔘️ A button: `primary` uses the primary color, `quiet` drops its frame, a pressed toggle uses the active background, and a `grouped` one is a cell of a row whose divider the shared border owns. */
export function Button(props: ButtonHTMLAttributes<HTMLButtonElement> & { readonly variant?: "default" | "primary" | "quiet"; readonly icon?: IconName; readonly grouped?: boolean }): ReactElement {
  const { variant = "default", icon, grouped, className, children, type = "button", ...rest } = props;
  const on = pressed(rest["aria-pressed"]);
  return (
    <button type={type} {...controlBox(grouped, variant === "quiet" && !on)} className={cn(CONTROL, grouped && CELL, variant === "primary" && !on && PRIMARY, variant === "quiet" && !on && "border-transparent bg-transparent", on && PRESSED, className)} {...rest}>
      {icon === undefined ? null : <Icon icon={icon} size="small" className="shrink-0" />}
      {children}
    </button>
  );
}

/** 🧷️ Buttons that are one control: a single frame with one shared hairline between neighbours instead of a bordered chip each. Children are `Button`s with `grouped` or elements marked `data-shared-border-cell`. */
export function ButtonGroup(props: { readonly label?: string; readonly className?: string; readonly children: ReactNode; readonly below?: "mobile" | "tablet" | "never" }): ReactElement {
  return (
    <Bar role="group" aria-label={props.label} {...(props.below === undefined ? {} : { below: props.below })} className={cn("max-w-full", props.className)}>
      {asCells(props.children)}
    </Bar>
  );
}

/** 🔗️ A link that looks like a button; navigation is the browser's own. A `grouped` one is a cell of a row. */
export function ButtonLink(props: { readonly href: string; readonly icon?: IconName; readonly variant?: "default" | "primary" | "quiet"; readonly className?: string; readonly grouped?: boolean; readonly children: ReactNode; readonly "aria-label"?: string; readonly "aria-current"?: "page"; readonly title?: string }): ReactElement {
  const { href, icon, variant = "default", className, grouped, children, ...rest } = props;
  const current = rest["aria-current"] === "page";
  return (
    <a href={href} {...controlBox(grouped, variant === "quiet" || current)} {...(current ? { "data-shared-border-state": "active" } : {})} className={cn(CONTROL, grouped && CELL, "no-underline", variant === "primary" && PRIMARY, variant === "quiet" && !current && "border-transparent bg-transparent", current && "border-transparent bg-transparent text-primary", className)} {...rest}>
      {icon === undefined ? null : <Icon icon={icon} size="small" className="shrink-0" />}
      {children}
    </a>
  );
}

/** 🎛️ One option of a segmented choice. */
export type SegmentOption<T extends string | number> = { readonly value: T; readonly label: ReactNode; readonly title?: string; readonly name?: string; readonly icon?: IconName; readonly lang?: string; readonly unavailable?: string };

/** 🎛️ A radio group drawn as segments of one shared-border row: arrow keys move and select, the selected segment is the only tab stop. Its label is a cell above the segments unless `hideLabel` makes it screen-reader only. */
export function Segmented<T extends string | number>(props: { readonly label: string; readonly options: readonly SegmentOption<T>[]; readonly value: T | null; readonly onChange: (value: T) => void; readonly className?: string; readonly hideLabel?: boolean; readonly inline?: boolean; readonly data?: Readonly<Record<`data-${string}`, string>> }): ReactElement {
  const { label, options, value, onChange, className, hideLabel, inline, data } = props;
  const id = useId();
  const refs = useRef<(HTMLButtonElement | null)[]>([]);
  const selected = Math.max(0, options.findIndex((option) => option.value === value));
  const move = (event: KeyboardEvent, index: number): void => {
    const step = event.key === "ArrowRight" || event.key === "ArrowDown" ? 1 : event.key === "ArrowLeft" || event.key === "ArrowUp" ? -1 : 0;
    if (step === 0) return;
    event.preventDefault();
    const next = (index + step + options.length) % options.length;
    refs.current[next]?.focus();
    if (options[next]!.unavailable === undefined) onChange(options[next]!.value);
  };
  const group = (
    <SharedBorderRow role="radiogroup" {...(hideLabel ? { "aria-label": label, ...data } : { "aria-labelledby": id })} className={cn("max-w-full", hideLabel ? className : inline ? "flex-1" : "w-full")}>
      {options.map((option, index) => (
        <button
          key={option.value}
          ref={(element) => void (refs.current[index] = element)}
          type="button"
          role="radio"
          aria-checked={option.value === value}
          tabIndex={index === selected ? 0 : -1}
          title={option.unavailable ?? option.title}
          aria-label={option.name}
          {...(option.unavailable === undefined ? {} : { "aria-disabled": true, "aria-description": option.unavailable, "data-unavailable": "" })}
          lang={option.lang}
          data-shared-border-cell=""
          onKeyDown={(event) => move(event, index)}
          onClick={() => option.unavailable === undefined && onChange(option.value)}
          className={cn("inline-flex min-h-6 min-w-0 items-center justify-center gap-single border-0 px-double py-single text-xs font-medium transition-colors motion-reduce:transition-none", !hideLabel && "flex-1", option.value === value ? "bg-active-base text-active-foreground" : "bg-panel text-foreground hover:bg-hover-panel", option.unavailable !== undefined && "cursor-not-allowed opacity-65")}
        >
          {option.icon === undefined ? null : <Icon icon={option.icon} size="small" className="shrink-0" />}
          {option.label}
        </button>
      ))}
    </SharedBorderRow>
  );
  if (hideLabel) return group;
  return (
    <SharedBorderRow orientation={inline ? "horizontal" : "vertical"} {...data} className={className}>
      <SharedBorderRowCell className="px-double py-single">
        <span id={id} className={LABEL_TEXT}>{label}</span>
      </SharedBorderRowCell>
      {group}
    </SharedBorderRow>
  );
}

const LABEL_TEXT = "text-[0.65rem] uppercase tracking-wide text-muted-foreground";
const FIELD = "min-w-0 flex-1 gap-double px-double py-single";
const FIELD_LABEL = cn(LABEL_TEXT, "shrink-0");

/** 🔽️ A native select with a visible label: one cell of a row, the label inside the cell. */
export function SelectField<T extends string>(props: { readonly label: string; readonly value: T; readonly options: readonly { readonly value: T; readonly label: string }[]; readonly onChange: (value: T) => void; readonly className?: string; readonly disabled?: boolean }): ReactElement {
  return (
    <SharedBorderRowField label={<span className={FIELD_LABEL}>{props.label}</span>} className={cn(FIELD, props.className)}>
      <select aria-label={props.label} value={props.value} disabled={props.disabled} onChange={(event) => props.onChange(event.target.value as T)} className="min-w-0 flex-1 text-xs text-foreground">
        {props.options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </SharedBorderRowField>
  );
}

/** ✏️ A text field with a visible label: one cell of a row, the label inside the cell. */
export function TextField(props: { readonly label: string; readonly value: string; readonly onChange: (value: string) => void; readonly type?: "text" | "search"; readonly placeholder?: string; readonly className?: string; readonly describedBy?: string; readonly readOnly?: boolean; readonly inputRef?: (element: HTMLInputElement | null) => void }): ReactElement {
  return (
    <SharedBorderRowField label={<span className={FIELD_LABEL}>{props.label}</span>} className={cn(FIELD, props.className)}>
      <input aria-label={props.label} ref={props.inputRef} type={props.type ?? "text"} value={props.value} placeholder={props.placeholder} aria-describedby={props.describedBy} readOnly={props.readOnly} onChange={(event) => props.onChange(event.target.value)} className="min-w-0 flex-1 text-xs text-foreground" />
    </SharedBorderRowField>
  );
}

/** 🔢️ A number field with a visible label and unit: one cell of a row. What is typed stays as typed while the reader edits; a value inside `min`..`max` is reported at once, an empty or out-of-range entry is flagged as invalid and reported when the field is left empty (`null`), and the field returns to the reported value when it loses focus. */
export function NumberField(props: { readonly label: string; readonly value: number | null; readonly onChange: (value: number | null) => void; readonly unit?: string; readonly min?: number; readonly max?: number; readonly step?: number; readonly className?: string }): ReactElement {
  const [draft, setDraft] = useState<string | null>(null);
  const parsed = draft === null || draft.trim() === "" ? null : Number(draft.replace(",", "."));
  const valid = draft === null || (parsed !== null && Number.isFinite(parsed) && (props.min === undefined || parsed >= props.min) && (props.max === undefined || parsed <= props.max));
  return (
    <SharedBorderRowField label={<span className={FIELD_LABEL}>{props.label}</span>} className={cn(FIELD, "flex-wrap", props.className)} {...(valid ? {} : { "data-invalid": "" })}>
      <input
        aria-label={props.label}
        type="text"
        inputMode="decimal"
        value={draft ?? (props.value === null ? "" : String(props.value))}
        aria-invalid={valid ? undefined : true}
        onChange={(event) => {
          const text = event.target.value;
          setDraft(text);
          const number = text.trim() === "" ? null : Number(text.replace(",", "."));
          if (number === null) props.onChange(null);
          else if (Number.isFinite(number) && (props.min === undefined || number >= props.min) && (props.max === undefined || number <= props.max)) props.onChange(number);
        }}
        onBlur={() => setDraft(null)}
        className={cn("w-16 min-w-0 flex-1 font-mono text-xs text-foreground", !valid && "text-destructive")}
      />
      {props.unit === undefined ? null : <span className="text-xs text-muted-foreground">{props.unit}</span>}
    </SharedBorderRowField>
  );
}

/** 🎚️ A slider with its label and its value as text: one cell of a column. */
export function SliderField(props: { readonly label: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly onChange: (value: number) => void; readonly text: string; readonly className?: string; readonly describedBy?: string }): ReactElement {
  const id = useId();
  return (
    <SharedBorderRowCell className={cn("flex-col items-stretch gap-single px-double py-single", props.className)}>
      <div className="flex items-baseline justify-between gap-double">
        <label htmlFor={id} className={LABEL_TEXT}>
          {props.label}
        </label>
        <output htmlFor={id} className="font-mono text-xs text-foreground">
          {props.text}
        </output>
      </div>
      <input id={id} type="range" min={props.min} max={props.max} step={props.step} value={props.value} aria-valuetext={props.text} aria-describedby={props.describedBy} onChange={(event) => props.onChange(Number(event.target.value))} className="min-h-6 w-full accent-[var(--accent)]" />
    </SharedBorderRowCell>
  );
}

/** ☑️ A checkbox with its label: one cell of a row. */
export function CheckField(props: { readonly label: ReactNode; readonly checked: boolean; readonly onChange: (checked: boolean) => void; readonly disabled?: boolean; readonly className?: string }): ReactElement {
  return (
    <label data-shared-border-cell="" className={cn("flex min-h-6 min-w-0 flex-1 items-center gap-single px-double py-single text-xs text-foreground", props.disabled && "opacity-50", props.className)}>
      <input type="checkbox" checked={props.checked} disabled={props.disabled} onChange={(event) => props.onChange(event.target.checked)} className="size-4 accent-[var(--accent)]" />
      {props.label}
    </label>
  );
}

/** 🏷️ A small label chip: tinted, never framed, so chips never stand as boxes beside the card's own lines. */
export function Tag(props: { readonly children: ReactNode; readonly title?: string; readonly className?: string }): ReactElement {
  return (
    <span title={props.title} className={cn("inline-flex items-center bg-muted/60 px-single py-0.5 text-[0.65rem] text-muted-foreground", props.className)}>
      {props.children}
    </span>
  );
}

/** 📑️ A section of a page: a column of cells (heading, text, tables, controls), itself a cell of the page's column. */
export function Section(props: Omit<SharedBorderRowProps, "orientation" | "as">): ReactElement {
  return <SharedBorderRow as="section" orientation="vertical" {...props} />;
}

/** 🔖️ The heading cell of a {@link Section}. */
export function SectionTitle(props: { readonly id?: string; readonly level?: 2 | 3; readonly children: ReactNode; readonly className?: string }): ReactElement {
  const Heading = props.level === 3 ? "h3" : "h2";
  return (
    <Cell>
      <Heading {...(props.id === undefined ? {} : { id: props.id })} className={cn("m-0 font-serif", props.level === 3 ? "text-sm font-semibold" : "text-lg", props.className)}>{props.children}</Heading>
    </Cell>
  );
}

/** 📝️ A paragraph cell: text that shares the dividers of the column it stands in. */
export function Note(props: { readonly children: ReactNode; readonly className?: string; readonly id?: string; readonly tone?: "muted" | "plain" }): ReactElement {
  return (
    <Cell>
      <p {...(props.id === undefined ? {} : { id: props.id })} className={cn("m-0 max-w-[70ch] text-xs leading-normal", props.tone === "plain" ? "text-foreground" : "text-muted-foreground", props.className)}>{props.children}</p>
    </Cell>
  );
}

/** 🗂️ A labelled group of controls: a column of a label cell and one row of cells (stacked on a narrow window), named for assistive technology by its label. */
export function Group(props: { readonly legend: ReactNode; readonly children: ReactNode; readonly className?: string; readonly below?: "mobile" | "tablet" | "never"; readonly stacked?: boolean }): ReactElement {
  const id = useId();
  return (
    <SharedBorderRow orientation="vertical" role="group" aria-labelledby={id} className={props.className}>
      <SharedBorderRowCell className="px-double py-single">
        <span id={id} className={LABEL_TEXT}>{props.legend}</span>
      </SharedBorderRowCell>
      {props.stacked ? props.children : <Bar {...(props.below === undefined ? {} : { below: props.below })}>{asCells(props.children)}</Bar>}
    </SharedBorderRow>
  );
}

/** 🚦️ The kinds of state a notice words. */
export type NoticeKind = "empty" | "loading" | "error" | "unsupported" | "unavailable" | "info";

const NOTICE_ICONS: Readonly<Record<NoticeKind, IconName>> = { empty: "search", loading: "loader-2", error: "triangle-alert", unsupported: "eye-off", unavailable: "plug", info: "info" };

/** 🚦️ A designed state — empty, loading, error, unsupported, unavailable, info — with what happened, why, and the way on: a column of cells (the words, then the actions as one row of buttons) that is itself a cell of the column it stands in. Errors are announced at once, the rest politely. */
export function Notice(props: { readonly kind: NoticeKind; readonly title: ReactNode; readonly children?: ReactNode; readonly actions?: ReactNode; readonly className?: string; readonly compact?: boolean; readonly plain?: boolean; readonly live?: boolean; readonly data?: Readonly<Record<`data-${string}`, string>> }): ReactElement {
  const { kind, title, children, actions, className, compact, plain, live = true, data } = props;
  const role = live ? (kind === "error" ? "alert" : "status") : undefined;
  if (plain)
    return (
      <SharedBorderRow orientation="vertical" framed={false} role={role} data-notice={kind} {...data} className={cn("bg-muted/40 text-foreground", className)}>
        <SharedBorderRowCell className="flex-col items-stretch gap-single p-double">
          <div className="flex items-center gap-single">
            <Icon icon={NOTICE_ICONS[kind]} size="small" className={cn("shrink-0 text-muted-foreground", kind === "loading" && "animate-spin motion-reduce:animate-none")} />
            <p className="m-0 text-sm font-semibold">{title}</p>
          </div>
          {children === undefined ? null : <div className="flex flex-col gap-single text-xs leading-normal text-muted-foreground">{children}</div>}
        </SharedBorderRowCell>
        {actions === undefined ? null : <Bar framed={false}>{asCells(actions)}</Bar>}
      </SharedBorderRow>
    );
  return (
    <SharedBorderRow orientation="vertical" role={role} data-notice={kind} {...data} className={cn("bg-muted/40 text-foreground", className)}>
      <SharedBorderRowCell className={cn("flex-col items-stretch gap-single p-double", !compact && "sm:p-4")}>
        <div className="flex items-center gap-single">
          <Icon icon={NOTICE_ICONS[kind]} size="small" className={cn("shrink-0 text-muted-foreground", kind === "loading" && "animate-spin motion-reduce:animate-none")} />
          <p className="m-0 text-sm font-semibold">{title}</p>
        </div>
        {children === undefined ? null : <div className="text-xs leading-normal text-muted-foreground">{children}</div>}
      </SharedBorderRowCell>
      {actions === undefined ? null : <Bar>{asCells(actions)}</Bar>}
    </SharedBorderRow>
  );
}

/** 📶️ A determinate progress bar with its label, count and a cancel button: a row of cells that is itself a cell of the column it stands in. */
export function ProgressNotice(props: { readonly label: string; readonly done: number; readonly total: number; readonly cancelLabel: string; readonly onCancel?: () => void; readonly className?: string }): ReactElement {
  const { label, done, total, cancelLabel, onCancel, className } = props;
  return (
    <SharedBorderRow role="status" className={cn("bg-panel text-xs", className)}>
      <SharedBorderRowCell className="flex-1 flex-wrap gap-double p-double">
        <progress aria-label={label} value={done} max={Math.max(total, 1)} className="h-2 min-w-24 flex-1 accent-[var(--accent)]" />
        <span className="font-mono">
          {label}: {done}/{total}
        </span>
      </SharedBorderRowCell>
      {onCancel === undefined ? null : (
        <Button grouped onClick={onCancel} icon="x">
          {cancelLabel}
        </Button>
      )}
    </SharedBorderRow>
  );
}

/** 🃏️ The archive's card: a titled section of the overview whose heading is a link; a click anywhere on it but on a control opens it. */
export function ArchiveCard(props: {
  readonly id: string;
  readonly card: string;
  readonly href?: string;
  readonly onOpen?: () => void;
  readonly revealed?: boolean;
  readonly icon: ReactNode;
  readonly title: ReactNode;
  readonly footerLeft?: ReactNode;
  readonly footerRight?: ReactNode;
  readonly className?: string;
  readonly bodyClassName?: string;
  readonly headingLevel?: 1 | 2 | 3;
  readonly children?: ReactNode;
}): ReactElement {
  const { onOpen } = props;
  const card = (
    <OverviewCard
      as="section"
      slot="staffage-card"
      headingId={props.id}
      headingLevel={props.headingLevel ?? 2}
      data={{ "data-card": props.card, "data-revealed": props.revealed ? "" : undefined }}
      icon={props.icon}
      title={props.href === undefined ? props.title : <a href={props.href} data-shared-border-frame="" data-shared-border-edges="" className="text-inherit no-underline outline-none hover:underline">{props.title}</a>}
      footerLeft={props.footerLeft}
      footerRight={props.footerRight}
      className={cn("pointer-events-auto", onOpen !== undefined && "cursor-pointer transition-transform duration-200 motion-reduce:transition-none", props.revealed && "-translate-y-0.5", props.className)}
      bodyClassName={props.bodyClassName}
      contentClassName="flex h-full min-h-0 flex-col gap-single"
    >
      {props.children}
    </OverviewCard>
  );
  if (onOpen === undefined) return card;
  return (
    <div
      className="contents"
      onClick={(event) => {
        if (!(event.target instanceof Element) || event.target.closest("a, button, summary, input, select, textarea, label") === null) onOpen();
      }}
    >
      {card}
    </div>
  );
}

/** 🔘️ An action chip in a card's footer (a real button at least 24 px tall). */
export function CardButton(props: ButtonHTMLAttributes<HTMLButtonElement> & { readonly icon?: IconName }): ReactElement {
  const { icon, children, className, ...rest } = props;
  return (
    <Button variant="quiet" icon={icon} className={cn("min-w-0 max-w-full whitespace-normal text-left", className)} {...rest}>
      {children}
    </Button>
  );
}

/** 📄️ A scrolling page: one column of cells that fills the width — the header (the one `h1`, its lede and its actions as a row of buttons) and then whatever the page puts in it, each child a cell or a row of cells. The column paints only the dividers between its cells; the navbar and the footer own the page's top and bottom lines. */
export function PageFrame(props: { readonly title: ReactNode; readonly lede?: ReactNode; readonly actions?: ReactNode; readonly wide?: boolean; readonly eyebrow?: ReactNode; readonly children: ReactNode; readonly page: string }): ReactElement {
  return (
    <div data-page={props.page} className="relative h-full min-h-0 w-full overflow-auto bg-background text-foreground">
      <SharedBorderRow orientation="vertical" framed={false} className="min-h-full">
        <header data-shared-border-cell="" className="flex flex-col gap-single p-double sm:p-4">
          {props.eyebrow === undefined ? null : <p className="m-0 font-mono text-[0.65rem] uppercase tracking-widest text-muted-foreground">{props.eyebrow}</p>}
          <h1 tabIndex={-1} className="m-0 font-serif text-3xl leading-tight text-foreground outline-none sm:text-4xl">
            {props.title}
          </h1>
          {props.lede === undefined ? null : <p className="m-0 max-w-[70ch] text-sm leading-normal text-muted-foreground">{props.lede}</p>}
        </header>
        {props.actions === undefined ? null : <Bar className="w-full">{asCells(props.actions)}</Bar>}
        {props.children}
      </SharedBorderRow>
    </div>
  );
}

/** 🔖️ A section heading of a page: a cell with the heading and its aside. */
export function SectionHeading(props: { readonly id: string; readonly children: ReactNode; readonly aside?: ReactNode }): ReactElement {
  return (
    <Cell className="items-baseline justify-between">
      <h2 id={props.id} className="m-0 font-serif text-xl text-foreground">
        {props.children}
      </h2>
      {props.aside === undefined ? null : <div className="text-xs text-muted-foreground">{props.aside}</div>}
    </Cell>
  );
}

/** ⭐️ A star, filled when set; the shape alone says "kept", so colour is never the only cue. */
export function StarIcon(props: { readonly filled: boolean; readonly className?: string }): ReactElement {
  return (
    <svg viewBox="0 0 24 24" width="16" height="16" aria-hidden="true" focusable="false" className={cn("shrink-0", props.className)}>
      <path d="M12 2.8l2.9 6 6.6.9-4.8 4.6 1.2 6.5L12 17.6l-5.9 3.2 1.2-6.5L2.5 9.7l6.6-.9z" fill={props.filled ? "currentColor" : "none"} stroke="currentColor" strokeWidth="1.8" strokeLinejoin="round" />
    </svg>
  );
}
