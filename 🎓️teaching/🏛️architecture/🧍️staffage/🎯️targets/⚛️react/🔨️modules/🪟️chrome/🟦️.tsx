/** 🪟️ The small controls and states every page of the archive is made of: buttons, segmented choices, fields, sliders, notices, progress with cancel and the archive card.
 *
 * Everything is a real native control (button, select, input range) with a visible label, at least 24 px high (WCAG 2.2 target size), styled with the semio tokens (sharp corners, hairlines, sans for interface, serif for editorial headings, mono for values).
 * Notices state the situation in words and offer the way out: nothing is conveyed by colour alone.
 *
 * @see ../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx — the card the archive card wraps
 * @see https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html — target size
 * @see https://www.w3.org/WAI/ARIA/apg/patterns/radio/ — the radio group keyboard model
 */
import { useId, useRef, useState, type ButtonHTMLAttributes, type KeyboardEvent, type ReactElement, type ReactNode } from "react";
import { Icon, OverviewCard, cn, type IconName } from "@semio-tech/ui-react/chrome";

export { Icon, cn, type IconName };

const CONTROL = "inline-flex min-h-6 items-center justify-center gap-single border border-border bg-panel px-double py-single text-xs font-medium text-foreground transition-colors hover:bg-hover-panel disabled:cursor-not-allowed disabled:opacity-50 motion-reduce:transition-none";

/** 🔘️ A button: `primary` inverts, `quiet` drops its frame. */
export function Button(props: ButtonHTMLAttributes<HTMLButtonElement> & { readonly variant?: "default" | "primary" | "quiet"; readonly icon?: IconName }): ReactElement {
  const { variant = "default", icon, className, children, type = "button", ...rest } = props;
  return (
    <button type={type} className={cn(CONTROL, variant === "primary" && "border-foreground bg-foreground text-background hover:bg-foreground/85", variant === "quiet" && "border-transparent bg-transparent", className)} {...rest}>
      {icon === undefined ? null : <Icon icon={icon} size="small" className="shrink-0" />}
      {children}
    </button>
  );
}

/** 🔗️ A link that looks like a button; navigation is the browser's own. */
export function ButtonLink(props: { readonly href: string; readonly icon?: IconName; readonly variant?: "default" | "primary" | "quiet"; readonly className?: string; readonly children: ReactNode; readonly "aria-label"?: string; readonly title?: string }): ReactElement {
  const { href, icon, variant = "default", className, children, ...rest } = props;
  return (
    <a href={href} className={cn(CONTROL, "no-underline", variant === "primary" && "border-foreground bg-foreground text-background hover:bg-foreground/85", variant === "quiet" && "border-transparent bg-transparent", className)} {...rest}>
      {icon === undefined ? null : <Icon icon={icon} size="small" className="shrink-0" />}
      {children}
    </a>
  );
}

/** 🎛️ One option of a segmented choice. */
export type SegmentOption<T extends string | number> = { readonly value: T; readonly label: ReactNode; readonly title?: string; readonly name?: string; readonly icon?: IconName; readonly lang?: string };

/** 🎛️ A radio group drawn as segments: arrow keys move and select, the selected segment is the only tab stop. */
export function Segmented<T extends string | number>(props: { readonly label: string; readonly options: readonly SegmentOption<T>[]; readonly value: T | null; readonly onChange: (value: T) => void; readonly className?: string; readonly hideLabel?: boolean }): ReactElement {
  const { label, options, value, onChange, className, hideLabel } = props;
  const id = useId();
  const refs = useRef<(HTMLButtonElement | null)[]>([]);
  const selected = Math.max(0, options.findIndex((option) => option.value === value));
  const move = (event: KeyboardEvent, index: number): void => {
    const step = event.key === "ArrowRight" || event.key === "ArrowDown" ? 1 : event.key === "ArrowLeft" || event.key === "ArrowUp" ? -1 : 0;
    if (step === 0) return;
    event.preventDefault();
    const next = (index + step + options.length) % options.length;
    refs.current[next]?.focus();
    onChange(options[next]!.value);
  };
  return (
    <div className={cn("flex min-w-0 flex-col gap-single", className)}>
      <span id={id} className={hideLabel ? "sr-only" : "text-[0.65rem] uppercase tracking-wide text-muted-foreground"}>
        {label}
      </span>
      <div role="radiogroup" aria-labelledby={id} className="inline-flex max-w-full flex-wrap border border-border">
        {options.map((option, index) => (
          <button
            key={option.value}
            ref={(element) => void (refs.current[index] = element)}
            type="button"
            role="radio"
            aria-checked={option.value === value}
            tabIndex={index === selected ? 0 : -1}
            title={option.title}
            aria-label={option.name}
            lang={option.lang}
            onKeyDown={(event) => move(event, index)}
            onClick={() => onChange(option.value)}
            className={cn("inline-flex min-h-6 items-center gap-single border-0 px-double py-single text-xs font-medium transition-colors motion-reduce:transition-none", option.value === value ? "bg-foreground text-background" : "bg-panel text-foreground hover:bg-hover-panel", index > 0 && "border-l border-border")}
          >
            {option.icon === undefined ? null : <Icon icon={option.icon} size="small" className="shrink-0" />}
            {option.label}
          </button>
        ))}
      </div>
    </div>
  );
}

