/** 📚️ `@semio-tech/course-react` — the web renderer and proctor client of `@semio-tech/course`.
 *
 * Domain-neutral: it renders whatever course the proctor serves for `tenant`. {@link mountCourse} is the whole seam a
 * site needs; {@link CourseApp} is the same as a React component. The overview opens directly, with a guided introduction on the first visit and an identity question when signing in.
 * Runs, results, leaderboards and badges follow; every string is English or German and none of the two is assumed — the
 * document's language, its title and every text follow the language the learner chose or the browser preselected, and
 * a browser that names neither is asked first. The theme and text size are the learner's, and answers survive short
 * connection shortages on the device; with the site's material in hand the device even decides by itself while the
 * proctor is away, and the proctor hears of everything once it answers again. The navbar leads the way — to the overview, back and forward along the steps the
 * learner went through and up to the place above — and names what the quizzes are about in its middle; the address
 * names the page of the overview that shows. Learners see who else is online and where, and the cursors of the learners
 * on the same page. A site may hand in pets: small animated companions that fit the quiz on screen, as lively as the learner
 * likes. Every screen ends with the footer that says what is stored and links the site's legal pages. A run and its
 * results come in script chunks of their own, fetched when the learner opens them, so that screen never waits for them. The look is
 * the semio card language of `🎡️play` and the `🧺️demonstrator`: the design system's navbar and window chrome from
 * `@semio-tech/ui-react/chrome`, its tokens, glass and fonts, light and dark through its surface chrome.
 *
 * @see ../../🧬️schema/🔣️.json — the contract every view, command and event follows
 * @see ../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome/🟦️.ts — the design system's slim chrome
 */

