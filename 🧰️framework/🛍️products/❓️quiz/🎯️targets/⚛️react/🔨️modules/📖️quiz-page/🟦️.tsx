/** 📖️ A quiz on home: its card on the overview and its read-only page behind it — title, description, its tasks with
 * their kinds, the best run as points with its challenge, the last result and the one action that fits the learner's
 * runs (start, resume or start again, each naming its challenge), on the page the challenge chooser — and, once the learner has submitted the quiz or asks for it, what everyone answered: how all runs scored and
 * a figure per task, labelled from a sheet of the learner's own runs of the quiz. Both are pure views over session state:
 * showing, hovering or revealing a quiz never starts a run; only its action, pressed, does.
 *
 * @see ../🏠️home/🟦️.tsx — the overview the card sits on and the pane the page fills
 */

import { useEffect, useId, useState, type ReactElement, type ReactNode } from "react";
import { challengeRules, roomScope, thinkingScope, type CatalogQuizView, type Challenge, type RunView, type ThinkingState } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatPoints } from "../📏️quantity/🟦️.ts";
import { CHALLENGE_LABELS, ChallengeChooser } from "../⛰️challenge/🟦️.tsx";
import { badgesEarnedIn } from "../🏅️badges/🟦️.tsx";
import { CardAction, CardIcon, Dialog, Facts, Glyph, IconLabel, PageFrame, QuizCard, TaskGlyph, textPresentation, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS, PanePeers, usePresenceView } from "../👥️presence/🟦️.tsx";
import { CrowdDoor, ScoreFigure, TaskFigures, crowdGate, crowdShown, type CrowdGate, type OthersChoice } from "../🗳️crowd/🟦️.tsx";
import { lastSubmittedRunOf, openChallengeOf, openRunOf, type QuizSession, type QuizState, type SessionFailure } from "../🧭️session/🟦️.ts";
import { petProp } from "../🐾️pets/🟦️.tsx";

/** 🎬️ Runs one interactive command of home (starting or resuming a run) with progress, cancellation and its message. */
export type Act = (run: (signal: AbortSignal) => Promise<SessionFailure | undefined>) => void;

interface QuizProps {
  readonly quiz: CatalogQuizView;
  readonly session: QuizSession;
  readonly state: QuizState;
  readonly text: QuizText;
  readonly locale: QuizLocale;
  readonly busy: boolean;
  readonly act: Act;
  readonly challenge: Challenge;
}

/** 🕹️ The actions of a quiz: the last result once there is one; without an open run the start at the chosen challenge
 * (again, once submitted); with one the way back into it at its own challenge — unless `ask` is given (the page) and
 * another challenge is chosen: then the start asks through `ask` before the open run is discarded, and the way back
 * stays beside it. */
function actionsOf(props: QuizProps, ask?: () => void): { readonly primary: ReactElement; readonly secondary: readonly ReactElement[] } {
  const { quiz, session, state, text, busy, act, challenge } = props;
  const open = openRunOf(state, quiz.id);
  const opened = openChallengeOf(state, quiz.id) ?? challenge;
  const last = lastSubmittedRunOf(state, quiz.id);
  const shown = last === undefined ? [] : [<CardAction key="last" onClick={() => session.open({ screen: "results", run: last })}>{text("quiz.home.lastResult")}</CardAction>];
  const start = (
    <CardAction primary disabled={busy} onClick={open === undefined ? () => act((signal) => session.startRun(quiz.id, challenge, signal)) : ask}>
      {text(state.learnerView?.best[quiz.id] === undefined ? "quiz.home.start" : "quiz.home.again", { challenge: text(CHALLENGE_LABELS[challenge]) })}
    </CardAction>
  );
  if (open === undefined) return { primary: start, secondary: shown };
  const resume = (
    <CardAction key="resume" primary={ask === undefined || opened === challenge} disabled={busy} onClick={() => act((signal) => session.resumeRun(open, signal))}>
      {text("quiz.home.resume", { challenge: text(CHALLENGE_LABELS[opened]) })}
    </CardAction>
  );
  return ask === undefined || opened === challenge ? { primary: resume, secondary: shown } : { primary: start, secondary: [...shown, resume] };
}

/** 📋️ The facts of a quiz: its tasks, the learner's best run as points with its challenge, an open run, and how many
 * are learning it now — a fact that keeps its place, invisible and unspoken, while nobody is, so the card does not move
 * when the first learner starts and the last one leaves. */
function facts(props: QuizProps, learning: number): readonly ReactNode[] {
  const { quiz, state, text, locale } = props;
  const best = state.learnerView?.best[quiz.id];
  return [
    text("quiz.home.tasks", { amount: quiz.tasks.length }),
    best === undefined ? text("quiz.home.notYet") : text("quiz.home.best", { points: formatPoints(best.points, locale), par: formatPoints(challengeRules(best.challenge).par, locale), challenge: text(CHALLENGE_LABELS[best.challenge]) }),
    ...(openRunOf(state, quiz.id) === undefined ? [] : [text("quiz.home.open")]),
    <span data-learning={learning} aria-hidden={learning === 0 ? true : undefined} className={learning === 0 ? "invisible" : undefined}>
      {text("quiz.presence.learningNow", { count: learning })}
    </span>,
  ];
}

