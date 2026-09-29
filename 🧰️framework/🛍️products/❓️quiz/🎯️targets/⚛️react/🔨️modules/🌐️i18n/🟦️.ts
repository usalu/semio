/** 🌐️ Every string the quiz client itself shows, in English and German, registered on the repo's shared i18n port.
 *
 * Learner content (titles, prompts, labels, explanations) is data and arrives as `{ en, de }` {@link Text}; only the
 * client's own chrome lives here. There is no default language: the locale comes from an explicit, persisted choice,
 * else from the browser's language list, with English winning when neither English nor German is listed.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx — `UiI18nPort`, `UiLabelValue`
 * @see ../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts — `@semio-tech/ui-react/i18n`: `registerUiTranslationBundles`, `uiI18n`
 */

import { LANGUAGES, type Rejection, type TaskKind, type Text } from "@semio-tech/quiz";
import { registerUiTranslationBundles, resolveUiLabel, setUiLocale, uiI18n, type DeepUiTranslationKeys, type UiLabelValue } from "@semio-tech/ui-react/i18n";

//#region 🗣️Locale
/** 🗣️ A language the quiz client speaks; the same set every learner {@link Text} carries. */
export type QuizLocale = (typeof LANGUAGES)[number];

/** 🗣️ Every {@link QuizLocale}, English first. */
export const QUIZ_LOCALES: readonly QuizLocale[] = LANGUAGES;

/** 🗣️ Whether `value` names a {@link QuizLocale}. */
export function isQuizLocale(value: unknown): value is QuizLocale {
  return typeof value === "string" && (QUIZ_LOCALES as readonly string[]).includes(value);
}

/** 🧭️ The first of the browser's `languages` (BCP 47 tags, preference order) the client speaks; English when none. */
export function preferredLocale(languages: readonly string[]): QuizLocale {
  for (const language of languages) {
    const primary = language.toLowerCase().split("-")[0];
    if (isQuizLocale(primary)) return primary;
  }
  return "en";
}

/** 🌍️ The learner-visible text of `text` in `locale`. */
export function localized(text: Text, locale: QuizLocale): string {
  return text[locale];
}

/** 🔁️ Makes `locale` the language of the shared i18n port and of the document. */
export function applyLocale(locale: QuizLocale): Promise<unknown> {
  return setUiLocale(locale);
}
//#endregion 🗣️Locale

//#region 📚️Bundles
const phrase = (normal: string, beginner: string = normal): UiLabelValue => ({ label: { normal, beginner } });

