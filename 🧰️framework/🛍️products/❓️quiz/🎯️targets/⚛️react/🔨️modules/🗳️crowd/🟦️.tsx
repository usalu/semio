/** 🗳️ What the others answered, when it shows, and how it is drawn.
 *
 * **When.** By default the others show once the learner has submitted: on the results always, on a quiz's page once a
 * run of that quiz is submitted, during a run not at all — unless the learner asks to see them now ({@link CrowdDoor}),
 * which holds until that run ends. The preference can also make them show always or never ({@link crowdGate}).
 *
 * **How.** Everything is a figure. The answers of a task are a small chart per item ({@link AnswerFigure}): one column
 * per category, place or value, as tall as the share of the submitted runs that chose it, the percentage on its cap;
 * the learners thinking along right now are dots in their presence colours on the same chart; ● marks the learner's
 * own answer, whose column wears the accent, and — once there is a result — ✓ the correct one. The scores of all runs
 * are a histogram over the ten score bins ({@link ScoreFigure}) with the learner's own bin in the accent. Sheets differ
 * per learner, so everything is semantic, keyed by item id and by category, place or value, never by position on a
 * sheet. Every figure is a real table or list whose cells say their numbers in words, so nothing depends on colour, on
 * hovering or on seeing the columns; nothing here is a live region, so updates never interrupt; and every figure keeps
 * its size whatever the crowd puts into it.
 *
 * @see ../../../../🧬️schema/🔣️.json — `CrowdView`, `ThinkingState`
 * @see ../../../../🔨️modules/👁️views/🟦️.ts — `crowdView`, `scoreBin`, `placeBin`
 * @see ../📊️plot/🟦️.tsx — the column every figure is drawn with
 */

import { useId, type ReactElement, type ReactNode } from "react";
import { CROWD_SCORE_BINS, placeBin, scoreBin, thinkingCrowd, valueKey, type Answer, type CrowdItem, type CrowdView, type Icon, type SheetTask, type Slug, type TaskKind, type TaskResult, type ThinkingItem, type ThinkingState } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatNumber, formatQuantity, formatScore, withUnit } from "../📏️quantity/🟦️.ts";
import { BodyButton, IconLabel, Missing } from "../🪟️chrome/🟦️.tsx";
import { Column, columnShare, formatShare, peakOf } from "../📊️plot/🟦️.tsx";
import { paintStyle, usePresenceView } from "../👥️presence/🟦️.tsx";

//#region 🚪️Gate
/** 👥️ When the others' answers show: never, once the learner has submitted, or always. */
export const OTHERS_CHOICES = ["never", "submitted", "always"] as const;

/** 🔘️ One of {@link OTHERS_CHOICES}. */
export type OthersChoice = (typeof OTHERS_CHOICES)[number];

/** 🗺️ Where the others could show: in a run, on the page of a quiz, on the results of a run. */
export type CrowdPlace = "run" | "quiz" | "results";

/** 🚦️ Whether the others show at a place: `open` (they do), `asked` (they do because the learner asked to see them
 * before submitting), `locked` (not yet — the learner can ask) or `off` (never, by the learner's choice). */
export type CrowdGate = "open" | "asked" | "locked" | "off";

/** 🔐️ The gate of `place` for a learner who chose `others`: results are open, a quiz's page is once the learner has
 * `submitted` a run of that quiz, a run never is by itself; where it is not, an ask opens it. */
export function crowdGate(others: OthersChoice, place: CrowdPlace, facts: { readonly asked: boolean; readonly submitted: boolean }): CrowdGate {
  if (others === "never") return "off";
  if (others === "always" || place === "results" || (place === "quiz" && facts.submitted)) return "open";
  return facts.asked ? "asked" : "locked";
}

/** 👀️ Whether a gate lets the others show. */
export function crowdShown(gate: CrowdGate): boolean {
  return gate === "open" || gate === "asked";
}

/** 🚪️ The door of a gate that an ask opens: locked, the button that shows the others now and why they do not show
 * yet; asked, the same button to hide them again — one button in one place, first in its row, so it keeps the focus
 * and stays under the pointer through both. Nothing for a gate that is open by itself or off. */
