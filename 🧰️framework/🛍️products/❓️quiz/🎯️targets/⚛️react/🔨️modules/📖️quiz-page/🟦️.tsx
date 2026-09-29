/** 📖️ A quiz on home: its card on the overview and its read-only page behind it — title, description, its tasks with
 * their kinds, the best score, the last result and the one action that fits the learner's runs (start, resume or start
 * again) — and what the others think of its items: live while anyone thinks along, else what the submitted runs
 * answered, labelled from a sheet of the learner's own runs of the quiz. Both are pure views over session state:
 * showing, hovering or revealing a quiz never starts a run; only its action, pressed, does.
 *
 * @see ../🏠️home/🟦️.tsx — the overview the card sits on and the pane the page fills
 */

import { useEffect, useId, type ReactElement } from "react";
import { roomScope, thinkingScope, type CatalogQuizView, type RunView, type SheetTask } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatQuantity, formatScore } from "../📏️quantity/🟦️.ts";
import { badgesEarnedIn } from "../🏅️badges/🟦️.tsx";
import { TASK_KIND_ICONS } from "../▶️run/🟦️.tsx";
import { CardAction, CardIcon, Facts, Glyph, PageFrame, QuizCard, textPresentation, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS, PanePeers, usePresenceView } from "../👥️presence/🟦️.tsx";
import { CrowdChoices, CrowdPosition, CrowdProvider, CrowdSource, chooseCrowd, type Crowd } from "../🗳️crowd/🟦️.tsx";
import { lastSubmittedRunOf, openRunOf, type QuizSession, type QuizState, type SessionFailure } from "../🧭️session/🟦️.ts";

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
}

function actionsOf(props: QuizProps): { readonly primary: ReactElement; readonly last: ReactElement | undefined } {
  const { quiz, session, state, text, busy, act } = props;
  const open = openRunOf(state, quiz.id);
  const best = state.learnerView?.best[quiz.id];
  const last = lastSubmittedRunOf(state, quiz.id);
  const primary =
    open === undefined ? (
      <CardAction primary disabled={busy} onClick={() => act((signal) => session.startRun(quiz.id, signal))}>
        {text(best === undefined ? "quiz.home.start" : "quiz.home.again")}
      </CardAction>
    ) : (
      <CardAction primary disabled={busy} onClick={() => act((signal) => session.resumeRun(open, signal))}>
        {text("quiz.home.resume")}
      </CardAction>
    );
  return { primary, last: last === undefined ? undefined : <CardAction onClick={() => session.open({ screen: "results", run: last })}>{text("quiz.home.lastResult")}</CardAction> };
}

function facts(props: QuizProps, learning: number): readonly string[] {
  const { quiz, state, text, locale } = props;
  const best = state.learnerView?.best[quiz.id];
  return [
    text("quiz.home.tasks", { amount: quiz.tasks.length }),
    best === undefined ? text("quiz.home.notYet") : text("quiz.home.best", { score: formatScore(best, locale) }),
    ...(openRunOf(state, quiz.id) === undefined ? [] : [text("quiz.home.open")]),
    ...(learning === 0 ? [] : [text("quiz.presence.learningNow", { count: learning })]),
  ];
}

function Earned(props: QuizProps): ReactElement | null {
  const { quiz, state, text, locale } = props;
  const earned = badgesEarnedIn(state, quiz.id);
  if (earned.length === 0) return null;
  return <p className="m-0 mt-auto text-xs text-muted-foreground">{text("quiz.home.earnedHere", { badges: earned.map((badge) => `${textPresentation(badge.emoji)} ${localized(badge.label, locale)}`).join(", ") })}</p>;
}

