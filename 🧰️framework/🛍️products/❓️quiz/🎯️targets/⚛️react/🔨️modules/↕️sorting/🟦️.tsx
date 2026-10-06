/** ↕️ Sorting: order the items ascending by one quantity — by their keys where the challenge shows them, by guessing
 * their values where it hides them.
 *
 * Keys shown: every place shows its key — the presented values as an ascending ladder — so the learner assigns each
 * item the place of its value; the key is read with its place ("Place 3: 2.5 kW"), never as the value of the item
 * standing there. Every item has move-up and move-down buttons (they stay focusable at the ends and
 * announce why nothing moved, and where the item now stands and its key when it moved) and a grip to drag it onto
 * another item's place. Focus stays on the button that moved an item. The presented order only counts as an answer
 * once the learner moves an item or keeps the order explicitly; keeping it says so and moves focus to the list,
 * because its button leaves with the question it answered. On an easy run an item whose key lies far off its value
 * shows a hint beside its label, which also describes its move buttons: a question about what its key claims against
 * another item's in the task's quantity, naming both by their short forms ("Are you sure “Old house” is only 3 times as
 * high in heating demand as “Passive house”?", or "Are you sure “Passive house” is higher in heating demand than “Old
 * house”?" where the keys have the two the wrong way round), never the direction to move it.
 *
 * Keys hidden: every item takes a numeric guess typed with an optional SI prefix and unit (`2 kW`) — the answer. The
 * field shows the unit and what it reads as. A guess counts once it is committed (Enter or leaving the field) and the
 * answer orders the guessed items by it at once, but the rows reorder only on Enter — focus stays on that field — or
 * when focus leaves the list, so Tab from field to field never lands on a row that moved under it. Nothing is moved by
 * hand. How many items have a guess shows above the list.
 *
 * A task whose time is up shows its answer and takes no change; its controls are described by why. Each item is a
 * presence anchor. An item stands on one
 * line — label, key or guess, move buttons — wherever its list is wide enough; in a narrow one the label has the first
 * line to itself and the key or guess and the buttons share the second.
 *
 * @see ../../🎨️.css — `.quiz-sort`, `.quiz-guess`, `.quiz-sort-key`
 */

import { useId, useState, type FocusEvent, type ReactElement } from "react";
import type { CompareHint, Hint, SheetSortingTask, Slug, SortingAnswer } from "@semio-tech/quiz";
import { localized } from "../🌐️i18n/🟦️.ts";
import { BodyButton, IconLabel, LiveRegion, useAnnouncement } from "../🪟️chrome/🟦️.tsx";
import { DragGrip, GuessField, HintNote, ICON_BUTTON_CLASS, LockedNote, compareText, describedBy, elementId, exampleGuess, hintName, useFocusAfterRender, useHintAnnouncement, type TaskViewProps } from "../🧩️task/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { formatQuantity } from "../📏️quantity/🟦️.ts";
import { petProp, usePetTopic } from "../🐾️pets/🟦️.tsx";

const ITEM_ZONE = "item:";

type Guesses = Readonly<Record<Slug, number>>;

/** 🔀️ `order` with the item at `from` moved to `to`. */
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

