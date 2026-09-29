/** 🏁️ Results of a submitted run: the score, and per task the learner's answer beside the solution — true values
 * with their units, the correct categories, positions and explanations — plus the badges the run newly earned. The
 * summary and every task are cards. Verdicts are spelled out in words and symbols, never by colour alone. Unless the
 * learner hides what the others think, every item stands beside everyone: what all submitted runs answered.
 */

import { useId, type ReactElement, type ReactNode } from "react";
import type { Id, MatchingTaskResult, SheetClassificationTask, SheetMatchingTask, SheetSortingTask, SheetTask, Slug, SortingTaskResult, TaskResult, Text, ClassificationTaskResult } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatQuantity, formatScore } from "../📏️quantity/🟦️.ts";
import { TASK_KIND_ICONS } from "../▶️run/🟦️.tsx";
import { CardAction, CardIcon, Glyph, QuizCard, cn } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, runAwards, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { CrowdChoices, CrowdPosition, CrowdProvider, CrowdSource, chooseCrowd, type Crowd, type ItemCrowd } from "../🗳️crowd/🟦️.tsx";

const EDGE = "border-b border-normal px-single py-single text-left align-top";
const HEAD = cn(EDGE, "quiz-nowrap border-b-2 font-semibold");
const NUMBER = cn(EDGE, "quiz-nowrap tabular-nums");

function verdict(credit: number, text: QuizText): string {
  if (credit >= 1) return `✓ ${text("quiz.results.correct")}`;
  return credit > 0 ? `◐ ${text("quiz.results.partial")}` : `✗ ${text("quiz.results.wrong")}`;
}

function labelOf(items: readonly { readonly id: Slug; readonly label: Text }[], id: Slug, locale: QuizLocale): string {
  return localized(items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
}

function Explanation(props: { readonly explanation: Text | undefined; readonly locale: QuizLocale }): ReactElement {
  return <>{props.explanation === undefined ? "" : localized(props.explanation, props.locale)}</>;
}

function ResultTable(props: { readonly head: readonly string[]; readonly children: ReactNode }): ReactElement {
  return (
    <div className="max-w-full overflow-x-auto">
      <table className="w-full border-collapse text-sm">
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

function Others(props: { readonly crowd: ReadonlyMap<string, ItemCrowd> | undefined; readonly item: string; readonly text: QuizText; readonly children: (item: ItemCrowd) => ReactNode }): ReactElement | null {
  const { crowd, text } = props;
  if (crowd === undefined) return null;
  const item = crowd.get(props.item);
  return (
    <td className={EDGE}>
      {item === undefined ? (
        <>
          <span aria-hidden="true">–</span>
          <span className="sr-only">{text("quiz.crowd.nobody")}</span>
        </>
      ) : (
        props.children(item)
      )}
    </td>
  );
}

function ClassificationResult(props: {
  readonly task: SheetClassificationTask;
  readonly result: ClassificationTaskResult;
  readonly crowd: ReadonlyMap<string, ItemCrowd> | undefined;
  readonly text: QuizText;
  readonly locale: QuizLocale;
}): ReactElement {
  const { task, result, crowd, text, locale } = props;
  return (
    <ResultTable head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), text("quiz.results.credit"), ...(crowd === undefined ? [] : [text("quiz.crowd.everyone")]), text("quiz.results.explanation")]}>
      {result.items.map((item) => (
        <tr key={item.item}>
          <th scope="row" className={cn(EDGE, "font-semibold")}>
            {labelOf(task.items, item.item, locale)}
          </th>
          <td className={EDGE}>{labelOf(task.categories, item.assigned, locale)}</td>
          <td className={EDGE}>{labelOf(task.categories, item.correct, locale)}</td>
          <td className={EDGE}>
            {verdict(item.credit, text)} ({formatScore(item.credit, locale)})
          </td>
          <Others crowd={crowd} item={item.item} text={text}>
            {(others) => <CrowdChoices item={others} subject={labelOf(task.items, item.item, locale)} label={(key) => labelOf(task.categories, key, locale)} text={text} />}
          </Others>
          <td className={EDGE}>
            <Explanation explanation={item.explanation} locale={locale} />
          </td>
        </tr>
      ))}
    </ResultTable>
  );
}