/** 📚️ English chrome of the quiz client. */
export const QUIZ_BUNDLE_EN = {
  quiz: {
    app: {
      skip: phrase("Skip to main content"),
      loading: phrase("Loading…"),
      fetching: phrase("Loading from the quiz server…"),
      unreachable: phrase("The quiz server cannot be reached right now – trying again."),
      failed: phrase("The quiz server could not answer this request."),
      retry: phrase("Try again now"),
      dismiss: phrase("Dismiss"),
    },
    nav: {
      label: phrase("Main navigation"),
      home: phrase("Quizzes"),
      leaderboard: phrase("Leaderboard"),
      introduction: phrase("Introduction"),
      settings: phrase("Settings"),
    },
    connection: {
      label: phrase("Connection"),
      connecting: phrase("Connecting to the quiz server…"),
      saved: phrase("All answers saved"),
      saving: phrase("Saving answers ({{waiting}} waiting)"),
      reconnecting: phrase("Connection lost – retrying"),
      reconnectingWaiting: phrase("Connection lost – retrying ({{waiting}} answers kept on this device)"),
      offline: phrase("Offline – answers stay on this device until the connection returns"),
    },
    preferences: {
      title: phrase("Settings"),
      language: phrase("Language"),
      theme: phrase("Theme"),
      themeSystem: phrase("Like the system"),
      themeLight: phrase("Light"),
      themeDark: phrase("Dark"),
      textSize: phrase("Text size"),
      textNormal: phrase("Normal"),
      textLarge: phrase("Large"),
      textLarger: phrase("Larger"),
      textLargest: phrase("Largest"),
      english: phrase("English"),
      german: phrase("Deutsch"),
      close: phrase("Close"),
    },
    introduction: {
      continue: phrase("Continue"),
    },
    identity: {
      title: phrase("How do you want to appear?"),
      lead: phrase("Choose how your results appear on the leaderboard."),
      kind: phrase("Identity"),
      anonymous: phrase("Anonymous"),
      anonymousHint: phrase("You appear as “Anonymous” with a short code. Your progress stays tied to this device."),
      pseudonym: phrase("Pseudonym"),
      pseudonymHint: phrase("You appear under a name you make up."),
      name: phrase("Name"),
      nameHint: phrase("You appear under your name."),
      handlePseudonym: phrase("Your pseudonym"),
      handleName: phrase("Your name"),
      noPassword: phrase(
        "No password is needed. Entering a pseudonym or name that is already known loads its existing progress – on any device. Anyone who enters the same pseudonym or name continues that progress, so pick one others will not guess.",
      ),
      submit: phrase("Continue"),
      working: phrase("Signing in…"),
      handleInvalid: phrase("Please enter 1 to 64 characters."),
      current: phrase("Signed in as {{name}}"),
      switch: phrase("Switch identity"),
    },
    home: {
      title: phrase("Quizzes"),
      welcome: phrase("Welcome, {{name}}"),
      total: phrase("Total: {{points}} points"),
      tasks: phrase("Tasks: {{amount}}"),
      best: phrase("Best score: {{score}}"),
      notYet: phrase("Not attempted yet"),
      open: phrase("In progress"),
      start: phrase("Start quiz"),
      resume: phrase("Resume quiz"),
      again: phrase("Start again"),
      lastResult: phrase("View last result"),
      starting: phrase("Preparing your quiz…"),
      badges: phrase("Badges"),
      locked: phrase("Not yet earned"),
      earnedAt: phrase("Earned on {{date}}"),
    },
    task: {
      classification: phrase("Classification"),
      sorting: phrase("Sorting"),
      matching: phrase("Matching"),
    },
    run: {
      tasks: phrase("Tasks"),
      progress: phrase("{{done}} of {{total}} tasks complete"),
      task: phrase("Task {{index}} of {{total}}"),
      complete: phrase("Complete"),
      incomplete: phrase("Incomplete"),
      previous: phrase("Previous task"),
      next: phrase("Next task"),
      submit: phrase("Submit quiz"),
      submitHint: phrase("Complete every task to submit. Results are shown after submitting."),
      confirmTitle: phrase("Submit this quiz?"),
      confirmBody: phrase("After submitting, your answers can no longer be changed."),
      confirm: phrase("Submit now"),
      keepWorking: phrase("Keep working"),
      progressLabel: phrase("Submission progress"),
      saving: phrase("Saving answers ({{done}} of {{total}})"),
      submitting: phrase("Submitting…"),
      loadingResults: phrase("Loading results…"),
      cancel: phrase("Cancel"),
      cancelled: phrase("Cancelled. Your answers are kept."),
      voided: phrase("This quiz has been revised, so your run was voided. Please start again."),
    },
    classification: {
      pool: phrase("Items to classify"),
      poolEmpty: phrase("All items are assigned."),
      categories: phrase("Categories"),
      binEmpty: phrase("No items yet – drop items here."),
      categoryFor: phrase("Category for {{item}}"),
      unassigned: phrase("Not assigned"),
      drag: phrase("Drag {{item}}"),
      assigned: phrase("{{item}} assigned to {{category}}"),
      released: phrase("{{item}} is no longer assigned"),
    },
    sorting: {
      hint: phrase("Order the items by {{quantity}}: smallest at the top, largest at the bottom."),
      list: phrase("Order by {{quantity}}"),
      smallest: phrase("Smallest"),
      largest: phrase("Largest"),
      up: phrase("Move {{item}} up"),
      down: phrase("Move {{item}} down"),
      drag: phrase("Drag {{item}}"),
      moved: phrase("{{item}} is now at position {{position}} of {{total}}"),
      keep: phrase("Keep this order"),
      keepHint: phrase("The order counts once you move an item or keep the current order."),
    },
    matching: {
      hint: phrase("Assign each item one value card per quantity. Each card can be used once."),
      cards: phrase("Value cards: {{quantity}}"),
      item: phrase("Item"),
      free: phrase("available"),
      used: phrase("used by {{item}}"),
      cardFor: phrase("{{quantity}} of {{item}}"),
      none: phrase("Not assigned"),
      unassign: phrase("Remove the value of {{item}}"),
      drag: phrase("Drag card {{value}}"),
      assigned: phrase("{{value}} assigned to {{item}}"),
      released: phrase("{{item}} has no value"),
    },
    radar: {
      label: phrase("Spider diagram of {{name}}"),
      table: phrase("Values as a table"),
      axis: phrase("Axis"),
      value: phrase("Value"),
      minimum: phrase("Minimum"),
      maximum: phrase("Maximum"),
    },
    results: {
      title: phrase("Results: {{quiz}}"),
      score: phrase("Your score: {{score}}"),
      taskScore: phrase("Task score: {{score}}"),
      item: phrase("Item"),
      yourAnswer: phrase("Your answer"),
      solution: phrase("Solution"),
      credit: phrase("Credit"),
      explanation: phrase("Explanation"),
      correct: phrase("Correct"),
      partial: phrase("Partly correct"),
      wrong: phrase("Not correct"),
      position: phrase("Your position"),
      rank: phrase("Correct position"),
      value: phrase("Value"),
      trueOrder: phrase("Correct order"),
      dimension: phrase("{{quantity}}: {{score}}"),
      newBadges: phrase("New badges"),
      home: phrase("Back to quizzes"),
      leaderboard: phrase("Open the leaderboard"),
      missing: phrase("The result of this run is not available yet."),
    },
    leaderboard: {
      title: phrase("Leaderboard"),
      caption: phrase("All learners with a submitted quiz, ranked by total points. Select a column heading to sort."),
      rank: phrase("Rank"),
      learner: phrase("Learner"),
      total: phrase("Total"),
      badges: phrase("Badges"),
      runs: phrase("Runs"),
      lastActivity: phrase("Last activity"),
      anonymous: phrase("Anonymous"),
      you: phrase("you"),
      empty: phrase("No submitted quizzes yet."),
      updated: phrase("Updated {{time}}"),
      sort: phrase("Sort by {{column}}"),
    },
    rejection: {
      unknownLearner: phrase("This identity is unknown to the quiz server. Please choose your identity again."),
      unknownQuiz: phrase("This quiz is not offered anymore."),
      unknownRun: phrase("This run is unknown to the quiz server."),
      unknownTask: phrase("This task is not part of the run."),
      runOpen: phrase("A run of this quiz is already open."),
      runClosed: phrase("This run is already closed."),
      runIncomplete: phrase("Please complete every task before submitting."),
      answerInvalid: phrase("This answer could not be accepted."),
      quizRevised: phrase("This quiz has been revised; please start again."),
      handleInvalid: phrase("Please enter 1 to 64 characters."),
      refused: phrase("The quiz server refused the request ({{detail}})."),
    },
  },
};

