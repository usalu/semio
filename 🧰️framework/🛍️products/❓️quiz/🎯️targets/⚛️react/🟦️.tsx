/** ❓️ `@semio-tech/quiz-react` — the web renderer and proctor client of `@semio-tech/quiz`.
 *
 * Domain-neutral: it renders whatever catalog the proctor serves for `tenant`. {@link mountQuiz} is the whole seam a
 * site needs; {@link QuizApp} is the same as a React component. The steps are introduction (first visit), identity,
 * home, run, results, leaderboard and badges; every string is English or German and none of the two is assumed — the
 * document's language, its title and every text follow the language the learner chose or the browser preselected, and
 * a browser that names neither is asked first. The theme and text size are the learner's, and answers survive short
 * connection shortages on the device; with the site's material in hand the device even decides by itself while the
 * proctor is away, and the proctor hears of everything once it answers again. The navbar leads the way — to the overview, back and forward along the steps the
 * learner went through and up to the place above — and names what the quizzes are about in its middle; the address
 * names the page of the overview that shows. Learners see who else is online and where, and the cursors of the learners
 * on the same page. A site may hand in pets: small animated companions that fit the quiz on screen, as lively as the learner
 * likes. Every screen ends with the footer that says what is stored and links the site's legal pages. The look is
 * the semio card language of `🎡️play` and the `🧺️demonstrator`: the design system's navbar and window chrome from
 * `@semio-tech/ui-react/chrome`, its tokens, glass and fonts, light and dark through its surface chrome.
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
import { applyLocale, localized, preferredLocale, quizText, type QuizLabelKey, type QuizLocale, type QuizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
import { browserStorageArea, localStore, type LocalStore, type StorageArea } from "./🔨️modules/💾️persistence/🟦️.ts";
import { ProctorClient, proctorTransport, type ProctorConnect, type RetryTiming } from "./🔨️modules/🛂️proctor/🟦️.ts";
import { Deputy, type QuizMaterial } from "./🔨️modules/🫡️deputy/🟦️.ts";
import { QuizSession, type QuizConnection, type QuizNotice, type QuizState, type QuizStep } from "./🔨️modules/🧭️session/🟦️.ts";
import { LanguageChoice, LanguageSwitch, PreferencesPanelCard, everyLanguage, readPreferences, textScale, withPets, writePreferences, type QuizPreferences } from "./🔨️modules/🎛️preferences/🟦️.tsx";
import { IntroductionScreen } from "./🔨️modules/👋️introduction/🟦️.tsx";
import { IdentityScreen, noticeProblem } from "./🔨️modules/🪪️identity/🟦️.tsx";
import { HomeScreen, pageLabel } from "./🔨️modules/🏠️home/🟦️.tsx";
import { NavigationControls, placeName, useAddress } from "./🔨️modules/🚏️navigation/🟦️.tsx";
import { RunScreen } from "./🔨️modules/▶️run/🟦️.tsx";
import { ResultsScreen } from "./🔨️modules/🏁️results/🟦️.tsx";
import { LegalFooter, type QuizLegal } from "./🔨️modules/⚖️legal/🟦️.tsx";
import { BodyButton, CardIcon, ProblemNote, QuizCard, cn } from "./🔨️modules/🪟️chrome/🟦️.tsx";
import { PresenceOverlay, PresenceProvider, PresenceStatus, QuizPresence, presencePlace, presenceSelf, presenceView, presenceDrafts, sheetItemLabels, useDocumentVisible, usePresencePointer, type PresenceConnect } from "./🔨️modules/👥️presence/🟦️.tsx";
import { PetsSwitch, QuizPets, QuizPetsProvider, effectivePetMode, switchedPets, usePetsReduced, type QuizPetsSource } from "./🔨️modules/🐾️pets/🟦️.tsx";
import "./🎨️.css";

//#region 🔁️Reexports
export { QUIZ_BUNDLE_DE, QUIZ_BUNDLE_EN, QUIZ_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, applyLocale, isQuizLocale, localized, preferredLocale, quizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
export type { QuizLabelKey, QuizLocale, QuizText } from "./🔨️modules/🌐️i18n/🟦️.ts";
export { SIGNIFICANT_DIGITS, SI_PREFIXES, engineering, formatClock, formatDate, formatInstant, formatNumber, formatPoints, formatQuantity, formatScore, oneDecimal, parseQuantity, withUnit } from "./🔨️modules/📏️quantity/🟦️.ts";
export { RADAR_LABEL_EM, RADAR_METRICS, RadarChart, estimateTextWidth, radarAngle, radarFraction, radarLayout, radarPoint, radarPolygon, textMeasure, wrapLabel } from "./🔨️modules/🕸️radar/🟦️.tsx";
export type { RadarAxis, RadarBox, RadarFrame, RadarLabel, RadarLayout, RadarLegendEntry, RadarPoint, TextMeasure } from "./🔨️modules/🕸️radar/🟦️.tsx";
export { browserStorageArea, isRecord, localChange, localStore, memoryStorageOrigin } from "./🔨️modules/💾️persistence/🟦️.ts";
export type { LocalChange, LocalCollection, LocalSlice, LocalStore, StorageArea } from "./🔨️modules/💾️persistence/🟦️.ts";
export {
  ProctorClient,
  ProctorThrottled,
  ProctorUnavailable,
  QUIZ_WIRE_VERSION,
  RETRY_AFTER_MAX_MS,
  RETRY_TIMING,
  SIGN_UP_ALLOWANCE,
  abortable,
  commandEnvelope,
  commandTarget,
  commandVerdict,
  errorRejection,
  isNotFound,
  isThrottled,
  isTransient,
  learnerPrincipal,
  newId,
  pause,
  proctorTransport,
  queryEnvelope,
  quizRejection,
  retryAfterMs,
  retryTransient,
  retryWait,
  signUpsSpent,
} from "./🔨️modules/🛂️proctor/🟦️.ts";
export type { CommandVerdict, HttpRequest, HttpResponse, HttpTransport, ProctorConnect, ProctorReachability, RetryTiming } from "./🔨️modules/🛂️proctor/🟦️.ts";
export { Outbox, coalesce, coalescingKey, commandRun } from "./🔨️modules/📮️outbox/🟦️.ts";
export type { OutboxActivity, OutboxEntry, OutboxOptions, OutboxStatus } from "./🔨️modules/📮️outbox/🟦️.ts";
export { Deputy, REVISED, materialRevision } from "./🔨️modules/🫡️deputy/🟦️.ts";
export type { HeldLearner, QuizMaterial } from "./🔨️modules/🫡️deputy/🟦️.ts";
export { DEFAULT_BOARD, DEPUTY_PATIENCE_MS, EMPTY_TRAIL, HOME_PAGES, PROJECTION_GRACE_MS, QuizSession, TRAIL_LIMIT, boardKey, evolveQuizState, initialQuizState, lastSubmittedRunOf, mergeRunViews, openRunOf, overallLeaderboard, restoreQuizState, runAwards, sameStep, shownLeaderboard, stepAbove, stepAfter, stepBefore } from "./🔨️modules/🧭️session/🟦️.ts";
export type { BoardChoice, HeldLeaderboard, QuizClientEvent, QuizConnection, QuizLearner, QuizNotice, QuizSessionOptions, QuizSnapshot, QuizState, QuizStep, QuizTrail, RefreshOutcome, SessionFailure, SubmissionPhase } from "./🔨️modules/🧭️session/🟦️.ts";
export { NAVIGATION_WAYS, NavigationControls, addressPage, navigationWays, placeName, stepAddress, useAddress } from "./🔨️modules/🚏️navigation/🟦️.tsx";
export type { NavigationWay } from "./🔨️modules/🚏️navigation/🟦️.tsx";
export { EveryLanguage, LOCALE_NAMES, LanguageChoice, LanguageSwitch, PreferencesCard, PreferencesPage, PreferencesPanel, PreferencesPanelCard, TEXT_SIZES, THEME_CHOICES, everyLanguage, readPreferences, textScale, writePreferences } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export type { QuizPreferences, TextSize, ThemeChoice } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export { OTHERS_CHOICE_LABELS, PET_CHOICE_LABELS, withPets } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export { PET_CHOICES, PET_HOME_SCENE, PetsSwitch, QUIZ_PETS_TEMPO, QUIZ_PET_KEEPOUTS, QUIZ_PET_SURFACES, QuizPets, QuizPetsProvider, effectivePetMode, peerGlances, petNames, petScene, petsTempo, switchedPets, usePetCast, usePetsForced, usePetsReduced } from "./🔨️modules/🐾️pets/🟦️.tsx";
export type { PetChoice, PetLiveliness, QuizPetsSource, QuizPetsStage } from "./🔨️modules/🐾️pets/🟦️.tsx";
export { DRAG_THRESHOLD_PX, dropZoneAt, startPointerDrag } from "./🔨️modules/🤏️drag/🟦️.ts";
export { DROP_ZONE_CLASS, DragGrip, ICON_BUTTON_CLASS, LiveRegion, SELECT_ANNOUNCEMENT_DELAY_MS, SELECT_CLASS, elementId, useAnnouncement, useFocusAfterRender } from "./🔨️modules/🧩️task/🟦️.tsx";
export type { Announcement, TaskViewProps } from "./🔨️modules/🧩️task/🟦️.tsx";
export { BodyButton, CardAction, CardIcon, Dialog, Facts, Glyph, Mark, Missing, ProblemNote, QuizCard, Segments, textPresentation } from "./🔨️modules/🪟️chrome/🟦️.tsx";
export type { Problem, Segment } from "./🔨️modules/🪟️chrome/🟦️.tsx";
export { ClassificationTaskView } from "./🔨️modules/🗂️classification/🟦️.tsx";
export { SortingTaskView, ordered, reordered } from "./🔨️modules/↕️sorting/🟦️.tsx";
export { MatchingTaskView, assignCard } from "./🔨️modules/🃏️matching/🟦️.tsx";
export { IntroductionCard, IntroductionPage, IntroductionScreen } from "./🔨️modules/👋️introduction/🟦️.tsx";
export { LearnerCard, LearnerPage, SwitchIdentity } from "./🔨️modules/📇️profile/🟦️.tsx";
export { QuizCardView, QuizPage } from "./🔨️modules/📖️quiz-page/🟦️.tsx";
export type { Act } from "./🔨️modules/📖️quiz-page/🟦️.tsx";
export { IdentityScreen, failureProblem, handleFault, handleFaultMessage, learnerName, noticeProblem, thrownProblem } from "./🔨️modules/🪪️identity/🟦️.tsx";
export type { HandleFault } from "./🔨️modules/🪪️identity/🟦️.tsx";
export { HOME_CHROME_HEIGHT_PX, HOME_GRID_ROW_HEIGHT_PX, HOME_GRID_TRACKS, HomeScreen, homeCells, homeGridMinHeight, homeLayoutQueries, homePages, homeTrackTemplate, pageLabel } from "./🔨️modules/🏠️home/🟦️.tsx";
export type { HomeLayout } from "./🔨️modules/🏠️home/🟦️.tsx";
export { RunScreen, TASK_KIND_ICONS, TaskGlyph, TaskView } from "./🔨️modules/▶️run/🟦️.tsx";
export { ResultsScreen, TaskResultView } from "./🔨️modules/🏁️results/🟦️.tsx";
export { BOARD_EXCERPT_SIZE, LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, POLL_BACKOFF_MAX_MS, RANK_ORDER, boardExcerpt, leaderboardColumns, nextSort, ownRow, pollDelay, sortLeaderboard, usePolling } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
export type { LeaderboardColumn, LeaderboardKey, LeaderboardSort } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
export { BadgesCard, BadgesPage, awardOf, badgesEarnedIn } from "./🔨️modules/🏅️badges/🟦️.tsx";
export { LegalFooter, PrivacyNotice, legalLink } from "./🔨️modules/⚖️legal/🟦️.tsx";
export type { QuizLegal } from "./🔨️modules/⚖️legal/🟦️.tsx";
export {
  EMPTY_PRESENCE_VIEW,
  MAX_WATCHED_ROOMS,
  OnlineMark,
  PEER_LABEL_CLEARANCE_PX,
  PRESENCE_ANCHORS,
  PRESENCE_FRAME_HZ,
  PRESENCE_FRAME_INTERVAL_MS,
  PRESENCE_STABLE_MS,
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
  peerInk,
  placePeers,
  placeText,
  presenceDrafts,
  presencePlace,
  presenceSelf,
  presenceView,
  rejoinDelay,
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
export { AnswerFigure, CROWD_DOTS, CrowdDoor, OTHERS_CHOICES, ScoreFigure, TaskFigures, answerFigure, crowdGate, crowdPlace, crowdShown, livePlace, scoreFigure } from "./🔨️modules/🗳️crowd/🟦️.tsx";
export type { AnswerCell, AnswerColumn, AnswerFigureInput, AnswerFigureModel, AnswerRow, CrowdGate, CrowdPlace, OthersChoice, ScoreFigureModel } from "./🔨️modules/🗳️crowd/🟦️.tsx";
export { Column, columnShare, formatShare, peakOf } from "./🔨️modules/📊️plot/🟦️.tsx";
//#endregion 🔁️Reexports

//#region ❓️App
/** ⚙️ What a quiz site passes in: the proctor base URL (`""` is the site's own origin), the catalog id as tenant, the
 * site's material (the catalog and its quizzes: with it the catalog shows at once and the device decides by itself
 * while the proctor is away), the site's logo (inline SVG markup) for the navbar, the site's legal pages for the footer
 * and the site's pets (where its menagerie comes from; fetched only for a learner who wants pets). The optional seams
 * replace the network, the presence sockets, the storage, the browser's language list and the retry timing. */