function SortingResult(props: { readonly task: SheetSortingTask; readonly result: SortingTaskResult; readonly crowd: ReadonlyMap<string, ItemCrowd> | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, crowd, text, locale } = props;
  const value = (amount: number): string => formatQuantity(amount, task.quantity, locale);
  return (
    <>
      <ResultTable head={[text("quiz.results.position"), text("quiz.results.item"), text("quiz.results.value"), text("quiz.results.rank"), ...(crowd === undefined ? [] : [text("quiz.crowd.everyone")]), text("quiz.results.explanation")]}>
        {result.items.map((item) => (
          <tr key={item.item}>
            <td className={NUMBER}>{item.position + 1}</td>
            <th scope="row" className={cn(EDGE, "font-semibold")}>
              {labelOf(task.items, item.item, locale)}
            </th>
            <td className={NUMBER}>{value(item.value)}</td>
            <td className={EDGE}>
              {item.rank + 1} {item.rank === item.position ? `✓ ${text("quiz.results.correct")}` : `✗ ${text("quiz.results.wrong")}`}
            </td>
            <Others crowd={crowd} item={item.item} text={text}>
              {(others) => <CrowdPosition item={others} total={result.items.length} subject={labelOf(task.items, item.item, locale)} text={text} locale={locale} />}
            </Others>
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
          .map((item) => (
            <li key={item.item}>
              {labelOf(task.items, item.item, locale)} — {value(item.value)}
            </li>
          ))}
      </ol>
    </>
  );
}

function MatchingResult(props: { readonly task: SheetMatchingTask; readonly result: MatchingTaskResult; readonly crowd: Crowd | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, crowd, text, locale } = props;
  return (
    <>
      {result.dimensions.map((dimension) => {
        const sheet = task.dimensions.find((candidate) => candidate.id === dimension.dimension);
        const quantity = sheet === undefined ? dimension.dimension : localized(sheet.quantity.label, locale);
        const value = (amount: number): string => (sheet === undefined ? String(amount) : formatQuantity(amount, sheet.quantity, locale));
        const others = crowd?.items(task, dimension.dimension);
        return (
          <section key={dimension.dimension} className="flex flex-col gap-single">
            <h3 className="m-0 text-sm font-semibold">{text("quiz.results.dimension", { quantity, score: formatScore(dimension.score, locale) })}</h3>
            <ResultTable head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), ...(others === undefined ? [] : [text("quiz.crowd.everyone")]), text("quiz.results.explanation")]}>
              {dimension.items.map((item) => (
                <tr key={item.item}>
                  <th scope="row" className={cn(EDGE, "font-semibold")}>
                    {labelOf(task.items, item.item, locale)}
                  </th>
                  <td className={cn(EDGE, "tabular-nums")}>
                    {value(item.assigned)} {item.assigned === item.correct ? `✓ ${text("quiz.results.correct")}` : `✗ ${text("quiz.results.wrong")}`}
                  </td>
                  <td className={NUMBER}>{value(item.correct)}</td>
                  <Others crowd={others} item={item.item} text={text}>
                    {(entry) => <CrowdChoices item={entry} subject={labelOf(task.items, item.item, locale)} label={(key) => value(Number(key))} text={text} />}
                  </Others>
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

/** 🧾️ The result of one task beside its presented form, and beside the others (`crowd`) when given. */
export function TaskResultView(props: { readonly task: SheetTask; readonly result: TaskResult; readonly crowd?: Crowd; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { task, result, crowd, text, locale } = props;
  if (task.kind === "classification" && result.kind === "classification") return <ClassificationResult task={task} result={result} crowd={crowd?.items(task)} text={text} locale={locale} />;
  if (task.kind === "sorting" && result.kind === "sorting") return <SortingResult task={task} result={result} crowd={crowd?.items(task)} text={text} locale={locale} />;
  if (task.kind === "matching" && result.kind === "matching") return <MatchingResult task={task} result={result} crowd={crowd} text={text} locale={locale} />;
  return null;
}

/** 🏁️ The results screen of `run`. */
export function ResultsScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale; readonly showAnswers?: boolean }): ReactElement | null {
  const { session, state, run, text, locale } = props;
  const scope = useId();
  const view = state.runs[run];
  const result = view?.status === "submitted" ? view.result : undefined;
  if (view === undefined) return null;
  const emoji = state.catalog?.quizzes.find((quiz) => quiz.id === view.quiz)?.emoji;
  const icon = emoji === undefined ? <CardIcon icon="award" /> : <Glyph emoji={emoji} className="text-sm" />;
  const title = text("quiz.results.title", { quiz: localized(view.sheet.title, locale) });
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
  const crowd = props.showAnswers === false ? undefined : chooseCrowd([], state.crowds[view.quiz]);
  return (
    <CrowdProvider crowd={crowd}>
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
          {crowd === undefined ? null : <CrowdSource crowd={crowd} text={text} />}
          {badges.length === 0 ? null : (
            <section aria-labelledby={`${scope}-badges`} className="flex flex-col gap-single">
              <h2 id={`${scope}-badges`} className="m-0 text-sm font-semibold">
                {text("quiz.results.newBadges")}
              </h2>
              <ul className="m-0 flex list-none flex-col gap-single p-0">
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
            <QuizCard
              key={taskResult.task}
              id={`${scope}-${taskResult.task}`}
              card="task-result"
              anchor={PRESENCE_ANCHORS.result(taskResult.task)}
              icon={<CardIcon icon={TASK_KIND_ICONS[task.kind]} />}
              title={`${index + 1}. ${localized(task.title, locale)}`}
            >
              <p className="m-0 text-xs text-muted-foreground">
                {text(TASK_KIND_LABELS[task.kind])} · {text("quiz.results.taskScore", { score: formatScore(taskResult.score, locale) })}
              </p>
              <TaskResultView task={task} result={taskResult} crowd={crowd} text={text} locale={locale} />
            </QuizCard>
          );
        })}
      </div>
    </CrowdProvider>
  );
}
