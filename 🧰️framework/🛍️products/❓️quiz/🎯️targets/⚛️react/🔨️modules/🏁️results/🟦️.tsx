/** 🏁️ Results of a submitted run: the score, and per task the learner's answer beside the solution — true values
 * with their units, the correct categories, positions and explanations — plus the badges the run newly earned. The
 * summary and every task are cards. Verdicts are spelled out in words and symbols, never by colour alone; the symbol
 * only repeats the word and is not spoken. Every table is named by the task or quantity it belongs to. Unless the
 * learner never wants to see the others, the run is submitted and so they show: beside the score how all runs scored,
 * and below every task's table what all runs answered there and how they scored on it — figures that are there before
 * the crowd is known and keep their size when it arrives or changes, so nothing on the page moves. A table whose card
 * is too narrow for its columns is a list of records instead, and a card wide enough shows a task's tables and its
 * figures side by side.
 *
 * The summary names the challenge and the points the run earned of the most it could. Where the keys were hidden a
 * sorting or matching row shows the learner's guess, whether it lies far off or not (a word and a symbol) and by which
 * factor (logarithmic scale) or amount (linear scale) it lies from the true value, below a line that says how far a
 * guess may lie off and still count — the core's reach over the presented true values, which play never shows since it
 * would tell the spread of the hidden keys; an item
 * left without an answer when the time ran out reads "Not answered", and so does every position of a sorting nobody
 * ordered, which has no position of the learner's to judge.
 *
 * @see ../../🎨️.css — `.quiz-records`, `.quiz-result`, `.quiz-summary`, `.quiz-miss`
 */

import { useId, type ReactElement, type ReactNode } from "react";
import { reach, type Answer, type CrowdView, type Icon, type Id, type MatchingTaskResult, type Quantity, type SheetClassificationTask, type SheetMatchingTask, type SheetSortingTask, type SheetTask, type Slug, type SortingTaskResult, type TaskResult, type Text, type ClassificationTaskResult } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { ceilSignificant, floorSignificant, formatFactor, formatQuantity, formatScore } from "../📏️quantity/🟦️.ts";
import { CardAction, CardIcon, Glyph, IconLabel, Mark, QuizCard, Records, TABLE, TaskGlyph, cn } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, runAwards, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { ScoreFigure, TaskFigures, crowdGate, crowdShown, type OthersChoice } from "../🗳️crowd/🟦️.tsx";
import { challengeScored } from "../⛰️challenge/🟦️.tsx";
import { PetTopic, petProp, usePetTopic } from "../🐾️pets/🟦️.tsx";

const EDGE = "border-b border-normal px-single py-single text-left align-top";
const HEAD = "quiz-nowrap border-b-2 border-normal px-single py-single text-left align-top font-semibold";
const NUMBER = `${EDGE} quiz-nowrap tabular-nums`;
const ANSWER = `${EDGE} quiz-nowrap`;

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

/** 🎯️ A guess where the keys were hidden: the guessed value and whether it lies far off or not, each a word after its
 * symbol, and below them how far it lies from the true value; "Not answered" without a guess. */