export interface QuizOptions {
  readonly proctor: string;
  readonly tenant: string;
  readonly material?: QuizMaterial;
  readonly logo?: string;
  readonly legal?: QuizLegal;
  readonly pets?: QuizPetsSource;
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

/** 🚥️ How urgent the state of the connection is: all saved, something under way, or answers cannot leave the device. */
export type ConnectionTone = "calm" | "busy" | "alert";

/** 📶️ One sentence about the connection before it is put into a language: its key and the values it carries. */
interface ConnectionSentence {
  readonly key: QuizLabelKey;
  readonly values?: Readonly<Record<string, number>>;
}

/** 📶️ The state of the connection as the indicator shows it: connecting until the proctor first answers, then saved,
 * saving, a busy proctor (it answers, but asks to slow down), or answers held on the device — where a deputy decides
 * meanwhile, everything is saved there. */
export function connectionState(connection: QuizConnection): ConnectionSentence & { readonly tone: ConnectionTone } {
  if (!connection.online) return { key: "quiz.connection.offline", tone: "alert" };
  if (connection.reachability === "unreachable" && connection.deputy) return connection.pending > 0 ? { key: "quiz.connection.localWaiting", values: { waiting: connection.pending }, tone: "alert" } : { key: "quiz.connection.local", tone: "alert" };
  if (connection.reachability === "unreachable") return connection.pending > 0 ? { key: "quiz.connection.reconnectingWaiting", values: { waiting: connection.pending }, tone: "alert" } : { key: "quiz.connection.reconnecting", tone: "alert" };
  if (connection.reachability === "throttled") return { key: "quiz.connection.throttled", tone: "busy" };
  if (connection.pending > 0) return { key: "quiz.connection.saving", values: { waiting: connection.pending }, tone: "busy" };
  if (connection.reachability === "unknown") return { key: "quiz.connection.connecting", tone: "busy" };
  return { key: "quiz.connection.saved", tone: "calm" };
}

/** 📶️ What the connection indicator says and how urgently. */
export function connectionMessage(connection: QuizConnection, text: QuizText): { readonly message: string; readonly tone: ConnectionTone } {
  const state = connectionState(connection);
  return { message: text(state.key, state.values), tone: state.tone };
}

/** ⏳️ What a screen whose data has not arrived yet says: connecting, unreachable (with a retry), busy (the client
 * retries by itself and offers no button to ask a busy proctor even more often), refused, or loading. */
export function waitingMessage(connection: QuizConnection, notice: QuizNotice | undefined, text: QuizText): { readonly message: string; readonly retry: boolean } {
  if (!connection.online) return { message: text("quiz.connection.offline"), retry: true };
  if (connection.reachability === "unreachable") return { message: text("quiz.app.unreachable"), retry: true };
  if (connection.reachability === "throttled") return { message: text("quiz.app.busy"), retry: false };
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

function stepKey(step: QuizStep): string {
  return step.screen === "run" || step.screen === "results" ? `${step.screen}:${step.run}` : step.screen;
}

function screenTitle(state: QuizState, locale: QuizLocale, text: QuizText): string {
  const { step, catalog, learner } = state;
  if (step.screen === "introduction") return catalog === undefined ? text("quiz.app.loading") : localized(catalog.introduction.title, locale);
  if (step.screen === "identity" || learner === undefined) return text("quiz.identity.title");
  if (step.screen !== "home") return placeName(step, state, locale, text) ?? text("quiz.app.loading");
  if (catalog === undefined) return text("quiz.app.loading");
  return (step.page === undefined ? undefined : pageLabel(step.page, state, locale, text)) ?? text("quiz.home.title");
}

/** 🏷️ The title of the document: what the screen shows, then the catalog's title, in `locale` — the loading text
 * until the catalog arrived; without a locale the word for language in every offered language, as the chooser shows. */
export function documentTitle(state: QuizState, locale: QuizLocale | undefined): string {
  if (locale === undefined) return everyLanguage("quiz.preferences.language");
  const screen = screenTitle(state, locale, quizText(locale));
  const site = state.catalog === undefined ? undefined : localized(state.catalog.title, locale);
  return site === undefined || site === screen ? screen : `${screen} · ${site}`;
}

/** 🖼️ A screen other than home: it scrolls on its own, centred at a comfortable reading width; the first visit shows
 * the preferences beside. */
function Page(props: { readonly aside?: ReactNode; readonly children: ReactNode }): ReactElement {
  return (
    <div className="relative min-h-0 flex-1 overflow-auto p-double">
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
          <RunScreen key={step.run} session={session} state={state} run={step.run} text={text} locale={locale} others={preferences.others} />
        </Page>
      );
    case "results":
      return state.runs[step.run] === undefined ? (
        waiting
      ) : (
        <Page>
          <ResultsScreen session={session} state={state} run={step.run} text={text} locale={locale} others={preferences.others} />
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

const TONE_GLYPHS: { readonly [T in ConnectionTone]: string } = { alert: "⚠", busy: "↻", calm: "✓" };

/** 📶️ The connection indicator: text to see — never a live region, because saving and saved alternate with every
 * answer — and beside it one polite status that speaks only when the connection enters an alert state (once, however
 * the waiting count or the kind of outage changes meanwhile) and when it has recovered. */
export function ConnectionStatus(props: { readonly connection: QuizConnection; readonly text: QuizText }): ReactElement {
  const { text } = props;
  const shown = connectionState(props.connection);
  const alert = shown.tone === "alert";
  const [spoken, setSpoken] = useState<{ readonly alert: boolean; readonly said?: ConnectionSentence }>({ alert: false });
  if (alert !== spoken.alert) setSpoken({ alert, said: alert ? { key: shown.key, values: shown.values } : { key: "quiz.connection.restored" } });
  return (
    <>
      <p data-tone={shown.tone} className="quiz-connection m-0 flex min-w-0 items-center gap-single text-xs text-muted-foreground md:px-single">
        <span aria-hidden="true">{TONE_GLYPHS[shown.tone]}</span>
        <span className="sr-only">{text("quiz.connection.label")}: </span>
        <span className="truncate max-md:sr-only">{text(shown.key, shown.values)}</span>
      </p>
      <p role="status" data-connection-announcer="" className="sr-only">
        {spoken.said === undefined ? "" : text(spoken.said.key, spoken.said.values)}
      </p>
    </>
  );
}

interface ClientProps {
  readonly options: QuizOptions;
  readonly session: QuizSession;
  readonly presence: QuizPresence;
  readonly state: QuizState;
  readonly connection: QuizConnection;
  readonly locale: QuizLocale;
  readonly preferences: QuizPreferences;
  readonly onPreferences: (preferences: QuizPreferences) => void;
}

/** 🖥️ The client once a language is known: the navbar — the ways of an identified learner on its left (overview, back,
 * forward, up), what the quizzes are about with the site's logo in its middle, the connection, who is online and the
 * language on its right — the screen of the step, the footer, the others' cursors and the pets. */
function Client(props: ClientProps): ReactElement {
  const { options, session, presence, state, connection, locale, preferences, onPreferences } = props;
  const text = useMemo(() => quizText(locale), [locale]);
  const [task, setTask] = useState<string | undefined>(undefined);
  const visible = useDocumentVisible();
  const pets = effectivePetMode(preferences.pets, preferences.petsChosen, usePetsReduced());
  const identified = state.introduced && state.learner !== undefined;
  const self = useMemo(() => presenceSelf(identified ? state.learner : undefined), [identified, state.learner]);
  const at = presencePlace(state.step, state, task);
  const onScreen = state.step.screen === "run" ? state.runs[state.step.run] : undefined;
  const drafts = useMemo(() => presenceDrafts(onScreen), [onScreen]);
  const itemLabel = useMemo(() => sheetItemLabels(onScreen, locale), [onScreen, locale]);
  const quizzes = useMemo(() => state.catalog?.quizzes.map((quiz) => quiz.id) ?? [], [state.catalog]);
  useEffect(() => presence.update({ catalog: state.catalog?.id, quizzes, self, at, home: state.step.screen === "home", drafts, active: visible }));
  usePresencePointer(presence);
  useAddress(session, state);
  const shared = useSyncExternalStore(presence.subscribe, presence.getSnapshot, presence.getSnapshot);
  const view = useMemo(() => presenceView(shared, text), [shared, text]);
  const main = useRef<HTMLElement>(null);
  const shown = useRef<string | undefined>(undefined);
  const key = stepKey(state.step);
  useEffect(() => {
    if (shown.current !== undefined && shown.current !== key) main.current?.querySelector<HTMLElement>("h1")?.focus();
    shown.current = key;
  }, [key]);

  const title = state.catalog === undefined ? "" : localized(state.catalog.title, locale);
  const brand = (
    <span data-quiz-brand="" className="quiz-brand">
      {options.logo === undefined ? null : (
        <span aria-hidden="true" className="quiz-brand-logo inline-flex">
          <ShellBrandLogo svg={options.logo} className="size-workbench shrink-0" />
        </span>
      )}
      {title === "" ? null : (
        <span title={title} className="quiz-brand-title truncate px-single text-sm font-semibold text-foreground">
          {title}
        </span>
      )}
    </span>
  );
  const items: NavbarItem[] = [
    ...(identified ? [{ key: "ways", content: <NavigationControls session={session} state={state} locale={locale} text={text} /> }] : []),
    navbarFillItem("fill"),
    { key: "connection", className: "min-w-0 shrink", content: <ConnectionStatus connection={connection} text={text} /> },
    ...(view.roster === undefined ? [] : [{ key: "presence", className: "shrink-0", content: <PresenceStatus text={text} /> }]),
    { key: "language", content: <LanguageSwitch compact locale={locale} text={text} onChange={(chosen) => onPreferences({ ...preferences, locale: chosen })} /> },
    { key: "brand", centered: true, className: "min-w-0 px-single", content: brand },
  ];
  return (
    <PresenceProvider view={view} showCursors={preferences.showCursors} setTask={setTask}>
      <div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale} data-icon-motion={preferences.animateIcons ? "on" : "off"} data-pets={pets}>
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
            <ProblemNote problem={noticeProblem(state.notice, text)} text={text} className="m-double mb-0">
              <BodyButton onClick={() => session.dismissNotice()}>{text("quiz.app.dismiss")}</BodyButton>
            </ProblemNote>
          )}
          <Screen session={session} state={state} connection={connection} text={text} locale={locale} preferences={preferences} onPreferences={onPreferences} />
        </main>
        <LegalFooter legal={options.legal} locale={locale} text={text}>
          <PetsSwitch shown={preferences.pets !== "off"} label={text("quiz.preferences.petsShown")} onChange={(shown) => onPreferences(withPets(preferences, switchedPets(shown, preferences.petsLiveliness)))} />
        </LegalFooter>
        <PresenceOverlay view={view} show={preferences.showCursors} itemLabel={itemLabel} />
        <QuizPets />
      </div>
    </PresenceProvider>
  );
}

function useSetup(options: QuizOptions): { readonly store: LocalStore; readonly presence: QuizPresence; readonly session: QuizSession } {
  const [setup] = useState(() => {
    const store = localStore(options.storage ?? browserStorageArea(), options.tenant);
    const proctor = new ProctorClient(options.transport ?? proctorTransport(options.proctor), options.tenant);
    const presence = new QuizPresence({ proctor: options.proctor, connect: options.presence, timing: options.timing });
    const deputy = options.material === undefined ? undefined : new Deputy(options.material);
    return { store, presence, session: new QuizSession({ proctor, store, deputy, timing: options.timing }) };
  });
  return setup;
}

/** ❓️ The whole quiz client for one proctor tenant. It speaks the language the learner stored, else the one the
 * browser's list preselects among the offered ones; while neither exists it shows only the language chooser. */
export function QuizApp(options: QuizOptions): ReactElement {
  const { session, store, presence } = useSetup(options);
  useLayoutEffect(() => {
    session.start();
    return () => session.stop();
  }, [session]);
  useEffect(() => () => presence.stop(), [presence]);
  const { state, connection } = useSyncExternalStore(session.subscribe, session.getSnapshot, session.getSnapshot);
  const [preferences, setPreferences] = useState<QuizPreferences>(() => readPreferences(store));
  const locale = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  useElementsSurfaceChrome({ appearance: preferences.theme, device: elementsSurfaceDeviceForMatches({ mobile, tablet }), driver: DEFAULT_UI_DRIVER, browserDefaults: "native" });
  useRootTextScale(textScale(preferences.textSize));
  useLayoutEffect(() => {
    void applyLocale(locale);
  }, [locale]);
  const title = documentTitle(state, locale);
  useEffect(() => {
    document.title = title;
  }, [title]);
  useEffect(() => store.watch((change) => (change.kind === "cleared" || (change.kind === "slice" && change.slice === "preferences")) && setPreferences(readPreferences(store))), [store]);

  const change = (next: QuizPreferences): void => {
    setPreferences(next);
    writePreferences(store, next);
  };
  if (locale !== undefined)
    return (
      <QuizPetsProvider source={options.pets} choice={preferences.pets} chosen={preferences.petsChosen} state={state} locale={locale}>
        <Client options={options} session={session} presence={presence} state={state} connection={connection} locale={locale} preferences={preferences} onPreferences={change} />
      </QuizPetsProvider>
    );
  return (
    <div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }}>
      <main id="quiz-main" className="relative flex min-h-0 flex-1 flex-col overflow-auto p-double">
        <div className="mx-auto w-full max-w-xl">
          <LanguageChoice onChoose={(chosen) => change({ ...preferences, locale: chosen })} />
        </div>
      </main>
      <LegalFooter legal={options.legal} locale={undefined} text={undefined} />
    </div>
  );
}

/** 🚀️ Mounts the quiz client into `root` — with the stored theme and the known language applied to the document
 * before the first paint — and returns the function that unmounts it again. */
export function mountQuiz(root: HTMLElement, options: QuizOptions): () => void {
  const preferences = readPreferences(localStore(options.storage ?? browserStorageArea(), options.tenant));
  bootstrapElementsSurfaceChromeDocument(preferences.theme);
  void applyLocale(preferences.locale ?? preferredLocale(options.languages ?? browserLanguages()));
  const reactRoot = createRoot(root);
  reactRoot.render(
    <StrictMode>
      <QuizApp {...options} />
    </StrictMode>,
  );
  return () => reactRoot.unmount();
}
//#endregion ❓️App
