/** 🚶️ The whole learner journey against a proctor double speaking the framework wire (design §9a) and deciding with
 * the core's own pure deciders: introduction → identity → home → run (keyboard answers, a short outage) → submit →
 * results → leaderboard, then a reload that resumes locally in the chosen language — and the session on every
 * challenge: the sheet of each, switching voids the open run, the clock of an expert run on the session clock (openings,
 * time up, a submission with tasks unanswered, openings decided on the device and delivered before their answers), the
 * hints of an easy run with and without a deputy, points by challenge, and stored runs and learner views restored only
 * in the shape of the challenges — and that what the device shows of a run stays true to its answers: hints never
 * outlive the answer they were given for (a change, a waiting answer, a stale or overtaken read, a closed run), another
 * tab's copy merges its openings and hints, a refused answer is read again whatever waits for other runs, a revised quiz
 * voids a run whose opening the deputy decided, and an answer is judged at the instant of its own last edit — and a
 * proctor that cannot take what the device sends: one of an older or newer contract (asked nothing but its contract,
 * also when it replaces the proctor in the middle of a run), a network's sign-in page in its place and views outside
 * the contract are waited out without a loss or a word, and a screen that fails shows what happened in its place.
 */

import { StrictMode, type ReactElement } from "react";
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { decodeCommandEnvelope, decodeQueryEnvelope, encodeCommandOutcome, encodeQueryResult, type CommandEnvelope, type CommandOutcome, type EventRecord, type HttpRequest, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import {
  CHALLENGES,
  DEFAULT_LIMITS,
  WIRE_VERSION,
  catalogView,
  challengeRules,
  crowdView,
  decideHandle,
  decideLearner,
  emptyHandleState,
  emptyLearnerState,
  evolveHandle,
  evolveLearner,
  handleActorId,
  hintsOf,
  leaderboard,
  learnerTag,
  learnerView,
  normalizeHandle,
  runView,
  transcript,
  type Answer,
  type Catalog,
  type CatalogView,
  type Challenge,
  type Command,
  type Event,
  type HandleState,
  type Hint,
  type LearnerState,
  type LoadedQuiz,
  type Query,
  type Quiz,
  type RecordAnswerCommand,
  type RunView,
  type SheetTask,
} from "@semio-tech/quiz";
import {
  AGREEMENT_RECHECK_MS,
  DEFAULT_BOARD,
  Deputy,
  ProctorClient,
  ProctorUnavailable,
  QUIZ_LOCALES,
  QuizApp,
  QuizSession,
  ScreenBoundary,
  commandTarget,
  connectionMessage,
  evolveQuizState,
  formatQuantity,
  initialQuizState,
  isChallenge,
  keysHidden,
  lastSubmittedRunOf,
  learnerName,
  localStore,
  memoryStorageOrigin,
  mergeRunViews,
  newId,
  openChallengeOf,
  openRunOf,
  overallLeaderboard,
  quizInstance,
  quizText,
  restoreQuizState,
  shownLeaderboard,
  waitingMessage,
  type PresenceConnect,
  type PresenceSocket,
  type QuizMaterial,
  type StorageArea,
} from "@semio-tech/quiz-react";
import stylesheet from "../../🎯️targets/⚛️react/🎨️.css?raw";

const text = (en: string, de: string) => ({ en, de });

const QUIZ: Quiz = {
  schema: "semio.quiz/v1",
  id: "household",
  emoji: "🏠",
  title: text("Household physics", "Haushaltsphysik"),
  description: text("Three small tasks.", "Drei kleine Aufgaben."),
  tasks: [
    {
      kind: "classification",
      id: "climates",
      title: text("Climates", "Klimazonen"),
      prompt: text("Assign each place its climate.", "Ordnen Sie jedem Ort sein Klima zu."),
      axes: [
        { id: "temperature", label: text("Temperature", "Temperatur"), unit: "°C", min: -30, max: 50 },
        { id: "rain", label: text("Rain", "Regen"), unit: "mm", min: 0, max: 3000 },
        { id: "sun", label: text("Sunshine", "Sonnenschein"), unit: "h", min: 0, max: 4000 },
      ],
      categories: [
        { id: "hot-dry", label: text("Hot and dry", "Heiß und trocken"), profile: { temperature: 35, rain: 100, sun: 3800 } },
        { id: "cold-wet", label: text("Cold and wet", "Kalt und nass"), profile: { temperature: -5, rain: 1500, sun: 1200 } },
      ],
      items: [
        { id: "desert", label: text("Desert", "Wüste"), category: "hot-dry", explanation: text("Little rain, much sun.", "Wenig Regen, viel Sonne.") },
        { id: "fjord", label: text("Fjord", "Fjord"), category: "cold-wet" },
      ],
    },
    {
      kind: "sorting",
      id: "masses",
      title: text("Masses", "Massen"),
      prompt: text("Sort by mass.", "Nach Masse sortieren."),
      quantity: { label: text("Mass", "Masse"), unit: "g", scale: "logarithmic", prefixed: true, additive: true },
      items: [
        { id: "mouse", label: text("Mouse", "Maus"), value: 20 },
        { id: "cat", label: text("Cat", "Katze"), value: 4000 },
        { id: "horse", label: text("Horse", "Pferd"), value: 500000 },
      ],
    },
    {
      kind: "matching",
      id: "lamps",
      title: text("Lamps", "Lampen"),
      prompt: text("Match each lamp its power.", "Ordnen Sie jeder Lampe ihre Leistung zu."),
      dimensions: [{ id: "power", quantity: { label: text("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true, additive: true } }],
      items: [
        { id: "led", label: text("LED bulb", "LED-Lampe"), values: { power: 8 } },
        { id: "halogen", label: text("Halogen spot", "Halogenstrahler"), values: { power: 50 } },
        { id: "floodlight", label: text("Floodlight", "Flutlicht"), values: { power: 2000 } },
      ],
    },
  ],
};

const CATALOG: Catalog = {
  schema: "semio.quiz.catalog/v1",
  id: "test-catalog",
  title: text("Test catalog", "Testkatalog"),
  introduction: { title: text("Welcome to the test catalog", "Willkommen im Testkatalog"), paragraphs: [text("Quizzes about everyday physics.", "Quizze über Alltagsphysik.")] },
  quizzes: ["household.json"],
  badges: [
    { id: "all-done", emoji: "🎓", label: text("All done", "Alles erledigt"), description: text("Submitted every quiz.", "Jedes Quiz abgegeben."), rule: { kind: "completed-quizzes" } },
    { id: "flawless", emoji: "💎", label: text("Flawless", "Makellos"), description: text("A perfect household quiz.", "Ein perfektes Haushaltsquiz."), rule: { kind: "perfect-quiz", quiz: "household" } },
  ],
};

const encoder = new TextEncoder();
const decoder = new TextDecoder();

function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** ⏳️ How many reads of a fresh learner or run view the double answers "not found", like a projection catching up. */
const PROJECTION_LAG = 2;

/** 🛂️ The proctor double: the core's deciders behind the framework wire — one handle state per handle key, one learner
 * state per learner, a registration under a handle relayed to its learner at once — idempotent by command id, with
 * views that appear only after a short projection lag and a recall that is the `handle` read. While it is `away` no
 * request reaches it at all. It serves the household catalog unless it is given other material, and a quiz it
 * {@link FakeProctor.revise}s is one its runs were not started against. It speaks the wire version `contract` (this
 * client's unless a test replaces it): it declares it at `GET /instance` and refuses an envelope of another version
 * as the proctor does, while it is `unreadable` every answer is a page that is no JSON, and `mangle` changes the views
 * it answers. It notes every request it gets (`requests`) and how often it was asked for its contract (`instances`). */
class FakeProctor {
  readonly envelopes: CommandEnvelope[] = [];
  readonly requests: string[] = [];
  mangle: ((view: unknown, query: Query) => unknown) | undefined;
  readonly catalog: Catalog;
  readonly quizzes: Record<string, LoadedQuiz>;
  readonly view: CatalogView;
  outage = false;
  away = false;
  contract: number = WIRE_VERSION;
  unreadable = false;
  instances = 0;
  lagged = 0;
  runQueries = 0;
  submissions = 0;
  private held: Promise<void> | undefined;
  private releaseHeld: () => void = () => undefined;
  private readonly lagging = new Map<string, number>();
  private readonly handles = new Map<string, HandleState>();
  private readonly learners = new Map<string, LearnerState>();
  private readonly outcomes = new Map<string, CommandOutcome>();
  private seq = 0;
  now = 1_760_000_000_000;
  private readonly queries: Query[] = [];

  constructor(material: QuizMaterial = { catalog: CATALOG, quizzes: [QUIZ] }) {
    this.catalog = material.catalog;
    this.quizzes = Object.fromEntries(material.quizzes.map((quiz) => [quiz.id, { quiz, revision: "0".repeat(64) }]));
    this.view = catalogView(material.catalog, material.quizzes);
  }

  /** ✏️ From now on `quiz` stands at another revision, so its runs started before are stale. */
  revise(quiz: string): void {
    this.quizzes[quiz] = { quiz: this.quizzes[quiz]!.quiz, revision: "1".repeat(64) };
  }

  /** 🔍️ Every query of one type the proctor was asked so far, in order. */
  asked<T extends Query["type"]>(type: T): Extract<Query, { type: T }>[] {
    return this.queries.filter((query): query is Extract<Query, { type: T }> => query.type === type);
  }

  readonly transport: HttpTransport = { send: async (request) => this.handle(request) };

  decide(envelope: CommandEnvelope): CommandOutcome {
    const known = this.outcomes.get(envelope.idempotencyKey ?? envelope.commandId);
    if (known !== undefined) return known;
    const command = JSON.parse(decoder.decode(envelope.payload)) as Command;
    this.now += 1000;
    const key = command.type === "identify-learner" && command.identity.kind !== "anonymous" ? (normalizeHandle(command.identity.handle)?.key ?? "") : undefined;
    const decision =
      command.type === "identify-learner" && key !== undefined
        ? decideHandle(this.handles.get(key) ?? emptyHandleState(key), command, this.now)
        : decideLearner(this.learners.get(command.learner) ?? emptyLearnerState(command.learner), command, { now: this.now, catalog: this.catalog, quizzes: this.quizzes, limits: DEFAULT_LIMITS });
    const receipt = { commandId: envelope.commandId, actor: envelope.target, revision: this.seq, acceptedAt: envelope.clientHlc };
    const outcome: CommandOutcome =
      "rejection" in decision
        ? { status: "rejected", receipt, reason: { kind: "invalid", detail: decision.rejection }, notices: [{ code: decision.rejection, message: decision.rejection }] }
        : { status: "accepted", receipt, events: decision.events.map((event) => this.append(envelope, event)), frontier: null };
    this.outcomes.set(envelope.idempotencyKey ?? envelope.commandId, outcome);
    return outcome;
  }

  seed(command: Command): readonly Event[] {
    const outcome = this.decide({ ...this.envelope(command) });
    return outcome.status === "accepted" ? outcome.events.map((record) => JSON.parse(decoder.decode(record.payload)) as Event) : [];
  }

  runOf(run: string): LearnerState | undefined {
    return [...this.learners.values()].find((state) => state.runs.some((candidate) => candidate.run === run));
  }

  answersOf(run: string): Readonly<Record<string, Answer>> {
    return this.runOf(run)?.runs.find((candidate) => candidate.run === run)?.answers ?? {};
  }

  /** ✋️ From now on a submission is committed at once but answered only after {@link releaseSubmissions}. */
  holdSubmissions(): void {
    this.held = new Promise((resolve) => {
      this.releaseHeld = resolve;
    });
  }

  releaseSubmissions(): void {
    this.releaseHeld();
    this.held = undefined;
  }

  private envelope(command: Command): CommandEnvelope {
    return {
      commandId: command.id,
      kind: `quiz.${command.type}`,
      version: this.contract,
      target: commandTarget(command, CATALOG.id),
      scope: CATALOG.id,
      principal: { kind: "anonymous" },
      session: null,
      device: null,
      payload: encoder.encode(JSON.stringify(command)),
      causalFrontier: null,
      clientHlc: { millis: this.now, counter: 0 },
      expectedRevision: null,
      idempotencyKey: command.id,
      capabilityProof: null,
      trace: { traceId: "0".repeat(32), spanId: "0".repeat(16) },
    };
  }

  private append(envelope: CommandEnvelope, event: Event): EventRecord {
    if (event.type === "learner-registered") this.lagging.set(event.learner, PROJECTION_LAG);
    if (event.type === "run-started") this.lagging.set(event.run, PROJECTION_LAG);
    if (event.type === "learner-registered" && event.identity.kind !== "anonymous") {
      const key = normalizeHandle(event.identity.handle)!.key;
      this.handles.set(key, evolveHandle(this.handles.get(key) ?? emptyHandleState(key), event));
    }
    this.learners.set(event.learner, evolveLearner(this.learners.get(event.learner) ?? emptyLearnerState(event.learner), event));
    this.seq += 1;
    return { stream: envelope.target, seq: this.seq, hlc: { millis: event.at, counter: 0 }, kind: `quiz.${event.type}`, payload: encoder.encode(JSON.stringify(event)) };
  }

  private async handle(request: HttpRequest): Promise<HttpResponse> {
    if (this.away) throw new ProctorUnavailable("the proctor is away");
    this.requests.push(`${request.method} ${request.path}`);
    if (this.unreadable) return { status: 200, text: async () => "<!doctype html><title>Sign in</title>", bytes: async () => encoder.encode("<!doctype html>") };
    if (request.method === "GET" && request.path === "/instance") {
      this.instances += 1;
      return reply(200, quizInstance(this.contract));
    }
    const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
    if (request.method === "POST" && request.path === "/commands") {
      const envelope = decodeCommandEnvelope(body);
      if (envelope.version !== this.contract) {
        const receipt = { commandId: envelope.commandId, actor: envelope.target, revision: this.seq, acceptedAt: envelope.clientHlc };
        return reply(200, encodeCommandOutcome({ status: "rejected", receipt, reason: { kind: "invalid", detail: `envelope-mismatch: version ${envelope.version} is not ${this.contract}` }, notices: [] }));
      }
      this.envelopes.push(envelope);
      if (this.outage && envelope.kind === "quiz.record-answer") return reply(503, { kind: "actorUnavailable", message: "outage" });
      const outcome = this.decide(envelope);
      if (envelope.kind === "quiz.submit-run") {
        this.submissions += 1;
        if (this.held !== undefined) await this.held;
      }
      return reply(200, encodeCommandOutcome(outcome));
    }
    if (request.method === "POST" && request.path === "/queries") {
      const envelope = decodeQueryEnvelope(body);
      if (envelope.version !== this.contract) return reply(400, { kind: "badRequest", message: `${envelope.kind} speaks version ${this.contract}, not ${envelope.version}` });
      const query = JSON.parse(decoder.decode(envelope.arguments)) as Query;
      this.queries.push(query);
      if (query.type === "run") this.runQueries += 1;
      if (query.type === "handle" && normalizeHandle(query.handle) === undefined) return reply(400, { kind: "invalid", message: "handle-invalid: the handle is outside the policy" });
      const value = this.answer(query);
      if (value === undefined) return reply(404, { kind: "notFound", message: envelope.kind });
      return reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(this.mangle?.(value, query) ?? value)), frontier: null }));
    }
    return reply(404, { kind: "notFound", message: request.path });
  }

  private projecting(key: string): boolean {
    const misses = this.lagging.get(key) ?? 0;
    if (misses === 0) return false;
    this.lagging.set(key, misses - 1);
    this.lagged += 1;
    return true;
  }

  private answer(query: Query): unknown {
    if ((query.type === "learner" && this.projecting(query.learner)) || (query.type === "run" && this.projecting(query.run))) return undefined;
    switch (query.type) {
      case "catalog":
        return this.view;
      case "learner": {
        const state = this.learners.get(query.learner);
        return state === undefined ? undefined : learnerView(state, this.view);
      }
      case "run": {
        const state = this.runOf(query.run);
        return state === undefined ? undefined : runView(state, query.run, this.quizzes);
      }
      case "leaderboard":
        return leaderboard(
          [...this.learners.values()].flatMap((state) => transcript(state) ?? []),
          this.view,
          query,
          this.now,
          query.learner,
        );
      case "handle": {
        const handle = normalizeHandle(query.handle)!;
        const holder = this.handles.get(handle.key)?.holder;
        const identity = holder === undefined ? undefined : this.learners.get(holder)?.identity;
        return holder === undefined || identity === undefined ? { display: handle.display } : { display: handle.display, holder: { learner: holder, identity } };
      }
      case "crowd":
        return Object.hasOwn(this.quizzes, query.quiz)
          ? crowdView(
              this.quizzes[query.quiz]!.quiz,
              [...this.learners.values()].flatMap((state) => state.runs.flatMap((run) => (run.result === undefined ? [] : [run.result]))),
            )
          : undefined;
    }
  }
}

function sloppyAnswer(task: SheetTask): Answer {
  switch (task.kind) {
    case "classification":
      return { kind: "classification", assignments: Object.fromEntries(task.items.map((item) => [item.id, task.categories[0]!.id])) };
    case "sorting":
      return { kind: "sorting", order: task.items.map((item) => item.id).reverse() };
    case "matching":
      return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item, index) => [item.id, index]))])) };
  }
}

