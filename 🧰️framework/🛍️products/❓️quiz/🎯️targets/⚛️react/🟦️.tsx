/** ❓️ `@semio-tech/quiz-react` — the web renderer and proctor client of `@semio-tech/quiz`.
 *
 * Domain-neutral: it renders whatever catalog the proctor serves for `tenant`. {@link mountQuiz} is the whole seam a
 * site needs; {@link QuizApp} is the same as a React component. The steps are introduction (first visit), identity,
 * home, run, results and leaderboard; every string is English or German, the theme and text size are the learner's,
 * and answers survive short connection shortages on the device.
 *
 * @see ../../🧬️schema/🔣️.json — the contract every view, command and event follows
 */

import { StrictMode, useEffect, useMemo, useRef, useState, useSyncExternalStore, type CSSProperties, type ReactElement } from "react";
import { createRoot } from "react-dom/client";
import { learnerTag } from "@semio-tech/quiz";
import { REJECTION_LABELS, applyLocale, localized, preferredLocale, quizText, type QuizLabelKey, type QuizLocale, type QuizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
import { browserStorageArea, localStore, type StorageArea } from "./🔨️modules/💾️persistence/🟦️.ts";
import { ProctorClient, proctorTransport, type ProctorConnect, type RetryTiming } from "./🔨️modules/🛂️proctor/🟦️.ts";
import { QuizSession, type QuizConnection, type QuizNotice, type QuizState, type QuizStep } from "./🔨️modules/🧭️session/🟦️.ts";
import { LanguageSwitch, PreferencesPanel, readPreferences, resolvedTheme, textScale, useSystemDark, writePreferences, type QuizPreferences } from "./🔨️modules/🎛️preferences/🟦️.tsx";
import { IntroductionScreen } from "./🔨️modules/👋️introduction/🟦️.tsx";
import { IdentityScreen, learnerName } from "./🔨️modules/🪪️identity/🟦️.tsx";
import { HomeScreen } from "./🔨️modules/🏠️home/🟦️.tsx";
import { RunScreen } from "./🔨️modules/▶️run/🟦️.tsx";
import { ResultsScreen } from "./🔨️modules/🏁️results/🟦️.tsx";
import { LeaderboardScreen } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
import "./🎨️.css";

//#region 🔁️Reexports
export { QUIZ_BUNDLE_DE, QUIZ_BUNDLE_EN, QUIZ_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, applyLocale, isQuizLocale, localized, preferredLocale, quizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
export type { QuizLabelKey, QuizLocale, QuizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
export { SIGNIFICANT_DIGITS, SI_PREFIXES, engineering, formatDate, formatInstant, formatNumber, formatPoints, formatQuantity, formatScore, oneDecimal, withUnit } from "./🔨️modules/📏️quantity/🟦️.ts";
export { RADAR_LABEL_EM, RADAR_METRICS, RadarChart, estimateTextWidth, radarAngle, radarFraction, radarLayout, radarPoint, radarPolygon, textMeasure, wrapLabel } from "./🔨️modules/🕸️radar/🟦️.tsx";
export type { RadarAxis, RadarBox, RadarFrame, RadarLabel, RadarLayout, RadarLegendEntry, RadarPoint, TextMeasure } from "./🔨️modules/🕸️radar/🟦️.tsx";
export { browserStorageArea, isRecord, localChange, localStore, memoryStorageOrigin } from "./🔨️modules/💾️persistence/🟦️.ts";
export type { LocalChange, LocalCollection, LocalSlice, LocalStore, StorageArea } from "./🔨️modules/💾️persistence/🟦️.ts";
export {
  ProctorClient,
  ProctorUnavailable,
  QUIZ_WIRE_VERSION,
  RETRY_TIMING,
  abortable,
  commandEnvelope,
  commandTarget,
  commandVerdict,
  isNotFound,
  isTransient,
  learnerPrincipal,
  newId,
  proctorTransport,
  queryEnvelope,
  quizRejection,
  retryTransient,
} from "./🔨️modules/🛂️proctor/🟦️.ts";
export type { CommandVerdict, HttpRequest, HttpResponse, HttpTransport, ProctorConnect, ProctorReachability, RetryTiming } from "./🔨️modules/🛂️proctor/🟦️.ts";
export { Outbox, coalesce, coalescingKey } from "./🔨️modules/📮️outbox/🟦️.ts";
export type { OutboxActivity, OutboxEntry, OutboxOptions, OutboxStatus } from "./🔨️modules/📮️outbox/🟦️.ts";
export { PROJECTION_GRACE_MS, QuizSession, evolveQuizState, initialQuizState, lastSubmittedRunOf, mergeRunViews, openRunOf, restoreQuizState, runAwards } from "./🔨️modules/🧭️session/🟦️.ts";
export type { QuizClientEvent, QuizConnection, QuizLearner, QuizNotice, QuizSessionOptions, QuizSnapshot, QuizState, QuizStep, SessionFailure, SubmissionPhase } from "./🔨️modules/🧭️session/🟦️.ts";
export { LOCALE_NAMES, LanguageSwitch, PreferencesPanel, TEXT_SIZES, THEME_CHOICES, readPreferences, resolvedTheme, textScale, useSystemDark, writePreferences } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export type { QuizPreferences, TextSize, ThemeChoice } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export { DRAG_THRESHOLD_PX, dropZoneAt, startPointerDrag } from "./🔨️modules/🤏️drag/🟦️.ts";
export { DragGrip, LiveRegion, elementId, useAnnouncement, useFocusAfterRender } from "./🔨️modules/🧩️task/🟦️.tsx";
export type { TaskViewProps } from "./🔨️modules/🧩️task/🟦️.tsx";
export { ClassificationTaskView } from "./🔨️modules/🗂️classification/🟦️.tsx";
export { SortingTaskView, reordered } from "./🔨️modules/↕️sorting/🟦️.tsx";
export { MatchingTaskView, assignCard } from "./🔨️modules/🃏️matching/🟦️.tsx";
export { IntroductionScreen } from "./🔨️modules/👋️introduction/🟦️.tsx";
export { IdentityScreen, failureMessage, learnerName, thrownMessage } from "./🔨️modules/🪪️identity/🟦️.tsx";
export { HomeScreen } from "./🔨️modules/🏠️home/🟦️.tsx";
export { RunScreen, TaskView } from "./🔨️modules/▶️run/🟦️.tsx";
export { ResultsScreen, TaskResultView } from "./🔨️modules/🏁️results/🟦️.tsx";
export { LEADERBOARD_POLL_MS, LeaderboardScreen, sortLeaderboard, usePolling } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
export type { LeaderboardKey, LeaderboardSort } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
//#endregion 🔁️Reexports

//#region ❓️App
/** ⚙️ What a quiz site passes in: the proctor base URL (`""` is the site's own origin) and the catalog id as tenant.
 * The optional seams replace the network, the storage, the browser's language list and the retry timing. */
export interface QuizOptions {
  readonly proctor: string;
  readonly tenant: string;
  readonly transport?: ProctorConnect;
  readonly storage?: StorageArea;
  readonly languages?: readonly string[];
  readonly timing?: RetryTiming;
}

function browserLanguages(): readonly string[] {
  if (typeof navigator === "undefined") return [];
  return navigator.languages !== undefined && navigator.languages.length > 0 ? navigator.languages : [navigator.language];
}

/** 📶️ What the connection indicator says and how urgently: connecting until the proctor first answers, then saved,
 * saving, or answers held on the device. */
export function connectionMessage(connection: QuizConnection, text: QuizText): { readonly message: string; readonly tone: "calm" | "busy" | "alert" } {
  if (!connection.online) return { message: text("quiz.connection.offline"), tone: "alert" };
  if (connection.reachability === "unreachable")
    return connection.pending > 0 ? { message: text("quiz.connection.reconnectingWaiting", { waiting: connection.pending }), tone: "alert" } : { message: text("quiz.connection.reconnecting"), tone: "alert" };
  if (connection.pending > 0) return { message: text("quiz.connection.saving", { waiting: connection.pending }), tone: "busy" };
  if (connection.reachability === "unknown") return { message: text("quiz.connection.connecting"), tone: "busy" };
  return { message: text("quiz.connection.saved"), tone: "calm" };
}

/** ⏳️ What a screen whose data has not arrived yet says: connecting, unreachable (with a retry), refused, or loading. */
export function waitingMessage(connection: QuizConnection, notice: QuizNotice | undefined, text: QuizText): { readonly message: string; readonly retry: boolean } {
  if (!connection.online) return { message: text("quiz.connection.offline"), retry: true };
  if (connection.reachability === "unreachable") return { message: text("quiz.app.unreachable"), retry: true };
  if (connection.reachability === "unknown") return { message: text("quiz.connection.connecting"), retry: false };
  if (notice?.kind === "refused") return { message: text("quiz.app.failed"), retry: true };
  return { message: text("quiz.app.fetching"), retry: false };
}

function Waiting(props: { readonly connection: QuizConnection; readonly notice: QuizNotice | undefined; readonly text: QuizText; readonly onRetry: () => void }): ReactElement {
  const { connection, notice, text, onRetry } = props;
  const waiting = waitingMessage(connection, notice, text);
  return (
    <section className="quiz-panel quiz-waiting" aria-busy="true">
      <h1 tabIndex={-1}>{text("quiz.app.loading")}</h1>
      <p role="status">{waiting.message}</p>
      {waiting.retry ? (
        <div className="quiz-actions">
          <button type="button" className="quiz-button" onClick={onRetry}>
            {text("quiz.app.retry")}
          </button>
        </div>
      ) : null}
    </section>
  );
}

function noticeMessage(notice: QuizNotice, text: QuizText): string {
  switch (notice.kind) {
    case "rejection":
      return text(REJECTION_LABELS[notice.rejection]);
    case "refused":
      return text("quiz.rejection.refused", { detail: notice.detail });
    case "voided":
      return text("quiz.run.voided");
  }
}

const NAVIGATION: readonly { readonly screen: "home" | "leaderboard" | "introduction"; readonly label: QuizLabelKey }[] = [
  { screen: "home", label: "quiz.nav.home" },
  { screen: "leaderboard", label: "quiz.nav.leaderboard" },
  { screen: "introduction", label: "quiz.nav.introduction" },
];

function stepKey(step: QuizStep): string {
  return step.screen === "run" || step.screen === "results" ? `${step.screen}:${step.run}` : step.screen;
}

function Screen(props: { readonly session: QuizSession; readonly state: QuizState; readonly connection: QuizConnection; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { session, state, connection, text, locale } = props;
  const step = state.step;
  const waiting = <Waiting connection={connection} notice={state.notice} text={text} onRetry={() => session.reconnect()} />;
  if (step.screen === "introduction") {
    if (state.catalog === undefined) return waiting;
    return <IntroductionScreen catalog={state.catalog} text={text} locale={locale} onContinue={() => (state.introduced ? session.open({ screen: state.learner === undefined ? "identity" : "home" }) : session.readIntroduction())} />;
  }
  if (step.screen === "identity" || state.learner === undefined) return <IdentityScreen session={session} text={text} />;
  switch (step.screen) {
    case "home":
      return state.catalog === undefined ? waiting : <HomeScreen session={session} state={state} text={text} locale={locale} />;
    case "leaderboard":
      return state.catalog === undefined ? waiting : <LeaderboardScreen session={session} state={state} text={text} locale={locale} />;
    case "run":
      return state.runs[step.run] === undefined ? waiting : <RunScreen key={step.run} session={session} state={state} run={step.run} text={text} locale={locale} />;
    case "results":
      return state.runs[step.run] === undefined ? waiting : <ResultsScreen session={session} state={state} run={step.run} text={text} locale={locale} />;
  }
}

/** ❓️ The whole quiz client for one proctor tenant. */
export function QuizApp(options: QuizOptions): ReactElement {
  const [setup] = useState(() => {
    const store = localStore(options.storage ?? browserStorageArea(), options.tenant);
    const proctor = new ProctorClient(options.transport ?? proctorTransport(options.proctor), options.tenant);
    return { store, session: new QuizSession({ proctor, store, timing: options.timing }) };
  });
  const { session, store } = setup;
  useEffect(() => {
    session.start();
    return () => session.stop();
  }, [session]);
  const { state, connection } = useSyncExternalStore(session.subscribe, session.getSnapshot, session.getSnapshot);
  const [preferences, setPreferences] = useState<QuizPreferences>(() => readPreferences(store));
  const [settingsOpen, setSettingsOpen] = useState(false);
  const systemDark = useSystemDark();
  const locale = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());
  const text = useMemo(() => quizText(locale), [locale]);
  const main = useRef<HTMLElement>(null);
  const settingsButton = useRef<HTMLButtonElement>(null);
  const shown = useRef<string | undefined>(undefined);
  const key = stepKey(state.step);
  useEffect(() => {
    void applyLocale(locale);
  }, [locale]);
  useEffect(() => store.watch((change) => (change.kind === "cleared" || (change.kind === "slice" && change.slice === "preferences")) && setPreferences(readPreferences(store))), [store]);
  useEffect(() => {
    if (shown.current !== undefined && shown.current !== key) main.current?.querySelector<HTMLElement>("h1")?.focus();
    shown.current = key;
  }, [key]);

  const change = (next: QuizPreferences): void => {
    setPreferences(next);
    writePreferences(store, next);
  };
  const status = connectionMessage(connection, text);
  const identified = state.introduced && state.learner !== undefined;
  const style = { "--quiz-text-scale": String(textScale(preferences.textSize)) } as CSSProperties;
  return (
    <div className="quiz-app" data-theme={resolvedTheme(preferences.theme, systemDark)} lang={locale} style={style}>
      <a
        className="quiz-skip"
        href="#quiz-main"
        onClick={(event) => {
          event.preventDefault();
          main.current?.focus();
        }}
      >
        {text("quiz.app.skip")}
      </a>
      <header className="quiz-header">
        <p className="quiz-brand">{state.catalog === undefined ? "" : localized(state.catalog.title, locale)}</p>
        {identified ? (
          <nav className="quiz-nav" aria-label={text("quiz.nav.label")}>
            <ul>
              {NAVIGATION.map((entry) => (
                <li key={entry.screen}>
                  <button type="button" aria-current={state.step.screen === entry.screen ? "page" : undefined} onClick={() => session.open({ screen: entry.screen })}>
                    {text(entry.label)}
                  </button>
                </li>
              ))}
            </ul>
          </nav>
        ) : null}
        <div className="quiz-tools">
          {identified && state.learner !== undefined ? <p className="quiz-learner">{text("quiz.identity.current", { name: learnerName(state.learner.identity, learnerTag(state.learner.id), text) })}</p> : null}
          <p className="quiz-connection" data-tone={status.tone} role="status" aria-live="polite">
            <span className="quiz-visually-hidden">{text("quiz.connection.label")}: </span>
            <span aria-hidden="true">{status.tone === "alert" ? "⚠ " : status.tone === "busy" ? "↻ " : "✓ "}</span>
            {status.message}
          </p>
          <LanguageSwitch locale={locale} text={text} onChange={(chosen) => change({ ...preferences, locale: chosen })} />
          <button ref={settingsButton} type="button" className="quiz-button quiz-button-quiet" aria-expanded={settingsOpen} aria-controls="quiz-settings" onClick={() => setSettingsOpen(!settingsOpen)}>
            {text("quiz.nav.settings")}
          </button>
        </div>
      </header>
      {settingsOpen ? (
        <section id="quiz-settings" className="quiz-settings" aria-labelledby="quiz-settings-title">
          <h2 id="quiz-settings-title">{text("quiz.preferences.title")}</h2>
          <PreferencesPanel preferences={preferences} text={text} onChange={change} />
          <button
            type="button"
            className="quiz-button"
            onClick={() => {
              setSettingsOpen(false);
              settingsButton.current?.focus();
            }}
          >
            {text("quiz.preferences.close")}
          </button>
        </section>
      ) : null}
      {state.notice === undefined ? null : (
        <div className="quiz-notice" role="alert">
          <p>{noticeMessage(state.notice, text)}</p>
          <button type="button" className="quiz-button" onClick={() => session.dismissNotice()}>
            {text("quiz.app.dismiss")}
          </button>
        </div>
      )}
      <main id="quiz-main" ref={main} className="quiz-main" tabIndex={-1}>
        <Screen session={session} state={state} connection={connection} text={text} locale={locale} />
      </main>
    </div>
  );
}

/** 🚀️ Mounts the quiz client into `root` and returns the function that unmounts it again. */
export function mountQuiz(root: HTMLElement, options: QuizOptions): () => void {
  const reactRoot = createRoot(root);
  reactRoot.render(
    <StrictMode>
      <QuizApp {...options} />
    </StrictMode>,
  );
  return () => reactRoot.unmount();
}
//#endregion ❓️App
