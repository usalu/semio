/** 🗂️ Classification: every item goes into one category bin.
 *
 * Keyboard and screen readers use the category select of each item; pointer users drag an item by its grip onto a bin
 * or back onto the pool. Focus follows an item when it moves between bins. Categories with a profile show their spider
 * diagram over the task's axes.
 */

import { useId, type ReactElement } from "react";
import type { ClassificationAnswer, SheetClassificationTask, SheetItem, Slug } from "@semio-tech/quiz";
import { localized } from "../🌐️i18n/🟦️.ts";
import { RadarChart, type RadarAxis } from "../🕸️radar/🟦️.tsx";
import { DragGrip, LiveRegion, elementId, useAnnouncement, useFocusAfterRender, type TaskViewProps } from "../🧩️task/🟦️.tsx";

const POOL_ZONE = "pool";
const CATEGORY_ZONE = "category:";

/** 🗂️ The classification interaction. */
export function ClassificationTaskView(props: TaskViewProps<SheetClassificationTask, ClassificationAnswer>): ReactElement {
  const { task, answer, onAnswer, text, locale } = props;
  const scope = useId();
  const { announcement, announce } = useAnnouncement();
  const focusAfterRender = useFocusAfterRender();
  const assignments = answer?.assignments ?? {};
  const itemLabel = (item: SheetItem): string => localized(item.label, locale);
  const categoryLabel = (id: Slug): string => localized(task.categories.find((category) => category.id === id)?.label ?? { en: id, de: id }, locale);
  const axes: readonly RadarAxis[] = (task.axes ?? []).map((axis) => ({ id: axis.id, label: localized(axis.label, locale), unit: axis.unit, min: axis.min, max: axis.max }));

  const assign = (item: SheetItem, category: Slug | undefined, refocus: boolean): void => {
    if (assignments[item.id] === category) return;
    const next: Record<Slug, Slug> = { ...assignments };
    if (category === undefined) delete next[item.id];
    else next[item.id] = category;
    onAnswer({ kind: "classification", assignments: next });
    announce(category === undefined ? text("quiz.classification.released", { item: itemLabel(item) }) : text("quiz.classification.assigned", { item: itemLabel(item), category: categoryLabel(category) }));
    if (refocus) focusAfterRender(elementId(scope, item.id));
  };

  const drop =
    (item: SheetItem) =>
    (zone: string): void => {
      if (zone === POOL_ZONE) assign(item, undefined, false);
      else if (zone.startsWith(CATEGORY_ZONE)) assign(item, zone.slice(CATEGORY_ZONE.length), false);
    };

  const chip = (item: SheetItem): ReactElement => (
    <li key={item.id} className="quiz-chip" data-quiz-drag="">
      <DragGrip title={text("quiz.classification.drag", { item: itemLabel(item) })} onDrop={drop(item)} />
      <span className="quiz-chip-label">{itemLabel(item)}</span>
      <select
        id={elementId(scope, item.id)}
        aria-label={text("quiz.classification.categoryFor", { item: itemLabel(item) })}
        value={assignments[item.id] ?? ""}
        onChange={(event) => assign(item, event.target.value === "" ? undefined : event.target.value, true)}
      >
        <option value="">{text("quiz.classification.unassigned")}</option>
        {task.categories.map((category) => (
          <option key={category.id} value={category.id}>
            {localized(category.label, locale)}
          </option>
        ))}
      </select>
    </li>
  );

  const pool = task.items.filter((item) => assignments[item.id] === undefined);
  return (
    <div className="quiz-classification">
      <section className="quiz-pool" data-quiz-drop={POOL_ZONE} aria-labelledby={elementId(scope, "pool")}>
        <h3 id={elementId(scope, "pool")}>{text("quiz.classification.pool")}</h3>
        {pool.length === 0 ? <p className="quiz-muted">{text("quiz.classification.poolEmpty")}</p> : <ul className="quiz-chips">{pool.map(chip)}</ul>}
      </section>
      <section aria-labelledby={elementId(scope, "categories")}>
        <h3 id={elementId(scope, "categories")}>{text("quiz.classification.categories")}</h3>
        <div className="quiz-bins">
          {task.categories.map((category) => {
            const members = task.items.filter((item) => assignments[item.id] === category.id);
            const label = localized(category.label, locale);
            return (
              <section key={category.id} className="quiz-bin" data-quiz-drop={`${CATEGORY_ZONE}${category.id}`} aria-labelledby={elementId(scope, "category", category.id)}>
                <h4 id={elementId(scope, "category", category.id)}>{label}</h4>
                {category.description === undefined ? null : <p className="quiz-muted">{localized(category.description, locale)}</p>}
                {category.profile === undefined || axes.length === 0 ? null : <RadarChart name={label} axes={axes} values={category.profile} text={text} locale={locale} />}
                {members.length === 0 ? <p className="quiz-drop-hint">{text("quiz.classification.binEmpty")}</p> : <ul className="quiz-chips">{members.map(chip)}</ul>}
              </section>
            );
          })}
        </div>
      </section>
      <LiveRegion message={announcement} />
    </div>
  );
}
