/** 🧩️ What every task interaction shares: its props, polite announcements of moves for screen readers, focus that
 * follows an item after it moved, the pointer grip that starts a drag, the field a numeric guess is typed into and the
 * hints of an easy run — each a question about two concrete items beside what it is about, with a question mark in a
 * circle, and spoken once when it appears.
 *
 * @see https://www.w3.org/WAI/WCAG22/Techniques/aria/ARIA22 — status messages through `role="status"`
 */

import { useCallback, useEffect, useLayoutEffect, useRef, useState, type FocusEvent, type KeyboardEvent, type PointerEvent, type ReactElement } from "react";
import type { Answer, CompareHint, Hint, Quantity, SheetTask, Slug, Text } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { startPointerDrag } from "../🤏️drag/🟦️.ts";
import { useAnnouncement, type Announcement } from "../🪟️chrome/🟦️.tsx";
import { formatCount, formatQuantity, formatTimes, parseQuantity, withUnit } from "../📏️quantity/🟦️.ts";

/** 🧾️ The props of one task interaction: the presented task, the learner's current answer and where changes go; the
 * `hints` the run gives on the recorded answer (none unless the challenge hints) and whether the task is `locked`
 * because its time is up — then it shows the answer but takes no change, its controls say why, and an entry typed but
 * not committed when the lock came is handed on as a last answer when it reads as one, else lost through `onDropped`. */
export interface TaskViewProps<T extends SheetTask, A extends Answer> {
  readonly task: T;
  readonly answer: A | undefined;
  readonly onAnswer: (answer: A) => void;
  readonly text: QuizText;
  readonly locale: QuizLocale;
  readonly hints?: readonly Hint[];
  readonly locked?: boolean;
  readonly onDropped?: () => void;
}

/** 🔒️ What describes every control of a locked task, `id` for their `aria-describedby`: hidden itself, since the clock
 * shows it, but read with each control. */
export function LockedNote(props: { readonly id: string; readonly text: QuizText }): ReactElement {
  return (
    <span id={props.id} hidden>
      {props.text("quiz.task.locked")}
    </span>
  );
}

/** 🧷️ The ids that describe a control, joined, or `undefined` when none does. */
export function describedBy(...ids: readonly (string | undefined)[]): string | undefined {
  const joined = ids.filter((id) => id !== undefined).join(" ");
  return joined === "" ? undefined : joined;
}

/** 🧱️ The frame of a drop zone (the pool, a category bin): a dashed hairline that the drag highlight fills. */
export const DROP_ZONE_CLASS = "flex min-w-0 flex-col gap-single border border-dashed border-normal p-double";

/** 🔽️ A select of a task: at least 24 px tall, framed by the hairline, on the page colour. */
export const SELECT_CLASS = "quiz-target border border-normal bg-background px-single text-sm text-foreground";

/** 🔘️ A small square action of a task row (move, unassign). */
export const ICON_BUTTON_CLASS =
  "quiz-target inline-flex min-w-[1.75em] cursor-pointer items-center justify-center border border-normal bg-transparent px-single text-foreground hover:bg-hover-interactive-fill aria-disabled:cursor-not-allowed aria-disabled:opacity-50";

/** ⏱️ How long a select-driven task waits before it announces a change: a closed select commits every option the arrow
 * keys pass, so only where the item ends up is spoken. */
export const SELECT_ANNOUNCEMENT_DELAY_MS = 400;

/** 🎯️ Asks to focus the element with a DOM id after the next render — for controls that moved or were re-created. */
export function useFocusAfterRender(): (id: string) => void {
  const pending = useRef<string | undefined>(undefined);
  useLayoutEffect(() => {
    const id = pending.current;
    if (id === undefined) return;
    pending.current = undefined;
    document.getElementById(id)?.focus();
  });
  return useCallback((id: string) => {
    pending.current = id;
  }, []);
}