/** 📚️ German chrome of the quiz client — the same keys as {@link QUIZ_BUNDLE_EN}, enforced by its type. */
export const QUIZ_BUNDLE_DE: typeof QUIZ_BUNDLE_EN = {
  quiz: {
    app: {
      skip: phrase("Zum Hauptinhalt springen"),
      loading: phrase("Wird geladen…"),
      fetching: phrase("Wird vom Quiz-Server geladen…"),
      unreachable: phrase("Der Quiz-Server ist gerade nicht erreichbar – neuer Versuch läuft."),
      failed: phrase("Der Quiz-Server konnte diese Anfrage nicht beantworten."),
      retry: phrase("Jetzt erneut versuchen"),
      dismiss: phrase("Schließen"),
    },
    nav: {
      label: phrase("Hauptnavigation"),
      home: phrase("Quizze"),
      leaderboard: phrase("Rangliste"),
      introduction: phrase("Einführung"),
      settings: phrase("Einstellungen"),
    },
    connection: {
      label: phrase("Verbindung"),
      connecting: phrase("Verbindung zum Quiz-Server wird aufgebaut…"),
      saved: phrase("Alle Antworten gespeichert"),
      saving: phrase("Antworten werden gespeichert ({{waiting}} ausstehend)"),
      reconnecting: phrase("Verbindung unterbrochen – neuer Versuch läuft"),
      reconnectingWaiting: phrase("Verbindung unterbrochen – neuer Versuch läuft ({{waiting}} Antworten auf diesem Gerät gesichert)"),
      offline: phrase("Offline – Antworten bleiben auf diesem Gerät, bis die Verbindung zurück ist"),
    },
    preferences: {
      title: phrase("Einstellungen"),
      language: phrase("Sprache"),
      theme: phrase("Farbschema"),
      themeSystem: phrase("Wie das System"),
      themeLight: phrase("Hell"),
      themeDark: phrase("Dunkel"),
      textSize: phrase("Textgröße"),
      textNormal: phrase("Normal"),
      textLarge: phrase("Groß"),
      textLarger: phrase("Größer"),
      textLargest: phrase("Am größten"),
      english: phrase("English"),
      german: phrase("Deutsch"),
      close: phrase("Schließen"),
    },
    introduction: {
      continue: phrase("Weiter"),
    },
    identity: {
      title: phrase("Wie möchtest du erscheinen?"),
      lead: phrase("Wähle, wie deine Ergebnisse in der Rangliste erscheinen."),
      kind: phrase("Identität"),
      anonymous: phrase("Anonym"),
      anonymousHint: phrase("Du erscheinst als „Anonym“ mit einem kurzen Code. Dein Fortschritt bleibt an dieses Gerät gebunden."),
      pseudonym: phrase("Pseudonym"),
      pseudonymHint: phrase("Du erscheinst unter einem selbst gewählten Namen."),
      name: phrase("Name"),
      nameHint: phrase("Du erscheinst unter deinem Namen."),
      handlePseudonym: phrase("Dein Pseudonym"),
      handleName: phrase("Dein Name"),
      noPassword: phrase(
        "Es ist kein Passwort nötig. Wer ein bereits bekanntes Pseudonym oder einen bekannten Namen eingibt, lädt den bisherigen Fortschritt – auf jedem Gerät. Wer dasselbe Pseudonym oder denselben Namen eingibt, setzt diesen Fortschritt fort; wähle also etwas, das andere nicht erraten.",
      ),
      submit: phrase("Weiter"),
      working: phrase("Anmeldung läuft…"),
      handleInvalid: phrase("Bitte gib 1 bis 64 Zeichen ein."),
      current: phrase("Angemeldet als {{name}}"),
      switch: phrase("Identität wechseln"),
    },
    home: {
      title: phrase("Quizze"),
      welcome: phrase("Willkommen, {{name}}"),
      total: phrase("Gesamt: {{points}} Punkte"),
      tasks: phrase("Aufgaben: {{amount}}"),
      best: phrase("Beste Wertung: {{score}}"),
      notYet: phrase("Noch nicht versucht"),
      open: phrase("Begonnen"),
      start: phrase("Quiz starten"),
      resume: phrase("Quiz fortsetzen"),
      again: phrase("Erneut starten"),
      lastResult: phrase("Letztes Ergebnis ansehen"),
      starting: phrase("Dein Quiz wird vorbereitet…"),
      badges: phrase("Abzeichen"),
      locked: phrase("Noch nicht erhalten"),
      earnedAt: phrase("Erhalten am {{date}}"),
    },
    task: {
      classification: phrase("Klassifizierung"),
      sorting: phrase("Sortierung"),
      matching: phrase("Zuordnung"),
    },
    run: {
      tasks: phrase("Aufgaben"),
      progress: phrase("{{done}} von {{total}} Aufgaben erledigt"),
      task: phrase("Aufgabe {{index}} von {{total}}"),
      complete: phrase("Erledigt"),
      incomplete: phrase("Offen"),
      previous: phrase("Vorherige Aufgabe"),
      next: phrase("Nächste Aufgabe"),
      submit: phrase("Quiz abgeben"),
      submitHint: phrase("Erledige alle Aufgaben, um abzugeben. Die Ergebnisse erscheinen nach der Abgabe."),
      confirmTitle: phrase("Dieses Quiz abgeben?"),
      confirmBody: phrase("Nach der Abgabe kannst du deine Antworten nicht mehr ändern."),
      confirm: phrase("Jetzt abgeben"),
      keepWorking: phrase("Weiterbearbeiten"),
      progressLabel: phrase("Fortschritt der Abgabe"),
      saving: phrase("Antworten werden gespeichert ({{done}} von {{total}})"),
      submitting: phrase("Wird abgegeben…"),
      loadingResults: phrase("Ergebnisse werden geladen…"),
      cancel: phrase("Abbrechen"),
      cancelled: phrase("Abgebrochen. Deine Antworten bleiben erhalten."),
      voided: phrase("Dieses Quiz wurde überarbeitet, daher wurde dein Durchgang verworfen. Bitte starte neu."),
    },
    classification: {
      pool: phrase("Zu klassifizierende Elemente"),
      poolEmpty: phrase("Alle Elemente sind zugeordnet."),
      categories: phrase("Kategorien"),
      binEmpty: phrase("Noch keine Elemente – Elemente hier ablegen."),
      categoryFor: phrase("Kategorie für {{item}}"),
      unassigned: phrase("Nicht zugeordnet"),
      drag: phrase("{{item}} ziehen"),
      assigned: phrase("{{item}} zugeordnet zu {{category}}"),
      released: phrase("{{item}} ist nicht mehr zugeordnet"),
    },
    sorting: {
      hint: phrase("Ordne die Elemente nach {{quantity}}: das kleinste oben, das größte unten."),
      list: phrase("Reihenfolge nach {{quantity}}"),
      smallest: phrase("Kleinstes"),
      largest: phrase("Größtes"),
      up: phrase("{{item}} nach oben verschieben"),
      down: phrase("{{item}} nach unten verschieben"),
      drag: phrase("{{item}} ziehen"),
      moved: phrase("{{item}} steht jetzt an Position {{position}} von {{total}}"),
      keep: phrase("Diese Reihenfolge übernehmen"),
      keepHint: phrase("Die Reihenfolge zählt, sobald du ein Element verschiebst oder die aktuelle Reihenfolge übernimmst."),
    },
    matching: {
      hint: phrase("Ordne jedem Element pro Größe eine Wertkarte zu. Jede Karte kann einmal verwendet werden."),
      cards: phrase("Wertkarten: {{quantity}}"),
      item: phrase("Element"),
      free: phrase("verfügbar"),
      used: phrase("verwendet von {{item}}"),
      cardFor: phrase("{{quantity}} von {{item}}"),
      none: phrase("Nicht zugeordnet"),
      unassign: phrase("Wert von {{item}} entfernen"),
      drag: phrase("Karte {{value}} ziehen"),
      assigned: phrase("{{value}} zugeordnet zu {{item}}"),
      released: phrase("{{item}} hat keinen Wert"),
    },
    radar: {
      label: phrase("Netzdiagramm von {{name}}"),
      table: phrase("Werte als Tabelle"),
      axis: phrase("Achse"),
      value: phrase("Wert"),
      minimum: phrase("Minimum"),
      maximum: phrase("Maximum"),
    },
    results: {
      title: phrase("Ergebnisse: {{quiz}}"),
      score: phrase("Deine Wertung: {{score}}"),
      taskScore: phrase("Wertung der Aufgabe: {{score}}"),
      item: phrase("Element"),
      yourAnswer: phrase("Deine Antwort"),
      solution: phrase("Lösung"),
      credit: phrase("Anrechnung"),
      explanation: phrase("Erklärung"),
      correct: phrase("Richtig"),
      partial: phrase("Teilweise richtig"),
      wrong: phrase("Nicht richtig"),
      position: phrase("Deine Position"),
      rank: phrase("Richtige Position"),
      value: phrase("Wert"),
      trueOrder: phrase("Richtige Reihenfolge"),
      dimension: phrase("{{quantity}}: {{score}}"),
      newBadges: phrase("Neue Abzeichen"),
      home: phrase("Zurück zu den Quizzen"),
      leaderboard: phrase("Zur Rangliste"),
      missing: phrase("Das Ergebnis dieses Durchgangs ist noch nicht verfügbar."),
    },
    leaderboard: {
      title: phrase("Rangliste"),
      caption: phrase("Alle Lernenden mit mindestens einem abgegebenen Quiz, nach Gesamtpunkten gereiht. Wähle eine Spaltenüberschrift zum Sortieren."),
      rank: phrase("Rang"),
      learner: phrase("Person"),
      total: phrase("Gesamt"),
      badges: phrase("Abzeichen"),
      runs: phrase("Durchgänge"),
      lastActivity: phrase("Zuletzt aktiv"),
      anonymous: phrase("Anonym"),
      you: phrase("du"),
      empty: phrase("Noch keine abgegebenen Quizze."),
      updated: phrase("Aktualisiert {{time}}"),
      sort: phrase("Nach {{column}} sortieren"),
    },
    rejection: {
      unknownLearner: phrase("Diese Identität ist dem Quiz-Server unbekannt. Bitte wähle deine Identität erneut."),
      unknownQuiz: phrase("Dieses Quiz wird nicht mehr angeboten."),
      unknownRun: phrase("Dieser Durchgang ist dem Quiz-Server unbekannt."),
      unknownTask: phrase("Diese Aufgabe gehört nicht zum Durchgang."),
      runOpen: phrase("Ein Durchgang dieses Quiz ist bereits offen."),
      runClosed: phrase("Dieser Durchgang ist bereits abgeschlossen."),
      runIncomplete: phrase("Bitte erledige vor der Abgabe alle Aufgaben."),
      answerInvalid: phrase("Diese Antwort konnte nicht angenommen werden."),
      quizRevised: phrase("Dieses Quiz wurde überarbeitet; bitte starte neu."),
      handleInvalid: phrase("Bitte gib 1 bis 64 Zeichen ein."),
      refused: phrase("Der Quiz-Server hat die Anfrage abgelehnt ({{detail}})."),
    },
  },
};

