/** 🃏️ Matching: per dimension, give each item one value card; every card can be used once.
 *
 * Each item has one select per dimension listing every card with who uses it. A card another item uses is a disabled
 * option: a closed select commits every option the arrow keys pass, so browsing must never take a card away — the
 * learner frees it with the other item's remove button first. A pointer drag of a card onto an item's row is one
 * deliberate act and does take it over. The card pool shows every card as available or used by whom, each with the
 * icon of its dimension; items show their own. Each row is a
 * presence anchor. The table's columns are fixed shares of its width — the item, the card, the remove
 * button — so it fits its card at every width and text size instead of growing with its longest option.
 *
 * @see https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element — keyboard steps fire `change`
 */

import { useId, type ReactElement } from "react";
import type { MatchingAnswer, SheetDimension, SheetMatchingTask, SheetItem, Slug } from "@semio-tech/quiz";
import { localized } from "../🌐️i18n/🟦️.ts";
import { formatQuantity } from "../📏️quantity/🟦️.ts";
import { DragGrip, ICON_BUTTON_CLASS, LiveRegion, SELECT_ANNOUNCEMENT_DELAY_MS, SELECT_CLASS, elementId, useAnnouncement, type TaskViewProps } from "../🧩️task/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { IconLabel } from "../🪟️chrome/🟦️.tsx";

const EDGE = "border-b border-normal px-single py-single text-left align-middle";
const HEAD = `${EDGE} border-b-2 font-semibold`;
const ROW_HEAD = `${EDGE} font-semibold`;
const SLOT_SELECT = `${SELECT_CLASS} w-full min-w-0`;

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
  const { announcement, announce } = useAnnouncement(SELECT_ANNOUNCEMENT_DELAY_MS);
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
    <div className="flex flex-col gap-double">
      <p className="m-0 text-sm leading-normal">{text("quiz.matching.hint")}</p>
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
          <section key={dimension.id} className="flex flex-col gap-single" aria-labelledby={elementId(scope, dimension.id)}>
            <h3 id={elementId(scope, dimension.id)} className="m-0 text-sm font-semibold">
              <IconLabel icon={dimension.icon}>{quantity}</IconLabel>
            </h3>
            <ul role="list" className="m-0 flex list-none flex-wrap gap-single p-0" aria-label={text("quiz.matching.cards", { quantity })}>
              {dimension.cards.map((_, index) => {
                const holder = holders.get(index);
                return (
                  <li
                    key={index}
                    className="inline-grid grid-cols-[auto_auto] items-center gap-x-single border border-normal bg-background py-single pe-double ps-single data-[used]:border-dashed"
                    data-quiz-drag=""
                    data-used={holder === undefined ? undefined : ""}
                  >
                    <DragGrip title={text("quiz.matching.drag", { value: cardText(dimension, index) })} onDrop={drop(index)} />
                    <span className="text-sm font-semibold tabular-nums">
                      <IconLabel icon={dimension.icon} order={index}>
                        {cardText(dimension, index)}
                      </IconLabel>
                    </span>
                    <span className="col-start-2 text-xs text-muted-foreground">{holder === undefined ? text("quiz.matching.free") : text("quiz.matching.used", { item: itemLabel(holder) })}</span>
                  </li>
                );
              })}
            </ul>
            <table className="w-full table-fixed border-collapse text-sm" aria-labelledby={elementId(scope, dimension.id)}>
              <colgroup>
                <col className="w-[38%]" />
                <col />
                <col className="w-[3.25em]" />
              </colgroup>
              <thead>
                <tr>
                  <th scope="col" className={HEAD}>
                    {text("quiz.matching.item")}
                  </th>
                  <th scope="col" className={HEAD}>
                    {quantity}
                  </th>
                  <th scope="col" className={HEAD}>
                    <span className="sr-only">{text("quiz.matching.actions")}</span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {task.items.map((item, place) => {
                  const card = current[item.id];
                  return (
                    <tr key={item.id} data-quiz-drop={`${SLOT_ZONE}${dimension.id}:${item.id}`} data-presence-anchor={PRESENCE_ANCHORS.item(item.id)}>
                      <th scope="row" className={ROW_HEAD}>
                        <IconLabel icon={item.icon} order={place}>
                          {itemLabel(item.id)}
                        </IconLabel>
                      </th>
                      <td className={EDGE}>
                        <select
                          className={SLOT_SELECT}
                          aria-label={text("quiz.matching.cardFor", { quantity, item: itemLabel(item.id) })}
                          value={card === undefined ? "" : String(card)}
                          onChange={(event) => set(dimension, item, event.target.value === "" ? undefined : Number(event.target.value))}
                        >
                          <option value="">{text("quiz.matching.none")}</option>
                          {dimension.cards.map((_, index) => {
                            const holder = holders.get(index);
                            const taken = holder !== undefined && holder !== item.id;
                            return (
                              <option key={index} value={index} disabled={taken}>
                                {taken ? `${cardText(dimension, index)} – ${text("quiz.matching.used", { item: itemLabel(holder) })}` : cardText(dimension, index)}
                              </option>
                            );
                          })}
                        </select>
                      </td>
                      <td className={EDGE}>
                        <button type="button" className={ICON_BUTTON_CLASS} aria-label={text("quiz.matching.unassign", { item: itemLabel(item.id) })} aria-disabled={card === undefined} onClick={() => set(dimension, item, undefined)}>
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
      <LiveRegion announcement={announcement} />
    </div>
  );
}