/** 🔖️ A DOM id for one element of one task instance; slugs never contain the separator. */
export function elementId(scope: string, ...parts: readonly string[]): string {
  return [scope, ...parts].join("--");
}

/** 🤏️ The pointer grip of a draggable element; hidden from assistive technology, which uses the keyboard path. A
 * `locked` grip starts no drag. */
export function DragGrip(props: { readonly title: string; readonly onDrop: (zone: string) => void; readonly locked?: boolean }): ReactElement {
  const { title, onDrop, locked = false } = props;
  return (
    <span
      data-quiz-grip=""
      className={`quiz-target inline-grid min-w-[1.75em] touch-none select-none place-items-center text-muted-foreground ${locked ? "cursor-not-allowed opacity-50" : "cursor-grab"}`}
      aria-hidden="true"
      title={title}
      onPointerDown={locked ? undefined : (event: PointerEvent<HTMLSpanElement>) => startPointerDrag(event, onDrop)}
    >
      ⠿
    </span>
  );
}

/** 🧪️ An example guess in the unit of `quantity`, such as `2 kW`. */
export function exampleGuess(quantity: Pick<Quantity, "unit" | "prefixed">): string {
  return withUnit("2", quantity.prefixed ? `k${quantity.unit}` : quantity.unit);
}

/** 🔢️ The field a numeric guess of one item is typed into, named `name`: the unit shows in its example, and what a
 * prefixed text reads as beside it. It commits on Enter and on leaving the field (with the id of the element focus
 * went to); Escape restores what was committed; an unreadable text stays, flagged, until it is fixed or emptied, and
 * an empty one removes the guess. A text left as shown never overwrites an exact guess with its rounded display. A
 * `readOnly` field keeps its guess and takes no other; when it turns read-only over a text not yet committed, that
 * text is committed once more if it reads as a guess (whoever takes it decides whether it came in time), and is lost
 * through `onDropped` if it does not. `describedBy` names what else describes it, such as why it is read-only. */
