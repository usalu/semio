/** ↕️ Sorting: order the items ascending by one quantity.
 *
 * Every item has move-up and move-down buttons (they stay focusable at the ends and announce why nothing moved) and a
 * grip to drag it onto another item's place. Focus stays on the button that moved an item. The presented order only
 * counts as an answer once the learner moves an item or keeps the order explicitly.
 */

import { useId, type ReactElement } from "react";
import type { SheetSortingTask, Slug, SortingAnswer } from "@semio-tech/quiz";
import { localized } from "../🌐️i18n/🟦️.ts";
import { DragGrip, LiveRegion, elementId, useAnnouncement, useFocusAfterRender, type TaskViewProps } from "../🧩️task/🟦️.tsx";

const ITEM_ZONE = "item:";

/** ↕️ `order` with the item at `from` moved to `to`. */
export function reordered(order: readonly Slug[], from: number, to: number): readonly Slug[] {
  if (from === to || from < 0 || to < 0 || from >= order.length || to >= order.length) return order;
  const next = [...order];
  const [moved] = next.splice(from, 1);
  next.splice(to, 0, moved!);
  return next;
}

/** ↕️ The sorting interaction. */
export function SortingTaskView(props: TaskViewProps<SheetSortingTask, SortingAnswer>): ReactElement {
  const { task, answer, onAnswer, text, locale } = props;
  const scope = useId();
  const { announcement, announce } = useAnnouncement();
  const focusAfterRender = useFocusAfterRender();
  const order = answer?.order ?? task.items.map((item) => item.id);
  const label = (id: Slug): string => localized(task.items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
  const quantity = localized(task.quantity.label, locale);

  const move = (from: number, to: number, button?: "up" | "down"): void => {
    const id = order[from];
    if (id === undefined) return;
    const next = reordered(order, from, to);
    if (next !== order) onAnswer({ kind: "sorting", order: next });
    announce(text("quiz.sorting.moved", { item: label(id), position: next.indexOf(id) + 1, total: next.length }));
    if (button !== undefined) focusAfterRender(elementId(scope, id, button));
  };

  return (
    <div className="quiz-sorting">
      <p>{text("quiz.sorting.hint", { quantity })}</p>
      {answer === undefined ? (
        <div className="quiz-keep">
          <p className="quiz-muted">{text("quiz.sorting.keepHint")}</p>
          <button type="button" className="quiz-button" onClick={() => onAnswer({ kind: "sorting", order })}>
            {text("quiz.sorting.keep")}
          </button>
        </div>
      ) : null}
      <p className="quiz-order-end" aria-hidden="true">
        ▲ {text("quiz.sorting.smallest")}
      </p>
      <ol className="quiz-order" aria-label={text("quiz.sorting.list", { quantity })}>
        {order.map((id, index) => (
          <li key={id} className="quiz-order-item" data-quiz-drag="" data-quiz-drop={`${ITEM_ZONE}${id}`}>
            <DragGrip title={text("quiz.sorting.drag", { item: label(id) })} onDrop={(zone) => zone.startsWith(ITEM_ZONE) && move(index, order.indexOf(zone.slice(ITEM_ZONE.length)))} />
            <span className="quiz-order-position" aria-hidden="true">
              {index + 1}
            </span>
            <span className="quiz-order-label">{label(id)}</span>
            <button type="button" id={elementId(scope, id, "up")} className="quiz-icon-button" aria-label={text("quiz.sorting.up", { item: label(id) })} aria-disabled={index === 0} onClick={() => move(index, index - 1, "up")}>
              ↑
            </button>
            <button type="button" id={elementId(scope, id, "down")} className="quiz-icon-button" aria-label={text("quiz.sorting.down", { item: label(id) })} aria-disabled={index === order.length - 1} onClick={() => move(index, index + 1, "down")}>
              ↓
            </button>
          </li>
        ))}
      </ol>
      <p className="quiz-order-end" aria-hidden="true">
        ▼ {text("quiz.sorting.largest")}
      </p>
      <LiveRegion message={announcement} />
    </div>
  );
}
