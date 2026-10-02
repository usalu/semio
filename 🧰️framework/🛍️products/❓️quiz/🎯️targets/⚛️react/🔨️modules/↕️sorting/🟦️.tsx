/** ↕️ Sorting: order the items ascending by one quantity.
 *
 * Every item has move-up and move-down buttons (they stay focusable at the ends and announce why nothing moved) and a
 * grip to drag it onto another item's place. Focus stays on the button that moved an item. Every item also takes a
 * numeric guess typed with an optional SI prefix and unit (`2 kW`): the field shows the unit and what it reads as, and
 * once a guess is committed (Enter or leaving the field) the items with a guess reorder among their places by it.
 * Moving a guessed item by hand removes its guess, so the guesses and the order never disagree. The presented order
 * only counts as an answer once the learner moves an item, guesses or keeps the order explicitly; keeping it says so
 * and moves focus to the list, because its button leaves with the question it answered. Each item is a presence
 * anchor.
 */

import { useId, useState, type FocusEvent, type KeyboardEvent, type ReactElement } from "react";
import type { Quantity, SheetSortingTask, Slug, SortingAnswer } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { BodyButton, IconLabel } from "../🪟️chrome/🟦️.tsx";
import { DragGrip, ICON_BUTTON_CLASS, LiveRegion, elementId, useAnnouncement, useFocusAfterRender, type TaskViewProps } from "../🧩️task/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { formatQuantity, parseQuantity, withUnit } from "../📏️quantity/🟦️.ts";

const ITEM_ZONE = "item:";

type Guesses = Readonly<Record<Slug, number>>;

/** ↕️ `order` with the item at `from` moved to `to`. */
export function reordered(order: readonly Slug[], from: number, to: number): readonly Slug[] {
  if (from === to || from < 0 || to < 0 || from >= order.length || to >= order.length) return order;
  const next = [...order];
  const [moved] = next.splice(from, 1);
  next.splice(to, 0, moved!);
  return next;
}

/** 🔢️ `order` with the items that have a guess in ascending guess order (ties keep their order) among the places those
 * items occupy; the items without a guess stay where they are. */
export function ordered(order: readonly Slug[], guesses: Guesses): readonly Slug[] {
  const places = order.flatMap((id, index) => (Object.hasOwn(guesses, id) ? [index] : []));
  const sorted = places.map((place) => order[place]!).sort((left, right) => guesses[left]! - guesses[right]!);
  if (places.every((place, rank) => order[place] === sorted[rank])) return order;
  const next = [...order];
  for (const [rank, place] of places.entries()) next[place] = sorted[rank]!;
  return next;
}

/** 🧾️ The answer for `order` and `guesses`; no guesses leave the member out. */
function answerOf(order: readonly Slug[], guesses: Guesses): SortingAnswer {
  return Object.keys(guesses).length === 0 ? { kind: "sorting", order } : { kind: "sorting", order, guesses };
}

/** ✂️ `guesses` without the guess of item `id`. */
function without(guesses: Guesses, id: Slug): Guesses {
  return Object.fromEntries(Object.entries(guesses).filter(([item]) => item !== id));
}

/** 🧪️ An example guess in the unit of `quantity`, such as `2 kW`. */
function exampleGuess(quantity: Quantity): string {
  return withUnit("2", quantity.prefixed ? `k${quantity.unit}` : quantity.unit);
}

/** 🔢️ The guess field of one item: the unit is shown beside what is typed, and what it reads as once it has a prefix.
 * It commits on Enter and on leaving the field; an unreadable text stays, flagged, until it is fixed or emptied. */
function GuessField(props: {
  readonly id: string;
  readonly label: string;
  readonly quantity: Quantity;
  readonly value: number | undefined;
  readonly onCommit: (value: number | undefined, focus: string | undefined) => void;
  readonly text: QuizText;
  readonly locale: QuizLocale;
}): ReactElement {
  const { id, label, quantity, value, onCommit, text, locale } = props;
  const [draft, setDraft] = useState<string | undefined>(undefined);
  const [rejected, setRejected] = useState(false);
  const shown = value === undefined ? "" : formatQuantity(value, quantity, locale);
  const typed = draft ?? shown;
  const parsed = draft === undefined || draft.trim() === "" ? undefined : parseQuantity(draft, quantity, locale);
  const example = exampleGuess(quantity);
  const errorId = `${id}--error`;
  const previewId = `${id}--preview`;

  const stop = (): void => {
    setDraft(undefined);
    setRejected(false);
  };

  const commit = (focus: string | undefined): void => {
    if (draft === undefined || draft === shown) return stop();
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
    <div className="flex min-w-0 flex-wrap items-center gap-x-double gap-y-single">
      <input
        id={id}
        type="text"
        value={typed}
        autoComplete="off"
        spellCheck={false}
        placeholder={text("quiz.sorting.guessPlaceholder", { example })}
        aria-label={text("quiz.sorting.guess", { item: label })}
        aria-invalid={rejected}
        aria-describedby={rejected ? errorId : parsed === undefined ? undefined : previewId}
        className="quiz-input w-40 max-w-full text-sm tabular-nums"
        onChange={(event) => {
          setDraft(event.target.value);
          setRejected(false);
        }}
        onBlur={leave}
        onKeyDown={press}
      />
      {parsed === undefined || rejected ? null : (
        <span id={previewId} className="text-xs text-muted-foreground tabular-nums">
          {text("quiz.sorting.guessPreview", { value: formatQuantity(parsed, quantity, locale) })}
        </span>
      )}
      {rejected ? (
        <p id={errorId} role="alert" className="m-0 text-xs text-muted-foreground">
          {text(quantity.scale === "logarithmic" ? "quiz.sorting.guessInvalidPositive" : "quiz.sorting.guessInvalid", { example })}
        </p>
      ) : null}
    </div>
  );
}