export function CrowdDoor(props: { readonly gate: CrowdGate; readonly onAsk: () => void; readonly onUnask: () => void; readonly text: QuizText }): ReactElement | null {
  const { gate, text } = props;
  const id = useId();
  if (gate !== "locked" && gate !== "asked") return null;
  return (
    <div data-crowd-gate={gate} className="flex flex-wrap items-center gap-x-double gap-y-single border-t border-normal pt-double text-xs text-muted-foreground">
      <BodyButton onClick={gate === "locked" ? props.onAsk : props.onUnask} describedBy={gate === "locked" ? id : undefined}>
        {text(gate === "locked" ? "quiz.crowd.ask" : "quiz.crowd.unask")}
      </BodyButton>
      {gate === "locked" ? (
        <p id={id} className="m-0 min-w-0 flex-1 basis-[16em]">
          {text("quiz.crowd.locked")}
        </p>
      ) : null}
    </div>
  );
}
//#endregion 🚪️Gate

//#region 🧮️Model
/** 🏛️ One column of an answer figure: a category (with its icon), a place or a value, under its key in the crowd's counts. */
export interface AnswerColumn {
  readonly key: string;
  readonly label: string;
  readonly icon?: Icon;
}

/** 🧫️ What the crowd gave one item under one column: how many submitted runs (`count`, `share` of the item's answers),
 * who thinks so right now (`tags`), and whether it is the learner's `own` answer or the `correct` one. */
export interface AnswerCell {
  readonly key: string;
  readonly count: number;
  readonly share: number;
  readonly tags: readonly string[];
  readonly own: boolean;
  readonly correct: boolean;
}

/** 🧾️ One item of an answer figure: how many submitted runs answered it, its cells in column order, and for a sorting
 * the average place (1 … places, one decimal) the runs gave it. */
export interface AnswerRow {
  readonly item: Slug;
  readonly label: string;
  readonly icon?: Icon;
  readonly answers: number;
  readonly cells: readonly AnswerCell[];
  readonly mean?: number;
}

/** 📋️ The answers of one task (of one dimension of a matching) as a figure: `runs` submitted runs of the quiz,
 * `thinkers` learners thinking along now, and the `unit` every column's value is in when the columns only carry the
 * numbers (a matching whose quantity keeps one unit; one with SI prefixes names the unit with every value). */
export interface AnswerFigureModel {
  readonly kind: TaskKind;
  readonly runs: number;
  readonly thinkers: number;
  readonly unit?: string;
  readonly columns: readonly AnswerColumn[];
  readonly rows: readonly AnswerRow[];
}

/** 📥️ What an answer figure is made from: the task as the learner's sheet presents it (`dimension` for a matching),
 * the submitted `crowd` of its quiz, the drafts of the others `thinking` along, the learner's own `answer` and, once
 * the run is submitted, its `result`. */
export interface AnswerFigureInput {
  readonly task: SheetTask;
  readonly dimension?: Slug;
  readonly crowd?: CrowdView;
  readonly thinking?: readonly ThinkingState[];
  readonly answer?: Answer;
  readonly result?: TaskResult;
  readonly locale: QuizLocale;
}

/** 📍️ The place (1 … `total`) a normalized position stands for, rounded half up to one decimal. */
export function crowdPlace(position: number, total: number): number {
  return Math.round((1 + position * Math.max(0, total - 1)) * 10) / 10;
}

/** 🪜️ The place (0 … `places` − 1) a normalized position (0 smallest … 1 largest) falls on, rounded half up. */
export function livePlace(position: number, places: number): number {
  return Math.min(Math.max(0, places - 1), Math.floor(position * Math.max(0, places - 1) + 0.5));
}

interface Choices {
  readonly columns: readonly AnswerColumn[];
  readonly order: readonly Slug[];
  readonly count: (item: CrowdItem | undefined, key: string) => number;
  readonly live: (item: ThinkingItem | undefined, key: string) => readonly string[];
  readonly own: (item: Slug) => string | undefined;
  readonly correct: (item: Slug) => string | undefined;
  readonly places?: number;
  readonly unit?: string;
}

function counted(item: CrowdItem | undefined, key: string): number {
  return item?.counts?.find((entry) => entry.key === key)?.count ?? 0;
}