function seedAnonymousRival(proctor: FakeProctor, learner = "c".repeat(32), run = "d".repeat(32)): string {
  proctor.seed({ type: "identify-learner", id: newId(), learner, identity: { kind: "anonymous" } });
  proctor.seed({ type: "start-run", id: newId(), learner, run, quiz: QUIZ.id, challenge: "medium", at: proctor.now });
  const sheet = runView(proctor.runOf(run)!, run, proctor.quizzes)!.sheet;
  for (const task of sheet.tasks) proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: sloppyAnswer(task), at: proctor.now });
  const submitted = proctor.seed({ type: "submit-run", id: newId(), learner, run });
  expect(submitted.some((event) => event.type === "run-submitted")).toBe(true);
  return learner;
}

const TIMING = { minMs: 1, maxMs: 4 };

/** 🔇️ Presence sockets that never open: the journey is about the proctor's commands and queries. */
const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });

function navbar(): HTMLElement {
  return screen.getByRole("navigation", { name: /^(?:Main navigation|Hauptnavigation)$/u });
}

function app(proctor: FakeProctor, storage: StorageArea, material?: QuizMaterial) {
  return (
    <StrictMode>
      <QuizApp proctor="" tenant={CATALOG.id} material={material} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={storage} languages={["de-CH", "fr"]} timing={TIMING} />
    </StrictMode>
  );
}

async function answerCurrentTask(user: ReturnType<typeof userEvent.setup>): Promise<void> {
  const heading = screen.getByRole("heading", { level: 2, name: /Climates|Masses|Lamps/u }).textContent;
  if (heading === "Climates") {
    await user.selectOptions(screen.getByRole("combobox", { name: "Category for Desert" }), "hot-dry");
    await user.selectOptions(screen.getByRole("combobox", { name: "Category for Fjord" }), "cold-wet");
    return;
  }
  if (heading === "Masses") {
    const target = ["Mouse", "Cat", "Horse"];
    const order = (): string[] =>
      within(screen.getByRole("list", { name: "Order by Mass" }))
        .getAllByRole("listitem")
        .map((item) => item.querySelector(".quiz-sort-label")?.textContent ?? "");
    for (const [position, label] of target.entries()) {
      while (order().indexOf(label) > position) {
        screen.getByRole("button", { name: `Move ${label} up` }).focus();
        await user.keyboard("{Enter}");
      }
    }
    expect(order()).toEqual(target);
    return;
  }
  for (const [item, value] of [
    ["LED bulb", 8],
    ["Halogen spot", 50],
    ["Floodlight", 2000],
  ] as const) {
    const select = screen.getByRole("combobox", { name: `Power of ${item}` });
    const option = within(select)
      .getAllByRole("option")
      .find((candidate) => candidate.textContent?.startsWith(formatQuantity(value, { unit: "W", prefixed: true }, "en")));
    await user.selectOptions(select, option!);
  }
}

