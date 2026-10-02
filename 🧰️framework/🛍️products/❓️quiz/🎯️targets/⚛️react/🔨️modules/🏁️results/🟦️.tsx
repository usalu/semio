/** 🏁️ Results of a submitted run: the score, and per task the learner's answer beside the solution — true values
 * with their units, the correct categories, positions and explanations — plus the badges the run newly earned. The
 * summary and every task are cards. Verdicts are spelled out in words and symbols, never by colour alone; the symbol
 * only repeats the word and is not spoken. Every table is named by the task or quantity it belongs to. Unless the
 * learner never wants to see the others, the run is submitted and so they show: beside the score how all runs scored,
 * and below every task's table what all runs answered there and how they scored on it — figures that are there before
 * the crowd is known and keep their size when it arrives or changes, so nothing on the page moves.
 */

import { useId, type ReactElement, type ReactNode } from "react";
import type { Answer, CrowdView, Icon, Id, MatchingTaskResult, SheetClassificationTask, SheetMatchingTask, SheetSortingTask, SheetTask, Slug, SortingTaskResult, TaskResult, Text, ClassificationTaskResult } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatQuantity, formatScore } from "../📏️quantity/🟦️.ts";
import { TaskGlyph } from "../▶️run/🟦️.tsx";
import { CardAction, CardIcon, Glyph, IconLabel, Mark, Missing, QuizCard, cn } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, runAwards, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { ScoreFigure, TaskFigures, crowdGate, crowdShown, type OthersChoice } from "../🗳️crowd/🟦️.tsx";

const EDGE = "border-b border-normal px-single py-single text-left align-top";
const HEAD = cn(EDGE, "quiz-nowrap border-b-2 font-semibold");
const NUMBER = cn(EDGE, "quiz-nowrap tabular-nums");

/** ⚖️ A verdict as its symbol (shown only) and its word: correct, partly correct or not correct. */
function Verdict(props: { readonly credit: number; readonly text: QuizText }): ReactElement {
  const { credit, text } = props;
  return (
    <>
      <Mark symbol={credit >= 1 ? "✓" : credit > 0 ? "◐" : "✗"} />
      {credit >= 1 ? text("quiz.results.correct") : credit > 0 ? text("quiz.results.partial") : text("quiz.results.wrong")}
    </>
  );
}

