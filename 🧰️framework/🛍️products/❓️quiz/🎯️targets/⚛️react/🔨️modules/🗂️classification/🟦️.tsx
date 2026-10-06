/** 🗂️ Classification: every item goes into one category bin.
 *
 * Keyboard and screen readers use the category select of each item; pointer users drag an item by its grip onto a bin
 * or back onto the pool. Focus follows an item when it moves between bins, and only where it ends up is announced: a
 * closed select moves the item with every option the arrow keys pass. Categories with a profile show their spider
 * diagram over the task's axes. Items and bins show their icons before their labels. Items and bins are presence anchors, so the others' pointers land on the same item
 * wherever it sits on each sheet. An item stands on one line — its label on the left, its select on the right — wherever
 * its list is wide enough, and on two where it is not; the bins share the width in as many columns as fit, and a wide
 * task shows the pool beside them.
 *
 * Where the challenge hides the keys the categories have no descriptions and their diagrams show the shape of their
 * profiles only: every value is its share of its axis range, without units, ranges or numbers. On an easy run an item put
 * in a category that is not its own shows a question beside its label, which also describes its select, and never
 * says which of the two items it names is the wrong one: whether it is higher (or lower) than an item placed rightly on the
 * axis it misses most, or else whether it fits the profile it was given, with that profile's value on that axis; whether
 * it belongs with an item of another category it was put beside, or apart
 * from one of its own category it was put elsewhere; or else whether it belongs to the category it was given, with
 * that category's description. A task whose time is up shows its answer and takes no change: its selects are described by why, and say it again when the arrow keys try to
 * change them.
 *
 * @see ../../🎨️.css — `.quiz-classify`, `.quiz-chip`
 */

import { useId, type ReactElement } from "react";
import type { Category, ClassificationAnswer, Hint, SheetClassificationTask, SheetItem, Slug } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { RadarChart, type RadarAxis } from "../🕸️radar/🟦️.tsx";
import { IconLabel, LiveRegion, useAnnouncement } from "../🪟️chrome/🟦️.tsx";
import { DROP_ZONE_CLASS, DragGrip, HintNote, LockedNote, SELECT_ANNOUNCEMENT_DELAY_MS, SELECT_CLASS, describedBy, elementId, hintName, hintTerm, useFocusAfterRender, useHintAnnouncement, type TaskViewProps } from "../🧩️task/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { petProp, usePetTopic } from "../🐾️pets/🟦️.tsx";
import { formatNumber, withMinusSign, withUnit } from "../📏️quantity/🟦️.ts";

const POOL_ZONE = "pool";
const CATEGORY_ZONE = "category:";

/** ❓️ The question of a classification hint in `locale`, naming items and categories by their short forms
 * ({@link hintName}), items quoted, and axes as written mid-sentence ({@link hintTerm}): a profile hint asks whether
 * the item is higher (or lower) than another item on the axis
 * where it names one, else gives the assigned category's profile value on the axis with the axis unit and a true minus
 * sign; a category hint gives the category's description (its closing full stop dropped, since the question mark ends
 * the sentence) or nothing after the category when it has none. */
export function classificationHintText(task: SheetClassificationTask, hint: Hint, text: QuizText, locale: QuizLocale): string {
  const labelOf = (id: Slug): string => {
    const found = task.items.find((candidate) => candidate.id === id);
    return found === undefined ? id : hintName(found, locale);
  };
  const categoryOf = (id: Slug): Category | undefined => task.categories.find((category) => category.id === id);
  const named = (id: Slug): string => {
    const category = categoryOf(id);
    return category === undefined ? id : hintName(category, locale);
  };
  const item = labelOf(hint.item);
  switch (hint.kind) {
    case "profile": {
      const axis = task.axes?.find((candidate) => candidate.id === hint.axis);
      const axisName = axis === undefined ? hint.axis : hintTerm(axis, locale);
      if (hint.other !== undefined) return hint.above ? text("quiz.classification.above", { item, other: labelOf(hint.other), axis: axisName }) : text("quiz.classification.below", { item, other: labelOf(hint.other), axis: axisName });
      const value = categoryOf(hint.category)?.profile?.[hint.axis];
      return text("quiz.classification.fits", { item, category: named(hint.category), axis: axisName, value: value === undefined ? "" : withUnit(withMinusSign(formatNumber(value, locale)), axis?.unit ?? "") });
    }
    case "group":
      return hint.together ? text("quiz.classification.together", { item, other: labelOf(hint.other) }) : text("quiz.classification.apart", { item, other: labelOf(hint.other) });
    case "category": {
      const description = categoryOf(hint.category)?.description;
      return description === undefined ? text("quiz.classification.belongsBare", { item, category: named(hint.category) }) : text("quiz.classification.belongs", { item, category: named(hint.category), description: localized(description, locale).replace(/\.$/u, "") });
    }
    case "compare":
      return "";
  }
}