function Earned(props: QuizProps): ReactElement | null {
  const { quiz, state, text, locale } = props;
  const earned = badgesEarnedIn(state, quiz.id);
  if (earned.length === 0) return null;
  return <p className="m-0 mt-auto text-xs text-muted-foreground">{text("quiz.home.earnedHere", { badges: earned.map((badge) => `${textPresentation(badge.emoji)} ${localized(badge.label, locale)}`).join(", ") })}</p>;
}

/** 🃏️ The card of a quiz on the overview; its heading opens the quiz's page (`#<quiz>`). Pets may match it with their
 * topic, the quiz, but never lift it. */
export function QuizCardView(props: QuizProps & { readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { quiz, locale, revealed, onOpen } = props;
  const id = useId();
  const learning = usePresenceView().roster?.quizzes[quiz.id] ?? 0;
  const { primary, secondary } = actionsOf(props);
  return (
    <QuizCard
      id={id}
      card={`quiz:${quiz.id}`}
      anchor={PRESENCE_ANCHORS.home(`quiz:${quiz.id}`)}
      topic={quiz.id}
      href={`#${quiz.id}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<Glyph emoji={quiz.emoji} className="text-sm" />}
      title={localized(quiz.title, locale)}
      footerLeft={secondary.length === 0 ? undefined : secondary}
      footerRight={primary}
    >
      <p className="m-0 text-xs leading-normal text-foreground">{localized(quiz.description, locale)}</p>
      <Facts items={facts(props, learning)} />
      <Earned {...props} />
    </QuizCard>
  );
}

/** 📄️ The newest run of `quiz` the session holds with its sheet, whose items label what everyone answered. */
export function sheetOfQuiz(state: QuizState, quiz: string): RunView | undefined {
  return Object.values(state.runs)
    .filter((view) => view.quiz === quiz)
    .sort((left, right) => right.startedAt - left.startedAt)[0];
}

/** 👥️ What everyone answered in the quiz. Behind a locked gate only the door that an ask opens; else how all runs
 * scored — the learner's best marked — and a figure per task of the learner's own sheet with the learner's newest
 * answers marked (only the scores while the learner has no sheet of the quiz yet), or that nobody answered while nobody
 * did. The card is the last of its page, so nothing moves when it grows. Nothing for a learner who never wants to see
 * the others. */
function QuizCrowd(props: QuizProps & { readonly gate: CrowdGate; readonly thinking: readonly ThinkingState[] }): ReactElement | null {
  const { quiz, session, state, text, locale, gate, thinking } = props;
  const id = useId();
  const sheet = sheetOfQuiz(state, quiz.id);
  const crowd = state.crowds[quiz.id];
  if (gate === "off") return null;
  const title = localized(quiz.title, locale);
  return (
    <QuizCard id={`${id}-crowd`} card="quiz-crowd" icon={<CardIcon icon="users" />} title={text("quiz.crowd.title")}>
      <CrowdDoor gate={gate} onAsk={() => session.askCrowd(quiz.id)} onUnask={() => session.unaskCrowd(quiz.id)} text={text} />
      {!crowdShown(gate) ? null : (crowd?.runs ?? 0) === 0 && thinking.length === 0 ? (
        <p data-crowd-empty="" className="m-0 text-sm text-muted-foreground">
          {text("quiz.crowd.nobody")}
        </p>
      ) : (
        <>
          <ScoreFigure name={text("quiz.crowd.scoresFigure", { subject: title })} bins={crowd?.scores} own={state.learnerView?.best[quiz.id]?.score} text={text} locale={locale} />
          {(sheet?.sheet.tasks ?? []).map((task) => (
            <section key={task.id} aria-labelledby={`${id}-${task.id}`} className="flex flex-col gap-single">
              <h3 id={`${id}-${task.id}`} className="m-0 text-sm font-semibold">
                <IconLabel icon={task.icon}>{localized(task.title, locale)}</IconLabel>
              </h3>
              <TaskFigures task={task} title={localized(task.title, locale)} crowd={crowd} thinking={thinking} answer={sheet?.answers[task.id]} text={text} locale={locale} />
            </section>
          ))}
        </>
      )}
    </QuizCard>
  );
}

/** 📃️ The page of a quiz behind its card: what it asks, how the learner stands, the challenge of the next run — the
 * one chosen last, remembered on the device —, the action that fits, what everyone answered — for a learner who chose
 * when the `others` show (once submitted, unless said otherwise; never while a run that hides the keys is open) — and
 * the others on this page. With an open run at another challenge than the chosen one, the start asks before the open
 * run is discarded, and the way back into it stays offered. Every task of the list is marked with its topic
 * (`<quiz>/<task>`), so the pets of that task may play with it. */
export function QuizPage(props: QuizProps & { readonly view: PaneView; readonly others?: OthersChoice; readonly onChallenge: (challenge: Challenge) => void }): ReactElement {
  const { quiz, session, state, text, locale, view, challenge, act, onChallenge } = props;
  const id = useId();
  const [asking, setAsking] = useState(false);
  const { roster, thinking } = usePresenceView();
  const learning = roster?.quizzes[quiz.id] ?? 0;
  const { primary, secondary } = actionsOf(props, () => setAsking(true));
  const opened = openChallengeOf(state, quiz.id);
  const catalog = state.catalog?.id;
  const latest = state.learnerView?.runs.filter((run) => run.quiz === quiz.id).sort((left, right) => right.startedAt - left.startedAt)[0]?.run;
  const held = sheetOfQuiz(state, quiz.id) !== undefined;
  useEffect(() => {
    if (!held && latest !== undefined) void session.loadRun(latest);
  }, [held, latest, session]);
  const gate = crowdGate(props.others ?? "submitted", "quiz", { asked: state.asked.includes(quiz.id), submitted: lastSubmittedRunOf(state, quiz.id) !== undefined, hidden: opened !== undefined && !challengeRules(opened).keys });
  const close = (): void => setAsking(false);
  return (
    <PageFrame page={`quiz:${quiz.id}`} overlay={<PanePeers scope={catalog === undefined ? undefined : roomScope(catalog, { screen: "quiz", quiz: quiz.id })} opened={view.opened} />}>
      <QuizCard id={`${id}-quiz`} card="quiz" anchor={PRESENCE_ANCHORS.quiz(quiz.id)} icon={<Glyph emoji={quiz.emoji} className="text-sm" />} title={localized(quiz.title, locale)} footerLeft={secondary.length === 0 ? undefined : secondary} footerRight={primary}>
        <p className="m-0 text-sm leading-normal">{localized(quiz.description, locale)}</p>
        <Facts items={facts(props, learning)} />
        <ChallengeChooser challenge={challenge} onChange={onChallenge} text={text} locale={locale} />
        {opened === undefined ? null : <p className="m-0 text-xs">{text("quiz.challenge.openRun", { challenge: text(CHALLENGE_LABELS[opened]) })}</p>}
        <p className="m-0 text-xs text-muted-foreground">{text("quiz.quizPage.hint")}</p>
        <Earned {...props} />
      </QuizCard>
      {asking && opened !== undefined ? (
        <Dialog
          id={`${id}-discard`}
          role="alertdialog"
          icon={<CardIcon icon="triangle-alert" />}
          title={text("quiz.challenge.discardTitle")}
          describedBy={`${id}-discard-body`}
          onEscape={close}
          footerLeft={
            <CardAction data-autofocus="" onClick={close}>
              {text("quiz.challenge.keep")}
            </CardAction>
          }
          footerRight={
            <CardAction
              primary
              onClick={() => {
                close();
                act((signal) => session.startRun(quiz.id, challenge, signal));
              }}
            >
              {text("quiz.challenge.discard")}
            </CardAction>
          }
        >
          <p id={`${id}-discard-body`} className="m-0 text-sm leading-normal">
            {text("quiz.challenge.discardBody", { open: text(CHALLENGE_LABELS[opened]), chosen: text(CHALLENGE_LABELS[challenge]) })}
          </p>
        </Dialog>
      ) : null}
      <QuizCard id={`${id}-tasks`} card="quiz-tasks" anchor={PRESENCE_ANCHORS.quizTasks(quiz.id)} icon={<CardIcon icon="list" />} title={text("quiz.quizPage.tasks")}>
        <ol role="list" className="m-0 flex list-none flex-col gap-single p-0">
          {quiz.tasks.map((task, index) => (
            <li key={task.id} data-pet-prop={petProp(quiz.id, task.id)} className="flex min-w-0 items-center gap-double border border-normal px-double py-single text-sm">
              <span className="quiz-nowrap w-[2ch] shrink-0 font-semibold tabular-nums text-muted-foreground">{index + 1}</span>
              <TaskGlyph task={task} />
              <span className="min-w-0 flex-1 font-medium">{localized(task.title, locale)}</span>
              <span className="text-xs text-muted-foreground">{text(TASK_KIND_LABELS[task.kind])}</span>
            </li>
          ))}
        </ol>
      </QuizCard>
      <QuizCrowd {...props} gate={gate} thinking={catalog === undefined ? [] : (thinking.get(thinkingScope(catalog, quiz.id)) ?? [])} />
    </PageFrame>
  );
}