//#region 🔁️Reexports
export { COURSE_BUNDLE_DE } from "./🔨️modules/🌐️i18n/🔡️german-bundle/🟦️.ts";
export { COURSE_BUNDLE_EN } from "./🔨️modules/🌐️i18n/🔠️english-bundle/🟦️.ts";
export { COURSE_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, applyLocale, isCourseLocale, localized, preferredLocale, courseText, titled } from "./🔨️modules/🌐️i18n/🟦️.ts";
export type { CourseLabelKey, CourseLocale, CourseText } from "./🔨️modules/🌐️i18n/🟦️.ts";
export { SIGNIFICANT_DIGITS, SI_PREFIXES, ceilSignificant, engineering, floorSignificant, formatClock, formatCount, formatCountdown, formatDate, formatDuration, formatFactor, formatInstant, formatNumber, formatPoints, formatQuantity, formatScore, formatTimes, oneDecimal, parseQuantity, quantityReading, quantitySymbol, quantityUnits, splitGuess, withMinusSign, withUnit } from "./🔨️modules/📏️quantity/🟦️.ts";
export type { QuantityUnit, Toward } from "./🔨️modules/📏️quantity/🟦️.ts";
export { RADAR_LABEL_EM, RADAR_METRICS, RadarChart, estimateTextWidth, radarAngle, radarFraction, radarLayout, radarPoint, radarPolygon, textMeasure, wrapLabel } from "./🔨️modules/🕸️radar/🟦️.tsx";
export type { RadarAxis, RadarBox, RadarFrame, RadarLabel, RadarLayout, RadarLegendEntry, RadarPoint, TextMeasure } from "./🔨️modules/🕸️radar/🟦️.tsx";
export { browserStorageArea, isCourseView, isChallenge, isLearnerView, isRecord, isRunView, isTimestamp, localChange, localStore, memoryStorageOrigin } from "./🔨️modules/💾️persistence/🟦️.ts";
export type { LocalChange, LocalCollection, LocalSlice, LocalStore, StorageArea } from "./🔨️modules/💾️persistence/🟦️.ts";
export { AGREEMENT_RECHECK_MS, ProctorClient, ProctorIncompatible, ProctorThrottled, ProctorUnavailable, COURSE_KINDS, RETRY_AFTER_MAX_MS, RETRY_TIMING, SIGN_UP_ALLOWANCE, abortable, commandEnvelope, commandTarget, commandVerdict, disagreement, errorRejection, isNotFound, isThrottled, isTransient, learnerPrincipal, newId, pause, proctorTransport, queryEnvelope, courseInstance, courseRejection, retryAfterMs, retryTransient, retryWait, signUpsSpent } from "./🔨️modules/🛂️proctor/🟦️.ts";
export type { CommandVerdict, HttpRequest, HttpResponse, HttpTransport, ProctorConnect, ProctorContract, ProctorReachability, RetryTiming } from "./🔨️modules/🛂️proctor/🟦️.ts";
export { Outbox, coalesce, coalescingKey, commandRun } from "./🔨️modules/📮️outbox/🟦️.ts";
export type { OutboxActivity, OutboxEntry, OutboxOptions, OutboxStatus } from "./🔨️modules/📮️outbox/🟦️.ts";
export { Deputy, REVISED, materialRevision } from "./🔨️modules/🫡️deputy/🟦️.ts";
export type { HeldLearner, CourseMaterial } from "./🔨️modules/🫡️deputy/🟦️.ts";
export { DEFAULT_BOARD, EMPTY_TRAIL, HOME_PAGES, TRAIL_LIMIT, boardKey, evolveCourseState, initialCourseState, lastSubmittedRunOf, mergeRunViews, openChallengeOf, openRunOf, overallLeaderboard, runAwards, sameStep, shownLeaderboard, stepAbove, stepAfter, stepBefore } from "./🔨️modules/🧭️session/🧮️state/🟦️.ts";
export { DEPUTY_PATIENCE_MS, PROJECTION_GRACE_MS } from "./🔨️modules/🧭️session/📋️session-model/🟦️.ts";
export { CourseSession } from "./🔨️modules/🧭️session/🟦️.ts";
export { restoreCourseState } from "./🔨️modules/🧭️session/💾️restore/🟦️.ts";
export type { BoardChoice, HeldLeaderboard, CourseClientEvent, CourseLearner, CourseNotice, CourseState, CourseStep, CourseTrail } from "./🔨️modules/🧭️session/🧮️state/🟦️.ts";
export type { CourseConnection, CourseSessionOptions, CourseSnapshot, RefreshOutcome, SessionFailure, SubmissionPhase } from "./🔨️modules/🧭️session/📋️session-model/🟦️.ts";
export { NAVIGATION_WAYS, NavigationControls, addressPage, navigationWays, placeName, stepAddress, useAddress } from "./🔨️modules/🚏️navigation/🟦️.tsx";
export type { NavigationWay } from "./🔨️modules/🚏️navigation/🟦️.tsx";
export { EveryLanguage, LOCALE_NAMES, LanguageChoice, LanguageSwitch, PreferencesCard, PreferencesPage, PreferencesPanel, PreferencesPanelCard, COURSE_DEVICES, TEXT_SIZES, THEME_CHOICES, challengeOf, effectiveIconMotion, everyLanguage, readPreferences, textScale, withChallenge, withIcons, writePreferences } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export type { CourseDevice, CoursePreferences, TextSize, ThemeChoice } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export { OTHERS_CHOICE_LABELS, PET_CHOICE_LABELS, withPets } from "./🔨️modules/🎛️preferences/🟦️.tsx";
export { PET_CHOICES, PetTopic, PetsPlay, PetsSwitch, CoursePets, CoursePetsProvider, effectivePetMode, petProp, switchedPets, usePetCast, usePetQuestion, usePetTopic, usePetsForced, usePetsReduced } from "./🔨️modules/🐾️pets/🟦️.tsx";
export type { PetChoice, PetLiveliness, CoursePetsSource, ShownPets } from "./🔨️modules/🐾️pets/🟦️.tsx";
export type { PetPlay, PetPlayer, CoursePetsStage } from "./🔨️modules/🐾️pets/🎪️stage/🟦️.tsx";
export { PET_DEEDS, PET_DEED_LABELS, PET_DEED_SAID, PET_HOME_SCENE, PetsPlayground, COURSE_PETS_TEMPO, COURSE_PET_CONTROLS, COURSE_PET_KEEPOUTS, COURSE_PET_PROPS, COURSE_PET_SURFACES, CoursePetLayer, peerGlances, petNames, petPlayers, petScene, petsTempo, shinyPetSpecies } from "./🔨️modules/🐾️pets/🎪️stage/🟦️.tsx";
export type { PetDeed } from "./🔨️modules/🐾️pets/🎪️stage/🟦️.tsx";
export { DRAG_THRESHOLD_PX, dropZoneAt, startPointerDrag } from "./🔨️modules/🤏️drag/🟦️.ts";
export { DROP_ZONE_CLASS, DragGrip, GuessField, HINT_SYMBOL, HintNote, ICON_BUTTON_CLASS, LockedNote, SELECT_ANNOUNCEMENT_DELAY_MS, SELECT_CLASS, compareText, describedBy, elementId, exampleGuess, hintName, hintTerm, useFocusAfterRender, useHintAnnouncement } from "./🔨️modules/🧩️task/🟦️.tsx";
export type { TaskViewProps } from "./🔨️modules/🧩️task/🟦️.tsx";
export { BodyButton, CardAction, CardIcon, Dialog, Facts, Glyph, ICON_HOSTS, ICON_LOOP_MS, IconLabel, LiveRegion, Mark, Missing, ProblemNote, CourseCard, Records, Segments, TABLE, TASK_KIND_ICONS, TaskGlyph, playTappedIcons, textPresentation, useAnnouncement } from "./🔨️modules/🪟️chrome/🟦️.tsx";
export type { Announcement, Problem, Segment } from "./🔨️modules/🪟️chrome/🟦️.tsx";
export { ClassificationTaskView, classificationHintText } from "./🔨️modules/🗂️classification/🟦️.tsx";
export { SortingTaskView, ordered, reordered } from "./🔨️modules/↕️sorting/🟦️.tsx";
export { MatchingTaskView, assignCard } from "./🔨️modules/🃏️matching/🟦️.tsx";
export { CourseIntroduction, courseIntroduction } from "./🔨️modules/👋️introduction/🟦️.tsx";
export { LearnerCard, LearnerPage, SwitchIdentity } from "./🔨️modules/📇️profile/🟦️.tsx";
export { QuizCardView, QuizPage } from "./🔨️modules/📖️quiz-page/🟦️.tsx";
export { CHALLENGE_HINTS, CHALLENGE_LABELS, ChallengeChooser, challengeScored } from "./🔨️modules/⛰️challenge/🟦️.tsx";
export type { Act } from "./🔨️modules/📖️quiz-page/🟦️.tsx";
export { IdentityScreen, failureProblem, handleFault, handleFaultMessage, learnerName, noticeProblem, thrownProblem } from "./🔨️modules/🪪️identity/🟦️.tsx";
export type { HandleFault } from "./🔨️modules/🪪️identity/🟦️.tsx";
export { HOME_CHROME_HEIGHT_PX, HOME_GRID_ROW_HEIGHT_PX, HOME_GRID_TRACKS, HomeScreen, addressPages, homeCells, homeGridMinHeight, homeLayout, homeLayoutQueries, homePages, homeTrackTemplate, isHomeGrid, openedChild, pageLabel, pagesWithin, parentPage, previewCamera } from "./🔨️modules/🏠️home/🟦️.tsx";
export type { HomeGrid, HomeLayout, PreviewCamera } from "./🔨️modules/🏠️home/🟦️.tsx";