function voted(item: ThinkingItem | undefined, key: string): readonly string[] {
  return item?.votes?.find((vote) => vote.key === key)?.tags ?? [];
}

function choicesOf(input: AnswerFigureInput, submitted: readonly CrowdItem[], thinking: readonly ThinkingItem[]): Choices {
  const { task, answer, result, locale } = input;
  const sheet = task.items.map((item) => item.id);
  switch (task.kind) {
    case "classification": {
      const scored = result?.kind === "classification" ? result.items : [];
      return {
        columns: task.categories.map((category) => ({ key: category.id, label: localized(category.label, locale), ...(category.icon === undefined ? {} : { icon: category.icon }) })),
        order: sheet,
        count: counted,
        live: voted,
        own: (item) => scored.find((entry) => entry.item === item)?.assigned ?? (answer?.kind === "classification" ? answer.assignments[item] : undefined),
        correct: (item) => scored.find((entry) => entry.item === item)?.correct,
      };
    }
    case "sorting": {
      const places = submitted.find((item) => item.places !== undefined)?.places?.length ?? sheet.length;
      const scored = result?.kind === "sorting" ? result.items : [];
      const drafted = answer?.kind === "sorting" ? answer.order : undefined;
      const placed = (item: Slug): number | undefined => {
        const found = scored.find((entry) => entry.item === item);
        if (found !== undefined) return placeBin(found.position, scored.length, places);
        const index = drafted?.indexOf(item) ?? -1;
        return index < 0 || drafted === undefined ? undefined : placeBin(index, drafted.length, places);
      };
      const ranked = (item: Slug): number | undefined => {
        const found = scored.find((entry) => entry.item === item);
        return found === undefined ? undefined : placeBin(found.rank, scored.length, places);
      };
      const order = scored.length > 0 ? [...scored].sort((left, right) => left.position - right.position).map((entry) => entry.item) : drafted !== undefined && drafted.length === sheet.length ? drafted : sheet;
      return {
        columns: Array.from({ length: places }, (_, place) => ({ key: String(place), label: String(place + 1) })),
        order,
        count: (item, key) => item?.places?.[Number(key)] ?? 0,
        live: (item, key) => (item?.positions ?? []).filter((position) => livePlace(position.position, places) === Number(key)).map((position) => position.tag),
        own: (item) => placed(item)?.toString(),
        correct: (item) => ranked(item)?.toString(),
        places,
      };
    }
    case "matching": {
      const dimension = task.dimensions.find((candidate) => candidate.id === input.dimension);
      const cards = dimension?.cards ?? [];
      const scored = result?.kind === "matching" ? (result.dimensions.find((candidate) => candidate.dimension === input.dimension)?.items ?? []) : [];
      const drafted = answer?.kind === "matching" && input.dimension !== undefined ? answer.assignments[input.dimension] : undefined;
      const keys = new Set([...cards.filter(Number.isFinite).map(valueKey), ...submitted.flatMap((item) => (item.counts ?? []).map((entry) => entry.key)), ...thinking.flatMap((item) => (item.votes ?? []).map((vote) => vote.key))]);
      const key = (value: number | undefined): string | undefined => (value === undefined || !Number.isFinite(value) ? undefined : valueKey(value));
      const bare = dimension !== undefined && !dimension.quantity.prefixed && dimension.quantity.unit !== "";
      return {
        columns: [...keys].sort((left, right) => Number(left) - Number(right)).map((value) => ({ key: value, label: dimension === undefined ? value : bare ? formatNumber(Number(value), locale) : formatQuantity(Number(value), dimension.quantity, locale) })),
        ...(bare ? { unit: dimension.quantity.unit } : {}),
        order: sheet,
        count: counted,
        live: voted,
        own: (item) => key(scored.find((entry) => entry.item === item)?.assigned ?? (drafted?.[item] === undefined ? undefined : cards[drafted[item]])),
        correct: (item) => key(scored.find((entry) => entry.item === item)?.correct),
      };
    }
  }
}

/** 🏗️ The answer figure of one task: a row per item of the learner's sheet — a sorting in the learner's own order, so
 * the learner's marks run down the diagonal —, a column per category in sheet order, per place, or per value given by
 * anyone, ascending. */
