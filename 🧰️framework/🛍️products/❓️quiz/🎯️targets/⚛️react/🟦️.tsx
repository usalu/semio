/** ❓️ `@semio-tech/quiz-react` — the web renderer and proctor client of `@semio-tech/quiz`.
 *
 * Domain-neutral: it renders whatever catalog the proctor serves for `tenant`. {@link mountQuiz} is the whole seam a
 * site needs; {@link QuizApp} is the same as a React component. The steps are introduction (first visit), identity,
 * home, run, results, leaderboard and badges; every string is English or German, the theme and text size are the
 * learner's, and answers survive short connection shortages on the device. Learners see who else is online and where,
 * and the cursors of the learners on the same page. The look is the semio card language of
 * `🎡️play` and the `🧺️demonstrator`: the design system's navbar and window chrome from `@semio-tech/ui-react/chrome`,
 * its tokens, glass and fonts, light and dark through its surface chrome.
 *
 * @see ../../🧬️schema/🔣️.json — the contract every view, command and event follows
 * @see ../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome/🟦️.ts — the design system's slim chrome
 */

import { StrictMode, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore, type ReactElement, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
  DEFAULT_UI_DRIVER,
  Navbar,
  ShellBrandLogo,
  UI_AVAILABLE_HEIGHT,
  UI_MOBILE_MEDIA_QUERY,
  UI_TABLET_MEDIA_QUERY,
  bootstrapElementsSurfaceChromeDocument,
  elementsSurfaceDeviceForMatches,
  navbarFillItem,
  useElementsSurfaceChrome,
  useMediaQuery,
  type NavbarItem,
} from "@semio-tech/ui-react/chrome";
import { REJECTION_LABELS, applyLocale, localized, preferredLocale, quizText, type QuizLocale, type QuizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
import { browserStorageArea, localStore, type StorageArea } from "./🔨️modules/💾️persistence/🟦️.ts";
import { ProctorClient, proctorTransport, type ProctorConnect, type RetryTiming } from "./🔨️modules/🛂️proctor/🟦️.ts";
import { QuizSession, type QuizConnection, type QuizNotice, type QuizState, type QuizStep } from "./🔨️modules/🧭️session/🟦️.ts";
import { LanguageSwitch, PreferencesPanelCard, readPreferences, textScale, writePreferences, type QuizPreferences } from "./🔨️modules/🎛️preferences/🟦️.tsx";
import { IntroductionScreen } from "./🔨️modules/👋️introduction/🟦️.tsx";
import { IdentityScreen } from "./🔨️modules/🪪️identity/🟦️.tsx";
import { HomeScreen } from "./🔨️modules/🏠️home/🟦️.tsx";
import { RunScreen } from "./🔨️modules/▶️run/🟦️.tsx";
import { ResultsScreen } from "./🔨️modules/🏁️results/🟦️.tsx";
import { BodyButton, CardIcon, QuizCard, cn } from "./🔨️modules/🪟️chrome/🟦️.tsx";
import { PresenceOverlay, PresenceProvider, PresenceStatus, QuizPresence, presencePlace, presenceSelf, presenceView, presenceDrafts, sheetItemLabels, useDocumentVisible, usePresencePointer, type PresenceConnect } from "./🔨️modules/👥️presence/🟦️.tsx";
import "./🎨️.css";

//#region 🔁️Reexports
export { QUIZ_BUNDLE_DE, QUIZ_BUNDLE_EN, QUIZ_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, applyLocale, isQuizLocale, localized, preferredLocale, quizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
export type { QuizLabelKey, QuizLocale, QuizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
export { SIGNIFICANT_DIGITS, SI_PREFIXES, engineering, formatClock, formatDate, formatInstant, formatNumber, formatPoints, formatQuantity, formatScore, oneDecimal, withUnit } from "./🔨️modules/📏️quantity/🟦️.ts";
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
export { LOCALE_NAMES, LanguageSwitch, PreferencesCard, PreferencesPage, PreferencesPanel, PreferencesPanelCard, TEXT_SIZES, THEME_CHOICES, readPreferences, textScale, writePreferences } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export type { QuizPreferences, TextSize, ThemeChoice } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export { DRAG_THRESHOLD_PX, dropZoneAt, startPointerDrag } from "./🔨️modules/🤏️drag/🟦️.ts";
export { DROP_ZONE_CLASS, DragGrip, ICON_BUTTON_CLASS, LiveRegion, SELECT_CLASS, elementId, useAnnouncement, useFocusAfterRender } from "./🔨️modules/🧩️task/🟦️.tsx";
export type { TaskViewProps } from "./🔨️modules/🧩️task/🟦️.tsx";
export { BodyButton, CardAction, CardIcon, Facts, Glyph, QuizCard, Segments, textPresentation } from "./🔨️modules/🪟️chrome/🟦️.tsx";
export type { Segment } from "./🔨️modules/🪟️chrome/🟦️.tsx";
export { ClassificationTaskView } from "./🔨️modules/🗂️classification/🟦️.tsx";
export { SortingTaskView, reordered } from "./🔨️modules/↕️sorting/🟦️.tsx";
export { MatchingTaskView, assignCard } from "./🔨️modules/🃏️matching/🟦️.tsx";
export { IntroductionCard, IntroductionPage, IntroductionScreen } from "./🔨️modules/👋️introduction/🟦️.tsx";
export { LearnerCard, LearnerPage } from "./🔨️modules/📇️profile/🟦️.tsx";
export { QuizCardView, QuizPage } from "./🔨️modules/📖️quiz-page/🟦️.tsx";
export type { Act } from "./🔨️modules/📖️quiz-page/🟦️.tsx";
export { IdentityScreen, failureMessage, learnerName, thrownMessage } from "./🔨️modules/🪪️identity/🟦️.tsx";
export { HOME_GRID_TRACKS, HomeScreen, homeCells, homePages } from "./🔨️modules/🏠️home/🟦️.tsx";
export type { HomeLayout } from "./🔨️modules/🏠️home/🟦️.tsx";
export { RunScreen, TASK_KIND_ICONS, TaskView } from "./🔨️modules/▶️run/🟦️.tsx";
export { ResultsScreen, TaskResultView } from "./🔨️modules/🏁️results/🟦️.tsx";
export { BOARD_EXCERPT_SIZE, LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, boardExcerpt, sortLeaderboard, usePolling } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
export type { LeaderboardKey, LeaderboardSort } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
export { BadgesCard, BadgesPage, awardOf, badgesEarnedIn } from "./🔨️modules/🏅️badges/🟦️.tsx";
export {
  EMPTY_PRESENCE_VIEW,
  MAX_WATCHED_ROOMS,
  OnlineMark,
  PRESENCE_ANCHORS,
  PRESENCE_FRAME_HZ,
  PRESENCE_FRAME_INTERVAL_MS,
  PanePeers,
  PresenceList,
  PresenceOverlay,
  PresenceProvider,
  PresenceRoom,
  PresenceStatus,
  QuizPresence,
  THINKING_FRAME_INTERVAL_MS,
  WATCH_INTERVAL_MS,
  browserPresenceConnect,
  cursorAt,
  homeWatchScopes,
  paintStyle,
  placePeers,
  placeText,
  presenceDrafts,
  presencePlace,
  presenceSelf,
  presenceView,
  sheetItemLabels,
  useDocumentVisible,
  usePresencePointer,
  usePresenceTask,
  usePresenceView,
} from "./🔨️modules/👥️presence/🟦️.tsx";
export type {
  PeerCursor,
  PresenceConnect,
  PresenceDrafts,
  PresencePlace,
  PresenceRoomOptions,
  PresenceSelf,
  PresenceSocket,
  PresenceView,
  QuizPresenceInput,
  QuizPresenceOptions,
  QuizPresenceSnapshot,
  RoomMember,
  RoomSnapshot,
  RoomStatus,
  WatchedState,
} from "./🔨️modules/👥️presence/🟦️.tsx";
export { CrowdChoices, CrowdPosition, CrowdProvider, CrowdSource, chooseCrowd, crowdPlace, liveItems, meanPosition, submittedItems, useCrowd } from "./🔨️modules/🗳️crowd/🟦️.tsx";
export type { Crowd, CrowdChoice, ItemCrowd } from "./🔨️modules/🗳️crowd/🟦️.tsx";
//#endregion 🔁️Reexports

//#region ❓️App
/** ⚙️ What a quiz site passes in: the proctor base URL (`""` is the site's own origin), the catalog id as tenant and
 * the site's logo (inline SVG markup) for the navbar. The optional seams replace the network, the presence sockets,
 * the storage, the browser's language list and the retry timing. */
export interface QuizOptions {
  readonly proctor: string;
  readonly tenant: string;
  readonly logo?: string;
  readonly transport?: ProctorConnect;
  readonly presence?: PresenceConnect;
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
    <div aria-busy="true" className="min-w-0">
      <QuizCard id="quiz-waiting" card="waiting" headingLevel={1} focusableHeading icon={<CardIcon icon="info" />} title={text("quiz.app.loading")}>
        <p role="status" className="m-0 text-sm">
          {waiting.message}
        </p>
        {waiting.retry ? <BodyButton onClick={onRetry}>{text("quiz.app.retry")}</BodyButton> : null}
      </QuizCard>
    </div>
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

function stepKey(step: QuizStep): string {
  return step.screen === "run" || step.screen === "results" ? `${step.screen}:${step.run}` : step.screen;
}

/** 🖼️ A screen other than home: it scrolls on its own, centred at a comfortable reading width; the first visit shows
 * the preferences beside. */
function Page(props: { readonly aside?: ReactNode; readonly children: ReactNode }): ReactElement {
  return (
    <div className="min-h-0 flex-1 overflow-auto p-double">
      <div className={cn("quiz-page mx-auto w-full", props.aside === undefined ? "max-w-6xl" : "quiz-pair max-w-6xl")}>
        {props.children}
        {props.aside}
      </div>
    </div>
  );
}

function Screen(props: {
  readonly session: QuizSession;
  readonly state: QuizState;
  readonly connection: QuizConnection;
  readonly text: QuizText;
  readonly locale: QuizLocale;
  readonly preferences: QuizPreferences;
  readonly onPreferences: (preferences: QuizPreferences) => void;
}): ReactElement {
  const { session, state, connection, text, locale, preferences, onPreferences } = props;
  const step = state.step;
  const waiting = (
    <Page>
      <Waiting connection={connection} notice={state.notice} text={text} onRetry={() => session.reconnect()} />
    </Page>
  );
  const firstVisit = state.learner === undefined ? <PreferencesPanelCard preferences={preferences} locale={locale} text={text} onChange={onPreferences} /> : undefined;
  if (step.screen === "introduction") {
    if (state.catalog === undefined) return waiting;
    return (
      <Page aside={firstVisit}>
        <IntroductionScreen catalog={state.catalog} text={text} locale={locale} onContinue={() => (state.introduced ? session.open({ screen: state.learner === undefined ? "identity" : "home" }) : session.readIntroduction())} />
      </Page>
    );
  }
  if (step.screen === "identity" || state.learner === undefined) {
    return (
      <Page aside={firstVisit}>
        <IdentityScreen session={session} text={text} />
      </Page>
    );
  }
  switch (step.screen) {
    case "home":
      return state.catalog === undefined ? waiting : <HomeScreen session={session} state={state} text={text} locale={locale} preferences={preferences} onPreferences={onPreferences} />;
    case "run":
      return state.runs[step.run] === undefined ? (
        waiting
      ) : (
        <Page>
          <RunScreen key={step.run} session={session} state={state} run={step.run} text={text} locale={locale} showAnswers={preferences.showAnswers} />
        </Page>
      );
    case "results":
      return state.runs[step.run] === undefined ? (
        waiting
      ) : (
        <Page>
          <ResultsScreen session={session} state={state} run={step.run} text={text} locale={locale} showAnswers={preferences.showAnswers} />
        </Page>
      );
  }
}

function useRootTextScale(scale: number): void {
  useLayoutEffect(() => {
    const root = document.documentElement;
    root.style.setProperty("--quiz-text-scale", String(scale));
    return () => {
      root.style.removeProperty("--quiz-text-scale");
    };
  }, [scale]);
}

function ConnectionStatus(props: { readonly connection: QuizConnection; readonly text: QuizText }): ReactElement {
  const status = connectionMessage(props.connection, props.text);
  return (
    <p role="status" aria-live="polite" data-tone={status.tone} className="quiz-connection m-0 flex min-w-0 items-center gap-single px-single text-xs text-muted-foreground">
      <span aria-hidden="true">{status.tone === "alert" ? "⚠" : status.tone === "busy" ? "↻" : "✓"}</span>
      <span className="sr-only">{props.text("quiz.connection.label")}: </span>
      <span className="truncate max-md:sr-only">{status.message}</span>
    </p>
  );
}

/** ❓️ The whole quiz client for one proctor tenant. */
export function QuizApp(options: QuizOptions): ReactElement {
  const [setup] = useState(() => {
    const store = localStore(options.storage ?? browserStorageArea(), options.tenant);
    const proctor = new ProctorClient(options.transport ?? proctorTransport(options.proctor), options.tenant);
    const presence = new QuizPresence({ proctor: options.proctor, connect: options.presence, timing: options.timing });
    return { store, presence, session: new QuizSession({ proctor, store, timing: options.timing }) };
  });
  const { session, store, presence } = setup;
  useLayoutEffect(() => {
    session.start();
    return () => session.stop();
  }, [session]);
  useEffect(() => () => presence.stop(), [presence]);
  const { state, connection } = useSyncExternalStore(session.subscribe, session.getSnapshot, session.getSnapshot);
  const [preferences, setPreferences] = useState<QuizPreferences>(() => readPreferences(store));
  const locale = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());
  const text = useMemo(() => quizText(locale), [locale]);
  const [task, setTask] = useState<string | undefined>(undefined);
  const visible = useDocumentVisible();
  const identified = state.introduced && state.learner !== undefined;
  const self = useMemo(() => presenceSelf(identified ? state.learner : undefined), [identified, state.learner]);
  const at = presencePlace(state.step, state, task);
  const onScreen = state.step.screen === "run" ? state.runs[state.step.run] : undefined;
  const drafts = useMemo(() => presenceDrafts(onScreen), [onScreen]);
  const itemLabel = useMemo(() => sheetItemLabels(onScreen, locale), [onScreen, locale]);
  const quizzes = useMemo(() => state.catalog?.quizzes.map((quiz) => quiz.id) ?? [], [state.catalog]);
  useEffect(() => presence.update({ catalog: state.catalog?.id, quizzes, self, at, home: state.step.screen === "home", drafts, active: visible }));
  usePresencePointer(presence);
  const shared = useSyncExternalStore(presence.subscribe, presence.getSnapshot, presence.getSnapshot);
  const view = useMemo(() => presenceView(shared, text), [shared, text]);
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  useElementsSurfaceChrome({ appearance: preferences.theme, device: elementsSurfaceDeviceForMatches({ mobile, tablet }), driver: DEFAULT_UI_DRIVER, browserDefaults: "native" });
  useRootTextScale(textScale(preferences.textSize));
  const main = useRef<HTMLElement>(null);
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
  const title = state.catalog === undefined ? "" : localized(state.catalog.title, locale);
  const brand = (
    <>
      {options.logo === undefined ? null : <ShellBrandLogo svg={options.logo} className="size-workbench shrink-0" />}
      <span className="truncate px-single text-sm font-semibold text-foreground">{title}</span>
    </>
  );
  const items: NavbarItem[] = [
    {
      key: "brand",
      className: "min-w-0 shrink",
      content: identified ? (
        <button type="button" onClick={() => session.open({ screen: "home" })} className="flex min-w-0 cursor-pointer items-center gap-single border-0 bg-transparent p-0 text-left">
          {brand}
        </button>
      ) : (
        <span className="flex min-w-0 items-center gap-single">{brand}</span>
      ),
    },
    navbarFillItem("fill"),
    { key: "connection", className: "min-w-0 shrink", content: <ConnectionStatus connection={connection} text={text} /> },
    ...(view.roster === undefined ? [] : [{ key: "presence", className: "shrink-0", content: <PresenceStatus text={text} /> }]),
    { key: "language", content: <LanguageSwitch locale={locale} text={text} onChange={(chosen) => change({ ...preferences, locale: chosen })} /> },
  ];
  return (
    <PresenceProvider view={view} showCursors={preferences.showCursors} setTask={setTask}>
      <div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale}>
        <a
          href="#quiz-main"
          className="sr-only focus:not-sr-only focus:absolute focus:start-double focus:top-double focus:z-50 focus:bg-background focus:p-double focus:text-sm focus:text-foreground"
          onClick={(event) => {
            event.preventDefault();
            main.current?.focus();
          }}
        >
          {text("quiz.app.skip")}
        </a>
        <header className="shrink-0">
          <Navbar label={text("quiz.nav.label")} items={items} showFullscreenToggle={false} />
        </header>
        <main id="quiz-main" ref={main} tabIndex={-1} className="flex min-h-0 flex-1 flex-col outline-none">
          {state.notice === undefined ? null : (
            <div role="alert" className="quiz-alert m-double mb-0 flex flex-wrap items-center gap-double border border-normal px-double py-single text-sm">
              <p className="m-0 flex-1">{noticeMessage(state.notice, text)}</p>
              <BodyButton onClick={() => session.dismissNotice()}>{text("quiz.app.dismiss")}</BodyButton>
            </div>
          )}
          <Screen session={session} state={state} connection={connection} text={text} locale={locale} preferences={preferences} onPreferences={change} />
        </main>
        <PresenceOverlay view={view} show={preferences.showCursors} itemLabel={itemLabel} />
      </div>
    </PresenceProvider>
  );
}

/** 🚀️ Mounts the quiz client into `root` — with the stored theme applied to the document before the first paint — and
 * returns the function that unmounts it again. */
export function mountQuiz(root: HTMLElement, options: QuizOptions): () => void {
  bootstrapElementsSurfaceChromeDocument(readPreferences(localStore(options.storage ?? browserStorageArea(), options.tenant)).theme);
  const reactRoot = createRoot(root);
  reactRoot.render(
    <StrictMode>
      <QuizApp {...options} />
    </StrictMode>,
  );
  return () => reactRoot.unmount();
}
//#endregion ❓️App
