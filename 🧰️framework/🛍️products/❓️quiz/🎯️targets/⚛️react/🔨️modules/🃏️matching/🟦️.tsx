/** 🃏️ Matching: per dimension, give each item one value card; every card can be used once.
 *
 * Each item has one select per dimension listing every card with who uses it; choosing a used card takes it over, so a
 * card is never used twice. A button unassigns. The card pool shows every card as available or used by whom, and
 * pointer users drag a card by its grip onto an item's row.
 */

import { useId, type ReactElement } from "react";
import type { MatchingAnswer, SheetDimension, SheetMatchingTask, SheetItem, Slug } from "@semio-tech/quiz";
import { localized } from "../🌐️i18n/🟦️.ts";
import { formatQuantity } from "../📏️quantity/🟦️.ts";
import { DragGrip, LiveRegion, elementId, useAnnouncement, type TaskViewProps } from "../🧩️task/🟦️.tsx";

const SLOT_ZONE = "slot:";

/** 🃏️ The assignments of one dimension after `item` gets `card` (or none), taking the card from its previous holder. */
export function assignCard(assignments: Readonly<Record<Slug, number>>, item: Slug, card: number | undefined): Readonly<Record<Slug, number>> {
  const next = Object.fromEntries(Object.entries(assignments).filter(([holder, index]) => holder !== item && index !== card));
  return card === undefined ? next : { ...next, [item]: card };
}

/** 🃏️ The matching interaction. */
export function MatchingTaskView(props: TaskViewProps<SheetMatchingTask, MatchingAnswer>): ReactElement {
  const { task, answer, onAnswer, text, locale } = props;
  const scope = useId();
  const { announcement, announce } = useAnnouncement();
  const assignments = answer?.assignments ?? {};
  const itemLabel = (id: Slug): string => localized(task.items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
  const cardText = (dimension: SheetDimension, index: number): string => formatQuantity(dimension.cards[index] ?? 0, dimension.quantity, locale);

  const set = (dimension: SheetDimension, item: SheetItem, card: number | undefined): void => {
    const current = assignments[dimension.id] ?? {};
    if (current[item.id] === card) return;
    onAnswer({ kind: "matching", assignments: { ...assignments, [dimension.id]: assignCard(current, item.id, card) } });
    announce(card === undefined ? text("quiz.matching.released", { item: itemLabel(item.id) }) : text("quiz.matching.assigned", { value: cardText(dimension, card), item: itemLabel(item.id) }));
  };

  return (
    <div className="quiz-matching">
      <p>{text("quiz.matching.hint")}</p>
      {task.dimensions.map((dimension) => {
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
          <section key={dimension.id} className="quiz-dimension" aria-labelledby={elementId(scope, dimension.id)}>
            <h3 id={elementId(scope, dimension.id)}>{quantity}</h3>
            <ul className="quiz-cards" aria-label={text("quiz.matching.cards", { quantity })}>
              {dimension.cards.map((_, index) => {
                const holder = holders.get(index);
                return (
                  <li key={index} className="quiz-card" data-quiz-drag="" data-used={holder === undefined ? undefined : ""}>
                    <DragGrip title={text("quiz.matching.drag", { value: cardText(dimension, index) })} onDrop={drop(index)} />
                    <span className="quiz-card-value">{cardText(dimension, index)}</span>
                    <span className="quiz-card-state">{holder === undefined ? text("quiz.matching.free") : text("quiz.matching.used", { item: itemLabel(holder) })}</span>
                  </li>
                );
              })}
            </ul>
            <table className="quiz-slots">
              <thead>
                <tr>
                  <th scope="col">{text("quiz.matching.item")}</th>
                  <th scope="col">{quantity}</th>
                  <th scope="col">
                    <span className="quiz-visually-hidden">{text("quiz.matching.none")}</span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {task.items.map((item) => {
                  const card = current[item.id];
                  return (
                    <tr key={item.id} data-quiz-drop={`${SLOT_ZONE}${dimension.id}:${item.id}`}>
                      <th scope="row">{itemLabel(item.id)}</th>
                      <td>
                        <select
                          aria-label={text("quiz.matching.cardFor", { quantity, item: itemLabel(item.id) })}
                          value={card === undefined ? "" : String(card)}
                          onChange={(event) => set(dimension, item, event.target.value === "" ? undefined : Number(event.target.value))}
                        >
                          <option value="">{text("quiz.matching.none")}</option>
                          {dimension.cards.map((_, index) => {
                            const holder = holders.get(index);
                            return (
                              <option key={index} value={index}>
                                {holder === undefined || holder === item.id ? cardText(dimension, index) : `${cardText(dimension, index)} – ${text("quiz.matching.used", { item: itemLabel(holder) })}`}
                              </option>
                            );
                          })}
                        </select>
                      </td>
                      <td>
                        <button type="button" className="quiz-icon-button" aria-label={text("quiz.matching.unassign", { item: itemLabel(item.id) })} aria-disabled={card === undefined} onClick={() => set(dimension, item, undefined)}>
                          ✕
                        </button>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </section>
        );
      })}
      <LiveRegion message={announcement} />
    </div>
  );
}