/** ↕️ The sorting interaction. */
export function SortingTaskView(props: TaskViewProps<SheetSortingTask, SortingAnswer>): ReactElement {
  const { task, answer, onAnswer, text, locale } = props;
  const scope = useId();
  const { announcement, announce } = useAnnouncement();
  const focusAfterRender = useFocusAfterRender();
  const order = answer?.order ?? task.items.map((item) => item.id);
  const guesses: Guesses = answer?.guesses ?? {};
  const label = (id: Slug): string => localized(task.items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
  const quantity = localized(task.quantity.label, locale);

  const listId = elementId(scope, "list");

  const move = (from: number, to: number, button?: "up" | "down"): void => {
    const id = order[from];
    if (id === undefined || from === to) return;
    const next = reordered(order, from, to);
    if (next === order) {
      if (button !== undefined) announce(text(button === "up" ? "quiz.sorting.first" : "quiz.sorting.last", { item: label(id) }));
    } else {
      onAnswer(answerOf(next, without(guesses, id)));
      announce(text(!Object.hasOwn(guesses, id) ? "quiz.sorting.moved" : "quiz.sorting.moveClears", { item: label(id), position: next.indexOf(id) + 1, total: next.length }));
    }
    if (button !== undefined) focusAfterRender(elementId(scope, id, button));
  };

  const guess = (id: Slug, value: number | undefined, focus: string | undefined): void => {
    const next: Guesses = value === undefined ? without(guesses, id) : { ...without(guesses, id), [id]: value };
    const sorted = ordered(order, next);
    onAnswer(answerOf(sorted, next));
    announce(value === undefined ? text("quiz.sorting.guessCleared", { item: label(id) }) : text("quiz.sorting.guessed", { item: label(id), value: formatQuantity(value, task.quantity, locale), position: sorted.indexOf(id) + 1, total: sorted.length }));
    if (sorted !== order && focus !== undefined) focusAfterRender(focus);
  };

  const keep = (): void => {
    onAnswer(answerOf(order, guesses));
    announce(text("quiz.sorting.kept"));
    focusAfterRender(listId);
  };

  return (
    <div className="flex flex-col gap-double">
      <p className="m-0 text-sm leading-normal">{text("quiz.sorting.hint", { quantity })}</p>
      <p className="m-0 text-xs leading-normal text-muted-foreground">{text("quiz.sorting.guessHint", { example: exampleGuess(task.quantity) })}</p>
      {answer === undefined ? (
        <div className="flex flex-col gap-single border-l-2 border-normal ps-double">
          <p className="m-0 text-xs text-muted-foreground">{text("quiz.sorting.keepHint")}</p>
          <BodyButton onClick={keep}>{text("quiz.sorting.keep")}</BodyButton>
        </div>
      ) : null}
      <p className="m-0 text-xs text-muted-foreground" aria-hidden="true">
        ▲ {text("quiz.sorting.smallest")}
      </p>
      <ol id={listId} role="list" tabIndex={-1} className="m-0 flex list-none flex-col gap-single p-0 outline-none" aria-label={text("quiz.sorting.list", { quantity })}>
        {order.map((id, index) => (
          <li
            key={id}
            className="grid grid-cols-[auto_2em_minmax(0,1fr)_auto_auto] items-center gap-single border border-normal bg-background px-single py-single"
            data-quiz-drag=""
            data-quiz-item={id}
            data-presence-anchor={PRESENCE_ANCHORS.item(id)}
            data-quiz-drop={`${ITEM_ZONE}${id}`}
          >
            <DragGrip title={text("quiz.sorting.drag", { item: label(id) })} onDrop={(zone) => zone.startsWith(ITEM_ZONE) && move(index, order.indexOf(zone.slice(ITEM_ZONE.length)))} />
            <span className="text-center text-sm font-semibold text-muted-foreground tabular-nums" aria-hidden="true">
              {index + 1}
            </span>
            <span className="min-w-0 text-sm">
              <IconLabel icon={task.items.find((item) => item.id === id)?.icon} order={index}>
                {label(id)}
              </IconLabel>
            </span>
            <button type="button" id={elementId(scope, id, "up")} className={ICON_BUTTON_CLASS} aria-label={text("quiz.sorting.up", { item: label(id) })} aria-disabled={index === 0} onClick={() => move(index, index - 1, "up")}>
              ↑
            </button>
            <button type="button" id={elementId(scope, id, "down")} className={ICON_BUTTON_CLASS} aria-label={text("quiz.sorting.down", { item: label(id) })} aria-disabled={index === order.length - 1} onClick={() => move(index, index + 1, "down")}>
              ↓
            </button>
            <div className="col-span-3 col-start-3">
              <GuessField id={elementId(scope, id, "guess")} label={label(id)} quantity={task.quantity} value={guesses[id]} onCommit={(value, focus) => guess(id, value, focus)} text={text} locale={locale} />
            </div>
          </li>
        ))}
      </ol>
      <p className="m-0 text-xs text-muted-foreground" aria-hidden="true">
        ▼ {text("quiz.sorting.largest")}
      </p>
      <LiveRegion announcement={announcement} />
    </div>
  );
}