export function GuessField(props: {
  readonly id: string;
  readonly name: string;
  readonly quantity: Quantity;
  readonly value: number | undefined;
  readonly onCommit: (value: number | undefined, focus: string | undefined) => void;
  readonly onDropped?: () => void;
  readonly readOnly?: boolean;
  readonly describedBy?: string;
  readonly text: QuizText;
  readonly locale: QuizLocale;
}): ReactElement {
  const { id, name, quantity, value, onCommit, readOnly = false, text, locale } = props;
  const [draft, setDraft] = useState<string | undefined>(undefined);
  const [rejected, setRejected] = useState(false);
  const shown = value === undefined ? "" : formatQuantity(value, quantity, locale);
  const typed = readOnly ? shown : (draft ?? shown);
  const parsed = readOnly || draft === undefined || draft.trim() === "" ? undefined : parseQuantity(draft, quantity, locale);
  const example = exampleGuess(quantity);
  const errorId = `${id}--error`;
  const previewId = `${id}--preview`;
  const described = describedBy(rejected ? errorId : parsed === undefined ? undefined : previewId, props.describedBy);
  const pending = useRef({ draft, shown, value, onCommit, onDropped: props.onDropped, quantity, locale });
  pending.current = { draft, shown, value, onCommit, onDropped: props.onDropped, quantity, locale };

  const stop = (): void => {
    setDraft(undefined);
    setRejected(false);
  };

  useEffect(() => {
    const left = pending.current;
    if (!readOnly || left.draft === undefined) return;
    setDraft(undefined);
    setRejected(false);
    if (left.draft === left.shown) return;
    const settled = left.draft.trim() === "" ? undefined : parseQuantity(left.draft, left.quantity, left.locale);
    if (left.draft.trim() !== "" && settled === undefined) left.onDropped?.();
    else if (settled !== left.value) left.onCommit(settled, undefined);
  }, [readOnly]);

  const commit = (focus: string | undefined): void => {
    if (readOnly || draft === undefined || draft === shown) return stop();
    if (draft.trim() === "") {
      stop();
      return onCommit(undefined, focus);
    }
    if (parsed === undefined) return setRejected(true);
    stop();
    if (parsed !== value) onCommit(parsed, focus);
  };

  const leave = (event: FocusEvent<HTMLInputElement>): void => {
    const next = event.relatedTarget;
    commit(next instanceof HTMLElement && next.id !== "" ? next.id : undefined);
  };

  const press = (event: KeyboardEvent<HTMLInputElement>): void => {
    if (event.key === "Enter") {
      event.preventDefault();
      commit(id);
    } else if (event.key === "Escape" && draft !== undefined) {
      event.preventDefault();
      stop();
    }
  };

  return (
    <div className="quiz-guess min-w-0">
      <input
        id={id}
        type="text"
        value={typed}
        readOnly={readOnly}
        autoComplete="off"
        spellCheck={false}
        placeholder={text("quiz.task.guessPlaceholder", { example })}
        aria-label={name}
        aria-invalid={rejected}
        aria-describedby={described}
        className="quiz-input quiz-target min-w-0 text-sm tabular-nums"
        onChange={(event) => {
          if (readOnly) return;
          setDraft(event.target.value);
          setRejected(false);
        }}
        onBlur={leave}
        onKeyDown={press}
      />
      {parsed === undefined || rejected ? null : (
        <span id={previewId} className="text-xs text-muted-foreground tabular-nums">
          {text("quiz.task.guessPreview", { value: formatQuantity(parsed, quantity, locale) })}
        </span>
      )}
      {rejected ? (
        <p id={errorId} role="alert" className="m-0 text-xs text-muted-foreground">
          {text(quantity.scale === "logarithmic" ? "quiz.task.guessInvalidPositive" : "quiz.task.guessInvalid", { example })}
        </p>
      ) : null}
    </div>
  );
}

/** ❔️ The symbol of every hint, drawn in a circle: a hint asks, it never points the way. */
export const HINT_SYMBOL = "?";

/** 🏷️ How a hint names a labelled thing (an item, a category, an axis, a quantity) in `locale`: by its short form where
 * it has one, else by its label. */
export function hintName(named: { readonly label: Text; readonly short?: Text }, locale: QuizLocale): string {
  return localized(named.short ?? named.label, locale);
}

/** 🔡️ How a sentence of `locale` writes a quantity's or an axis's name inside it: English lower-cases a name's first
 * letter when its second is a lower-case letter ("Heating demand" → "heating demand"; "U-value", "CO₂", "PV yield"
 * stay), German keeps its capitalised nouns. */
const MID_SENTENCE: { readonly [L in QuizLocale]: (name: string) => string } = {
  en: (name) => {
    const [first = "", second = ""] = [...name];
    return /^\p{Ll}$/u.test(second) ? `${first.toLocaleLowerCase("en")}${name.slice(first.length)}` : name;
  },
  de: (name) => name,
};

/** 📛️ How a hint names a quantity or an axis in the middle of its question in `locale`: by its {@link hintName}, written
 * as the language writes such a name mid-sentence. */
export function hintTerm(named: { readonly label: Text; readonly short?: Text }, locale: QuizLocale): string {
  return MID_SENTENCE[locale](hintName(named, locale));
}

/** ⚖️ The question of a compare hint: the relation the learner's keys claim between `hint.item` and `hint.other`, named
 * by `label`, with the one the keys make higher first — below a factor of 1 or a difference of 0 the items swap roles
 * and the factor inverts — and always in the quantity it is about ({@link hintTerm}), so "higher" never reads as a
 * physical size. A `reversed` claim is asked about by its order alone, without a number; otherwise the factor shows in
 * words ({@link formatCount}) cut toward the claim's own side (down where it understates, up where it overstates) so
 * the question never tips over to the truth; amounts that add up (`quantity.additive`) are asked about together, others
 * as a ratio; a difference (linear scale) shows in the quantity's unit. */