describe("🚶️ learner journey", () => {
  it("goes from the introduction through a submitted run to the leaderboard and resumes after a reload", { timeout: 60_000 }, async () => {
    const proctor = new FakeProctor();
    const rival = seedAnonymousRival(proctor);
    const origin = memoryStorageOrigin();
    const user = userEvent.setup();
    render(app(proctor, origin.tab()));

    await screen.findByRole("heading", { level: 1, name: "Willkommen im Testkatalog" });
    await waitFor(() => expect(document.documentElement.lang).toBe("de"));
    await user.click(within(navbar()).getByRole("button", { name: "English" }));
    expect(screen.getByRole("heading", { level: 1, name: "Welcome to the test catalog" })).toBeTruthy();
    expect(within(navbar()).getByRole("button", { name: "English" }).getAttribute("aria-pressed")).toBe("true");
    expect(
      within(screen.getByRole("region", { name: "Settings" }))
        .getByRole("button", { name: "English" })
        .getAttribute("aria-pressed"),
    ).toBe("true");
    await user.click(screen.getByRole("button", { name: "Continue" }));

    await screen.findByRole("heading", { level: 1, name: "How do you want to appear?" });
    expect(screen.getByText(/There are no passwords/u)).toBeTruthy();
    await user.click(screen.getByRole("radio", { name: "Pseudonym" }));
    await user.type(screen.getByRole("textbox", { name: "Your pseudonym" }), "  Ada   Lovelace ");
    await user.click(screen.getByRole("button", { name: "Continue" }));

    const learnerCard = await screen.findByRole("region", { name: "Ada Lovelace" });
    expect(within(learnerCard).getByText("Points: 0")).toBeTruthy();
    expect(within(learnerCard).getByText("0 of 1 quizzes played")).toBeTruthy();
    const identify = proctor.envelopes.find((envelope) => envelope.kind === "quiz.identify-learner")!;
    expect(identify.principal).toEqual({ kind: "anonymous" });
    expect(identify.target).toEqual({ tenant: CATALOG.id, kind: "quiz-handle", id: handleActorId("ada lovelace") });
    expect(JSON.parse(decoder.decode(identify.payload)).identity).toEqual({ kind: "pseudonym", handle: "  Ada   Lovelace " });
    const quizCard = screen.getByRole("region", { name: "Household physics" });
    expect(within(quizCard).getByText("Not attempted yet")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("0 of 2 badges earned")).toBeTruthy();

    await user.click(within(quizCard).getByRole("button", { name: "Start (Medium)" }));
    await screen.findByRole("heading", { level: 1, name: "Household physics" });
    await waitFor(() => expect(proctor.lagged).toBe(2 * PROJECTION_LAG));
    expect(JSON.parse(decoder.decode(proctor.envelopes.find((envelope) => envelope.kind === "quiz.start-run")!.payload))).toMatchObject({ quiz: QUIZ.id, challenge: "medium" });
    expect(screen.queryByRole("alert")).toBeNull();
    expect(screen.getByText("0 of 3 tasks complete")).toBeTruthy();
    const submit = screen.getByRole("button", { name: "Submit quiz" });
    expect((submit as HTMLButtonElement).disabled).toBe(false);
    expect(submit.getAttribute("aria-disabled")).toBe("true");
    expect(document.getElementById(submit.getAttribute("aria-describedby") ?? "")?.textContent).toBe("Complete every task to submit. Results are shown after submitting.");
    await user.click(submit);
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(document.activeElement).toBe(submit);

    const taskButtons = within(screen.getByRole("navigation", { name: "Tasks" })).getAllByRole("button");
    expect(taskButtons).toHaveLength(3);
    for (const [index, button] of taskButtons.entries()) {
      await user.click(button);
      expect(button.getAttribute("aria-current")).toBe("step");
      if (index === 2) proctor.outage = true;
      await answerCurrentTask(user);
    }
    const lost = await screen.findAllByText(/^Connection lost – retrying\. Answers kept on this device: \d$/u);
    expect(lost.map((element) => element.closest("[role=status]") !== null)).toEqual([false, true]);
    proctor.outage = false;
    await screen.findByText("All answers saved", {}, { timeout: 5000 });
    expect(document.querySelector("[data-connection-announcer]")?.textContent).toBe("Connection restored");
    expect(screen.getByText("3 of 3 tasks complete")).toBeTruthy();

    const answers = proctor.envelopes.filter((envelope) => envelope.kind === "quiz.record-answer");
    for (const envelope of answers) {
      expect(envelope.idempotencyKey).toBe(envelope.commandId);
      expect(envelope.principal.kind).toBe("user");
      expect(envelope.target.kind).toBe("quiz-learner");
    }
    const attempts = new Map<string, number>();
    for (const envelope of answers) attempts.set(envelope.commandId, (attempts.get(envelope.commandId) ?? 0) + 1);
    expect(Math.max(...attempts.values())).toBeGreaterThan(1);

    expect(screen.getByRole("button", { name: "Submit quiz" }).getAttribute("aria-disabled")).toBe("false");
    await user.click(screen.getByRole("button", { name: "Submit quiz" }));
    const dialog = screen.getByRole("alertdialog", { name: "Submit this quiz?" });
    expect(document.activeElement).toBe(within(dialog).getByRole("button", { name: "Submit now" }));
    expect(dialog.closest(".quiz-app")).not.toBeNull();
    expect(document.querySelector("#quiz-main")?.hasAttribute("inert")).toBe(true);
    await user.keyboard("{Escape}");
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(document.querySelector("#quiz-main")?.hasAttribute("inert")).toBe(false);
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Submit quiz" }));
    await user.keyboard("{Enter}");
    expect(document.activeElement).toBe(within(screen.getByRole("alertdialog", { name: "Submit this quiz?" })).getByRole("button", { name: "Submit now" }));
    await user.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Submit now" }));

    await screen.findByRole("heading", { level: 1, name: "Results: Household physics" });
    expect(document.title).toBe("Results: Household physics · Test catalog");
    expect(screen.getByText("Your score: 100%").closest("p")?.textContent).toBe("Your score: 100%");
    const newBadges = screen.getByRole("region", { name: "New badges" });
    expect(within(newBadges).getByText("All done")).toBeTruthy();
    expect(within(newBadges).getByText("Flawless")).toBeTruthy();
    const climates = screen.getByRole("region", { name: /Climates/u });
    const [answered, everyone] = within(climates).getAllByRole("table");
    const desertRow = within(answered!).getByRole("rowheader", { name: "Desert" }).closest("tr")!;
    expect(
      within(desertRow)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toEqual(["Hot and dry", "Hot and dry", "✓ Correct (100%)", "Little rain, much sun."]);
    expect(everyone!.getAttribute("aria-label")).toBe("What everyone answered: Climates");
    await waitFor(() => expect(everyone!.querySelector('[data-crowd-item="desert"] > [data-key="hot-dry"] > .sr-only')?.textContent).toBe("100% (2 of 2), your answer, correct"));
    const scored = screen.getByRole("list", { name: "How everyone scored: Household physics" });
    expect(scored.closest("figure")?.getAttribute("data-runs")).toBe("2");
    expect(within(scored).getAllByRole("listitem").at(-1)?.hasAttribute("data-own")).toBe(true);
    expect(within(climates).getByRole("list", { name: "How everyone scored: Climates" })).toBeTruthy();
    const masses = screen.getByRole("region", { name: /Masses/u });
    expect(
      within(masses)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toContain("500\u00a0kg");
    expect(within(masses).getByRole("heading", { name: "Correct order" })).toBeTruthy();
    const lamps = screen.getByRole("region", { name: /Lamps/u });
    expect(within(lamps).getByRole("heading", { name: "Power: 100%" })).toBeTruthy();

    await user.click(screen.getByRole("button", { name: "Open the leaderboard" }));
    const table = await screen.findByRole("table", { name: "Leaderboard" });
    expect(window.location.hash).toBe("#board");
    const description = screen.getByText(/^Learners with a submitted quiz, ranked by total points/u);
    expect(table.parentElement?.contains(description)).toBe(false);
    expect(table.getAttribute("aria-describedby")).toBe(description.id);
    const rows = () => within(table).getAllByRole("row").slice(1);
    await waitFor(() => expect(rows()).toHaveLength(2));
    const me = rows()[0]!;
    expect(me.getAttribute("aria-current")).toBe("true");
    expect(within(me).getByRole("rowheader").textContent).toBe("Ada Lovelace (you)");
    expect(within(rows()[1]!).getByRole("rowheader").textContent).toBe(`Anonymous #${learnerTag(rival)}`);
    await user.click(within(table).getByRole("button", { name: /^Learner/u }));
    expect(
      within(table)
        .getByRole("columnheader", { name: /Learner/u })
        .getAttribute("aria-sort"),
    ).toBe("ascending");
    await user.click(within(table).getByRole("button", { name: /^Learner/u }));
    expect(
      within(table)
        .getByRole("columnheader", { name: /Learner/u })
        .getAttribute("aria-sort"),
    ).toBe("descending");
    expect(within(rows()[0]!).getByRole("rowheader").textContent).toBe(`Anonymous #${learnerTag(rival)}`);

    const page = table.closest<HTMLElement>("[data-card='leaderboard']")!;
    const headings = () => within(screen.getByRole("table", { name: "Leaderboard" })).getAllByRole("columnheader").map((heading) => heading.textContent?.trim());
    const pressed = (group: string) =>
      within(within(page).getByRole("group", { name: group }))
        .getAllByRole("button")
        .map((button) => `${button.textContent}${button.getAttribute("aria-pressed") === "true" ? " ✓" : ""}`);
    expect(pressed("Period")).toEqual(["Today", "This week", "This month", "All time ✓"]);
    expect(pressed("Category")).toEqual(["All ✓", expect.stringMatching(/Household physics$/u)]);
    expect(headings()).toEqual(["Rank", "Learner", "Total", "Household physics", "Badges", "Runs", "Last submission"]);
    expect(within(page).queryByText(/^Counts the quizzes submitted from /u)).toBeNull();
    await user.click(within(page).getByRole("button", { name: "Today" }));
    await within(page).findByText(/^Counts the quizzes submitted from .+ until .+\.$/u);
    expect(pressed("Period")).toEqual(["Today ✓", "This week", "This month", "All time"]);
    const today = await screen.findByRole("table", { name: "Leaderboard" });
    expect(within(today).getAllByRole("rowheader").map((name) => name.textContent)).toEqual([`Anonymous #${learnerTag(rival)}`, "Ada Lovelace (you)"]);
    expect(within(today).getByRole("columnheader", { name: /Learner/u }).getAttribute("aria-sort")).toBe("descending");
    await user.click(within(within(page).getByRole("group", { name: "Category" })).getByRole("button", { name: /Household physics$/u }));
    await waitFor(() => expect(headings()).toEqual(["Rank", "Learner", "Points", "Badges", "Runs", "Last submission"]));
    expect(pressed("Category")).toEqual(["All", expect.stringMatching(/Household physics ✓$/u)]);
    expect(proctor.asked("leaderboard").slice(-1)).toEqual([{ type: "leaderboard", period: "daily", quiz: QUIZ.id, learner: expect.any(String) }]);
    await user.click(within(page).getByRole("button", { name: "All time" }));
    await user.click(within(page).getByRole("button", { name: "All" }));
    await waitFor(() => expect(pressed("Period").concat(pressed("Category"))).toEqual(["Today", "This week", "This month", "All time ✓", "All ✓", expect.stringMatching(/Household physics$/u)]));
    expect(headings()).toEqual(["Rank", "Learner", "Total", "Household physics", "Badges", "Runs", "Last submission"]);
    await user.click(screen.getByRole("button", { name: "Overview" }));
    await screen.findByRole("region", { name: "Household physics" });
    expect(window.location.hash).toBe("");

    cleanup();
    render(app(proctor, origin.tab()));
    await screen.findByRole("region", { name: "Ada Lovelace" });
    expect(screen.getByRole("heading", { level: 1, name: "Quizzes" })).toBeTruthy();
    const card = screen.getByRole("region", { name: "Household physics" });
    await within(card).findByText("Best: 200 of 200 (Medium)");
    expect(within(card).getByRole("button", { name: "Again (Medium)" })).toBeTruthy();
    expect(within(card).getByText(/^Earned here: /u).textContent).toContain("All done");
    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("2 of 2 badges earned")).toBeTruthy();
  });

  it("asks a device to identify again once the proctor does not know its learner beyond the projection grace", async () => {
    const proctor = new FakeProctor();
    const store = localStore(memoryStorageOrigin().tab(), CATALOG.id);
    store.write("introduced", true);
    store.write("learner", { id: "e".repeat(32), identity: { kind: "anonymous" } });
    let clock = 0;
    const session = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store, timing: TIMING, now: () => (clock += 4000) });
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    session.start();
    await waitFor(() => expect(session.getSnapshot().state.step).toEqual({ screen: "identity" }));
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "unknown-learner" });
    expect(store.read("learner")).toBeUndefined();
    session.stop();
  });

  it("asks for the leaderboard the learner chose, shows one it holds at once and keeps the overall one fresh while another is looked at", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    seedAnonymousRival(proctor);
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    expect(await session.identify({ kind: "pseudonym", handle: "Board Chooser" }, new AbortController().signal)).toBeUndefined();
    await waitFor(() => expect(session.getSnapshot().state.catalog).toBeDefined());
    const state = () => session.getSnapshot().state;
    const boards = () => proctor.asked("leaderboard").map(({ period, quiz }) => (quiz === undefined ? period : `${period}/${quiz}`));
    const today = { period: "daily", quiz: QUIZ.id } as const;
    expect(state().board).toEqual(DEFAULT_BOARD);
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    expect(boards()).toEqual(["all-time"]);
    expect(shownLeaderboard(state())?.board).toMatchObject({ period: "all-time", learners: 1, submissions: 1 });
    expect(state().submissions).toBe(1);

    session.chooseBoard(today);
    expect(state().board).toEqual(today);
    expect(shownLeaderboard(state())).toBeUndefined();
    await waitFor(() => expect(shownLeaderboard(state())?.board).toMatchObject({ ...today, learners: 1, submissions: 1 }));
    expect(shownLeaderboard(state())?.board.window).toMatchObject({ from: expect.any(Number), until: expect.any(Number) });
    await session.refreshLeaderboard();
    expect(boards()).toEqual(["all-time", `daily/${QUIZ.id}`, `daily/${QUIZ.id}`]);

    session.chooseBoard(today);
    session.chooseBoard({ period: "weekly", quiz: "no-such-quiz" });
    expect(state().board).toEqual({ period: "weekly" });
    await waitFor(() => expect(shownLeaderboard(state())?.board.period).toBe("weekly"));
    await session.refreshLeaderboard();
    expect(boards().slice(3)).toEqual(["weekly", "weekly"]);

    session.chooseBoard(DEFAULT_BOARD);
    expect(shownLeaderboard(state())?.board.period).toBe("all-time");
    session.chooseBoard(today);
    expect(shownLeaderboard(state())?.board).toMatchObject(today);
    await session.refreshLeaderboard();
    await session.refreshLeaderboard();
    expect(new Set(boards().slice(5))).toEqual(new Set(["all-time", `daily/${QUIZ.id}`]));

    seedAnonymousRival(proctor, "a".repeat(32), "b".repeat(32));
    const before = boards().length;
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    expect(boards().slice(before)).toEqual([`daily/${QUIZ.id}`, "all-time"]);
    expect(overallLeaderboard(state())?.board).toMatchObject({ period: "all-time", learners: 2, submissions: 2 });
    expect(shownLeaderboard(state())?.board).toMatchObject({ ...today, learners: 2 });
    expect(state().submissions).toBe(2);
    const settled = boards().length;
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    expect(boards().slice(settled)).toEqual([`daily/${QUIZ.id}`]);
    session.stop();
  });
});

function sessionOn(proctor: FakeProctor, area: StorageArea): QuizSession {
  const session = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store: localStore(area, CATALOG.id), timing: TIMING });
  session.start();
  return session;
}