const FIELD_LABEL = "text-[0.65rem] uppercase tracking-wide text-muted-foreground";
const INPUT = "min-h-6 w-full min-w-0 border border-border bg-panel px-double py-single text-xs text-foreground";

/** 🔽️ A native select with a visible label. */
export function SelectField<T extends string>(props: { readonly label: string; readonly value: T; readonly options: readonly { readonly value: T; readonly label: string }[]; readonly onChange: (value: T) => void; readonly className?: string; readonly disabled?: boolean }): ReactElement {
  const id = useId();
  return (
    <div className={cn("flex min-w-0 flex-col gap-single", props.className)}>
      <label htmlFor={id} className={FIELD_LABEL}>
        {props.label}
      </label>
      <select id={id} value={props.value} disabled={props.disabled} onChange={(event) => props.onChange(event.target.value as T)} className={INPUT}>
        {props.options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </div>
  );
}

/** ✏️ A text field with a visible label. */
export function TextField(props: { readonly label: string; readonly value: string; readonly onChange: (value: string) => void; readonly type?: "text" | "search"; readonly placeholder?: string; readonly className?: string; readonly describedBy?: string; readonly readOnly?: boolean; readonly inputRef?: (element: HTMLInputElement | null) => void }): ReactElement {
  const id = useId();
  return (
    <div className={cn("flex min-w-0 flex-col gap-single", props.className)}>
      <label htmlFor={id} className={FIELD_LABEL}>
        {props.label}
      </label>
      <input id={id} ref={props.inputRef} type={props.type ?? "text"} value={props.value} placeholder={props.placeholder} aria-describedby={props.describedBy} readOnly={props.readOnly} onChange={(event) => props.onChange(event.target.value)} className={INPUT} />
    </div>
  );
}

/** 🔢️ A number field with a visible label and unit. What is typed stays as typed while the reader edits; a value inside `min`..`max` is reported at once, an empty or out-of-range entry is flagged as invalid and reported when the field is left empty (`null`), and the field returns to the reported value when it loses focus. */
export function NumberField(props: { readonly label: string; readonly value: number | null; readonly onChange: (value: number | null) => void; readonly unit?: string; readonly min?: number; readonly max?: number; readonly step?: number; readonly className?: string }): ReactElement {
  const id = useId();
  const [draft, setDraft] = useState<string | null>(null);
  const parsed = draft === null || draft.trim() === "" ? null : Number(draft.replace(",", "."));
  const valid = draft === null || (parsed !== null && Number.isFinite(parsed) && (props.min === undefined || parsed >= props.min) && (props.max === undefined || parsed <= props.max));
  return (
    <div className={cn("flex min-w-0 flex-col gap-single", props.className)}>
      <label htmlFor={id} className={FIELD_LABEL}>
        {props.label}
      </label>
      <div className="flex items-center gap-single">
        <input
          id={id}
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
          className={cn(INPUT, "font-mono", !valid && "border-destructive")}
        />
        {props.unit === undefined ? null : <span className="text-xs text-muted-foreground">{props.unit}</span>}
      </div>
    </div>
  );
}

/** 🎚️ A slider with its label and its value as text beside it. */
export function SliderField(props: { readonly label: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly onChange: (value: number) => void; readonly text: string; readonly className?: string; readonly describedBy?: string }): ReactElement {
  const id = useId();
  return (
    <div className={cn("flex min-w-0 flex-col gap-single", props.className)}>
      <div className="flex items-baseline justify-between gap-double">
        <label htmlFor={id} className={FIELD_LABEL}>
          {props.label}
        </label>
        <output htmlFor={id} className="font-mono text-xs text-foreground">
          {props.text}
        </output>
      </div>
      <input id={id} type="range" min={props.min} max={props.max} step={props.step} value={props.value} aria-valuetext={props.text} aria-describedby={props.describedBy} onChange={(event) => props.onChange(Number(event.target.value))} className="min-h-6 w-full accent-[var(--accent)]" />
    </div>
  );
}

/** ☑️ A checkbox with its label. */
export function CheckField(props: { readonly label: ReactNode; readonly checked: boolean; readonly onChange: (checked: boolean) => void; readonly disabled?: boolean; readonly className?: string }): ReactElement {
  return (
    <label className={cn("inline-flex min-h-6 items-center gap-single text-xs text-foreground", props.disabled && "opacity-50", props.className)}>
      <input type="checkbox" checked={props.checked} disabled={props.disabled} onChange={(event) => props.onChange(event.target.checked)} className="size-4 accent-[var(--accent)]" />
      {props.label}
    </label>
  );
}

/** 🏷️ A small label chip. */
export function Tag(props: { readonly children: ReactNode; readonly title?: string; readonly className?: string }): ReactElement {
  return (
    <span title={props.title} className={cn("inline-flex items-center border border-border px-single py-0.5 text-[0.65rem] text-muted-foreground", props.className)}>
      {props.children}
    </span>
  );
}

/** 🚦️ The kinds of state a notice words. */
export type NoticeKind = "empty" | "loading" | "error" | "unsupported" | "unavailable" | "info";

const NOTICE_ICONS: Readonly<Record<NoticeKind, IconName>> = { empty: "search", loading: "loader-2", error: "triangle-alert", unsupported: "eye-off", unavailable: "plug", info: "info" };

/** 🚦️ A designed state — empty, loading, error, unsupported, unavailable, info — with what happened, why, and the way on. Errors are announced at once, the rest politely. */
export function Notice(props: { readonly kind: NoticeKind; readonly title: ReactNode; readonly children?: ReactNode; readonly actions?: ReactNode; readonly className?: string; readonly compact?: boolean }): ReactElement {
  const { kind, title, children, actions, className, compact } = props;
  return (
    <div role={kind === "error" ? "alert" : "status"} data-notice={kind} className={cn("flex min-w-0 flex-col gap-single border border-dashed border-border bg-muted/40 text-foreground", compact ? "p-double" : "p-double sm:p-4", className)}>
      <div className="flex items-center gap-single">
        <Icon icon={NOTICE_ICONS[kind]} size="small" className={cn("shrink-0 text-muted-foreground", kind === "loading" && "animate-spin motion-reduce:animate-none")} />
        <p className="m-0 text-sm font-semibold">{title}</p>
      </div>
      {children === undefined ? null : <div className="text-xs leading-normal text-muted-foreground">{children}</div>}
      {actions === undefined ? null : <div className="mt-single flex flex-wrap gap-single">{actions}</div>}
    </div>
  );
}

/** 📶️ A determinate progress bar with its label, count and a cancel button. */
export function ProgressNotice(props: { readonly label: string; readonly done: number; readonly total: number; readonly cancelLabel: string; readonly onCancel?: () => void; readonly className?: string }): ReactElement {
  const { label, done, total, cancelLabel, onCancel, className } = props;
  return (
    <div role="status" className={cn("flex min-w-0 flex-wrap items-center gap-double border border-border bg-panel p-double text-xs", className)}>
      <progress aria-label={label} value={done} max={Math.max(total, 1)} className="h-2 min-w-24 flex-1 accent-[var(--accent)]" />
      <span className="font-mono">
        {label}: {done}/{total}
      </span>
      {onCancel === undefined ? null : (
        <Button onClick={onCancel} icon="x">
          {cancelLabel}
        </Button>
      )}
    </div>
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
      title={props.href === undefined ? props.title : <a href={props.href} className="text-inherit no-underline hover:underline focus-visible:underline">{props.title}</a>}
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

/** 📄️ A scrolling page: padded, centred, one `h1`. */
export function PageFrame(props: { readonly title: ReactNode; readonly lede?: ReactNode; readonly actions?: ReactNode; readonly wide?: boolean; readonly eyebrow?: ReactNode; readonly children: ReactNode; readonly page: string }): ReactElement {
  return (
    <div data-page={props.page} className="relative h-full min-h-0 w-full overflow-auto bg-background text-foreground">
      <div className={cn("mx-auto flex w-full flex-col gap-double p-double sm:p-4", props.wide ? "max-w-[96rem]" : "max-w-6xl")}>
        <header className="flex flex-col gap-single border-b border-border pb-double">
          {props.eyebrow === undefined ? null : <p className="m-0 font-mono text-[0.65rem] uppercase tracking-widest text-muted-foreground">{props.eyebrow}</p>}
          <div className="flex flex-wrap items-end justify-between gap-double">
            <h1 tabIndex={-1} className="m-0 font-serif text-3xl leading-tight text-foreground outline-none sm:text-4xl">
              {props.title}
            </h1>
            {props.actions === undefined ? null : <div className="flex flex-wrap gap-single">{props.actions}</div>}
          </div>
          {props.lede === undefined ? null : <p className="m-0 max-w-[70ch] text-sm leading-normal text-muted-foreground">{props.lede}</p>}
        </header>
        {props.children}
      </div>
    </div>
  );
}

/** 🔖️ A section heading of a page. */
export function SectionHeading(props: { readonly id: string; readonly children: ReactNode; readonly aside?: ReactNode }): ReactElement {
  return (
    <div className="flex flex-wrap items-baseline justify-between gap-double border-b border-border pb-single">
      <h2 id={props.id} className="m-0 font-serif text-xl text-foreground">
        {props.children}
      </h2>
      {props.aside === undefined ? null : <div className="text-xs text-muted-foreground">{props.aside}</div>}
    </div>
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