/** 🔑️ Every key of the quiz chrome as a dot path, e.g. `quiz.run.submit`. */
export type QuizLabelKey = DeepUiTranslationKeys<typeof QUIZ_BUNDLE_EN>;

const quizLabel = registerUiTranslationBundles({ en: { translation: QUIZ_BUNDLE_EN }, de: { translation: QUIZ_BUNDLE_DE } });

/** 🚫️ The chrome key explaining each proctor {@link Rejection}. */
export const REJECTION_LABELS: { readonly [R in Rejection]: QuizLabelKey } = {
  "unknown-learner": "quiz.rejection.unknownLearner",
  "unknown-quiz": "quiz.rejection.unknownQuiz",
  "unknown-run": "quiz.rejection.unknownRun",
  "unknown-task": "quiz.rejection.unknownTask",
  "run-open": "quiz.rejection.runOpen",
  "run-closed": "quiz.rejection.runClosed",
  "run-incomplete": "quiz.rejection.runIncomplete",
  "answer-invalid": "quiz.rejection.answerInvalid",
  "quiz-revised": "quiz.rejection.quizRevised",
  "handle-invalid": "quiz.rejection.handleInvalid",
};

/** 🧩️ The chrome key naming each {@link TaskKind}. */
export const TASK_KIND_LABELS: { readonly [K in TaskKind]: QuizLabelKey } = { classification: "quiz.task.classification", sorting: "quiz.task.sorting", matching: "quiz.task.matching" };
//#endregion 📚️Bundles

//#region 🔤️Text
/** 🔤️ Resolves one chrome key in the bound locale, interpolating `{{name}}` placeholders from `values`. */
export type QuizText = (key: QuizLabelKey, values?: Readonly<Record<string, string | number>>) => string;

/** 🔤️ A {@link QuizText} reading the shared i18n port in `locale`, independent of the port's current language. */
export function quizText(locale: QuizLocale): QuizText {
  return (key, values) => resolveUiLabel(uiI18n.t(quizLabel(key) as never, { ...values, lng: locale }), "normal") ?? key;
}
//#endregion 🔤️Text