async function startedRun(session: QuizSession, handle = `Tab ${newId().slice(0, 6)}`): Promise<{ readonly run: string; readonly tasks: readonly SheetTask[] }> {
  const signal = new AbortController().signal;
  expect(await session.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
  expect(await session.startRun(QUIZ.id, "medium", signal)).toBeUndefined();
  const { step, runs } = session.getSnapshot().state;
  if (step.screen !== "run") throw new Error("the run did not open");
  return { run: step.run, tasks: runs[step.run]?.sheet.tasks ?? [] };
}

function classification(tasks: readonly SheetTask[]): Extract<SheetTask, { readonly kind: "classification" }> {
  const task = tasks.find((candidate) => candidate.kind === "classification");
  if (task?.kind !== "classification") throw new Error("no classification task");
  return task;
}

function everyItemTo(task: Extract<SheetTask, { readonly kind: "classification" }>, category: number): Answer {
  return { kind: "classification", assignments: Object.fromEntries(task.items.map((item) => [item.id, task.categories[category]!.id])) };
}

describe("🚶️ several tabs, cancellation and other devices", () => {
  it("keeps both tabs' unsynced answers of one run and delivers them all once the proctor is back", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const origin = memoryStorageOrigin();
    const area = origin.tab();
    const left = sessionOn(proctor, origin.tab());
    const { run, tasks } = await startedRun(left);
    const right = sessionOn(proctor, origin.tab());
    expect(right.getSnapshot().state.runs[run]?.status).toBe("open");
    const [first, second] = tasks;
    proctor.outage = true;
    left.answer(run, first!.id, sloppyAnswer(first!));
    right.answer(run, second!.id, sloppyAnswer(second!));
    await waitFor(() => {
      for (const session of [left, right]) {
        expect(Object.keys(session.getSnapshot().state.runs[run]?.answers ?? {}).sort()).toEqual([first!.id, second!.id].sort());
        expect(session.outbox.queued(run)).toHaveLength(2);
      }
    });
    expect(area.keys().filter((key) => key.includes(".outbox/"))).toHaveLength(2);
    const reloaded = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store: localStore(origin.tab(), CATALOG.id), timing: TIMING });
    expect(Object.keys(reloaded.getSnapshot().state.runs[run]?.answers ?? {}).sort()).toEqual([first!.id, second!.id].sort());
    expect(reloaded.outbox.queued(run)).toHaveLength(2);
    proctor.outage = false;
    left.outbox.wake();
    right.outbox.wake();
    await waitFor(() => expect(Object.keys(proctor.answersOf(run)).sort()).toEqual([first!.id, second!.id].sort()));
    await waitFor(() => expect(area.keys().filter((key) => key.includes(".outbox/"))).toHaveLength(0));
    left.stop();
    right.stop();
  });

  it("reconciles a cancelled submission the proctor already committed and shows its results", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    for (const task of tasks) session.answer(run, task.id, sloppyAnswer(task));
    await session.outbox.settled(run);
    proctor.holdSubmissions();
    const controller = new AbortController();
    const submitting = session.submit(run, controller.signal, () => undefined);
    await waitFor(() => expect(proctor.submissions).toBe(1));
    controller.abort(new Error("cancelled by the learner"));
    await expect(submitting).rejects.toThrow("cancelled by the learner");
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.status).toBe("submitted"));
    expect(session.getSnapshot().state.runs[run]?.result?.quiz).toBe(QUIZ.id);
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    proctor.releaseSubmissions();
    session.stop();
  });

  it("lets the proctor's run view win over delivered answers and keeps only undelivered ones on top", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    const learner = session.getSnapshot().state.learner!.id;
    const task = classification(tasks);
    session.answer(run, task.id, everyItemTo(task, 0));
    await session.outbox.settled(run);
    proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: everyItemTo(task, 1), at: proctor.now });
    await session.loadRun(run);
    expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toEqual(everyItemTo(task, 1));
    proctor.outage = true;
    session.answer(run, task.id, everyItemTo(task, 0));
    await session.loadRun(run);
    expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toEqual(everyItemTo(task, 0));
    proctor.outage = false;
    session.stop();
  });

  it("refreshes a cached run from the proctor whenever it is opened, showing the cached view at once", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    const learner = session.getSnapshot().state.learner!.id;
    const task = classification(tasks);
    session.open({ screen: "home" });
    proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: everyItemTo(task, 1), at: proctor.now });
    const queried = proctor.runQueries;
    session.open({ screen: "run", run });
    expect(session.getSnapshot().state.step).toEqual({ screen: "run", run });
    expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toBeUndefined();
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toEqual(everyItemTo(task, 1)));
    expect(proctor.runQueries).toBeGreaterThan(queried);
    session.stop();
  });

  it("shows the connection state on a first visit while the proctor cannot be reached, and loads on retry", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    let up = false;
    const transport: HttpTransport = {
      send: async (request) => {
        if (!up) throw new ProctorUnavailable("connection refused");
        return proctor.transport.send(request);
      },
    };
    const user = userEvent.setup();
    render(<QuizApp proctor="" tenant={CATALOG.id} presence={QUIET_PRESENCE} transport={() => transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={{ minMs: 60_000, maxMs: 60_000 }} />);
    await screen.findByText("The quiz server cannot be reached right now – trying again.");
    expect(screen.getByRole("heading", { level: 1, name: "Loading…" })).toBeTruthy();
    expect(screen.getAllByText("Connection lost – retrying").map((element) => element.closest("[role=status]") !== null)).toEqual([false, true]);
    expect(screen.queryByText("All answers saved")).toBeNull();
    up = true;
    await user.click(screen.getByRole("button", { name: "Try again now" }));
    await screen.findByRole("heading", { level: 1, name: "Welcome to the test catalog" });
    expect(screen.getByText("All answers saved")).toBeTruthy();
  });

  it("offers the way to the overview while the catalog has not arrived yet and hides the logo from assistive technology", async () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, CATALOG.id);
    store.write("introduced", true);
    store.write("learner", { id: "f".repeat(32), identity: { kind: "anonymous" } });
    const silent: HttpTransport = { send: () => new Promise<never>(() => undefined) };
    render(<QuizApp proctor="" tenant={CATALOG.id} presence={QUIET_PRESENCE} transport={() => silent} storage={area} languages={["en"]} timing={TIMING} logo={'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"><path d="M0 0h1v1z"/></svg>'} />);
    expect(within(navbar()).getByRole("button", { name: "Overview" }).getAttribute("aria-disabled")).toBe("true");
    const brand = navbar().querySelector<HTMLElement>("[data-quiz-brand]")!;
    expect(brand.textContent).toBe("");
    expect(brand.querySelector("svg")?.closest('[aria-hidden="true"]')).not.toBeNull();
    expect(document.title).toBe("Loading…");
  });

  it("says it is connecting — never connected — until the proctor first answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    let open: () => void = () => undefined;
    const opened = new Promise<void>((resolve) => {
      open = resolve;
    });
    const transport: HttpTransport = {
      send: async (request) => {
        await opened;
        return proctor.transport.send(request);
      },
    };
    render(<QuizApp proctor="" tenant={CATALOG.id} presence={QUIET_PRESENCE} transport={() => transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />);
    expect(await screen.findAllByText("Connecting to the quiz server…")).toHaveLength(2);
    expect(screen.queryByText("All answers saved")).toBeNull();
    open();
    await screen.findByRole("heading", { level: 1, name: "Welcome to the test catalog" });
    expect(screen.queryByText("Connecting to the quiz server…")).toBeNull();
    expect(screen.getByText("All answers saved")).toBeTruthy();
  });

  it("lets long texts wrap at phone widths instead of clipping them", () => {
    expect(stylesheet).toMatch(/\.quiz-app \{[^}]*overflow-wrap: break-word;[^}]*\}/u);
    expect(stylesheet).toMatch(/\.quiz-app :where\(p, h1, h2, h3, h4, li, label, legend, summary, figcaption, dd, dt\) \{\s*overflow-wrap: anywhere;/u);
    expect(stylesheet).toMatch(/\.quiz-home-entry \{[^}]*inline-size: min\(100%, 36rem\);/u);
  });

  it("closes a cached run another device submitted as soon as the learner view says so, dropping its stale answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const deviceA = memoryStorageOrigin();
    const handle = `Device ${newId().slice(0, 6)}`;
    localStore(deviceA.tab(), CATALOG.id).write("introduced", true);
    const first = sessionOn(proctor, deviceA.tab());
    const { run, tasks } = await startedRun(first, handle);
    proctor.outage = true;
    first.answer(run, tasks[0]!.id, sloppyAnswer(tasks[0]!));
    first.stop();
    expect(first.outbox.queued(run)).toHaveLength(1);
    const deviceB = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await deviceB.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
    await deviceB.resumeRun(run, signal);
    proctor.outage = false;
    for (const task of tasks) deviceB.answer(run, task.id, sloppyAnswer(task));
    await deviceB.outbox.settled(run);
    expect(await deviceB.submit(run, signal, () => undefined)).toBeUndefined();
    deviceB.stop();
    proctor.outage = true;
    const reloaded = sessionOn(proctor, deviceA.tab());
    expect(openRunOf(reloaded.getSnapshot().state, QUIZ.id)).toBe(run);
    await waitFor(() => expect(reloaded.getSnapshot().state.runs[run]?.status).toBe("submitted"));
    expect(openRunOf(reloaded.getSnapshot().state, QUIZ.id)).toBeUndefined();
    expect(lastSubmittedRunOf(reloaded.getSnapshot().state, QUIZ.id)).toBe(run);
    expect(reloaded.outbox.queued(run)).toEqual([]);
    const learner = reloaded.getSnapshot().state.learner!.id;
    reloaded.stop();
    const cached = localStore(deviceA.tab(), CATALOG.id);
    cached.put("runs", run, { ...(cached.record("runs", run) as object), status: "open" });
    const stale: RecordAnswerCommand = { type: "record-answer", id: newId(), learner, run, task: tasks[1]!.id, answer: sloppyAnswer(tasks[1]!), at: 1 };
    cached.put("outbox", stale.id, { command: stale, queuedAt: 1 });
    const painted = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store: cached, timing: TIMING });
    expect(painted.getSnapshot().state.runs[run]?.status).toBe("submitted");
    expect(openRunOf(painted.getSnapshot().state, QUIZ.id)).toBeUndefined();
    expect(painted.outbox.queued()).toEqual([]);
    expect(cached.record("outbox", stale.id)).toBeUndefined();
    expect((cached.record("runs", run) as { readonly status: string }).status).toBe("submitted");
    proctor.outage = false;
    render(<QuizApp proctor="" tenant={CATALOG.id} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={deviceA.tab()} languages={["en"]} timing={TIMING} />);
    const card = await screen.findByRole("region", { name: "Household physics" });
    expect(within(card).queryByRole("button", { name: /^Resume/u })).toBeNull();
    expect(within(card).queryByText("In progress")).toBeNull();
    expect(within(card).getByRole("button", { name: "Again (Medium)" })).toBeTruthy();
    expect(within(card).getByRole("button", { name: "View last result" })).toBeTruthy();
  });

  it("names anonymous learners by their public tag, never by any part of their id", () => {
    const learner = newId();
    const tag = learnerTag(learner);
    expect(tag).toMatch(/^[0-9a-f]{8}$/u);
    expect(learnerName({ kind: "anonymous" }, tag, quizText("en"))).toBe(`Anonymous #${tag}`);
    expect(learnerName({ kind: "anonymous" }, tag, quizText("de"))).toBe(`Anonym #${tag}`);
    expect(learnerName({ kind: "pseudonym", handle: "Ada" }, tag, quizText("de"))).toBe("Ada");
  });
});

describe("👥️ presence through the whole app", () => {
  it("joins the roster and the page's room once identified, shows the others and follows the learner into a quiz", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const sockets: { readonly url: string; readonly sent: unknown[]; closed: boolean; readonly socket: PresenceSocket }[] = [];
    const presence: PresenceConnect = (url) => {
      const record = { url, sent: [] as unknown[], closed: false, socket: undefined as unknown as PresenceSocket };
      const socket: PresenceSocket = {
        readyState: 1,
        onopen: null,
        onmessage: null,
        onclose: null,
        onerror: null,
        send: (data) => record.sent.push(JSON.parse(data)),
        close: () => {
          record.closed = true;
        },
      };
      sockets.push(Object.assign(record, { socket }));
      return socket;
    };
    const open = (scope: string) => {
      const found = sockets.filter((entry) => !entry.closed && entry.url.split("?")[0] === `ws://localhost:3000/scopes/${encodeURIComponent(scope)}/presence/ws`);
      expect(found, sockets.map((entry) => `${entry.url} ${entry.closed ? "closed" : "open"}`).join("\n")).toHaveLength(1);
      return found[0]!;
    };
    const deliver = (entry: { readonly socket: PresenceSocket }, frame: unknown) => act(() => entry.socket.onmessage?.(new MessageEvent("message", { data: JSON.stringify(frame) })));
    const user = userEvent.setup();
    render(<QuizApp proctor="" tenant={CATALOG.id} presence={presence} transport={() => proctor.transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />);
    await screen.findByRole("heading", { level: 1, name: "Welcome to the test catalog" });
    expect(sockets).toEqual([]);
    await user.click(screen.getByRole("button", { name: "Continue" }));
    await user.click(await screen.findByRole("radio", { name: "Pseudonym" }));
    await user.type(screen.getByRole("textbox", { name: "Your pseudonym" }), "Ada");
    await user.click(screen.getByRole("button", { name: "Continue" }));
    await screen.findByRole("region", { name: "Ada" });

    const roster = open(CATALOG.id);
    const home = open(`${CATALOG.id}/home`);
    expect(roster.url.endsWith("?surface=quiz")).toBe(true);
    const mira = "d".repeat(32);
    await deliver(roster, {
      type: "welcome",
      session: "s-me",
      colour: 0,
      roster: [{ session: "s-mira", colour: 4, surface: "quiz", state: { active: true, identity: { handle: "Mira", kind: "pseudonym" }, place: { quiz: QUIZ.id, screen: "run" }, tag: learnerTag(mira) } }],
    });
    await waitFor(() => expect(roster.sent.at(-1)).toMatchObject({ type: "state", state: { identity: { kind: "pseudonym", handle: "Ada" }, place: { screen: "home" }, active: true } }));
    expect(within(navbar()).getByText("Online: 1")).toBeTruthy();
    await deliver(roster, { type: "batch", entries: [{ session: "s-me", colour: 0, surface: "quiz", state: (roster.sent.at(-1) as { readonly state: unknown }).state }], left: [] });
    expect(within(navbar()).getByText("Online: 2")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Household physics" })).getByText("Learning now: 1")).toBeTruthy();

    await deliver(home, { type: "welcome", session: "r-me", colour: 0, roster: [] });
    const watched = ["introduction", "leaderboard", "badges", `quiz/${QUIZ.id}`, `quiz/${QUIZ.id}/thinking`].map((room) => `${CATALOG.id}/${room}`);
    await waitFor(() =>
      expect(home.sent).toEqual([
        { type: "state", state: { tag: (roster.sent.at(-1) as { readonly state: { readonly tag: string } }).state.tag } },
        { type: "watch", scopes: watched, intervalMs: 250 },
      ]),
    );
    await deliver(home, {
      type: "watched",
      scope: `${CATALOG.id}/leaderboard`,
      entries: [{ session: "w-mira", colour: 4, surface: "leaderboard", state: { cursor: { anchor: "leaderboard", x: 0.5, y: 0.5 }, tag: learnerTag(mira) } }],
      left: [],
      snapshot: true,
    });
    const inBoard = await waitFor(() => {
      const mark = document.querySelector<HTMLElement>(`[data-layered-pane="board"] [data-pane-peers="${CATALOG.id}/leaderboard"] [data-peer="cursor"]`);
      expect(mark).not.toBeNull();
      return mark!;
    });
    expect(inBoard.textContent).toBe("Mira");
    expect(inBoard.closest("[aria-hidden]")).not.toBeNull();
    await deliver(home, { type: "watched", scope: `${CATALOG.id}/quiz/${QUIZ.id}/thinking`, entries: [{ session: "t-mira", colour: 4, surface: "thinking", state: { answers: {}, tag: learnerTag(mira) } }], left: [] });
    await waitFor(() => expect(document.querySelector(`[data-layered-pane="${QUIZ.id}"] [data-crowd-gate]`)?.getAttribute("data-crowd-gate")).toBe("locked"));
    expect(document.querySelector(`[data-layered-pane="${QUIZ.id}"] [data-crowd-figure], [data-layered-pane="${QUIZ.id}"] [data-crowd-dot]`)).toBeNull();
    await deliver(home, { type: "watched", scope: `${CATALOG.id}/leaderboard`, entries: [], left: ["w-mira"] });
    expect(document.querySelector("[data-pane-peers]")).toBeNull();
    await deliver(home, { type: "batch", entries: [{ session: "r-mira", colour: 4, surface: "home", state: { cursor: { anchor: "home:learner", x: 0.5, y: 0.5 }, tag: learnerTag(mira) } }], left: [] });
    const peer = await waitFor(() => {
      const mark = document.querySelector<HTMLElement>('[data-presence-layer] [data-peer="cursor"]');
      expect(mark).not.toBeNull();
      return mark!;
    });
    expect(peer.textContent).toBe("Mira");
    expect(peer.closest("[aria-hidden]")).not.toBeNull();
    await deliver(home, { type: "batch", entries: [], left: ["r-mira"] });
    expect(document.querySelector("[data-presence-layer]")).toBeNull();

    await user.click(within(screen.getByRole("region", { name: "Household physics" })).getByRole("link", { name: "Household physics" }));
    await waitFor(() => expect(home.closed).toBe(true));
    const page = open(`${CATALOG.id}/quiz/${QUIZ.id}`);
    expect(page.url.endsWith("?surface=quiz")).toBe(true);
    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: "quiz", quiz: QUIZ.id }));
    fireEvent.keyDown(document, { key: "Escape" });
    await screen.findByRole("region", { name: "Settings" });
    await waitFor(() => expect(page.closed).toBe(true));
    const back = open(`${CATALOG.id}/home`);

    await user.click(within(screen.getByRole("region", { name: "Settings" })).getByRole("button", { name: "Open" }));
    await waitFor(() => expect(back.closed).toBe(true));
    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: "preferences" }));
    expect(sockets.filter((entry) => !entry.closed).map((entry) => entry.url.split("?")[0])).toEqual([`ws://localhost:3000/scopes/${CATALOG.id}/presence/ws`]);
    await user.click(screen.getByRole("checkbox", { name: "Show others' cursors" }));
    fireEvent.keyDown(document, { key: "Escape" });
    await screen.findByRole("region", { name: "Household physics" });
    const again = open(`${CATALOG.id}/home`);
    await deliver(again, { type: "welcome", session: "r-me-2", colour: 0, roster: [{ session: "r-mira", colour: 4, surface: "home", state: { cursor: { anchor: "home:learner", x: 0.5, y: 0.5 }, tag: learnerTag(mira) } }] });
    expect(document.querySelector("[data-presence-layer]")).toBeNull();

    await user.click(within(screen.getByRole("region", { name: "Household physics" })).getByRole("button", { name: "Start (Medium)" }));
    await screen.findByRole("heading", { level: 1, name: "Household physics" });
    await waitFor(() => expect(again.closed).toBe(true));
    const room = open(`${CATALOG.id}/quiz/${QUIZ.id}`);
    expect(room.url.endsWith("?surface=run")).toBe(true);
    const task = screen.getByRole("heading", { level: 2, name: /Climates|Masses|Lamps/u }).closest("section")!;
    expect(task.getAttribute("data-presence-anchor")).toMatch(/^task:(climates|masses|lamps)$/u);
    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: "run", quiz: QUIZ.id, task: task.getAttribute("data-presence-anchor")!.slice("task:".length) }));

    const thinking = open(`${CATALOG.id}/quiz/${QUIZ.id}/thinking`);
    expect(thinking.url.endsWith("?surface=thinking")).toBe(true);
    const miraThinks = {
      answers: {
        climates: { kind: "classification", assignments: { desert: "hot-dry", fjord: "hot-dry" } },
        lamps: { kind: "matching", values: { power: { floodlight: 2000, led: 8 } } },
        masses: { kind: "sorting", order: ["mouse", "cat", "horse"] },
      },
      tag: learnerTag(mira),
    };
    await deliver(thinking, { type: "welcome", session: "t-me", colour: 0, roster: [{ session: "t-mira", colour: 4, surface: "thinking", state: miraThinks }] });
    await waitFor(() => expect(thinking.sent.at(-1)).toEqual({ type: "state", state: { answers: {}, tag: (roster.sent.at(-1) as { readonly state: { readonly tag: string } }).state.tag } }));
    expect(task.querySelector("[data-crowd-gate]")?.getAttribute("data-crowd-gate")).toBe("locked");
    expect(task.querySelector("[data-crowd-figure], [data-crowd-dot]")).toBeNull();
    await user.click(within(task).getByRole("button", { name: "Show it now" }));
    await waitFor(() => expect(task.querySelector<HTMLElement>('[data-crowd-figure="answers"]')?.dataset.thinkers).toBe("1"));
    expect(within(task).getByRole("button", { name: "Hide it again" })).toBeTruthy();
    const sentences = [...task.querySelectorAll("[data-crowd-item] > [data-live] > .sr-only")].map((sentence) => sentence.textContent);
    expect(sentences.length).toBeGreaterThan(0);
    for (const sentence of sentences) expect(sentence).toMatch(/^0% \(0 of 0\), thinking this now: 1$/u);
    expect(task.querySelectorAll("[data-crowd-dot]").length).toBe(sentences.length);
    await user.click(within(task).getByRole("button", { name: "Hide it again" }));
    await waitFor(() => expect(task.querySelector("[data-crowd-figure]")).toBeNull());
  });
});