function labelOf(items: readonly { readonly id: Slug; readonly label: Text }[], id: Slug, locale: QuizLocale): string {
  return localized(items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
}

/** 🏷️ The label of the item or category `id` with its icon before it; `order` is its row. */
function Labelled(props: { readonly items: readonly { readonly id: Slug; readonly label: Text; readonly icon?: Icon }[]; readonly id: Slug; readonly order: number; readonly locale: QuizLocale }): ReactElement {
  const { items, id } = props;
  return (
    <IconLabel icon={items.find((item) => item.id === id)?.icon} order={props.order}>
      {labelOf(items, id, props.locale)}
    </IconLabel>
  );
}

function Explanation(props: { readonly explanation: Text | undefined; readonly locale: QuizLocale }): ReactElement {
  return <>{props.explanation === undefined ? "" : localized(props.explanation, props.locale)}</>;
}

function ResultTable(props: { readonly name: string; readonly head: readonly string[]; readonly children: ReactNode }): ReactElement {
  return (
    <div className="relative max-w-full overflow-x-auto">
      <table className="w-full border-collapse text-sm" aria-labelledby={props.name}>
        <thead>
          <tr>
            {props.head.map((label) => (
              <th key={label} scope="col" className={HEAD}>
                {label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>{props.children}</tbody>
      </table>
    </div>
  );
}

function ClassificationResult(props: { readonly name: string; readonly task: SheetClassificationTask; readonly result: ClassificationTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, text, locale } = props;
  return (
    <ResultTable name={props.name} head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), text("quiz.results.credit"), text("quiz.results.explanation")]}>
      {result.items.map((item, row) => (
        <tr key={item.item}>
          <th scope="row" className={cn(EDGE, "font-semibold")}>
            <Labelled items={task.items} id={item.item} order={row} locale={locale} />
          </th>
          <td className={EDGE}>
            <Labelled items={task.categories} id={item.assigned} order={row} locale={locale} />
          </td>
          <td className={EDGE}>
            <Labelled items={task.categories} id={item.correct} order={row} locale={locale} />
          </td>
          <td className={EDGE}>
            <Verdict credit={item.credit} text={text} /> ({formatScore(item.credit, locale)})
          </td>
          <td className={EDGE}>
            <Explanation explanation={item.explanation} locale={locale} />
          </td>
        </tr>
      ))}
    </ResultTable>
  );
}

function SortingResult(props: { readonly name: string; readonly task: SheetSortingTask; readonly result: SortingTaskResult; readonly guesses: Readonly<Record<string, number>>; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, guesses, text, locale } = props;
  const value = (amount: number): string => formatQuantity(amount, task.quantity, locale);
  const guessed = Object.keys(guesses).length > 0;
  return (
    <>
      <ResultTable name={props.name} head={[text("quiz.results.position"), text("quiz.results.item"), ...(guessed ? [text("quiz.results.yourGuess")] : []), text("quiz.results.value"), text("quiz.results.rank"), text("quiz.results.explanation")]}>
        {result.items.map((item, row) => (
          <tr key={item.item}>
            <td className={NUMBER}>{item.position + 1}</td>
            <th scope="row" className={cn(EDGE, "font-semibold")}>
              <Labelled items={task.items} id={item.item} order={row} locale={locale} />
            </th>
            {guessed ? <td className={NUMBER}>{Object.hasOwn(guesses, item.item) ? value(guesses[item.item]!) : <Missing label={text("quiz.results.noGuess")} />}</td> : null}
            <td className={NUMBER}>{value(item.value)}</td>
            <td className={EDGE}>
              {item.rank + 1} <Verdict credit={item.rank === item.position ? 1 : 0} text={text} />
            </td>
            <td className={EDGE}>
              <Explanation explanation={item.explanation} locale={locale} />
            </td>
          </tr>
        ))}
      </ResultTable>
      <h3 className="m-0 text-sm font-semibold">{text("quiz.results.trueOrder")}</h3>
      <ol className="m-0 flex flex-col gap-single ps-double text-sm">
        {[...result.items]
          .sort((left, right) => left.rank - right.rank)
          .map((item, row) => (
            <li key={item.item}>
              <Labelled items={task.items} id={item.item} order={row} locale={locale} /> — {value(item.value)}
            </li>
          ))}
      </ol>
    </>
  );
}

function MatchingResult(props: { readonly name: string; readonly task: SheetMatchingTask; readonly result: MatchingTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, text, locale } = props;
  return (
    <>
      {result.dimensions.map((dimension) => {
        const sheet = task.dimensions.find((candidate) => candidate.id === dimension.dimension);
        const quantity = sheet === undefined ? dimension.dimension : localized(sheet.quantity.label, locale);
        const value = (amount: number): string => (sheet === undefined ? String(amount) : formatQuantity(amount, sheet.quantity, locale));
        return (
          <section key={dimension.dimension} className="flex flex-col gap-single">
            <h3 id={`${props.name}-${dimension.dimension}`} className="m-0 text-sm font-semibold">
              <IconLabel icon={sheet?.icon}>{text("quiz.results.dimension", { quantity, score: formatScore(dimension.score, locale) })}</IconLabel>
            </h3>
            <ResultTable name={`${props.name}-${dimension.dimension}`} head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), text("quiz.results.explanation")]}>
              {dimension.items.map((item, row) => (
                <tr key={item.item}>
                  <th scope="row" className={cn(EDGE, "font-semibold")}>
                    <Labelled items={task.items} id={item.item} order={row} locale={locale} />
                  </th>
                  <td className={cn(EDGE, "tabular-nums")}>
                    {value(item.assigned)} <Verdict credit={item.assigned === item.correct ? 1 : 0} text={text} />
                  </td>
                  <td className={NUMBER}>{value(item.correct)}</td>
                  <td className={EDGE}>
                    <Explanation explanation={item.explanation} locale={locale} />
                  </td>
                </tr>
              ))}
            </ResultTable>
          </section>
        );
      })}
    </>
  );
}