export function compareText(hint: CompareHint, quantity: Pick<Quantity, "label" | "short" | "unit" | "prefixed" | "additive">, label: (id: Slug) => string, text: QuizText, locale: QuizLocale): string {
  const swapped = (hint.factor ?? 1) < 1 || (hint.difference ?? 0) < 0;
  const large = label(swapped ? hint.other : hint.item);
  const small = label(swapped ? hint.item : hint.other);
  const named = hintTerm(quantity, locale);
  const under = hint.verdict === "under";
  if (hint.verdict === "reversed") return text("quiz.task.higher", { larger: large, smaller: small, quantity: named });
  if (hint.difference !== undefined) {
    const difference = formatQuantity(Math.abs(hint.difference), quantity, locale);
    return text(under ? "quiz.task.aboveUnder" : "quiz.task.aboveOver", { large, small, difference, quantity: named });
  }
  const factor = swapped ? 1 / (hint.factor ?? 1) : (hint.factor ?? 1);
  const toward = under ? "down" : "up";
  if (quantity.additive) return text(under ? "quiz.task.sumUnder" : "quiz.task.sumOver", { count: formatCount(factor, locale, toward), small, large, quantity: named });
  return text(under ? "quiz.task.ratioUnder" : "quiz.task.ratioOver", { large, times: formatTimes(factor, locale, toward), small, quantity: named });
}

/** 💡️ A hint beside what it is about, `id` for the controls it describes: the question mark in its circle (shown, not
 * spoken) and the question, marked by kind for the styles. */
export function HintNote(props: { readonly id: string; readonly kind: Hint["kind"]; readonly children: string }): ReactElement {
  return (
    <span id={props.id} className="quiz-hint text-xs" data-hint={props.kind}>
      <span className="quiz-hint-symbol" aria-hidden="true">
        {HINT_SYMBOL}
      </span>
      <span className="quiz-hint-question">{props.children}</span>
    </span>
  );
}

/** 🔑️ What makes a hint the same hint on the next render: everything its question says. */
function hintKey(hint: Hint): string {
  switch (hint.kind) {
    case "compare":
      return `compare:${hint.dimension ?? ""}:${hint.item}:${hint.other}:${hint.factor ?? ""}:${hint.difference ?? ""}:${hint.verdict}`;
    case "profile":
      return `profile:${hint.item}:${hint.category}:${hint.axis}:${hint.other ?? ""}:${hint.above ?? ""}`;
    case "group":
      return `group:${hint.item}:${hint.other}:${hint.together}`;
    case "category":
      return `category:${hint.item}:${hint.category}`;
  }
}

/** 📣️ The polite announcement of the hints that appeared while the task was shown — once, when they appear: a single
 * new hint by its question (`say`), several only by their count, since their questions stand beside their items; the
 * hints a task already has when it is shown are on screen and not announced. */
export function useHintAnnouncement(hints: readonly Hint[], say: (hint: Hint) => string, text: QuizText): Announcement {
  const { announcement, announce } = useAnnouncement();
  const seen = useRef<ReadonlySet<string> | undefined>(undefined);
  const latest = useRef({ hints, say, text });
  latest.current = { hints, say, text };
  const signature = hints.map(hintKey).join("|");
  useEffect(() => {
    const before = seen.current;
    const now = latest.current.hints;
    seen.current = new Set(now.map(hintKey));
    if (before === undefined) return;
    const fresh = now.filter((hint) => !before.has(hintKey(hint)));
    if (fresh.length === 1) announce(latest.current.say(fresh[0]!));
    else if (fresh.length > 1) announce(latest.current.text("quiz.task.hints", { count: fresh.length }));
  }, [signature, announce]);
  return announcement;
}