const MATERIAL: QuizMaterial = { catalog: CATALOG, quizzes: [QUIZ] };

/** 🫡️ A started session that holds the site's material, so its deputy decides while the proctor is away. */
function deputisedOn(proctor: FakeProctor, area: StorageArea): QuizSession {
  const session = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store: localStore(area, CATALOG.id), deputy: new Deputy(MATERIAL), timing: TIMING });
  session.start();
  return session;
}

/** 💯️ The answer that scores `task` of a sheet of the household quiz 1, from the quiz's own solutions. */
function perfectAnswer(task: SheetTask): Answer {
  const source = QUIZ.tasks.find((candidate) => candidate.id === task.id);
  if (task.kind === "classification" && source?.kind === "classification") return { kind: "classification", assignments: Object.fromEntries(task.items.map((item) => [item.id, source.items.find((known) => known.id === item.id)!.category])) };
  if (task.kind === "sorting" && source?.kind === "sorting") {
    const value = (id: string): number => source.items.find((known) => known.id === id)!.value;
    return { kind: "sorting", order: task.items.map((item) => item.id).sort((left, right) => value(left) - value(right)) };
  }
  if (task.kind === "matching" && source?.kind === "matching") {
    const card = (dimension: (typeof task.dimensions)[number], id: string): number => dimension.cards!.indexOf(source.items.find((known) => known.id === id)!.values[dimension.id]!);
    return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item) => [item.id, card(dimension, item.id)]))])) };
  }
  throw new Error(`the household quiz has no ${task.kind} task ${task.id}`);
}

/** ▶️ Starts a run of the household quiz in `session` and answers every task perfectly. */
async function playedRun(session: QuizSession): Promise<string> {
  expect(await session.startRun(QUIZ.id, "medium", new AbortController().signal)).toBeUndefined();
  const { step, runs } = session.getSnapshot().state;
  if (step.screen !== "run") throw new Error("the run did not open");
  for (const task of runs[step.run]!.sheet.tasks) session.answer(step.run, task.id, perfectAnswer(task));
  return step.run;
}

const WHOLE_RUN = ["start-run", "record-answer", "record-answer", "record-answer", "submit-run"];

describe("🫡️ the deputy while the proctor is away", () => {
  it("registers, plays and scores a first visit on the device, keeps it over a reload and hands it to the proctor once it answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const origin = memoryStorageOrigin();
    const signal = new AbortController().signal;
    const session = deputisedOn(proctor, origin.tab());
    expect(session.getSnapshot().state.catalog).toEqual(proctor.view);
    expect(session.getSnapshot().state.step).toEqual({ screen: "introduction" });
    session.readIntroduction();

    expect(await session.identify({ kind: "pseudonym", handle: " Grace   Hopper " }, signal)).toBeUndefined();
    const learner = session.getSnapshot().state.learner!;
    expect(learner.identity).toEqual({ kind: "pseudonym", handle: "Grace Hopper" });
    expect(session.getSnapshot().state.learnerView).toMatchObject({ learner: learner.id, identity: learner.identity, runs: [], badges: [], total: 0 });

    const run = await playedRun(session);
    const phases: string[] = [];
    expect(await session.submit(run, signal, (phase) => phases.push(phase.phase))).toBeUndefined();
    expect(phases).toEqual(["submitting", "results"]);
    const played = session.getSnapshot().state;
    expect(played.step).toEqual({ screen: "results", run });
    expect(played.runs[run]).toMatchObject({ learner: learner.id, status: "submitted", result: { quiz: QUIZ.id, score: 1 } });
    expect(played.learnerView).toMatchObject({ total: 200, best: { household: { challenge: "medium", score: 1, points: 200 } }, runs: [{ run, challenge: "medium", status: "submitted", score: 1, points: 200 }] });
    expect(played.learnerView?.badges.map((award) => award.badge)).toEqual(["all-done", "flawless"]);
    expect(played.awards[run]).toEqual(["all-done", "flawless"]);
    expect(played.notice).toBeUndefined();
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    expect(proctor.envelopes).toEqual([]);
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "unreachable", deputy: true, pending: 6 }));

    await session.refreshLeaderboard();
    expect(shownLeaderboard(session.getSnapshot().state)).toMatchObject({ local: true, board: { learners: 1, rows: [{ rank: 1, tag: learnerTag(learner.id), total: 200, badges: ["all-done", "flawless"] }] } });
    expect(await session.startRun(QUIZ.id, "medium", signal)).toBeUndefined();
    const again = session.getSnapshot().state.step;
    expect(again.screen === "run" && again.run !== run).toBe(true);
    session.stop();

    const reloaded = deputisedOn(proctor, origin.tab());
    expect(reloaded.getSnapshot().state.step).toEqual({ screen: "home" });
    expect(reloaded.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(reloaded.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN, "start-run"]);
    proctor.away = false;
    reloaded.reconnect();
    await waitFor(() => expect(reloaded.outbox.queued()).toEqual([]), { timeout: 10_000 });
    const known = proctor.runOf(run)!;
    expect(known).toMatchObject({ learner: learner.id, identity: learner.identity });
    expect(known.runs.map((candidate) => candidate.status)).toEqual(["submitted", "open"]);
    expect(known.runs[0]?.result?.score).toBe(1);
    expect(known.badges.map((award) => award.badge)).toEqual(["all-done", "flawless"]);
    expect(new Set(proctor.envelopes.map((envelope) => envelope.commandId)).size).toBe(7);
    await waitFor(() => expect(reloaded.getSnapshot().state.runs[run]?.submittedAt).toBe(known.runs[0]?.submittedAt));
    await waitFor(() => expect(reloaded.getSnapshot().state.learnerView?.badges.map((award) => award.at)).toEqual(known.badges.map((award) => award.at)));
    await reloaded.refreshLeaderboard();
    expect(shownLeaderboard(reloaded.getSnapshot().state)?.local).toBeUndefined();
    expect(shownLeaderboard(reloaded.getSnapshot().state)?.board.rows.map((row) => row.tag)).toEqual([learnerTag(learner.id)]);
    expect(reloaded.getSnapshot().state.notice).toBeUndefined();
    expect(reloaded.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 });
    reloaded.stop();
  });

  it("leaves every decision to the proctor while it answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const run = await playedRun(session);
    expect(session.outbox.queued().every((command) => command.type === "record-answer")).toBe(true);
    const phases: string[] = [];
    expect(await session.submit(run, new AbortController().signal, (phase) => phases.push(phase.phase))).toBeUndefined();
    expect(phases[0]).toBe("saving");
    expect(proctor.envelopes.map((envelope) => envelope.kind)).toEqual(["quiz.identify-learner", ...WHOLE_RUN.map((type) => `quiz.${type}`)]);
    expect(session.outbox.queued()).toEqual([]);
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", submittedAt: proctor.runOf(run)?.runs[0]?.submittedAt });
    session.stop();
  });

  it("decides a submission on the device when the proctor goes away in the middle of a run", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    const { run, tasks } = await startedRun(session);
    session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
    await session.outbox.settled(run);
    proctor.away = true;
    for (const task of tasks.slice(1)) session.answer(run, task.id, perfectAnswer(task));
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["record-answer", "record-answer", "submit-run"]);
    expect(proctor.runOf(run)?.runs[0]?.status).toBe("open");
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }));
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.submittedAt).toBe(proctor.runOf(run)?.runs[0]?.submittedAt));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("does not keep a learner waiting for a proctor that says nothing: the deputy decides once the patience is spent", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    let silent = false;
    const transport: HttpTransport = { send: (request) => (silent ? new Promise<never>((_, reject) => setTimeout(() => reject(new ProctorUnavailable("the request timed out")), 400)) : proctor.transport.send(request)) };
    const session = new QuizSession({ proctor: new ProctorClient(() => transport, CATALOG.id), store: localStore(memoryStorageOrigin().tab(), CATALOG.id), deputy: new Deputy(MATERIAL), patienceMs: 40, timing: TIMING });
    session.start();
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    silent = true;
    const began = Date.now();
    expect(await session.identify({ kind: "pseudonym", handle: "Patient Learner" }, new AbortController().signal)).toBeUndefined();
    expect(Date.now() - began).toBeLessThan(350);
    const run = await playedRun(session);
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    expect(proctor.envelopes).toEqual([]);
    silent = false;
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => expect(session.outbox.queued()).toEqual([]));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("continues as the holder when the proctor already knows the pseudonym the device registered by itself", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const holder = "e".repeat(32);
    const earlier = "f".repeat(32);
    proctor.seed({ type: "identify-learner", id: newId(), learner: holder, identity: { kind: "pseudonym", handle: "Grace Hopper" } });
    proctor.seed({ type: "start-run", id: newId(), learner: holder, run: earlier, quiz: QUIZ.id, challenge: "medium", at: proctor.now });
    for (const task of runView(proctor.runOf(earlier)!, earlier, proctor.quizzes)!.sheet.tasks) proctor.seed({ type: "record-answer", id: newId(), learner: holder, run: earlier, task: task.id, answer: sloppyAnswer(task), at: proctor.now });
    proctor.seed({ type: "submit-run", id: newId(), learner: holder, run: earlier });
    proctor.away = true;
    const area = memoryStorageOrigin().tab();
    const session = deputisedOn(proctor, area);
    expect(await session.identify({ kind: "pseudonym", handle: "grace hopper" }, new AbortController().signal)).toBeUndefined();
    const provisional = session.getSnapshot().state.learner!.id;
    expect(provisional).not.toBe(holder);
    const run = await playedRun(session);
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.learner).toEqual({ id: holder, identity: { kind: "pseudonym", handle: "Grace Hopper" } });
    expect(session.getSnapshot().state.notice).toEqual({ kind: "recalled", handle: "Grace Hopper" });
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    expect(localStore(area, CATALOG.id).read("learner")).toMatchObject({ id: holder });
    expect(proctor.runOf(run)?.learner).toBe(holder);
    expect(proctor.runOf(run)?.runs.map((candidate) => [candidate.run, candidate.status])).toEqual([
      [earlier, "submitted"],
      [run, "submitted"],
    ]);
    expect(proctor.envelopes.some((envelope) => envelope.target.id === provisional)).toBe(false);
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => summary.run)).toEqual([run, earlier]));
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ learner: holder, status: "submitted", result: { score: 1 } });
    session.stop();
  });

  it("voids a run of the device that the proctor would not start, and says so", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const learner = session.getSnapshot().state.learner!.id;
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.learner).toBe(learner));
    proctor.away = true;
    const elsewhere = "9".repeat(32);
    proctor.seed({ type: "start-run", id: newId(), learner, run: elsewhere, quiz: QUIZ.id, challenge: "medium", at: proctor.now });
    const run = await playedRun(session);
    expect(session.outbox.queued(run).map((command) => command.type)).toEqual(WHOLE_RUN.slice(0, 4));
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.runs[run]?.status).toBe("voided");
    expect(session.getSnapshot().state.notice).toEqual({ kind: "voided" });
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    expect(proctor.runOf(run)).toBeUndefined();
    expect(proctor.envelopes.filter((envelope) => envelope.kind === "quiz.record-answer")).toEqual([]);
    await waitFor(() => expect(openRunOf(session.getSnapshot().state, QUIZ.id)).toBe(elsewhere));
    session.stop();
  });

  it("keeps the queue of a run the deputy submitted when another tab adopts it, and both tabs deliver it once", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const origin = memoryStorageOrigin();
    const left = deputisedOn(proctor, origin.tab());
    const right = deputisedOn(proctor, origin.tab());
    expect(await left.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const run = await playedRun(left);
    expect(await left.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    await waitFor(() => expect(right.getSnapshot().state.runs[run]?.status).toBe("submitted"));
    expect(right.getSnapshot().state.learner?.id).toBe(left.getSnapshot().state.learner?.id);
    for (const tab of [left, right]) expect(tab.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    proctor.away = false;
    left.reconnect();
    right.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => {
      for (const tab of [left, right]) expect(tab.outbox.queued()).toEqual([]);
    });
    expect(proctor.runOf(run)?.runs).toHaveLength(1);
    for (const tab of [left, right]) expect(tab.getSnapshot().state.notice).toBeUndefined();
    left.stop();
    right.stop();
  });

  it("shows the catalog at once, says that everything is saved on the device and marks the leaderboard as this device's own", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const user = userEvent.setup();
    render(app(proctor, memoryStorageOrigin().tab(), MATERIAL));
    await screen.findByRole("heading", { level: 1, name: "Willkommen im Testkatalog" });
    expect((await screen.findAllByText("Quiz-Server nicht erreichbar – alles wird auf diesem Gerät gespeichert")).map((element) => element.closest("[role=status]") !== null)).toEqual([false, true]);
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    await screen.findByRole("heading", { level: 1, name: "Wie möchtest du erscheinen?" });
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    const quizCard = await screen.findByRole("region", { name: "Haushaltsphysik" });
    await screen.findByText("Quiz-Server nicht erreichbar – auf diesem Gerät gespeichert, noch zu senden: 1");
    await waitFor(() => expect(document.querySelector("[data-board-local]")?.textContent).toBe("Nur dieses Gerät – die anderen erscheinen, sobald der Quiz-Server antwortet"));
    await user.click(within(quizCard).getByRole("button", { name: "Starten (Mittel)" }));
    await screen.findByRole("heading", { level: 1, name: "Haushaltsphysik" });
    expect(screen.queryByRole("alert")).toBeNull();
    proctor.away = false;
    await screen.findByText("Alle Antworten gespeichert", {}, { timeout: 10_000 });
    expect(proctor.envelopes.map((envelope) => envelope.kind)).toEqual(["quiz.identify-learner", "quiz.start-run"]);
    await waitFor(() => expect(document.querySelector("[data-board-local]")).toBeNull());
  });
});