export function answerFigure(input: AnswerFigureInput): AnswerFigureModel {
  const { task, crowd, locale } = input;
  const submitted = crowd?.tasks.find((candidate) => candidate.task === task.id && candidate.dimension === input.dimension)?.items ?? [];
  const thinking = thinkingCrowd(input.thinking ?? [], task).find((candidate) => candidate.dimension === input.dimension)?.items ?? [];
  const choices = choicesOf(input, submitted, thinking);
  const rows = choices.order.map((id): AnswerRow => {
    const item = submitted.find((candidate) => candidate.item === id);
    const live = thinking.find((candidate) => candidate.item === id);
    const [own, correct] = [choices.own(id), choices.correct(id)];
    const answers = item?.answers ?? 0;
    const shown = task.items.find((candidate) => candidate.id === id);
    return {
      item: id,
      label: localized(shown?.label ?? { en: id, de: id }, locale),
      ...(shown?.icon === undefined ? {} : { icon: shown.icon }),
      answers,
      cells: choices.columns.map((column) => {
        const count = choices.count(item, column.key);
        return { key: column.key, count, share: columnShare(count, answers), tags: choices.live(live, column.key), own: own === column.key, correct: correct === column.key };
      }),
      ...(choices.places === undefined || item?.meanPosition === undefined ? {} : { mean: crowdPlace(item.meanPosition, choices.places) }),
    };
  });
  return { kind: task.kind, runs: crowd?.runs ?? 0, thinkers: new Set((input.thinking ?? []).map((state) => state.tag)).size, ...(choices.unit === undefined ? {} : { unit: choices.unit }), columns: choices.columns, rows };
}

/** 📉️ The scores of all runs as a figure: the count per score bin, how many there are, the tallest bin, and the bin of
 * the learner's `own` score. */
export interface ScoreFigureModel {
  readonly bins: readonly number[];
  readonly runs: number;
  readonly peak: number;
  readonly own?: number;
}

/** 📶️ The score figure of `bins` (none known yet: all empty) with the learner's `own` score in its bin. */
export function scoreFigure(bins: readonly number[] | undefined, own?: number): ScoreFigureModel {
  const counts = bins ?? Array.from({ length: CROWD_SCORE_BINS }, () => 0);
  return { bins: counts, runs: counts.reduce((sum, count) => sum + count, 0), peak: peakOf(counts), ...(own === undefined ? {} : { own: scoreBin(own) }) };
}
//#endregion 🧮️Model

//#region 🖼️Figures
/** 🔵️ How many thinkers a cell shows as dots before it counts the rest. */
export const CROWD_DOTS = 4;

function Dots(props: { readonly tags: readonly string[] }): ReactElement | null {
  const { colours } = usePresenceView();
  const { tags } = props;
  if (tags.length === 0) return null;
  return (
    <span className="inline-flex items-center gap-[0.125rem]">
      {tags.slice(0, CROWD_DOTS).map((tag) => (
        <span key={tag} data-crowd-dot={tag} className="quiz-online !m-0" style={colours.get(tag) === undefined ? undefined : paintStyle(colours.get(tag)!)} />
      ))}
      {tags.length > CROWD_DOTS ? <span className="tabular-nums">+{tags.length - CROWD_DOTS}</span> : null}
    </span>
  );
}

function Caption(props: { readonly title: string; readonly facts: readonly ReactNode[] }): ReactElement {
  return (
    <figcaption className="m-0 flex flex-wrap items-center gap-x-single text-xs text-muted-foreground">
      <span aria-hidden="true">👥</span>
      <span className="font-semibold text-foreground">{props.title}</span>
      {props.facts.map((fact, index) => (
        <span key={index}>· {fact}</span>
      ))}
    </figcaption>
  );
}

/** 🖼️ The answers of one task as a figure named `name`: a table with a row per item and a column per category, place
 * or value; every cell says its share, count and marks in words and draws them as a column, its cap and the marks
 * under its baseline. `title` heads it (a matching's quantity), else "What everyone answered". */