/** 🧺️ The classification interaction; inside a pets' topic every item is marked with its own. */
export function ClassificationTaskView(props: TaskViewProps<SheetClassificationTask, ClassificationAnswer>): ReactElement {
  const { task, answer, onAnswer, text, locale, hints = [], locked = false } = props;
  const scope = useId();
  const topic = usePetTopic();
  const { announcement, announce } = useAnnouncement(SELECT_ANNOUNCEMENT_DELAY_MS);
  const focusAfterRender = useFocusAfterRender();
  const assignments = answer?.assignments ?? {};
  const itemLabel = (item: SheetItem): string => localized(item.label, locale);
  const categoryLabel = (id: Slug): string => localized(task.categories.find((category) => category.id === id)?.label ?? { en: id, de: id }, locale);
  const normalised = (task.axes ?? []).some((axis) => axis.unit === undefined || axis.min === undefined || axis.max === undefined);
  const axes: readonly RadarAxis[] = (task.axes ?? []).map((axis) => ({ id: axis.id, label: localized(axis.label, locale), unit: axis.unit ?? "", min: normalised ? 0 : (axis.min ?? 0), max: normalised ? 1 : (axis.max ?? 1) }));
  const asked = new Map(hints.flatMap((hint): [Slug, Hint][] => (hint.kind === "compare" ? [] : [[hint.item, hint]])));
  const hinted = useHintAnnouncement(hints, (hint: Hint) => classificationHintText(task, hint, text, locale), text);
  const lockedId = locked ? elementId(scope, "locked") : undefined;

  const assign = (item: SheetItem, category: Slug | undefined, refocus: boolean): void => {
    if (locked) return announce(text("quiz.task.locked"));
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

  const chip = (item: SheetItem): ReactElement => {
    const hint = asked.get(item.id);
    const hintId = hint === undefined ? undefined : elementId(scope, item.id, "hint");
    return (
      <li key={item.id} data-icon-host="" data-quiz-drag="" data-quiz-item={item.id} data-presence-anchor={PRESENCE_ANCHORS.item(item.id)} data-pet-prop={petProp(topic, item.id)} className="quiz-chip border border-normal bg-background p-single">
        <DragGrip title={text("quiz.classification.drag", { item: itemLabel(item) })} locked={locked} onDrop={drop(item)} />
        <span className="quiz-row-label min-w-0 text-sm font-semibold">
          <IconLabel icon={item.icon}>
            {itemLabel(item)}
          </IconLabel>
          {hint === undefined || hintId === undefined ? null : (
            <HintNote id={hintId} kind={hint.kind}>
              {classificationHintText(task, hint, text, locale)}
            </HintNote>
          )}
        </span>
        <select
          id={elementId(scope, item.id)}
          aria-label={text("quiz.classification.categoryFor", { item: itemLabel(item) })}
          aria-describedby={describedBy(hintId, lockedId)}
          aria-disabled={locked || undefined}
          value={assignments[item.id] ?? ""}
          onChange={(event) => assign(item, event.target.value === "" ? undefined : event.target.value, true)}
          className={SELECT_CLASS}
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
  };

  const pool = task.items.filter((item) => assignments[item.id] === undefined);
  const chips = "quiz-rows m-0 flex list-none flex-col gap-single p-0";
  return (
    <div className="quiz-classify">
      <section data-quiz-drop={POOL_ZONE} aria-labelledby={elementId(scope, "pool")} className={`quiz-pool ${DROP_ZONE_CLASS}`}>
        <h3 id={elementId(scope, "pool")} className="m-0 text-sm font-semibold">
          {text("quiz.classification.pool")}
        </h3>
        {pool.length === 0 ? <p className="m-0 text-xs text-muted-foreground">{text("quiz.classification.poolEmpty")}</p> : <ul role="list" className={chips}>{pool.map(chip)}</ul>}
      </section>
      <section aria-labelledby={elementId(scope, "categories")} className="flex flex-col gap-single">
        <h3 id={elementId(scope, "categories")} className="m-0 text-sm font-semibold">
          {text("quiz.classification.categories")}
        </h3>
        <div className="quiz-bins">
          {task.categories.map((category) => {
            const members = task.items.filter((item) => assignments[item.id] === category.id);
            const label = localized(category.label, locale);
            return (
              <section key={category.id} data-quiz-drop={`${CATEGORY_ZONE}${category.id}`} data-presence-anchor={PRESENCE_ANCHORS.category(category.id)} aria-labelledby={elementId(scope, "category", category.id)} className={DROP_ZONE_CLASS}>
                <h4 data-icon-host="" id={elementId(scope, "category", category.id)} className="m-0 text-sm font-semibold">
                  <IconLabel icon={category.icon}>
                    {label}
                  </IconLabel>
                </h4>
                {category.description === undefined ? null : <p className="m-0 text-xs text-muted-foreground">{localized(category.description, locale)}</p>}
                {category.profile === undefined || axes.length === 0 ? null : <RadarChart name={label} axes={axes} values={category.profile} normalised={normalised} text={text} locale={locale} />}
                {members.length === 0 ? <p className="m-0 text-xs italic text-muted-foreground">{text("quiz.classification.binEmpty")}</p> : <ul role="list" className={chips}>{members.map(chip)}</ul>}
              </section>
            );
          })}
        </div>
      </section>
      {lockedId === undefined ? null : <LockedNote id={lockedId} text={text} />}
      <LiveRegion announcement={announcement} />
      <LiveRegion announcement={hinted} />
    </div>
  );
}