/** 🧩️ A started session with a deputy whose proctor client keeps time by `clock`, so a test decides when a proctor of
 * another contract is asked again; without `deputy` the proctor decides everything. */
function agreeingOn(proctor: FakeProctor, area: StorageArea, clock: { now: number }, deputy = true): QuizSession {
  const client = new ProctorClient(() => proctor.transport, CATALOG.id, () => clock.now);
  const session = new QuizSession({ proctor: client, store: localStore(area, CATALOG.id), ...(deputy ? { deputy: new Deputy(MATERIAL) } : {}), timing: TIMING });
  session.start();
  return session;
}

describe("🧩️ a proctor of another contract", () => {
  it("never sends a command or a query to a proctor of an older contract: the device decides, keeps everything and delivers it once the proctor agrees", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.contract = WIRE_VERSION - 1;
    const clock = { now: 1_760_000_000_000 };
    const signal = new AbortController().signal;
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock);
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "incompatible", contract: "older", deputy: true }));
    expect(connectionMessage(session.getSnapshot().connection, quizText("en"))).toEqual({ message: "Quiz server not reachable – everything is saved on this device", tone: "alert" });
    session.readIntroduction();
    expect(await session.identify({ kind: "pseudonym", handle: "Ada Lovelace" }, signal)).toBeUndefined();
    const run = await playedRun(session);
    expect(await session.submit(run, signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    await session.refreshLeaderboard();
    expect(shownLeaderboard(session.getSnapshot().state)?.local).toBe(true);
    expect(proctor.requests.every((request) => request === "GET /instance")).toBe(true);
    expect(proctor.instances).toBe(1);
    expect(session.getSnapshot().state.notice).toBeUndefined();

    proctor.contract = WIRE_VERSION;
    clock.now += AGREEMENT_RECHECK_MS;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(proctor.runOf(run)?.identity).toEqual({ kind: "pseudonym", handle: "Ada Lovelace" });
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 }));
    expect(session.getSnapshot().connection.contract).toBeUndefined();
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("takes a proctor replaced by one of an older contract in the middle of a run for one that is away: nothing it refuses is lost or told", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock);
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    const { run, tasks } = await startedRun(session);
    session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
    await session.outbox.settled(run);
    proctor.contract = WIRE_VERSION - 1;
    for (const task of tasks.slice(1)) session.answer(run, task.id, perfectAnswer(task));
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "incompatible", contract: "older" }));
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["record-answer", "record-answer", "submit-run"]);
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(proctor.runOf(run)?.runs[0]?.status).toBe("open");

    proctor.contract = WIRE_VERSION;
    clock.now += AGREEMENT_RECHECK_MS;
    session.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => expect(session.outbox.queued()).toEqual([]));
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.submittedAt).toBe(proctor.runOf(run)?.runs[0]?.submittedAt));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("waits with every answer for a proctor of a newer contract without a deputy, and says that the page is out of date", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock, false);
    const { run, tasks } = await startedRun(session);
    proctor.contract = WIRE_VERSION + 1;
    for (const task of tasks) session.answer(run, task.id, perfectAnswer(task));
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "incompatible", contract: "newer", deputy: false, pending: tasks.length }));
    for (const locale of QUIZ_LOCALES) expect(connectionMessage(session.getSnapshot().connection, quizText(locale)).message).toBe(quizText(locale)("quiz.connection.outdated"));
    expect(waitingMessage(session.getSnapshot().connection, undefined, quizText("de"))).toEqual({ message: "Diese Seite ist älter als der Quiz-Server – lade sie neu, dann geht alles, was auf diesem Gerät liegt, an den Server.", retry: false, reload: true });
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(proctor.answersOf(run)).toEqual({});
    proctor.contract = WIRE_VERSION;
    clock.now += AGREEMENT_RECHECK_MS;
    session.reconnect();
    await waitFor(() => expect(Object.keys(proctor.answersOf(run))).toHaveLength(tasks.length), { timeout: 10_000 });
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 }));
    session.stop();
  });

  it("reads an answer that is no JSON — a network's sign-in page — as a shortage: it waits, tells nothing and loses nothing", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock, false);
    const { run, tasks } = await startedRun(session);
    proctor.unreadable = true;
    session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "unreachable", pending: 1 }));
    expect(session.getSnapshot().connection.contract).toBeUndefined();
    expect(session.getSnapshot().state.notice).toBeUndefined();
    proctor.unreadable = false;
    session.reconnect();
    await waitFor(() => expect(proctor.answersOf(run)[tasks[0]!.id]).toEqual(perfectAnswer(tasks[0]!)), { timeout: 10_000 });
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 }));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("never adopts a view that is not of the contract — a sheet without its challenge — and shows the one it holds until a readable one arrives", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock, false);
    const { run } = await startedRun(session);
    const held = session.getSnapshot().state.runs[run]!;
    const asked = proctor.instances;
    proctor.mangle = (view, query) => (query.type === "run" ? { ...(view as RunView), sheet: { ...(view as RunView).sheet, challenge: undefined } } : view);
    const loading = session.loadRun(run);
    await waitFor(() => expect(proctor.asked("run").length).toBeGreaterThan(2));
    expect(proctor.instances).toBeGreaterThan(asked);
    expect(session.getSnapshot().state.runs[run]).toBe(held);
    expect(() => keysHidden(session.getSnapshot().state.runs[run]!)).not.toThrow();
    proctor.mangle = undefined;
    expect((await loading)?.sheet.challenge).toBe("medium");
    expect(session.getSnapshot().connection.reachability).toBe("reachable");
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("shows a page older than the proctor as out of date with a reload, and the device goes on deciding", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.contract = WIRE_VERSION + 1;
    const user = userEvent.setup();
    render(app(proctor, memoryStorageOrigin().tab(), MATERIAL));
    await screen.findByRole("heading", { level: 1, name: "Willkommen im Testkatalog" });
    const outdated = await screen.findByRole("alert");
    expect(outdated.textContent).toContain("Diese Seite ist älter als der Quiz-Server");
    expect(within(outdated).getByRole("button", { name: "Seite neu laden" })).toBeTruthy();
    expect(screen.getAllByText("Seite veraltet – alles bleibt auf diesem Gerät, bis du sie neu lädst").length).toBeGreaterThan(0);
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    await screen.findByRole("heading", { level: 1, name: "Wie möchtest du erscheinen?" });
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    const quizCard = await screen.findByRole("region", { name: "Haushaltsphysik" });
    await user.click(within(quizCard).getByRole("button", { name: "Starten (Mittel)" }));
    await screen.findByRole("heading", { level: 1, name: "Haushaltsphysik" });
    expect(proctor.envelopes).toEqual([]);
    expect(proctor.requests.every((request) => request === "GET /instance")).toBe(true);
  });
});

describe("🧯️ a screen that fails", () => {
  it("shows what happened in place of the screen, keeps everything else and shows the screen again on request", async () => {
    let failing = true;
    function Fragile(): ReactElement {
      if (failing) throw new Error("the sheet has no challenge");
      return <h1>Run</h1>;
    }
    const user = userEvent.setup();
    const quiet = vi.spyOn(console, "error").mockImplementation(() => undefined);
    try {
      render(
        <ScreenBoundary text={quizText("en")}>
          <Fragile />
        </ScreenBoundary>,
      );
      const alert = await screen.findByRole("alert");
      expect(alert.textContent).toContain("Something went wrong while showing this screen. Everything you did is kept.");
      expect(within(alert).getByText("the sheet has no challenge")).toBeTruthy();
      expect(within(alert).getByRole("button", { name: "Reload page" })).toBeTruthy();
      failing = false;
      await user.click(within(alert).getByRole("button", { name: "Try again now" }));
      expect(await screen.findByRole("heading", { level: 1, name: "Run" })).toBeTruthy();
      expect(screen.queryByRole("alert")).toBeNull();
    } finally {
      quiet.mockRestore();
    }
  });
});

/** 🕰️ A started session whose clock is the proctor double's own, so the instants it stamps and the proctor's agree. */
function clockedOn(proctor: FakeProctor, area: StorageArea, material?: QuizMaterial): QuizSession {
  const session = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store: localStore(area, CATALOG.id), ...(material === undefined ? {} : { deputy: new Deputy(material) }), timing: TIMING, now: () => proctor.now });
  session.start();
  return session;
}

/** 🎯️ The answer that scores `task` 1 where the sheet hides the keys: every true value guessed exactly. */
function guessedAnswer(task: SheetTask): Answer {
  const source = QUIZ.tasks.find((candidate) => candidate.id === task.id);
  if (task.kind === "sorting" && source?.kind === "sorting") {
    const value = (id: string): number => source.items.find((known) => known.id === id)!.value;
    return { kind: "sorting", order: task.items.map((item) => item.id).sort((left, right) => value(left) - value(right)), guesses: Object.fromEntries(task.items.map((item) => [item.id, value(item.id)])) };
  }
  if (task.kind === "matching" && source?.kind === "matching")
    return { kind: "matching", guesses: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item) => [item.id, source.items.find((known) => known.id === item.id)!.values[dimension.id]!]))])) };
  return perfectAnswer(task);
}

/** 🪨️ A valid but poor answer where the sheet hides the keys: the sloppy one, with every matching value guessed as 1. */
function roughAnswer(task: SheetTask): Answer {
  return task.kind === "matching" ? { kind: "matching", guesses: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item) => [item.id, 1]))])) } : sloppyAnswer(task);
}

/** 🙃️ A sorting of the household quiz in the reverse of the true order: on a ladder its outer keys miss by decades. */
function reversedSorting(task: SheetTask): Answer {
  const source = QUIZ.tasks.find((candidate) => candidate.id === task.id);
  if (task.kind !== "sorting" || source?.kind !== "sorting") throw new Error("not the sorting");
  const value = (id: string): number => source.items.find((known) => known.id === id)!.value;
  return { kind: "sorting", order: task.items.map((item) => item.id).sort((left, right) => value(right) - value(left)) };
}

function taskOf(session: QuizSession, run: string, kind: SheetTask["kind"]): SheetTask {
  return session.getSnapshot().state.runs[run]!.sheet.tasks.find((task) => task.kind === kind)!;
}