/** 🃏️ The card of a quiz on the overview; its heading opens the quiz's page (`#<quiz>`). */
export function QuizCardView(props: QuizProps & { readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { quiz, locale, revealed, onOpen } = props;
  const id = useId();
  const learning = usePresenceView().roster?.quizzes[quiz.id] ?? 0;
  const { primary, last } = actionsOf(props);
  return (
    <QuizCard
      id={id}
      card={`quiz:${quiz.id}`}
      anchor={PRESENCE_ANCHORS.home(`quiz:${quiz.id}`)}
      href={`#${quiz.id}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<Glyph emoji={quiz.emoji} className="text-sm" />}
      title={localized(quiz.title, locale)}
      footerLeft={last}
      footerRight={primary}
    >
      <p className="m-0 text-xs leading-normal text-foreground">{localized(quiz.description, locale)}</p>
      <Facts items={facts(props, learning)} />
      <Earned {...props} />
    </QuizCard>
  );
}

/** 📄️ The newest run of `quiz` the session holds with its sheet, whose items label what the others think. */
export function sheetOfQuiz(state: QuizState, quiz: string): RunView | undefined {
  return Object.values(state.runs)
    .filter((view) => view.quiz === quiz)
    .sort((left, right) => right.startedAt - left.startedAt)[0];
}

function CrowdTaskItems(props: { readonly task: SheetTask; readonly crowd: Crowd; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { task, crowd, text, locale } = props;
  const label = (id: string): string => localized(task.items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
  if (task.kind === "matching") {
    return (
      <>
        {task.dimensions.map((dimension) => {
          const items = crowd.items(task, dimension.id);
          return task.items.map((item) => (
            <CrowdChoices
              key={`${dimension.id}:${item.id}`}
              item={items.get(item.id)}
              subject={`${label(item.id)} (${localized(dimension.quantity.label, locale)})`}
              label={(key) => formatQuantity(Number(key), dimension.quantity, locale)}
              text={text}
            />
          ));
        })}
      </>
    );
  }
  const items = crowd.items(task);
  return (
    <>
      {task.items.map((item) =>
        task.kind === "sorting" ? (
          <CrowdPosition key={item.id} item={items.get(item.id)} total={task.items.length} subject={label(item.id)} text={text} locale={locale} />
        ) : (
          <CrowdChoices key={item.id} item={items.get(item.id)} subject={label(item.id)} label={(key) => localized(task.categories.find((category) => category.id === key)?.label ?? { en: key, de: key }, locale)} text={text} />
        ),
      )}
    </>
  );
}

/** 👥️ What the others think of the quiz, task by task and item by item (labelled from the learner's own sheet); only
 * where the crowd comes from while the learner has no sheet of it yet, and that nobody answered while nobody did.
 * Nothing while the learner hides what the others think (`shown`). */
function QuizCrowd(props: QuizProps & { readonly crowd: Crowd | undefined; readonly shown: boolean }): ReactElement | null {
  const { quiz, state, text, locale, crowd } = props;
  const id = useId();
  const sheet = sheetOfQuiz(state, quiz.id);
  if (!props.shown) return null;
  if (crowd === undefined)
    return (
      <QuizCard id={`${id}-crowd`} card="quiz-crowd" icon={<CardIcon icon="users" />} title={text("quiz.crowd.pastTitle")}>
        <p className="m-0 text-sm text-muted-foreground">{text("quiz.crowd.nobody")}</p>
      </QuizCard>
    );
  return (
    <QuizCard id={`${id}-crowd`} card="quiz-crowd" icon={<CardIcon icon="users" />} title={text(crowd.source === "live" ? "quiz.crowd.liveTitle" : "quiz.crowd.pastTitle")}>
      <CrowdSource crowd={crowd} text={text} />
      {(sheet?.sheet.tasks ?? []).map((task) => (
        <section key={task.id} aria-labelledby={`${id}-${task.id}`} className="flex flex-col gap-single">
          <h3 id={`${id}-${task.id}`} className="m-0 text-sm font-semibold">
            {localized(task.title, locale)}
          </h3>
          <CrowdTaskItems task={task} crowd={crowd} text={text} locale={locale} />
        </section>
      ))}
    </QuizCard>
  );
}

/** 📖️ The page of a quiz behind its card: what it asks, how the learner stands, the action that fits, what the others
 * think, and the others on this page. */
export function QuizPage(props: QuizProps & { readonly view: PaneView; readonly showAnswers?: boolean }): ReactElement {
  const { quiz, session, state, text, locale, view } = props;
  const id = useId();
  const { roster, thinking, colours } = usePresenceView();
  const learning = roster?.quizzes[quiz.id] ?? 0;
  const { primary, last } = actionsOf(props);
  const catalog = state.catalog?.id;
  const latest = state.learnerView?.runs.filter((run) => run.quiz === quiz.id).sort((left, right) => right.startedAt - left.startedAt)[0]?.run;
  const held = sheetOfQuiz(state, quiz.id) !== undefined;
  useEffect(() => {
    if (!held && latest !== undefined) void session.loadRun(latest);
  }, [held, latest, session]);
  const others = catalog === undefined ? [] : (thinking.get(thinkingScope(catalog, quiz.id)) ?? []);
  const crowd = props.showAnswers === false ? undefined : chooseCrowd(others, state.crowds[quiz.id]);
  return (
    <PageFrame page={`quiz:${quiz.id}`} overlay={<PanePeers scope={catalog === undefined ? undefined : roomScope(catalog, { screen: "quiz", quiz: quiz.id })} opened={view.opened} />}>
      <QuizCard id={`${id}-quiz`} card="quiz" anchor={PRESENCE_ANCHORS.quiz(quiz.id)} icon={<Glyph emoji={quiz.emoji} className="text-sm" />} title={localized(quiz.title, locale)} footerLeft={last} footerRight={primary}>
        <p className="m-0 text-sm leading-normal">{localized(quiz.description, locale)}</p>
        <Facts items={facts(props, learning)} />
        <p className="m-0 text-xs text-muted-foreground">{text("quiz.quizPage.hint")}</p>
        <Earned {...props} />
      </QuizCard>
      <QuizCard id={`${id}-tasks`} card="quiz-tasks" anchor={PRESENCE_ANCHORS.quizTasks(quiz.id)} icon={<CardIcon icon="list" />} title={text("quiz.quizPage.tasks")}>
        <ol className="m-0 flex list-none flex-col gap-single p-0">
          {quiz.tasks.map((task, index) => (
            <li key={task.id} className="flex min-w-0 items-center gap-double border border-normal px-double py-single text-sm">
              <span className="quiz-nowrap w-[2ch] shrink-0 font-semibold tabular-nums text-muted-foreground">{index + 1}</span>
              <CardIcon icon={TASK_KIND_ICONS[task.kind]} />
              <span className="min-w-0 flex-1 font-medium">{localized(task.title, locale)}</span>
              <span className="text-xs text-muted-foreground">{text(TASK_KIND_LABELS[task.kind])}</span>
            </li>
          ))}
        </ol>
      </QuizCard>
      <CrowdProvider crowd={crowd} colours={colours}>
        <QuizCrowd {...props} crowd={crowd} shown={props.showAnswers !== false} />
      </CrowdProvider>
    </PageFrame>
  );
}