/** 🪜️ The sorting interaction; inside a pets' topic every item is marked with its own. */
export function SortingTaskView(props: TaskViewProps<SheetSortingTask, SortingAnswer>): ReactElement {
  const { task, answer, onAnswer, onDropped, text, locale, hints = [], locked = false } = props;
  const scope = useId();
  const topic = usePetTopic();
  const { announcement, announce } = useAnnouncement();
  const focusAfterRender = useFocusAfterRender();
  const [held, setHeld] = useState<readonly Slug[] | undefined>(undefined);
  const keys = task.keys;
  const order = answer?.order ?? task.items.map((item) => item.id);
  const shownOrder = held !== undefined && held.length === order.length && held.every((id) => order.includes(id)) ? held : order;
  const guesses: Guesses = answer?.guesses ?? {};
  const label = (id: Slug): string => localized(task.items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
  const quantity = localized(task.quantity.label, locale);
  const keyText = (place: number): string => (keys?.[place] === undefined ? "" : formatQuantity(keys[place], task.quantity, locale));
  const far = new Map(hints.flatMap((hint): [Slug, CompareHint][] => (hint.kind === "compare" ? [[hint.item, hint]] : [])));
  const named = (id: Slug): string => {
    const item = task.items.find((candidate) => candidate.id === id);
    return item === undefined ? id : hintName(item, locale);
  };
  const hintText = (hint: CompareHint): string => compareText(hint, task.quantity, named, text, locale);
  const hinted = useHintAnnouncement(hints, (hint: Hint) => (hint.kind === "compare" ? hintText(hint) : ""), text);
  const guessed = task.items.filter((item) => Object.hasOwn(guesses, item.id)).length;

  const listId = elementId(scope, "list");
  const lockedId = locked ? elementId(scope, "locked") : undefined;

  const move = (from: number, to: number, button?: "up" | "down"): void => {
    const id = order[from];
    if (id === undefined || from === to) return;
    const next = locked ? order : reordered(order, from, to);
    if (locked) {
      if (button !== undefined) announce(text("quiz.task.locked"));
    } else if (next === order) {
      if (button !== undefined) announce(text(button === "up" ? "quiz.sorting.first" : "quiz.sorting.last", { item: label(id) }));
    } else {
      const position = next.indexOf(id);
      onAnswer(answerOf(next, {}));
      announce(text("quiz.sorting.moved", { item: label(id), position: position + 1, total: next.length, value: keyText(position) }));
    }
    if (button !== undefined) focusAfterRender(elementId(scope, id, button));
  };

  const guess = (id: Slug, value: number | undefined, focus: string | undefined): void => {
    const next: Guesses = value === undefined ? without(guesses, id) : { ...without(guesses, id), [id]: value };
    const sorted = ordered(order, next);
    onAnswer(answerOf(sorted, next));
    if (locked) return;
    announce(value === undefined ? text("quiz.sorting.guessCleared", { item: label(id) }) : text("quiz.sorting.guessed", { item: label(id), value: formatQuantity(value, task.quantity, locale), position: sorted.indexOf(id) + 1, total: sorted.length }));
    const target = focus === undefined ? null : document.getElementById(focus);
    if (focus !== elementId(scope, id, "guess") && target !== null && document.getElementById(listId)?.contains(target) === true) return setHeld(shownOrder);
    setHeld(undefined);
    if (sorted !== order && focus !== undefined) focusAfterRender(focus);
  };

  const release = (event: FocusEvent<HTMLOListElement>): void => {
    if (held !== undefined && !(event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget))) setHeld(undefined);
  };

  const keep = (): void => {
    onAnswer(answerOf(order, {}));
    announce(text("quiz.sorting.kept"));
    focusAfterRender(listId);
  };

  return (
    <div className="flex flex-col gap-double">
      <p className="quiz-prose m-0 text-sm leading-normal">{text("quiz.sorting.hint", { quantity })}</p>
      <p className="quiz-prose m-0 text-xs leading-normal text-muted-foreground">{keys === undefined ? text("quiz.sorting.guessHint", { example: exampleGuess(task.quantity) }) : text("quiz.sorting.keysHint")}</p>
      {keys !== undefined && answer === undefined && !locked ? (
        <div className="flex flex-col gap-single border-l-2 border-normal ps-double">
          <p className="m-0 text-xs text-muted-foreground">{text("quiz.sorting.keepHint")}</p>
          <BodyButton onClick={keep}>{text("quiz.sorting.keep")}</BodyButton>
        </div>
      ) : null}
      {keys === undefined ? <p className="m-0 text-xs text-muted-foreground tabular-nums">{text("quiz.sorting.guessCount", { done: guessed, total: task.items.length })}</p> : null}
      <p className="m-0 text-xs text-muted-foreground" aria-hidden="true">
        ▲ {text("quiz.sorting.smallest")}
      </p>
      <ol id={listId} role="list" tabIndex={-1} className="quiz-rows m-0 flex list-none flex-col gap-single p-0 outline-none" aria-label={text("quiz.sorting.list", { quantity })} onBlur={release}>
        {shownOrder.map((id, index) => {
          const hint = far.get(id);
          const hintId = hint === undefined ? undefined : elementId(scope, id, "hint");
          const described = describedBy(hintId, lockedId);
          return (
            <li
              data-icon-host=""
              key={id}
              className="quiz-sort border border-normal bg-background px-single py-single"
              data-quiz-item={id}
              data-presence-anchor={PRESENCE_ANCHORS.item(id)}
              data-quiz-drag={keys === undefined ? undefined : ""}
              data-quiz-drop={keys === undefined ? undefined : `${ITEM_ZONE}${id}`}
              data-pet-prop={petProp(topic, id)}
            >
              {keys === undefined ? null : <DragGrip title={text("quiz.sorting.drag", { item: label(id) })} locked={locked} onDrop={(zone) => zone.startsWith(ITEM_ZONE) && move(index, order.indexOf(zone.slice(ITEM_ZONE.length)))} />}
              <span className="quiz-sort-place text-center text-sm font-semibold text-muted-foreground tabular-nums" aria-hidden={keys === undefined ? "true" : undefined}>
                <span aria-hidden="true">{index + 1}</span>
                {keys === undefined ? null : <span className="sr-only">{text("quiz.sorting.place", { position: index + 1, value: keyText(index) })}</span>}
              </span>
              <span className="quiz-sort-label quiz-row-label min-w-0 text-sm">
                <IconLabel icon={task.items.find((item) => item.id === id)?.icon}>
                  {label(id)}
                </IconLabel>
                {hint === undefined || hintId === undefined ? null : (
                  <HintNote id={hintId} kind={hint.kind}>
                    {hintText(hint)}
                  </HintNote>
                )}
              </span>
              {keys === undefined ? (
                <GuessField
                  id={elementId(scope, id, "guess")}
                  name={text("quiz.sorting.guess", { item: label(id) })}
                  quantity={task.quantity}
                  value={guesses[id]}
                  readOnly={locked}
                  describedBy={lockedId}
                  onCommit={(value, focus) => guess(id, value, focus)}
                  onDropped={onDropped}
                  text={text}
                  locale={locale}
                />
              ) : (
                <>
                  <span className="quiz-sort-key text-sm font-semibold tabular-nums" aria-hidden="true">
                    {keyText(index)}
                  </span>
                  <button type="button" id={elementId(scope, id, "up")} className={`quiz-sort-up ${ICON_BUTTON_CLASS}`} aria-label={text("quiz.sorting.up", { item: label(id) })} aria-describedby={described} aria-disabled={locked || index === 0} onClick={() => move(index, index - 1, "up")}>
                    ↑
                  </button>
                  <button type="button" id={elementId(scope, id, "down")} className={`quiz-sort-down ${ICON_BUTTON_CLASS}`} aria-label={text("quiz.sorting.down", { item: label(id) })} aria-describedby={described} aria-disabled={locked || index === order.length - 1} onClick={() => move(index, index + 1, "down")}>
                    ↓
                  </button>
                </>
              )}
            </li>
          );
        })}
      </ol>
      <p className="m-0 text-xs text-muted-foreground" aria-hidden="true">
        ▼ {text("quiz.sorting.largest")}
      </p>
      {lockedId === undefined ? null : <LockedNote id={lockedId} text={text} />}
      <LiveRegion announcement={announcement} />
      <LiveRegion announcement={hinted} />
    </div>
  );
}