function ResultTables(props: { readonly name: string; readonly task: SheetTask; readonly result: TaskResult; readonly answer?: Answer; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { name, task, result, answer, text, locale } = props;
  if (task.kind === "classification" && result.kind === "classification") return <ClassificationResult name={name} task={task} result={result} text={text} locale={locale} />;
  if (task.kind === "sorting" && result.kind === "sorting") return <SortingResult name={name} task={task} result={result} guesses={answer?.kind === "sorting" ? (answer.guesses ?? {}) : {}} text={text} locale={locale} />;
  if (task.kind === "matching" && result.kind === "matching") return <MatchingResult name={name} task={task} result={result} text={text} locale={locale} />;
  return null;
}

/** 🧾️ The result of one task beside its presented form and — with `crowd` (`null` until the crowd is known) — below it
 * what all runs answered there and how they scored on it. `name` is the id of the heading that names its tables (a
 * matching names each by its quantity's own heading), `title` what the task is called. */
export function TaskResultView(props: { readonly name: string; readonly title: string; readonly task: SheetTask; readonly result: TaskResult; readonly answer?: Answer; readonly crowd?: CrowdView | null; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { task, result, answer, crowd, text, locale } = props;
  if (task.kind !== result.kind) return null;
  return (
    <>
      <ResultTables name={props.name} task={task} result={result} answer={answer} text={text} locale={locale} />
      {crowd === undefined ? null : <TaskFigures task={task} title={props.title} crowd={crowd ?? undefined} answer={answer} result={result} text={text} locale={locale} />}
    </>
  );
}

/** 🏁️ The results screen of `run` for a learner who chose when the `others` show (once submitted, unless said
 * otherwise). */
export function ResultsScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale; readonly others?: OthersChoice }): ReactElement | null {
  const { session, state, run, text, locale } = props;
  const scope = useId();
  const view = state.runs[run];
  const result = view?.status === "submitted" ? view.result : undefined;
  if (view === undefined) return null;
  const emoji = state.catalog?.quizzes.find((quiz) => quiz.id === view.quiz)?.emoji;
  const icon = emoji === undefined ? <CardIcon icon="award" /> : <Glyph emoji={emoji} className="text-sm" />;
  const quiz = localized(view.sheet.title, locale);
  const title = text("quiz.results.title", { quiz });
  const home = (
    <CardAction primary onClick={() => session.open({ screen: "home" })}>
      {text("quiz.results.home")}
    </CardAction>
  );
  if (result === undefined) {
    return (
      <QuizCard id={`${scope}-title`} card="results" anchor={PRESENCE_ANCHORS.results} headingLevel={1} focusableHeading icon={icon} title={title} footerRight={home}>
        <p role="status" className="m-0 text-sm">
          {text("quiz.results.missing")}
        </p>
      </QuizCard>
    );
  }
  const badges = runAwards(state, run).flatMap((id) => state.catalog?.badges.filter((badge) => badge.id === id) ?? []);
  const shown = crowdShown(crowdGate(props.others ?? "submitted", "results", { asked: false, submitted: true }));
  const crowd = state.crowds[view.quiz];
  return (
    <div className="quiz-results flex flex-col gap-double">
      <QuizCard
        id={`${scope}-title`}
        card="results"
        anchor={PRESENCE_ANCHORS.results}
        headingLevel={1}
        focusableHeading
        icon={icon}
        title={title}
        footerLeft={<CardAction onClick={() => session.open({ screen: "home", page: HOME_PAGES.leaderboard })}>{text("quiz.results.leaderboard")}</CardAction>}
        footerRight={home}
      >
        <p className="m-0 text-lg font-semibold tabular-nums">{text("quiz.results.score", { score: formatScore(result.score, locale) })}</p>
        {shown ? <ScoreFigure name={text("quiz.crowd.scoresFigure", { subject: quiz })} bins={crowd?.scores} own={result.score} text={text} locale={locale} /> : null}
        {badges.length === 0 ? null : (
          <section aria-labelledby={`${scope}-badges`} className="flex flex-col gap-single">
            <h2 id={`${scope}-badges`} className="m-0 text-sm font-semibold">
              {text("quiz.results.newBadges")}
            </h2>
            <ul role="list" className="m-0 flex list-none flex-col gap-single p-0">
              {badges.map((badge) => (
                <li key={badge.id} data-earned="" className="flex gap-double border border-normal p-double">
                  <Glyph emoji={badge.emoji} className="text-2xl leading-none" />
                  <div className="flex min-w-0 flex-col gap-single">
                    <h3 className="m-0 text-sm font-semibold">{localized(badge.label, locale)}</h3>
                    <p className="m-0 text-xs">{localized(badge.description, locale)}</p>
                  </div>
                </li>
              ))}
            </ul>
          </section>
        )}
      </QuizCard>
      {result.tasks.map((taskResult, index) => {
        const task = view.sheet.tasks.find((candidate) => candidate.id === taskResult.task);
        if (task === undefined) return null;
        return (
          <QuizCard key={taskResult.task} id={`${scope}-${taskResult.task}`} card="task-result" anchor={PRESENCE_ANCHORS.result(taskResult.task)} icon={<TaskGlyph task={task} />} title={`${index + 1}. ${localized(task.title, locale)}`}>
            <p className="m-0 text-xs text-muted-foreground">
              {text(TASK_KIND_LABELS[task.kind])} · {text("quiz.results.taskScore", { score: formatScore(taskResult.score, locale) })}
            </p>
            <TaskResultView name={`${scope}-${taskResult.task}`} title={localized(task.title, locale)} task={task} result={taskResult} answer={view.answers[taskResult.task]} crowd={shown ? (crowd ?? null) : undefined} text={text} locale={locale} />
          </QuizCard>
        );
      })}
    </div>
  );
}