export { BOARD_EXCERPT_SIZE, LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, POLL_BACKOFF_MAX_MS, RANK_ORDER, boardExcerpt, leaderboardColumns, nextSort, ownRow, pollDelay, sortLeaderboard, usePolling } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
export type { LeaderboardColumn, LeaderboardKey, LeaderboardSort } from "./🔨️modules/🏆️leaderboard/🟦️.tsx";
export { BadgesCard, BadgesPage, awardOf, badgesEarnedIn } from "./🔨️modules/🏅️badges/🟦️.tsx";
export { LegalFooter, PrivacyNotice, legalLink } from "./🔨️modules/⚖️legal/🟦️.tsx";
export type { CourseLegal } from "./🔨️modules/⚖️legal/🟦️.tsx";
export { EMPTY_PRESENCE_VIEW, PRESENCE_ANCHORS, placeText, presenceSelf, presenceView } from "./🔨️modules/👥️presence/👁️view/🟦️.ts";
export { MAX_WATCHED_ROOMS, PRESENCE_FRAME_HZ, PRESENCE_FRAME_INTERVAL_MS, PRESENCE_STABLE_MS, PresenceRoom, THINKING_FRAME_INTERVAL_MS, WATCH_INTERVAL_MS, browserPresenceConnect, rejoinDelay } from "./🔨️modules/👥️presence/🔌️socket/🟦️.ts";
export { OnlineMark, PEER_LABEL_CLEARANCE_PX, PanePeers, PresenceList, PresenceOverlay, PresenceProvider, PresenceStatus, PresenceFooter, CardPresence, cursorAt, viewportCursor, paintStyle, peerInk, placePeers, useDocumentVisible, usePresencePointer, usePresenceTask, usePresenceView } from "./🔨️modules/👥️presence/🟦️.tsx";
export { CoursePresence, homeWatchScopes, presenceDrafts, presencePlace, sheetItemLabels } from "./🔨️modules/👥️presence/🧭️rooms/🟦️.ts";
export type { PeerCursor, PresenceView } from "./🔨️modules/👥️presence/👁️view/🟦️.ts";
export type { PresenceConnect, PresenceRoomOptions, PresenceSocket, RoomMember, RoomSnapshot, RoomStatus } from "./🔨️modules/👥️presence/🔌️socket/🟦️.ts";
export type { PresenceDrafts, PresencePlace, PresenceSelf, CoursePresenceInput, CoursePresenceOptions, CoursePresenceSnapshot, WatchedState } from "./🔨️modules/👥️presence/🧭️rooms/🟦️.ts";
export { FOLLOW_DEPARTURE_GRACE_MS, FOLLOW_REJOIN_GRACE_MS, FollowProvider, evolveFollow, followedPreferences, followedCourseState, followStep, observerSession, publicPresenceRun, useFollow, usePresenceFollow, usePresenceViewport } from "./🔨️modules/👀️follow/🟦️.tsx";
export type { FollowEvent } from "./🔨️modules/👀️follow/🟦️.tsx";
export { AnswerFigure, CROWD_DOTS, CrowdDoor, OTHERS_CHOICES, ScoreFigure, TaskFigures, answerFigure, crowdGate, crowdPlace, crowdShown, keysHidden, livePlace, plotFold, scoreFigure } from "./🔨️modules/🗳️crowd/🟦️.tsx";
export type { AnswerCell, AnswerColumn, AnswerFigureInput, AnswerFigureModel, AnswerRow, CrowdGate, CrowdPlace, OthersChoice, ScoreFigureModel } from "./🔨️modules/🗳️crowd/🟦️.tsx";
export { Column, columnShare, formatShare, peakOf } from "./🔨️modules/📊️plot/🟦️.tsx";
//#endregion 🔁️Reexports

export * from "./🔨️modules/📚️app/🟦️.tsx";
