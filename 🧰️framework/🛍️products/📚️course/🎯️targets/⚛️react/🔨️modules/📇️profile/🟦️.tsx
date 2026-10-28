/** 🏫️ The learner on home: the card with the totals, who else is online and the way to switch identity, and the
 * profile page behind it — totals and the rank on the all-time leaderboard of every quiz, every run with its status, challenge, score, points and dates (the result of a submitted run,
 * the way back into an open one), and the badges earned with their days. Both are pure views over session state; the
 * profile shares its public presentation and presence cursors. Switching identity always asks first: an anonymous learner
 * gives up the only key to the progress and has to say so, a pseudonymous or named one is told how to come back.
 *
 * @see ../🏠️home/🟦️.tsx — the overview the card sits on and the pane the page fills
 * @see https://www.w3.org/WAI/WCAG22/Understanding/error-prevention-legal-financial-data.html — confirm what cannot be undone
 */

import { clientAccess, useReleaseClock } from "../🔓️access/🟦️.tsx";
import { useId, useState, type ReactElement } from "react";
import { challengeRules, learnerTag, roomScope, type RunSummary } from "@semio-tech/course";
import { localized, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { CHALLENGE_LABELS } from "../⛰️challenge/🏷️labels/🟦️.ts";
import { formatDate, formatInstant, formatPoints, formatScore } from "../📏️quantity/🟦️.ts";
import { learnerName } from "../🪪️identity/🟦️.tsx";
import { awardOf } from "../🏅️badges/🟦️.tsx";
import { ownRow } from "../🏆️leaderboard/🟦️.tsx";
import type { Act } from "../📖️quiz-page/🟦️.tsx";
import { CardAction, CardIcon, Dialog, Facts, Glyph, Missing, PageFrame, CourseCard, Records, TABLE, cn, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PanePeers, PresenceList } from "../👥️presence/🟦️.tsx";
import { usePresenceFollow } from "../👀️follow/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/👁️view/🟦️.ts";
import { HOME_PAGES, overallLeaderboard, type CourseState } from "../🧭️session/🧮️state/🟦️.ts";
import { type CourseSession } from "../🧭️session/🟦️.ts";

interface LearnerProps {
  readonly session: CourseSession;
  readonly state: CourseState;
  readonly text: CourseText;
  readonly locale: CourseLocale;
}

function totals(props: LearnerProps, publicTag?: string): { readonly name: string; readonly points: string; readonly facts: readonly string[] } | undefined {
  const { state, text, locale } = props;
  const { learner, learnerView, course } = state;
  const leaderboard = overallLeaderboard(state);
  if (learner === undefined || course === undefined) return undefined;
  const tag = publicTag ?? learnerTag(learner.id);
  const played = course.quizzes.filter((quiz) => learnerView?.best[quiz.id] !== undefined).length;
  const earned = course.badges.filter((badge) => awardOf(state, badge.id) !== undefined).length;
  const row = leaderboard === undefined ? undefined : ownRow(leaderboard.board, tag);
  return {
    name: learnerName(learner.identity, tag, text),
    points: text("course.home.points", { points: formatPoints(learnerView?.total ?? 0, locale) }),
    facts: [
      text("course.home.played", { played, total: course.quizzes.length }),
      text("course.home.badgesEarned", { earned, total: course.badges.length }),
      ...(row === undefined || leaderboard === undefined ? [] : [text("course.home.rank", { rank: row.rank, total: leaderboard.board.learners })]),
    ],
  };
}

/** 🔁️ "Switch identity" and the question it opens: what switching means for this learner, the safe answer focused. */
export function SwitchIdentity(props: Pick<LearnerProps, "session" | "state" | "text">): ReactElement | null {
  const { session, state, text } = props;
  const id = useId();
  const [asking, setAsking] = useState(false);
  const learner = state.learner;
  const publicTag = usePresenceFollow().following;
  if (learner === undefined) return null;
  const anonymous = learner.identity === undefined || learner.identity.kind === "anonymous";
  const name = learnerName(learner.identity, publicTag ?? learnerTag(learner.id), text);
  const close = (): void => setAsking(false);
  return (
    <>
      <CardAction onClick={() => setAsking(true)}>{text("course.identity.switch")}</CardAction>
      {asking ? (
        <Dialog
          id={`${id}-title`}
          role="alertdialog"
          icon={<CardIcon icon="user" />}
          title={text("course.identity.switchTitle")}
          describedBy={`${id}-body`}
          onEscape={close}
          footerLeft={
            <CardAction data-autofocus="" onClick={close}>
              {text("course.identity.switchKeep")}
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
              {text(anonymous ? "course.identity.switchLose" : "course.identity.switch")}
            </CardAction>
          }
        >
          <p id={`${id}-body`} className="m-0 text-sm leading-normal">
            {text(anonymous ? "course.identity.switchAnonymous" : "course.identity.switchNamed", { name })}
          </p>
        </Dialog>
      ) : null}
    </>
  );
}

/** 🃏️ The learner's card on the overview; its heading opens the profile (`#learner`). */
export function LearnerCard(props: LearnerProps & { readonly revealed: boolean; readonly onOpen: () => void }): ReactElement | null {
  const { session, state, text, locale, revealed, onOpen } = props;
  const id = useId();
  const shown = totals(props, usePresenceFollow().following);
  if (state.course === undefined) return null;
  if (shown === undefined) return (
    <CourseCard id={id} card="learner" anchor={PRESENCE_ANCHORS.home("learner")} revealed={revealed} icon={<CardIcon icon="user" />} title={text("course.identity.signIn")} footerRight={<CardAction primary onClick={() => session.open({ screen: "identity" })}>{text("course.identity.signIn")}</CardAction>}>
      <p className="m-0 text-xs leading-normal text-muted-foreground">{text("course.identity.lead")}</p>
    </CourseCard>
  );
  return (
    <CourseCard
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
      <PresenceList course={state.course} text={text} locale={locale} />
    </CourseCard>
  );
}

const STATUS = { open: "course.learner.statusOpen", submitted: "course.learner.statusSubmitted", voided: "course.learner.statusVoided" } as const;

function RunRow(props: LearnerProps & { readonly run: RunSummary; readonly busy: boolean; readonly act: Act; readonly at: number }): ReactElement {
  const { session, state, text, locale, run, busy, act } = props;
  const quiz = state.course?.quizzes.find((candidate) => candidate.id === run.quiz);
  const available = clientAccess(state.course, run.quiz, props.at);
  const cell = "border-b border-normal px-single py-single text-left align-top";
  return (
    <tr {...TABLE.row}>
      <th {...TABLE.name} data-cell="name" className={cn(cell, "course-name font-normal")}>
        {quiz === undefined ? run.quiz : localized(quiz.title, locale)}
      </th>
      <td {...TABLE.cell} data-label={text("course.learner.status")} className={cn(cell, "course-nowrap")}>
        {text(STATUS[run.status])}
      </td>
      <td {...TABLE.cell} data-label={text("course.learner.challenge")} className={cn(cell, "course-nowrap")}>
        {text(CHALLENGE_LABELS[run.challenge])}
      </td>
      <td {...TABLE.cell} data-label={text("course.learner.score")} className={cn(cell, "course-nowrap tabular-nums")}>
        {run.score === undefined ? <Missing label={text("course.learner.noScore")} /> : formatScore(run.score, locale)}
      </td>
      <td {...TABLE.cell} data-label={text("course.learner.points")} className={cn(cell, "course-nowrap tabular-nums")}>
        {run.points === undefined ? <Missing label={text("course.learner.noPoints")} /> : text("course.learner.pointsOf", { points: formatPoints(run.points, locale), par: formatPoints(challengeRules(run.challenge).par, locale) })}
      </td>
      <td {...TABLE.cell} data-label={text("course.learner.started")} className={cn(cell, "course-nowrap tabular-nums")}>
        {formatInstant(run.startedAt, locale)}
      </td>
      <td {...TABLE.cell} data-label={text("course.learner.submitted")} className={cn(cell, "course-nowrap tabular-nums")}>
        {run.submittedAt === undefined ? <Missing label={text("course.learner.notSubmitted")} /> : formatInstant(run.submittedAt, locale)}
      </td>
      <td {...TABLE.cell} data-cell="lead" className={cn(cell, "course-nowrap")}>
        {run.status === "submitted" ? (
          <CardAction disabled={!available} onClick={() => session.open({ screen: "results", run: run.run })}>{text("course.learner.viewResult")}</CardAction>
        ) : run.status === "open" ? (
          <CardAction primary disabled={busy || !available} onClick={() => act((signal) => session.resumeRun(run.run, signal))}>
            {text("course.home.resume", { challenge: text(CHALLENGE_LABELS[run.challenge]) })}
          </CardAction>
        ) : null}
      </td>
    </tr>
  );
}

/** 📇️ The learner's profile page behind the card: totals and rank, the runs newest first, and the badges earned. */
export function LearnerPage(props: LearnerProps & { readonly view: PaneView; readonly busy: boolean; readonly act: Act }): ReactElement | null {
  const { state, text, locale } = props;
  const at = useReleaseClock(state.course);
  const id = useId();
  const shown = totals(props, usePresenceFollow().following);
  if (shown === undefined) return <PageFrame page={HOME_PAGES.learner}><LearnerCard {...props} revealed onOpen={() => props.session.open({ screen: "identity" })} /></PageFrame>;
  const runs = [...(state.learnerView?.runs ?? [])].sort((left, right) => right.startedAt - left.startedAt);
  const held = (state.course?.badges ?? []).flatMap((badge) => {
    const award = awardOf(state, badge.id);
    return award === undefined ? [] : [{ badge, award }];
  });
  const head = "border-b-2 border-normal px-single py-single text-left font-semibold course-nowrap";
  const columns = ["course.learner.quiz", "course.learner.status", "course.learner.challenge", "course.learner.score", "course.learner.points", "course.learner.started", "course.learner.submitted"] as const;
  return (
    <PageFrame page={HOME_PAGES.learner} wide overlay={<PanePeers scope={state.course === undefined ? undefined : roomScope(state.course.id, { screen: "learner" })} opened={props.view.opened} />}>
      <CourseCard id={`${id}-profile`} card="profile" icon={<CardIcon icon="user" />} title={shown.name} footerRight={<SwitchIdentity {...props} />}>
        <p className="m-0 text-xs leading-normal">{text("course.home.totalHint")}</p>
        <p className="m-0 text-2xl font-semibold tabular-nums">{shown.points}</p>
        <Facts items={shown.facts} />
      </CourseCard>
      <CourseCard id={`${id}-runs`} card="runs" icon={<CardIcon icon="list" />} title={text("course.learner.runs")}>
        {runs.length === 0 ? (
          <p className="m-0 text-sm text-muted-foreground">{text("course.learner.noRuns")}</p>
        ) : (
          <Records fold={60}>
            <table {...TABLE.table} className="course-fold w-full border-collapse text-sm" aria-labelledby={`${id}-runs`}>
              <thead {...TABLE.group}>
                <tr {...TABLE.row}>
                  {columns.map((column) => (
                    <th key={column} {...TABLE.column} className={head}>
                      {text(column)}
                    </th>
                  ))}
                  <th {...TABLE.column} className={head}>
                    <span className="sr-only">{text("course.learner.actions")}</span>
                  </th>
                </tr>
              </thead>
              <tbody {...TABLE.group}>
                {runs.map((run) => (
                  <RunRow key={run.run} {...props} run={run} at={at} />
                ))}
              </tbody>
            </table>
          </Records>
        )}
      </CourseCard>
      <CourseCard id={`${id}-badges`} card="learner-badges" icon={<CardIcon icon="award" />} title={text("course.learner.badges")}>
        {held.length === 0 ? (
          <p className="m-0 text-sm text-muted-foreground">{text("course.learner.noBadges")}</p>
        ) : (
          <ul role="list" className="m-0 flex list-none flex-col gap-single p-0">
            {held.map(({ badge, award }) => (
              <li key={badge.id} className="flex min-w-0 items-center gap-double text-sm">
                <Glyph emoji={badge.emoji} className="text-lg" />
                <span className="min-w-0 flex-1 font-medium">{localized(badge.label, locale)}</span>
                <span className="course-nowrap text-xs text-muted-foreground">{text("course.home.earnedAt", { date: formatDate(award.at, locale) })}</span>
              </li>
            ))}
          </ul>
        )}
      </CourseCard>
    </PageFrame>
  );
}
