/** 🃏️ Matching: per dimension, give each item one value card; every card can be used once.
 *
 * Each item has one select per dimension listing every card with who uses it. A card another item uses is a disabled
 * option: a closed select commits every option the arrow keys pass, so browsing must never take a card away — the
 * learner frees it with the other item's remove button first. A pointer drag of a card onto an item's row is one
 * deliberate act and does take it over. The card pool shows every card as available or used by whom, each with the
 * icon of its dimension; items show their own. Each row is a
 * presence anchor. The cards are a grid of equal chips and the items a list of rows, each named by its label and its
 * select by the quantity and the item: the label on the left and the select with the remove button on the right where
 * the list is wide enough, the select below its label where it is not, and in a wide task the cards in a column beside
 * the rows — so every width has a layout of its own and nothing grows with its longest option.
 *
 * Where the challenge hides the keys a dimension has no cards: each item takes a typed guess per dimension instead —
 * the same field as a sorting guess — and the answer is the guesses. On an easy run an item whose card lies far off
 * its value shows a hint beside its label in that dimension, which also describes its select: a question about what
 * its card claims against another item's card in the dimension's quantity, naming both by their short forms ("Are you
 * sure it takes 1,000 × “LED bulb” to add up to the power of 1 × “Floodlight”?"). A
 * task whose time is up shows its answer and takes no change: its controls are described by why, and a select the
 * arrow keys try to change says it again.
 *
 * @see https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element — keyboard steps fire `change`
 * @see ../../🎨️.css — `.quiz-match`, `.quiz-cards`, `.quiz-slot`
 */

import { useId, type ReactElement } from "react";
import type { CompareHint, Hint, MatchingAnswer, SheetDimension, SheetMatchingTask, SheetItem, Slug } from "@semio-tech/quiz";
import { localized } from "../🌐️i18n/🟦️.ts";
import { formatQuantity } from "../📏️quantity/🟦️.ts";
import { DragGrip, GuessField, HintNote, ICON_BUTTON_CLASS, LockedNote, SELECT_ANNOUNCEMENT_DELAY_MS, SELECT_CLASS, compareText, describedBy, elementId, hintName, useHintAnnouncement, type TaskViewProps } from "../🧩️task/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { IconLabel, LiveRegion, useAnnouncement } from "../🪟️chrome/🟦️.tsx";
import { petProp, usePetTopic } from "../🐾️pets/🟦️.tsx";

const SLOT_ZONE = "slot:";

/** 🤝️ The assignments of one dimension after `item` gets `card` (or none), taking the card from its previous holder. */
export function assignCard(assignments: Readonly<Record<Slug, number>>, item: Slug, card: number | undefined): Readonly<Record<Slug, number>> {
  const next = Object.fromEntries(Object.entries(assignments).filter(([holder, index]) => holder !== item && index !== card));
  return card === undefined ? next : { ...next, [item]: card };
}

/** 🔗️ The matching interaction; inside a pets' topic every item's row is marked with the item's topic (the cards of the
 * pool are about no item and stay unmarked). */