export function AnswerFigure(props: AnswerFigureInput & { readonly name: string; readonly title?: string; readonly text: QuizText }): ReactElement {
  const { text, locale } = props;
  const figure = answerFigure(props);
  const sorting = figure.kind === "sorting";
  const marked = (mark: (cell: AnswerCell) => boolean): boolean => figure.rows.some((row) => row.cells.some(mark));
  const legend: readonly (readonly [string, ReactNode, string])[] = [
    ...(marked((cell) => cell.own) ? [["own", "●", text("quiz.crowd.own")] as const] : []),
    ...(marked((cell) => cell.correct) ? [["correct", "✓", text("quiz.crowd.correct")] as const] : []),
    ...(marked((cell) => cell.tags.length > 0) ? [["live", <span className="quiz-online !m-0" />, text("quiz.crowd.live")] as const] : []),
  ];
  return (
    <figure data-crowd-figure="answers" data-task={props.task.id} data-dimension={props.dimension} data-runs={figure.runs} data-thinkers={figure.thinkers} className="quiz-figure m-0 flex min-w-0 flex-col gap-single">
      <Caption title={`${props.title ?? text("quiz.crowd.title")}${figure.unit === undefined ? "" : ` (${figure.unit})`}`} facts={[text("quiz.crowd.runs", { count: figure.runs }), ...(figure.thinkers > 0 ? [text("quiz.crowd.thinking", { count: figure.thinkers })] : [])]} />
      <div className="relative max-w-full overflow-x-auto">
        <table className="quiz-plot" aria-label={props.name} style={{ ["--quiz-plot-columns" as string]: String(figure.columns.length + (sorting ? 1 : 0)) }}>
          <thead>
            <tr>
              <th scope="col" className="quiz-plot-item">
                {text("quiz.results.item")}
              </th>
              {figure.columns.map((column, place) => (
                <th key={column.key} scope="col">
                  {sorting ? (
                    <>
                      <span className="sr-only">{text("quiz.crowd.place", { place: column.label })}</span>
                      <span aria-hidden="true">{column.label}</span>
                    </>
                  ) : (
                    <>
                      <IconLabel icon={column.icon} order={place}>
                        {column.label}
                      </IconLabel>
                      {figure.unit === undefined ? null : <span className="sr-only">{withUnit("", figure.unit)}</span>}
                    </>
                  )}
                </th>
              ))}
              {sorting ? (
                <th scope="col">{text("quiz.crowd.meanPlace")}</th>
              ) : null}
            </tr>
          </thead>
          <tbody>
            {figure.rows.map((row, place) => (
              <tr key={row.item} data-crowd-item={row.item} data-answers={row.answers}>
                <th scope="row" className="quiz-plot-item">
                  <IconLabel icon={row.icon} order={place}>
                    {row.label}
                  </IconLabel>
                </th>
                {row.cells.map((cell) => {
                  const share = formatShare(cell.share, locale);
                  const sentence = [text("quiz.crowd.cell", { share, count: cell.count, answers: row.answers }), ...(cell.own ? [text("quiz.crowd.own")] : []), ...(cell.correct ? [text("quiz.crowd.correct")] : []), ...(cell.tags.length > 0 ? [text("quiz.crowd.liveCount", { count: cell.tags.length })] : [])].join(", ");
                  return (
                    <td key={cell.key} data-key={cell.key} data-count={cell.count} data-own={cell.own ? "" : undefined} data-correct={cell.correct ? "" : undefined} data-live={cell.tags.length > 0 ? cell.tags.length : undefined} title={sentence}>
                      <span className="sr-only">{sentence}</span>
                      <Column share={cell.share} label={cell.count > 0 ? share : undefined} emphasis={cell.own} />
                      <span aria-hidden="true" className="quiz-plot-marks">
                        {cell.own ? <span className="quiz-plot-own">●</span> : null}
                        {cell.correct ? <span>✓</span> : null}
                        <Dots tags={cell.tags} />
                      </span>
                    </td>
                  );
                })}
                {sorting ? <td className="quiz-plot-mean tabular-nums">{row.mean === undefined ? <Missing label={text("quiz.crowd.nobody")} /> : formatNumber(row.mean, locale)}</td> : null}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {legend.length === 0 ? null : (
        <ul role="list" aria-hidden="true" className="m-0 flex list-none flex-wrap gap-x-double gap-y-single p-0 text-xs text-muted-foreground">
          {legend.map(([key, mark, label]) => (
            <li key={key} data-legend={key} className="flex items-center gap-single">
              <span className={key === "own" ? "quiz-plot-own" : undefined}>{mark}</span>
              {label}
            </li>
          ))}
        </ul>
      )}
    </figure>
  );
}

/** 📈️ The scores of all runs as a figure named `name`: a list of the ten score bins, each saying its range and count
 * in words and drawing the count as a column over a percent axis; the bin of the learner's `own` score wears the accent
 * and the caption says the score. `title` heads it, else "How everyone scored". */
export function ScoreFigure(props: { readonly name: string; readonly title?: string; readonly bins: readonly number[] | undefined; readonly own?: number; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { text, locale, own } = props;
  const figure = scoreFigure(props.bins, own);
  const edge = (bin: number): string => formatScore(bin / figure.bins.length, locale);
  return (
    <figure data-crowd-figure="scores" data-runs={figure.runs} className="quiz-figure quiz-scores m-0 flex min-w-0 flex-col gap-single">
      <Caption
        title={props.title ?? text("quiz.crowd.scores")}
        facts={[
          text("quiz.crowd.runs", { count: figure.runs }),
          ...(own === undefined
            ? []
            : [
                <span data-own-score="">
                  <span aria-hidden="true" className="quiz-plot-own">
                    ●{" "}
                  </span>
                  {text("quiz.crowd.ownScore", { score: formatScore(own, locale) })}
                </span>,
              ]),
        ]}
      />
      <ol role="list" aria-label={props.name} className="quiz-histogram m-0 list-none p-0">
        {figure.bins.map((count, bin) => {
          const sentence = text("quiz.crowd.bin", { from: edge(bin), to: edge(bin + 1), count });
          return (
            <li key={bin} data-bin={bin} data-count={count} data-own={figure.own === bin ? "" : undefined} title={sentence}>
              <span className="sr-only">{figure.own === bin ? `${sentence}, ${text("quiz.crowd.ownScore", { score: formatScore(own ?? 0, locale) })}` : sentence}</span>
              <Column share={columnShare(count, figure.peak)} label={count > 0 ? formatNumber(count, locale) : undefined} emphasis={figure.own === bin} />
            </li>
          );
        })}
      </ol>
      <div aria-hidden="true" className="quiz-histogram-axis text-xs text-muted-foreground tabular-nums">
        <span>{edge(0)}</span>
        <span>{formatScore(0.5, locale)}</span>
        <span>{edge(figure.bins.length)}</span>
      </div>
    </figure>
  );
}

/** 🧩️ Every figure of one task named `title`: its answers — one figure per dimension of a matching, headed by the
 * quantity — and, once the run has a `result`, beside each the scores all runs reached there with the learner's own. */
export function TaskFigures(props: Omit<AnswerFigureInput, "dimension"> & { readonly title: string; readonly text: QuizText }): ReactElement {
  const { task, title, crowd, result, text, locale } = props;
  const parts = task.kind === "matching" ? task.dimensions.map((dimension) => ({ dimension: dimension.id, heading: localized(dimension.quantity.label, locale) })) : [{ dimension: undefined, heading: undefined }];
  return (
    <div data-crowd-task={task.id} className="flex min-w-0 flex-col gap-double">
      {parts.map(({ dimension, heading }) => {
        const subject = heading === undefined ? title : `${title} – ${heading}`;
        const own = result === undefined ? undefined : result.kind === "matching" ? result.dimensions.find((candidate) => candidate.dimension === dimension)?.score : result.score;
        return (
          <div key={dimension ?? task.id} className="quiz-figures">
            <AnswerFigure {...props} dimension={dimension} name={text("quiz.crowd.figure", { subject })} title={heading} />
            {result === undefined ? null : <ScoreFigure name={text("quiz.crowd.scoresFigure", { subject })} bins={crowd?.tasks.find((candidate) => candidate.task === task.id && candidate.dimension === dimension)?.scores} own={own} text={text} locale={locale} />}
          </div>
        );
      })}
    </div>
  );
}
//#endregion 🖼️Figures