function Guessed(props: { readonly guess: number | undefined; readonly truth: number; readonly miss: boolean; readonly quantity: Pick<Quantity, "unit" | "prefixed" | "scale">; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { guess, truth, miss, quantity, text, locale } = props;
  if (guess === undefined) return <>{text("quiz.results.unanswered")}</>;
  return (
    <>
      {formatQuantity(guess, quantity, locale)}{" "}
      <span className={miss ? "quiz-miss" : undefined} data-miss={miss ? "" : undefined}>
        <Mark symbol={miss ? "≉" : "≈"} />
        {text(miss ? "quiz.results.miss" : "quiz.results.near")}
      </span>
      <span className="block text-xs" data-off="">
        {text("quiz.results.offBy", { off: deviation(guess, truth, miss, quantity, locale) })}
      </span>
    </>
  );
}

/** 📐️ How far `guess` lies from `truth`: the factor between them on a logarithmic scale, the difference on a linear one,
 * at two significant digits rounded up where the guess misses and down where it does not, so it never contradicts the
 * tolerance shown beside it. */
export function deviation(guess: number, truth: number, miss: boolean, quantity: Pick<Quantity, "unit" | "prefixed" | "scale">, locale: QuizLocale): string {
  const round = miss ? ceilSignificant : floorSignificant;
  if (quantity.scale === "logarithmic") return `×${formatFactor(guess > truth ? guess / truth : truth / guess, locale, round)}`;
  return formatQuantity(round(Math.abs(guess - truth), 2), quantity, locale);
}

/** 🎚️ How far a guess may lie from the true value and still count: the core's reach over the presented true `values`,
 * "×N" on a logarithmic scale and "±d" on a linear one, cut down so it never claims more than the rule; none where
 * every guess counts. */
export function tolerance(values: readonly number[], quantity: Pick<Quantity, "unit" | "prefixed" | "scale">, locale: QuizLocale): string | undefined {
  const within = reach(values, quantity.scale);
  if (!Number.isFinite(within)) return undefined;
  return quantity.scale === "logarithmic" ? `×${formatFactor(within, locale)}` : `±${formatQuantity(floorSignificant(within, 2), quantity, locale)}`;
}

/** 📏️ The line above the results of guesses that says how far a guess may lie off, `id` for the table it describes. */
function ToleranceNote(props: { readonly id: string; readonly within: string | undefined; readonly text: QuizText }): ReactElement | null {
  if (props.within === undefined) return null;
  return (
    <p id={props.id} className="m-0 text-xs" data-tolerance="">
      {props.text("quiz.results.tolerance", { within: props.within })}
    </p>
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

/** 🗃️ A result table named by the heading `name`: a table where its card has the room for its columns, and below
 * `fold` rem a list of records — the item, then what was answered and what is true side by side, then the explanation;
 * `note`, when given, the id of what describes it. */
function ResultTable(props: { readonly name: string; readonly fold: number; readonly head: readonly string[]; readonly note?: string; readonly children: ReactNode }): ReactElement {
  return (
    <Records fold={props.fold}>
      <table {...TABLE.table} className="quiz-fold w-full border-collapse text-sm" aria-labelledby={props.name} aria-describedby={props.note}>
        <thead {...TABLE.group}>
          <tr {...TABLE.row}>
            {props.head.map((label) => (
              <th key={label} {...TABLE.column} className={HEAD}>
                {label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody {...TABLE.group}>{props.children}</tbody>
      </table>
    </Records>
  );
}

function ClassificationResult(props: { readonly name: string; readonly task: SheetClassificationTask; readonly result: ClassificationTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, text, locale } = props;
  return (
    <ResultTable name={props.name} fold={56} head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), text("quiz.results.credit"), text("quiz.results.explanation")]}>
      {result.items.map((item, row) => (
        <tr key={item.item} {...TABLE.row}>
          <th {...TABLE.name} data-cell="name" className={cn(EDGE, "font-semibold")}>
            <Labelled items={task.items} id={item.item} order={row} locale={locale} />
          </th>
          <td {...TABLE.cell} data-label={text("quiz.results.yourAnswer")} className={ANSWER}>
            {item.assigned === undefined ? text("quiz.results.unanswered") : <Labelled items={task.categories} id={item.assigned} order={row} locale={locale} />}
          </td>
          <td {...TABLE.cell} data-label={text("quiz.results.solution")} className={ANSWER}>
            <Labelled items={task.categories} id={item.correct} order={row} locale={locale} />
          </td>
          <td {...TABLE.cell} data-cell="lead" className={ANSWER}>
            <Verdict credit={item.credit} text={text} /> ({formatScore(item.credit, locale)})
          </td>
          <td {...TABLE.cell} data-cell="note" className={EDGE}>
            <Explanation explanation={item.explanation} locale={locale} />
          </td>
        </tr>
      ))}
    </ResultTable>
  );
}

function SortingResult(props: { readonly name: string; readonly task: SheetSortingTask; readonly result: SortingTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, text, locale } = props;
  const topic = usePetTopic();
  const value = (amount: number): string => formatQuantity(amount, task.quantity, locale);
  const guessed = result.items.some((item) => item.miss !== undefined);
  const unanswered = guessed && result.items.every((item) => item.guess === undefined);
  const within = guessed ? tolerance(result.items.map((item) => item.value), task.quantity, locale) : undefined;
  return (
    <>
      <ToleranceNote id={`${props.name}-tolerance`} within={within} text={text} />
      <ResultTable name={props.name} note={within === undefined ? undefined : `${props.name}-tolerance`} fold={58} head={[text("quiz.results.position"), text("quiz.results.item"), ...(guessed ? [text("quiz.results.yourGuess")] : []), text("quiz.results.value"), text("quiz.results.rank"), text("quiz.results.explanation")]}>
        {result.items.map((item, row) => (
          <tr key={item.item} {...TABLE.row}>
            <td {...TABLE.cell} data-label={text("quiz.results.position")} className={unanswered ? ANSWER : NUMBER}>
              {unanswered ? text("quiz.results.unanswered") : item.position + 1}
            </td>
            <th {...TABLE.name} data-cell="name" className={cn(EDGE, "font-semibold")}>
              <Labelled items={task.items} id={item.item} order={row} locale={locale} />
            </th>
            {guessed ? (
              <td {...TABLE.cell} data-label={text("quiz.results.yourGuess")} className={NUMBER}>
                <Guessed guess={item.guess} truth={item.value} miss={item.miss === true} quantity={task.quantity} text={text} locale={locale} />
              </td>
            ) : null}
            <td {...TABLE.cell} data-label={text("quiz.results.value")} className={NUMBER}>
              {value(item.value)}
            </td>
            <td {...TABLE.cell} data-label={text("quiz.results.rank")} className={ANSWER}>
              {item.rank + 1}
              {unanswered ? null : (
                <>
                  {" "}
                  <Verdict credit={item.rank === item.position ? 1 : 0} text={text} />
                </>
              )}
            </td>
            <td {...TABLE.cell} data-cell="note" className={EDGE}>
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
            <li key={item.item} data-pet-prop={petProp(topic, item.item)}>
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
        const guessed = dimension.items.some((item) => item.miss !== undefined);
        const answered = text(guessed ? "quiz.results.yourGuess" : "quiz.results.yourAnswer");
        const within = guessed && sheet !== undefined ? tolerance(dimension.items.map((item) => item.correct), sheet.quantity, locale) : undefined;
        const note = `${props.name}-${dimension.dimension}-tolerance`;
        return (
          <section key={dimension.dimension} className="flex flex-col gap-single">
            <h3 id={`${props.name}-${dimension.dimension}`} className="m-0 text-sm font-semibold">
              <IconLabel icon={sheet?.icon}>{text("quiz.results.dimension", { quantity, score: formatScore(dimension.score, locale) })}</IconLabel>
            </h3>
            <ToleranceNote id={note} within={within} text={text} />
            <ResultTable name={`${props.name}-${dimension.dimension}`} note={within === undefined ? undefined : note} fold={52} head={[text("quiz.results.item"), answered, text("quiz.results.solution"), text("quiz.results.explanation")]}>
              {dimension.items.map((item, row) => (
                <tr key={item.item} {...TABLE.row}>
                  <th {...TABLE.name} data-cell="name" className={cn(EDGE, "font-semibold")}>
                    <Labelled items={task.items} id={item.item} order={row} locale={locale} />
                  </th>
                  <td {...TABLE.cell} data-label={answered} className={NUMBER}>
                    {guessed && sheet !== undefined ? (
                      <Guessed guess={item.assigned} truth={item.correct} miss={item.miss === true} quantity={sheet.quantity} text={text} locale={locale} />
                    ) : item.assigned === undefined ? (
                      text("quiz.results.unanswered")
                    ) : (
                      <>
                        {value(item.assigned)} <Verdict credit={item.assigned === item.correct ? 1 : 0} text={text} />
                      </>
                    )}
                  </td>
                  <td {...TABLE.cell} data-label={text("quiz.results.solution")} className={NUMBER}>
                    {value(item.correct)}
                  </td>
                  <td {...TABLE.cell} data-cell="note" className={EDGE}>
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

function ResultTables(props: { readonly name: string; readonly task: SheetTask; readonly result: TaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { name, task, result, text, locale } = props;
  if (task.kind === "classification" && result.kind === "classification") return <ClassificationResult name={name} task={task} result={result} text={text} locale={locale} />;
  if (task.kind === "sorting" && result.kind === "sorting") return <SortingResult name={name} task={task} result={result} text={text} locale={locale} />;
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
    <div className="quiz-result" data-crowd={crowd === undefined ? undefined : ""}>
      <div className="flex flex-col gap-double">
        <ResultTables name={props.name} task={task} result={result} text={text} locale={locale} />
      </div>
      {crowd === undefined ? null : <TaskFigures task={task} title={props.title} crowd={crowd ?? undefined} answer={answer} result={result} text={text} locale={locale} />}
    </div>
  );
}

/** 🏆️ The results screen of `run` for a learner who chose when the `others` show (once submitted, unless said
 * otherwise). Every task's results lie in the pets' topic of the task (`<quiz>/<task>`): the true order of a sorting
 * marks its items for the pets of their topic, a table row never. */
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
  const shown = crowdShown(crowdGate(props.others ?? "submitted", "results", { asked: false, submitted: true, hidden: false }));
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
        <div className="quiz-summary">
          <div className="flex min-w-0 flex-col gap-double">
            <p className="m-0 text-lg font-semibold tabular-nums">{text("quiz.results.score", { score: formatScore(result.score, locale) })}</p>
            <p className="m-0 text-sm font-semibold tabular-nums">{challengeScored(result.challenge, result.points, text, locale)}</p>
            {shown ? <ScoreFigure name={text("quiz.crowd.scoresFigure", { subject: quiz })} bins={crowd?.scores} own={result.score} text={text} locale={locale} /> : null}
          </div>
          {badges.length === 0 ? null : (
            <section aria-labelledby={`${scope}-badges`} className="flex min-w-0 flex-col gap-single">
              <h2 id={`${scope}-badges`} className="m-0 text-sm font-semibold">
                {text("quiz.results.newBadges")}
              </h2>
              <ul role="list" className="m-0 grid list-none grid-cols-[repeat(auto-fill,minmax(min(100%,18rem),1fr))] gap-single p-0">
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
        </div>
      </QuizCard>
      {result.tasks.map((taskResult, index) => {
        const task = view.sheet.tasks.find((candidate) => candidate.id === taskResult.task);
        if (task === undefined) return null;
        return (
          <QuizCard key={taskResult.task} id={`${scope}-${taskResult.task}`} card="task-result" anchor={PRESENCE_ANCHORS.result(taskResult.task)} icon={<TaskGlyph task={task} />} title={`${index + 1}. ${localized(task.title, locale)}`}>
            <p className="m-0 text-xs text-muted-foreground">
              {text(TASK_KIND_LABELS[task.kind])} · {text("quiz.results.taskScore", { score: formatScore(taskResult.score, locale) })}
            </p>
            <PetTopic topic={petProp(view.quiz, taskResult.task)}>
              <TaskResultView name={`${scope}-${taskResult.task}`} title={localized(task.title, locale)} task={task} result={taskResult} answer={view.answers[taskResult.task]} crowd={shown ? (crowd ?? null) : undefined} text={text} locale={locale} />
            </PetTopic>
          </QuizCard>
        );
      })}
    </div>
  );
}