export function MatchingTaskView(props: TaskViewProps<SheetMatchingTask, MatchingAnswer>): ReactElement {
  const { task, answer, onAnswer, onDropped, text, locale, hints = [], locked = false } = props;
  const scope = useId();
  const topic = usePetTopic();
  const lockedId = locked ? elementId(scope, "locked") : undefined;
  const { announcement, announce } = useAnnouncement(SELECT_ANNOUNCEMENT_DELAY_MS);
  const assignments = answer?.assignments ?? {};
  const guesses = answer?.guesses ?? {};
  const shown = task.dimensions.some((dimension) => dimension.cards !== undefined);
  const hidden = task.dimensions.filter((dimension) => dimension.cards === undefined);
  const itemLabel = (id: Slug): string => localized(task.items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
  const cardText = (dimension: SheetDimension, index: number): string => formatQuantity(dimension.cards?.[index] ?? 0, dimension.quantity, locale);
  const far = new Map(hints.flatMap((hint): [string, CompareHint][] => (hint.kind === "compare" ? [[`${hint.dimension ?? ""}:${hint.item}`, hint]] : [])));
  const named = (id: Slug): string => {
    const item = task.items.find((candidate) => candidate.id === id);
    return item === undefined ? id : hintName(item, locale);
  };
  const hintText = (hint: CompareHint): string => {
    const dimension = task.dimensions.find((candidate) => candidate.id === hint.dimension);
    return dimension === undefined ? "" : compareText(hint, dimension.quantity, named, text, locale);
  };
  const hinted = useHintAnnouncement(hints, (hint: Hint) => (hint.kind === "compare" ? hintText(hint) : ""), text);
  const guessed = hidden.reduce((sum, dimension) => sum + task.items.filter((item) => guesses[dimension.id]?.[item.id] !== undefined).length, 0);

  const answerOf = (nextAssignments: NonNullable<MatchingAnswer["assignments"]>, nextGuesses: NonNullable<MatchingAnswer["guesses"]>): MatchingAnswer => ({
    kind: "matching",
    ...(shown ? { assignments: nextAssignments } : {}),
    ...(hidden.length > 0 ? { guesses: nextGuesses } : {}),
  });

  const set = (dimension: SheetDimension, item: SheetItem, card: number | undefined): void => {
    const current = assignments[dimension.id] ?? {};
    if (locked) return announce(text("quiz.task.locked"));
    if (current[item.id] === card) return;
    onAnswer(answerOf({ ...assignments, [dimension.id]: assignCard(current, item.id, card) }, guesses));
    announce(card === undefined ? text("quiz.matching.released", { item: itemLabel(item.id) }) : text("quiz.matching.assigned", { value: cardText(dimension, card), item: itemLabel(item.id) }));
  };

  const guess = (dimension: SheetDimension, item: SheetItem, value: number | undefined): void => {
    const rest = Object.fromEntries(Object.entries(guesses[dimension.id] ?? {}).filter(([id]) => id !== item.id));
    onAnswer(answerOf(assignments, { ...guesses, [dimension.id]: value === undefined ? rest : { ...rest, [item.id]: value } }));
    if (!locked)
      announce(value === undefined ? text("quiz.matching.guessCleared", { item: itemLabel(item.id) }) : text("quiz.matching.guessed", { item: itemLabel(item.id), value: formatQuantity(value, dimension.quantity, locale) }));
  };

  return (
    <div className="flex flex-col gap-double">
      <p className="quiz-prose m-0 text-sm leading-normal">{text(shown ? "quiz.matching.hint" : "quiz.matching.guessHint")}</p>
      {hidden.length === 0 ? null : <p className="m-0 text-xs text-muted-foreground tabular-nums">{text("quiz.matching.guessCount", { done: guessed, total: hidden.length * task.items.length })}</p>}
      {task.dimensions.map((dimension) => {
        const cards = dimension.cards;
        const current = assignments[dimension.id] ?? {};
        const holders = new Map(Object.entries(current).map(([item, index]) => [index, item]));
        const quantity = localized(dimension.quantity.label, locale);
        const drop =
          (card: number) =>
          (zone: string): void => {
            const [target, item] = zone.slice(SLOT_ZONE.length).split(":");
            const held = task.items.find((candidate) => candidate.id === item);
            if (zone.startsWith(SLOT_ZONE) && target === dimension.id && held !== undefined) set(dimension, held, card);
          };
        return (
          <section key={dimension.id} className="quiz-match" data-keys={cards === undefined ? "hidden" : undefined} aria-labelledby={elementId(scope, dimension.id)}>
            <h3 id={elementId(scope, dimension.id)} className="m-0 text-sm font-semibold">
              <IconLabel icon={dimension.icon}>{quantity}</IconLabel>
            </h3>
            {cards === undefined ? null : (
              <ul role="list" className="quiz-cards m-0 list-none p-0" aria-label={text("quiz.matching.cards", { quantity })}>
                {cards.map((_, index) => {
                  const holder = holders.get(index);
                  const state = holder === undefined ? text("quiz.matching.free") : text("quiz.matching.used", { item: itemLabel(holder) });
                  return (
                    <li key={index} className="quiz-value border border-normal bg-background py-single pe-double ps-single data-[used]:border-dashed" data-quiz-drag="" data-used={holder === undefined ? undefined : ""}>
                      <DragGrip title={text("quiz.matching.drag", { value: cardText(dimension, index) })} locked={locked} onDrop={drop(index)} />
                      <span className="min-w-0 text-sm font-semibold tabular-nums">
                        <IconLabel icon={dimension.icon} order={index}>
                          {cardText(dimension, index)}
                        </IconLabel>
                      </span>
                      <span className="quiz-value-state text-xs text-muted-foreground" title={state}>
                        {state}
                      </span>
                    </li>
                  );
                })}
              </ul>
            )}
            <ul role="list" className="quiz-rows quiz-slots m-0 list-none p-0 text-sm" aria-labelledby={elementId(scope, dimension.id)}>
              {task.items.map((item, place) => {
                const card = current[item.id];
                const hint = far.get(`${dimension.id}:${item.id}`);
                const hintId = hint === undefined ? undefined : elementId(scope, dimension.id, item.id, "hint");
                return (
                  <li key={item.id} className="quiz-slot" data-quiz-drop={cards === undefined ? undefined : `${SLOT_ZONE}${dimension.id}:${item.id}`} data-presence-anchor={PRESENCE_ANCHORS.item(item.id)} data-pet-prop={petProp(topic, item.id)}>
                    <span className="quiz-slot-label quiz-row-label min-w-0 font-semibold">
                      <IconLabel icon={item.icon} order={place}>
                        {itemLabel(item.id)}
                      </IconLabel>
                      {hint === undefined || hintId === undefined ? null : (
                        <HintNote id={hintId} kind={hint.kind}>
                          {hintText(hint)}
                        </HintNote>
                      )}
                    </span>
                    {cards === undefined ? (
                      <GuessField
                        id={elementId(scope, dimension.id, item.id, "guess")}
                        name={text("quiz.matching.guessFor", { quantity, item: itemLabel(item.id) })}
                        quantity={dimension.quantity}
                        value={guesses[dimension.id]?.[item.id]}
                        readOnly={locked}
                        describedBy={lockedId}
                        onCommit={(value) => guess(dimension, item, value)}
                        onDropped={onDropped}
                        text={text}
                        locale={locale}
                      />
                    ) : (
                      <>
                        <select
                          className={SELECT_CLASS}
                          aria-label={text("quiz.matching.cardFor", { quantity, item: itemLabel(item.id) })}
                          aria-describedby={describedBy(hintId, lockedId)}
                          aria-disabled={locked || undefined}
                          value={card === undefined ? "" : String(card)}
                          onChange={(event) => set(dimension, item, event.target.value === "" ? undefined : Number(event.target.value))}
                        >
                          <option value="">{text("quiz.matching.none")}</option>
                          {cards.map((_, index) => {
                            const holder = holders.get(index);
                            const taken = holder !== undefined && holder !== item.id;
                            return (
                              <option key={index} value={index} disabled={taken}>
                                {taken ? `${cardText(dimension, index)} – ${text("quiz.matching.used", { item: itemLabel(holder) })}` : cardText(dimension, index)}
                              </option>
                            );
                          })}
                        </select>
                        <button type="button" className={ICON_BUTTON_CLASS} aria-label={text("quiz.matching.unassign", { item: itemLabel(item.id) })} aria-describedby={lockedId} aria-disabled={locked || card === undefined} onClick={() => set(dimension, item, undefined)}>
                          ✕
                        </button>
                      </>
                    )}
                  </li>
                );
              })}
            </ul>
          </section>
        );
      })}
      {lockedId === undefined ? null : <LockedNote id={lockedId} text={text} />}
      <LiveRegion announcement={announcement} />
      <LiveRegion announcement={hinted} />
    </div>
  );
}