describe("⛰️ the challenges through the session", () => {
  it("starts a run at every challenge with the sheet of that challenge, and a start at the same challenge resumes it", { timeout: 30_000 }, async () => {
    for (const challenge of CHALLENGES) {
      const proctor = new FakeProctor();
      const session = sessionOn(proctor, memoryStorageOrigin().tab());
      const signal = new AbortController().signal;
      expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
      expect(await session.startRun(QUIZ.id, challenge, signal)).toBeUndefined();
      const { step, runs } = session.getSnapshot().state;
      if (step.screen !== "run") throw new Error("the run did not open");
      const view = runs[step.run]!;
      const rules = challengeRules(challenge);
      expect(view.sheet.challenge).toBe(challenge);
      expect(JSON.parse(decoder.decode(proctor.envelopes.find((envelope) => envelope.kind === "quiz.start-run")!.payload))).toMatchObject({ run: step.run, quiz: QUIZ.id, challenge });
      const sorting = taskOf(session, step.run, "sorting");
      const matching = taskOf(session, step.run, "matching");
      expect(sorting.kind === "sorting" && sorting.keys !== undefined, challenge).toBe(rules.keys);
      expect(matching.kind === "matching" && matching.dimensions.every((dimension) => dimension.cards !== undefined), challenge).toBe(rules.keys);
      expect(view.sheet.tasks.every((task) => (task.seconds !== undefined) === rules.timed), challenge).toBe(true);
      expect(view.opened, challenge).toEqual(rules.timed ? {} : undefined);
      expect(keysHidden(view), challenge).toBe(!rules.keys);
      expect(await session.startRun(QUIZ.id, challenge, signal)).toBeUndefined();
      expect(session.getSnapshot().state.step).toEqual({ screen: "run", run: step.run });
      expect(proctor.runOf(step.run)?.runs.map((run) => run.run)).toEqual([step.run]);
      session.stop();
    }
  });

  it("voids the open run when the learner starts the quiz at another challenge, without a notice, and drops what it queued", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    proctor.outage = true;
    session.answer(run, tasks[0]!.id, sloppyAnswer(tasks[0]!));
    expect(session.outbox.queued(run)).toHaveLength(1);
    expect(openChallengeOf(session.getSnapshot().state, QUIZ.id)).toBe("medium");
    expect(await session.startRun(QUIZ.id, "hard", new AbortController().signal)).toBeUndefined();
    const { step, runs, notice } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the new run did not open");
    expect(step.run).not.toBe(run);
    expect(runs[run]?.status).toBe("voided");
    expect(runs[step.run]?.sheet.challenge).toBe("hard");
    expect(notice).toBeUndefined();
    expect(session.outbox.queued(run)).toEqual([]);
    expect(openRunOf(session.getSnapshot().state, QUIZ.id)).toBe(step.run);
    expect(openChallengeOf(session.getSnapshot().state, QUIZ.id)).toBe("hard");
    expect(proctor.runOf(run)?.runs.map((candidate) => [candidate.run, candidate.challenge, candidate.status])).toEqual([
      [run, "medium", "voided"],
      [step.run, "hard", "open"],
    ]);
    proctor.outage = false;
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => [summary.challenge, summary.status])).toEqual([
      ["hard", "open"],
      ["medium", "voided"],
    ]));
    session.stop();
  });

  it("keeps the clock of an expert run by the session clock: opens a task once, takes its answers in time up to the deadline, refuses one after it on the device without sending it and submits with tasks unanswered", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(session.now()).toBe(proctor.now);
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = () => session.getSnapshot().state.runs[run]!;
    const [first, second] = view().sheet.tasks;
    expect(view().opened).toEqual({});

    session.answer(run, first!.id, guessedAnswer(first!));
    await session.outbox.settled(run);
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "task-unopened" });
    await waitFor(() => expect(view().answers[first!.id]).toBeUndefined());
    session.dismissNotice();

    const before = proctor.now;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    const opening = proctor.envelopes.filter((envelope) => envelope.kind === "quiz.open-task");
    expect(opening.map((envelope) => JSON.parse(decoder.decode(envelope.payload)))).toEqual([{ type: "open-task", id: opening[0]!.commandId, learner: session.getSnapshot().state.learner!.id, run, task: first!.id, at: before }]);
    expect(view().opened).toEqual({ [first!.id]: before });
    expect(proctor.runOf(run)?.runs[0]?.opened).toEqual({ [first!.id]: before });
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    expect(proctor.envelopes.filter((envelope) => envelope.kind === "quiz.open-task")).toHaveLength(1);

    const answeredAt = proctor.now;
    session.answer(run, first!.id, guessedAnswer(first!));
    await session.outbox.settled(run);
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(proctor.answersOf(run)[first!.id]).toEqual(guessedAnswer(first!));
    expect(JSON.parse(decoder.decode(proctor.envelopes.filter((envelope) => envelope.kind === "quiz.record-answer").at(-1)!.payload))).toMatchObject({ task: first!.id, at: answeredAt });

    expect(await session.openTask(run, second!.id, signal)).toBeUndefined();
    const deadline = view().opened![second!.id]! + second!.seconds! * 1000;
    proctor.now = deadline;
    const lastInTime = guessedAnswer(second!);
    session.answer(run, second!.id, lastInTime);
    expect(view().answers[second!.id]).toEqual(lastInTime);
    await session.outbox.settled(run);
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(session.now()).toBeGreaterThan(deadline);
    const sent = proctor.envelopes.length;
    session.answer(run, second!.id, roughAnswer(second!));
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "time-up" });
    expect(view().answers[second!.id]).toEqual(lastInTime);
    expect(session.outbox.queued(run)).toEqual([]);
    expect(proctor.envelopes).toHaveLength(sent);
    expect(proctor.answersOf(run)[second!.id]).toEqual(lastInTime);
    session.dismissNotice();

    expect(await session.submit(run, signal, () => undefined)).toBeUndefined();
    const result = view().result!;
    expect(result.challenge).toBe("expert");
    expect(result.tasks.map((task) => task.score)).toEqual(view().sheet.tasks.map((task) => (task.id === first!.id || task.id === second!.id ? 1 : 0)));
    expect(result.score).toBeCloseTo(2 / view().sheet.tasks.length, 12);
    expect(result.points).toBeCloseTo(result.score * 400, 9);
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.best[QUIZ.id]).toEqual({ challenge: "expert", score: result.score, points: result.points }));
    session.stop();
  });

  it("opens the tasks of an expert run on the device while the proctor is away and delivers each opening before the answers of its task", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first, second] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    proctor.away = true;
    const firstAt = proctor.now;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]?.opened).toEqual({ [first!.id]: firstAt });
    session.answer(run, first!.id, guessedAnswer(first!));
    proctor.now += 5_000;
    const secondAt = proctor.now;
    expect(await session.openTask(run, second!.id, signal)).toBeUndefined();
    session.answer(run, second!.id, roughAnswer(second!));
    session.answer(run, first!.id, roughAnswer(first!));
    expect(session.getSnapshot().state.runs[run]?.opened).toEqual({ [first!.id]: firstAt, [second!.id]: secondAt });
    expect(session.outbox.queued(run).map((command) => (command.type === "open-task" || command.type === "record-answer" ? `${command.type} ${command.task}` : command.type))).toEqual([
      `open-task ${first!.id}`,
      `open-task ${second!.id}`,
      `record-answer ${second!.id}`,
      `record-answer ${first!.id}`,
    ]);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    const delivered = proctor.envelopes.filter((envelope) => envelope.kind === "quiz.open-task" || envelope.kind === "quiz.record-answer").map((envelope) => JSON.parse(decoder.decode(envelope.payload)) as Command);
    expect(delivered.map((command) => (command.type === "open-task" || command.type === "record-answer" ? `${command.type} ${command.task}` : command.type))).toEqual([`open-task ${first!.id}`, `open-task ${second!.id}`, `record-answer ${second!.id}`, `record-answer ${first!.id}`]);
    expect(proctor.runOf(run)?.runs[0]?.opened).toEqual({ [first!.id]: firstAt, [second!.id]: secondAt });
    expect(proctor.answersOf(run)).toEqual({ [first!.id]: roughAnswer(first!), [second!.id]: roughAnswer(second!) });
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("never lets the deputy open a task or submit a run it knows from its listing alone: it loads the run from the proctor first", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const handle = `Clock ${newId().slice(0, 6)}`;
    const first = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    await waitFor(() => expect(first.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await first.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
    expect(await first.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = first.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [task] = first.getSnapshot().state.runs[run]!.sheet.tasks;
    expect(await first.openTask(run, task!.id, signal)).toBeUndefined();
    const opened = first.getSnapshot().state.runs[run]!.opened;
    first.stop();

    const other = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    await waitFor(() => expect(other.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await other.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
    await waitFor(() => expect(other.getSnapshot().state.learnerView?.runs.map((summary) => [summary.run, summary.challenge, summary.status])).toEqual([[run, "expert", "open"]]));
    expect(other.getSnapshot().state.runs[run]).toBeUndefined();
    proctor.away = true;
    for (const attempt of [(cancel: AbortSignal) => other.openTask(run, task!.id, cancel), (cancel: AbortSignal) => other.submit(run, cancel, () => undefined)]) {
      const controller = new AbortController();
      const waiting = attempt(controller.signal);
      await new Promise((resolve) => setTimeout(resolve, 60));
      controller.abort(new Error("the learner gave up waiting"));
      await expect(waiting).rejects.toThrow("the learner gave up waiting");
      expect(other.outbox.queued()).toEqual([]);
      expect(other.getSnapshot().state.runs[run]?.status ?? "open").toBe("open");
    }
    proctor.away = false;
    expect(await other.openTask(run, task!.id, signal)).toBeUndefined();
    expect(other.getSnapshot().state.runs[run]?.opened).toEqual(opened);
    expect(proctor.envelopes.filter((envelope) => envelope.kind === "quiz.open-task")).toHaveLength(1);
    other.stop();
  });

  it("tells the hints of an easy run after an answer once the proctor's view of it arrives, and none on medium", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const sorting = taskOf(session, run, "sorting");
    const source = QUIZ.tasks.find((task) => task.id === sorting.id)!;
    const reversed = reversedSorting(sorting);
    const expected = hintsOf(source, sorting, reversed);
    expect(expected.map((hint) => hint.kind)).toEqual(["compare", "compare"]);
    expect(session.getSnapshot().state.runs[run]?.hints).toBeUndefined();
    session.answer(run, sorting.id, reversed);
    expect(session.getSnapshot().state.runs[run]?.hints).toBeUndefined();
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.hints).toEqual({ [sorting.id]: expected }));
    session.answer(run, sorting.id, perfectAnswer(sorting));
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.hints).toBeUndefined());

    expect(await session.startRun(QUIZ.id, "medium", signal)).toBeUndefined();
    const medium = session.getSnapshot().state.step;
    if (medium.screen !== "run") throw new Error("the run did not open");
    const queried = proctor.runQueries;
    session.answer(medium.run, taskOf(session, medium.run, "sorting").id, reversedSorting(taskOf(session, medium.run, "sorting")));
    await session.outbox.settled(medium.run);
    expect(proctor.runQueries).toBe(queried);
    expect(session.getSnapshot().state.runs[medium.run]?.hints).toBeUndefined();
    session.stop();
  });

  it("tells the hints of an easy run at once from the deputy, also while the proctor is away", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const sorting = taskOf(session, step.run, "sorting");
    const reversed = reversedSorting(sorting);
    session.answer(step.run, sorting.id, reversed);
    expect(session.getSnapshot().state.runs[step.run]?.hints).toEqual({ [sorting.id]: hintsOf(QUIZ.tasks.find((task) => task.id === sorting.id)!, sorting, reversed) });
    const classifying = taskOf(session, step.run, "classification");
    if (classifying.kind !== "classification") throw new Error("not the classification");
    session.answer(step.run, classifying.id, everyItemTo(classifying, 0));
    const doubted = hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 0));
    expect(doubted.length).toBeGreaterThan(0);
    expect(session.getSnapshot().state.runs[step.run]?.hints?.[classifying.id]).toEqual(doubted);
    session.answer(step.run, sorting.id, perfectAnswer(sorting));
    expect(Object.keys(session.getSnapshot().state.runs[step.run]?.hints ?? {})).toEqual([classifying.id]);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    await waitFor(() => expect(session.getSnapshot().state.runs[step.run]?.hints).toEqual({ [classifying.id]: doubted }));
    session.stop();
  });

  it("scores a hard run's guesses at its par and keeps the best run by points across challenges", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    const played = async (challenge: Challenge, answer: (task: SheetTask) => Answer): Promise<string> => {
      expect(await session.startRun(QUIZ.id, challenge, signal)).toBeUndefined();
      const { step, runs } = session.getSnapshot().state;
      if (step.screen !== "run") throw new Error("the run did not open");
      for (const task of runs[step.run]!.sheet.tasks) session.answer(step.run, task.id, answer(task));
      expect(await session.submit(step.run, signal, () => undefined)).toBeUndefined();
      return step.run;
    };
    const hard = await played("hard", guessedAnswer);
    expect(session.getSnapshot().state.runs[hard]?.result).toMatchObject({ challenge: "hard", score: 1, points: 300 });
    const easy = await played("easy", perfectAnswer);
    expect(session.getSnapshot().state.runs[easy]?.result).toMatchObject({ challenge: "easy", score: 1, points: 100 });
    await waitFor(() => expect(session.getSnapshot().state.learnerView).toMatchObject({ best: { [QUIZ.id]: { challenge: "hard", score: 1, points: 300 } }, total: 300 }));
    expect(session.getSnapshot().state.learnerView?.runs.map((summary) => [summary.challenge, summary.points])).toEqual([
      ["easy", 100],
      ["hard", 300],
    ]);
    session.stop();
  });

  it("restores a stored run only with the challenge of its sheet", () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, CATALOG.id);
    const learner = "a".repeat(32);
    store.write("introduced", true);
    store.write("learner", { id: learner, identity: { kind: "anonymous" } });
    const sheet = { quiz: QUIZ.id, seed: 1, challenge: "hard", title: QUIZ.title, description: QUIZ.description, tasks: [] };
    const view = { run: "b".repeat(32), learner, quiz: QUIZ.id, status: "open", sheet, answers: {}, startedAt: 1 };
    store.put("runs", view.run, view);
    const { challenge: _, ...unchallenged } = sheet;
    store.put("runs", "c".repeat(32), { ...view, run: "c".repeat(32), sheet: unchallenged });
    store.put("runs", "d".repeat(32), { ...view, run: "d".repeat(32), sheet: { ...sheet, challenge: "lenient" } });
    expect(Object.keys(restoreQuizState(store).runs)).toEqual([view.run]);
    expect(isChallenge("expert")).toBe(true);
    expect([undefined, "Expert", "level", 3].some(isChallenge)).toBe(false);
  });

  it("restores a stored learner view only in the shape of the challenges: every run with its challenge, every best with the challenge, score and points of its run", () => {
    const store = localStore(memoryStorageOrigin().tab(), CATALOG.id);
    const learner = "a".repeat(32);
    store.write("introduced", true);
    store.write("learner", { id: learner, identity: { kind: "anonymous" } });
    const summary = { run: "b".repeat(32), quiz: QUIZ.id, challenge: "hard", status: "submitted", startedAt: 1, score: 0.9, points: 270, submittedAt: 2 };
    const current = { learner, identity: { kind: "anonymous" }, runs: [summary], badges: [], best: { [QUIZ.id]: { challenge: "hard", score: 0.9, points: 270 } }, total: 270 };
    store.write("learner-view", current);
    expect(restoreQuizState(store).learnerView).toEqual(current);
    const { challenge: _, ...unchallenged } = summary;
    const { best: __, ...bestless } = current;
    for (const old of [{ ...current, best: { [QUIZ.id]: 0.9 } }, { ...current, runs: [unchallenged] }, { ...current, best: { [QUIZ.id]: { score: 0.9, points: 270 } } }, { ...current, best: { [QUIZ.id]: { challenge: "hard", score: 0.9 } } }, bestless]) {
      store.write("learner-view", old);
      expect(restoreQuizState(store).learnerView, JSON.stringify(old)).toBeUndefined();
    }
  });
});

