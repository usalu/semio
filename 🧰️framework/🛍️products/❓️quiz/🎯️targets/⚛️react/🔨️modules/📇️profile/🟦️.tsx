/** 📇️ The learner on home: the card with the totals, who else is online and the way to switch identity, and the
 * profile page behind it — totals and the rank on the all-time leaderboard of every quiz, every run with its status, score and dates (the result of a submitted run,
 * the way back into an open one), and the badges earned with their days. Both are pure views over session state; the
 * profile is personal, so presence shares no cursor on it. Switching identity always asks first: an anonymous learner
 * gives up the only key to the progress and has to say so, a pseudonymous or named one is told how to come back.
 *
 * @see ../🏠️home/🟦️.tsx — the overview the card sits on and the pane the page fills
 * @see https://www.w3.org/WAI/WCAG22/Understanding/error-prevention-legal-financial-data.html — confirm what cannot be undone
 */

import { useId, useState, type ReactElement } from "react";
import { learnerTag, type RunSummary } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatDate, formatInstant, formatPoints, formatScore } from "../📏️quantity/🟦️.ts";
import { learnerName } from "../🪪️identity/🟦️.tsx";
import { awardOf } from "../🏅️badges/🟦️.tsx";
import { ownRow } from "../🏆️leaderboard/🟦️.tsx";
import type { Act } from "../📖️quiz-page/🟦️.tsx";
import { CardAction, CardIcon, Dialog, Facts, Glyph, Missing, PageFrame, QuizCard, cn, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS, PresenceList } from "../👥️presence/🟦️.tsx";
import { HOME_PAGES, overallLeaderboard, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";

interface LearnerProps {
  readonly session: QuizSession;
  readonly state: QuizState;
  readonly text: QuizText;
  readonly locale: QuizLocale;
}

function totals(props: LearnerProps): { readonly name: string; readonly points: string; readonly facts: readonly string[] } | undefined {
  const { state, text, locale } = props;
  const { learner, learnerView, catalog } = state;
  const leaderboard = overallLeaderboard(state);
  if (learner === undefined || catalog === undefined) return undefined;
  const tag = learnerTag(learner.id);
  const played = catalog.quizzes.filter((quiz) => learnerView?.best[quiz.id] !== undefined).length;
  const earned = catalog.badges.filter((badge) => awardOf(state, badge.id) !== undefined).length;
  const row = leaderboard === undefined ? undefined : ownRow(leaderboard.board, tag);
  return {
    name: learnerName(learner.identity, tag, text),
    points: text("quiz.home.points", { points: formatPoints(learnerView?.total ?? 0, locale) }),
    facts: [
      text("quiz.home.played", { played, total: catalog.quizzes.length }),
      text("quiz.home.badgesEarned", { earned, total: catalog.badges.length }),
      ...(row === undefined || leaderboard === undefined ? [] : [text("quiz.home.rank", { rank: row.rank, total: leaderboard.board.learners })]),
    ],
  };
}

/** 🔁️ "Switch identity" and the question it opens: what switching means for this learner, the safe answer focused. */
export function SwitchIdentity(props: Pick<LearnerProps, "session" | "state" | "text">): ReactElement | null {
  const { session, state, text } = props;
  const id = useId();
  const [asking, setAsking] = useState(false);
  const learner = state.learner;
  if (learner === undefined) return null;
  const anonymous = learner.identity === undefined || learner.identity.kind === "anonymous";
  const name = learnerName(learner.identity, learnerTag(learner.id), text);
  const close = (): void => setAsking(false);
  return (
    <>
      <CardAction onClick={() => setAsking(true)}>{text("quiz.identity.switch")}</CardAction>
      {asking ? (
        <Dialog
          id={`${id}-title`}
          role="alertdialog"
          icon={<CardIcon icon="user" />}
          title={text("quiz.identity.switchTitle")}
          describedBy={`${id}-body`}
          onEscape={close}
          footerLeft={
            <CardAction data-autofocus="" onClick={close}>
              {text("quiz.identity.switchKeep")}
            </CardAction>
          }
          footerRight={
            <CardAction
              primary
              onClick={() => {
                close();
                session.forgetLearner();
              }}
            >
              {text(anonymous ? "quiz.identity.switchLose" : "quiz.identity.switch")}
            </CardAction>
          }
        >
          <p id={`${id}-body`} className="m-0 text-sm leading-normal">
            {text(anonymous ? "quiz.identity.switchAnonymous" : "quiz.identity.switchNamed", { name })}
          </p>
        </Dialog>
      ) : null}
    </>
  );
}

/** 🃏️ The learner's card on the overview; its heading opens the profile (`#learner`). */
export function LearnerCard(props: LearnerProps & { readonly revealed: boolean; readonly onOpen: () => void }): ReactElement | null {
  const { state, text, locale, revealed, onOpen } = props;
  const id = useId();
  const shown = totals(props);
  if (shown === undefined || state.catalog === undefined) return null;
  return (
    <QuizCard
      id={id}
      card="learner"
      anchor={PRESENCE_ANCHORS.home("learner")}
      href={`#${HOME_PAGES.learner}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<CardIcon icon="user" />}
      title={shown.name}
      footerRight={<SwitchIdentity {...props} />}
    >
      <p className="m-0 text-lg font-semibold tabular-nums">{shown.points}</p>
      <Facts items={shown.facts} />
      <PresenceList catalog={state.catalog} text={text} locale={locale} />
    </QuizCard>
  );
}

const STATUS = { open: "quiz.learner.statusOpen", submitted: "quiz.learner.statusSubmitted", voided: "quiz.learner.statusVoided" } as const;

function RunRow(props: LearnerProps & { readonly run: RunSummary; readonly busy: boolean; readonly act: Act }): ReactElement {
  const { session, state, text, locale, run, busy, act } = props;
  const quiz = state.catalog?.quizzes.find((candidate) => candidate.id === run.quiz);
  const cell = "border-b border-normal px-single py-single text-left align-top";
  return (
    <tr>
      <th scope="row" className={cn(cell, "quiz-name font-normal")}>
        {quiz === undefined ? run.quiz : localized(quiz.title, locale)}
      </th>
      <td className={cn(cell, "quiz-nowrap")}>{text(STATUS[run.status])}</td>
      <td className={cn(cell, "quiz-nowrap tabular-nums")}>{run.score === undefined ? <Missing label={text("quiz.learner.noScore")} /> : formatScore(run.score, locale)}</td>
      <td className={cn(cell, "quiz-nowrap tabular-nums")}>{formatInstant(run.startedAt, locale)}</td>
      <td className={cn(cell, "quiz-nowrap tabular-nums")}>{run.submittedAt === undefined ? <Missing label={text("quiz.learner.notSubmitted")} /> : formatInstant(run.submittedAt, locale)}</td>
      <td className={cn(cell, "quiz-nowrap")}>
        {run.status === "submitted" ? (
          <CardAction onClick={() => session.open({ screen: "results", run: run.run })}>{text("quiz.learner.viewResult")}</CardAction>
        ) : run.status === "open" ? (
          <CardAction primary disabled={busy} onClick={() => act((signal) => session.resumeRun(run.run, signal))}>
            {text("quiz.home.resume")}
          </CardAction>
        ) : null}
      </td>
    </tr>
  );
}

/** 📇️ The learner's profile page behind the card: totals and rank, the runs newest first, and the badges earned. */
export function LearnerPage(props: LearnerProps & { readonly view: PaneView; readonly busy: boolean; readonly act: Act }): ReactElement | null {
  const { state, text, locale } = props;
  const id = useId();
  const shown = totals(props);
  if (shown === undefined) return null;
  const runs = [...(state.learnerView?.runs ?? [])].sort((left, right) => right.startedAt - left.startedAt);
  const held = (state.catalog?.badges ?? []).flatMap((badge) => {
    const award = awardOf(state, badge.id);
    return award === undefined ? [] : [{ badge, award }];
  });
  const head = "border-b-2 border-normal px-single py-single text-left font-semibold quiz-nowrap";
  const columns = ["quiz.learner.quiz", "quiz.learner.status", "quiz.learner.score", "quiz.learner.started", "quiz.learner.submitted"] as const;
  return (
    <PageFrame page={HOME_PAGES.learner} wide>
      <QuizCard id={`${id}-profile`} card="profile" icon={<CardIcon icon="user" />} title={shown.name} footerRight={<SwitchIdentity {...props} />}>
        <p className="m-0 text-xs leading-normal">{text("quiz.home.totalHint")}</p>
        <p className="m-0 text-2xl font-semibold tabular-nums">{shown.points}</p>
        <Facts items={shown.facts} />
      </QuizCard>
      <QuizCard id={`${id}-runs`} card="runs" icon={<CardIcon icon="list" />} title={text("quiz.learner.runs")}>
        {runs.length === 0 ? (
          <p className="m-0 text-sm text-muted-foreground">{text("quiz.learner.noRuns")}</p>
        ) : (
          <div className="relative max-w-full overflow-x-auto">
            <table className="w-full border-collapse text-sm" aria-labelledby={`${id}-runs`}>
              <thead>
                <tr>
                  {columns.map((column) => (
                    <th key={column} scope="col" className={head}>
                      {text(column)}
                    </th>
                  ))}
                  <th scope="col" className={head}>
                    <span className="sr-only">{text("quiz.learner.actions")}</span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {runs.map((run) => (
                  <RunRow key={run.run} {...props} run={run} />
                ))}
              </tbody>
            </table>
          </div>
        )}
      </QuizCard>
      <QuizCard id={`${id}-badges`} card="learner-badges" icon={<CardIcon icon="award" />} title={text("quiz.learner.badges")}>
        {held.length === 0 ? (
          <p className="m-0 text-sm text-muted-foreground">{text("quiz.learner.noBadges")}</p>
        ) : (
          <ul role="list" className="m-0 flex list-none flex-col gap-single p-0">
            {held.map(({ badge, award }) => (
              <li key={badge.id} className="flex min-w-0 items-center gap-double text-sm">
                <Glyph emoji={badge.emoji} className="text-lg" />
                <span className="min-w-0 flex-1 font-medium">{localized(badge.label, locale)}</span>
                <span className="quiz-nowrap text-xs text-muted-foreground">{text("quiz.home.earnedAt", { date: formatDate(award.at, locale) })}</span>
              </li>
            ))}
          </ul>
        )}
      </QuizCard>
    </PageFrame>
  );
}