/** ✋️ A transport to `proctor` that, while `hold.on`, keeps the answer to every read of a run back until its release in
 * `held` is called — the releases in the order the reads were asked. */
function heldTransport(proctor: FakeProctor): { readonly transport: HttpTransport; readonly hold: { on: boolean }; readonly held: (() => void)[] } {
  const hold = { on: false };
  const held: (() => void)[] = [];
  const transport: HttpTransport = {
    send: async (request) => {
      if (request.path !== "/queries") return proctor.transport.send(request);
      const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
      const read = request.path === "/queries" && (JSON.parse(decoder.decode(decodeQueryEnvelope(body).arguments)) as Query).type === "run";
      const kept = hold.on && read;
      const response = await proctor.transport.send(request);
      if (kept) await new Promise<void>((release) => held.push(release));
      return response;
    },
  };
  return { transport, hold, held };
}

/** 🍳️ A second quiz beside the household one, so that two runs of different quizzes can be open at once. */
const KITCHEN: Quiz = { ...QUIZ, id: "kitchen", emoji: "🍳", title: text("Kitchen physics", "Küchenphysik") };

/** 🗄️ The household catalog with the kitchen quiz beside it. */
const TWO_QUIZZES: QuizMaterial = { catalog: { ...CATALOG, quizzes: ["household.json", "kitchen.json"] }, quizzes: [QUIZ, KITCHEN] };

/** 🧷️ The run view of an open timed run of the household quiz with `answers` and `extra` members, for the pure folds. */
function heldRun(answers: RunView["answers"], extra: Partial<RunView> = {}): RunView {
  return { run: "b".repeat(32), learner: "a".repeat(32), quiz: QUIZ.id, status: "open", sheet: { quiz: QUIZ.id, seed: 1, challenge: "expert", title: QUIZ.title, description: QUIZ.description, tasks: [] }, answers, startedAt: 1, ...extra };
}

describe("🧷️ what the device shows of a run stays true to its answers", () => {
  it("drops the hints of a task the moment its answer changes and shows those of the new answer only once the proctor's view of it arrives (no deputy)", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = (): RunView => session.getSnapshot().state.runs[run]!;
    const sorting = taskOf(session, run, "sorting");
    const classifying = classification(view().sheet.tasks);
    const misplaced = { [classifying.id]: hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 0)) };
    expect(misplaced[classifying.id]!.length).toBeGreaterThan(0);
    session.answer(run, sorting.id, reversedSorting(sorting));
    session.answer(run, classifying.id, everyItemTo(classifying, 0));
    await waitFor(() => expect(view().hints).toEqual({ ...misplaced, [sorting.id]: hintsOf(QUIZ.tasks.find((task) => task.id === sorting.id)!, sorting, reversedSorting(sorting)) }));
    proctor.outage = true;
    session.answer(run, sorting.id, perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    await session.loadRun(run);
    expect(view().answers[sorting.id]).toEqual(perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    proctor.outage = false;
    session.outbox.wake();
    await waitFor(() => expect(proctor.answersOf(run)[sorting.id]).toEqual(perfectAnswer(sorting)));
    await waitFor(() => expect(session.outbox.queued()).toEqual([]));
    expect(view().hints).toEqual(misplaced);
    session.stop();
  });

  it("has the deputy tell the hints of the answers shown, also when a read of the run comes back while a newer answer waits", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = (): RunView => session.getSnapshot().state.runs[run]!;
    const sorting = taskOf(session, run, "sorting");
    const classifying = classification(view().sheet.tasks);
    const misplaced = { [classifying.id]: hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 0)) };
    expect(misplaced[classifying.id]!.length).toBeGreaterThan(0);
    session.answer(run, sorting.id, reversedSorting(sorting));
    session.answer(run, classifying.id, everyItemTo(classifying, 0));
    await session.outbox.settled(run);
    expect(view().hints).toEqual({ ...misplaced, [sorting.id]: hintsOf(QUIZ.tasks.find((task) => task.id === sorting.id)!, sorting, reversedSorting(sorting)) });
    proctor.outage = true;
    session.answer(run, sorting.id, perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    const queried = proctor.runQueries;
    await session.loadRun(run);
    expect(proctor.runQueries).toBeGreaterThan(queried);
    expect(proctor.answersOf(run)[sorting.id]).toEqual(reversedSorting(sorting));
    expect(view().answers[sorting.id]).toEqual(perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    proctor.outage = false;
    session.stop();
  });

  it("never lets a read of the run asked for before a newer answer bring back the older answer or its hints, and drops a read that a later one overtook", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const { transport, hold, held } = heldTransport(proctor);
    const session = new QuizSession({ proctor: new ProctorClient(() => transport, CATALOG.id), store: localStore(memoryStorageOrigin().tab(), CATALOG.id), timing: TIMING });
    session.start();
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = (): RunView => session.getSnapshot().state.runs[run]!;
    const sorting = taskOf(session, run, "sorting");
    session.answer(run, sorting.id, reversedSorting(sorting));
    await waitFor(() => expect(Object.keys(view().hints ?? {})).toEqual([sorting.id]));
    hold.on = true;
    const stale = session.loadRun(run);
    await waitFor(() => expect(held).toHaveLength(1));
    session.answer(run, sorting.id, perfectAnswer(sorting));
    await session.outbox.settled(run);
    await waitFor(() => expect(held).toHaveLength(2));
    held[0]!();
    await stale;
    expect(view().answers[sorting.id]).toEqual(perfectAnswer(sorting));
    expect(view().hints).toBeUndefined();
    held[1]!();
    const classifying = classification(view().sheet.tasks);
    const older = session.loadRun(run);
    await waitFor(() => expect(held).toHaveLength(3));
    proctor.seed({ type: "record-answer", id: newId(), learner: session.getSnapshot().state.learner!.id, run, task: classifying.id, answer: everyItemTo(classifying, 1), at: proctor.now });
    const newer = session.loadRun(run);
    await waitFor(() => expect(held).toHaveLength(4));
    held[3]!();
    await newer;
    expect(view().answers[classifying.id]).toEqual(everyItemTo(classifying, 1));
    const doubted = { [classifying.id]: hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 1)) };
    expect(doubted[classifying.id]!.length).toBeGreaterThan(0);
    expect(view().hints).toEqual(doubted);
    held[2]!();
    await older;
    expect(view().answers).toEqual({ [sorting.id]: perfectAnswer(sorting), [classifying.id]: everyItemTo(classifying, 1) });
    expect(view().hints).toEqual(doubted);
    hold.on = false;
    session.stop();
  });

  it("merges another tab's copy of an open run: the union of the answers, each task with the hints of the copy whose answer it keeps, every opening at its earliest instant", () => {
    const order = (...items: string[]): Answer => ({ kind: "sorting", order: items });
    const doubt = (item: string, other: string): Hint => ({ kind: "compare", item, other, factor: 2, verdict: "under" });
    const mine = heldRun({ a: order("x", "y"), b: order("y", "x") }, { opened: { a: 5, c: 9 }, hints: { a: [doubt("x", "y")], b: [doubt("y", "x")] } });
    const theirs = heldRun({ a: order("y", "x") }, { opened: { b: 7, c: 3 }, hints: { a: [doubt("y", "x")] } });
    expect(mergeRunViews(mine, theirs)).toEqual({ ...theirs, answers: { a: order("y", "x"), b: order("y", "x") }, opened: { a: 5, b: 7, c: 3 }, hints: { a: [doubt("y", "x")], b: [doubt("y", "x")] } });
    expect(mergeRunViews(heldRun({ a: order("x") }), heldRun({}))).toStrictEqual(heldRun({ a: order("x") }));
    const submitted = { ...theirs, status: "submitted" as const };
    expect(mergeRunViews(mine, submitted)).toBe(submitted);
    const voided = { ...mine, status: "voided" as const };
    expect(mergeRunViews(voided, theirs)).toBe(voided);
  });

  it("keeps no hints on a run once it is submitted, voided or closed by its listing — the openings of a timed run stay, as in the proctor's view — and drops only those of a task whose answer changes", () => {
    const learner = "a".repeat(32);
    const order = (...items: string[]): Answer => ({ kind: "sorting", order: items });
    const view = heldRun({ a: order("x", "y"), b: order("y", "x") }, { opened: { a: 2, b: 3 }, hints: { a: [{ kind: "group", item: "x", other: "y", together: true }], b: [{ kind: "category", item: "y", category: "c" }] } });
    const held = { ...initialQuizState({ introduced: true, learner: { id: learner }, runs: { [view.run]: view } }), step: { screen: "run", run: view.run } as const };
    const result = { quiz: QUIZ.id, challenge: "expert" as const, score: 1, points: 400, tasks: [] };
    const listed = { learner, identity: { kind: "anonymous" as const }, runs: [{ run: view.run, quiz: QUIZ.id, challenge: "expert" as const, status: "submitted" as const, startedAt: 1, score: 1, points: 400, submittedAt: 4 }], badges: [], best: {}, total: 0 };
    for (const event of [{ type: "run-submitted", run: view.run, result, badges: [], at: 4 }, { type: "run-voided", run: view.run }, { type: "learner-loaded", view: listed }] as const) {
      const closed = evolveQuizState(held, event).runs[view.run]!;
      expect(closed.status, event.type).not.toBe("open");
      expect(Object.hasOwn(closed, "hints"), event.type).toBe(false);
      expect(closed.opened, event.type).toEqual({ a: 2, b: 3 });
    }
    expect(evolveQuizState(held, { type: "answer-given", run: view.run, task: "a", answer: order("y", "x") }).runs[view.run]!.hints).toEqual({ b: [{ kind: "category", item: "y", category: "c" }] });
    const once = evolveQuizState(held, { type: "answer-given", run: view.run, task: "a", answer: order("y", "x") });
    expect(Object.hasOwn(evolveQuizState(once, { type: "answer-given", run: view.run, task: "b", answer: order("x", "y") }).runs[view.run]!, "hints")).toBe(false);
  });

  it("reads a run again at once after the proctor refused one of its answers, while the deputy's start of a run of another quiz still waits", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor(TWO_QUIZZES);
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), TWO_QUIZZES);
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    proctor.outage = true;
    session.answer(run, first!.id, guessedAnswer(first!));
    expect(session.getSnapshot().state.runs[run]!.answers[first!.id]).toEqual(guessedAnswer(first!));
    proctor.away = true;
    expect(await session.startRun(KITCHEN.id, "medium", signal)).toBeUndefined();
    const other = session.getSnapshot().state.step;
    if (other.screen !== "run" || other.run === run) throw new Error("the kitchen run did not open");
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["record-answer", "start-run"]);
    proctor.outage = false;
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "task-unopened" });
    await waitFor(() => expect(session.getSnapshot().state.runs[run]!.answers[first!.id]).toBeUndefined(), { timeout: 10_000 });
    expect(proctor.runOf(run)?.runs.map((candidate) => [candidate.quiz, candidate.status])).toEqual([
      [QUIZ.id, "open"],
      [KITCHEN.id, "open"],
    ]);
    session.stop();
  });

  it("voids a run on the device when the proctor refuses an opening the deputy decided because the quiz was revised meanwhile", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    proctor.away = true;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    expect(session.outbox.queued(run).map((command) => command.type)).toEqual(["open-task"]);
    proctor.revise(QUIZ.id);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.status).toBe("voided"), { timeout: 10_000 });
    expect(session.getSnapshot().state.notice).toEqual({ kind: "voided" });
    expect(session.outbox.queued()).toEqual([]);
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => summary.run)).toEqual([run]));
    expect(openRunOf(session.getSnapshot().state, QUIZ.id)).toBeUndefined();
    session.stop();
  });

  it("delivers an answer replaced while it waited with the instant of its own last edit, which is what the deciders judge", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    proctor.away = true;
    session.answer(run, first!.id, roughAnswer(first!));
    await waitFor(() => expect(session.getSnapshot().connection.activity).toBe("retrying"));
    proctor.now += 2_000;
    const lastAt = proctor.now;
    session.answer(run, first!.id, guessedAnswer(first!));
    expect(session.outbox.queued(run)).toEqual([expect.objectContaining({ type: "record-answer", task: first!.id, answer: guessedAnswer(first!), at: lastAt })]);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    const delivered = proctor.envelopes.filter((envelope) => envelope.kind === "quiz.record-answer").map((envelope) => JSON.parse(decoder.decode(envelope.payload)) as RecordAnswerCommand);
    expect(delivered.map((command) => [command.answer, command.at])).toEqual([[guessedAnswer(first!), lastAt]]);
    expect(proctor.answersOf(run)[first!.id]).toEqual(guessedAnswer(first!));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });
});
