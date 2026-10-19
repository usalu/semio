/** 🧰️ Shared kit of the learner-journey suites (the topics sit in the sibling folders `🚶️journey-and-tabs`, `🧩️deputy-and-contract`, `🏔️challenge-session` and `🧷️run-truth`): the quiz and the course, the proctor double and the builders of sessions, runs and answers. Together the suites check the whole learner journey against a proctor double speaking the framework wire (design §9a) and deciding with
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
 *
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test
 */
import { StrictMode } from "react";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, expect } from "vitest";
import { decodeCommandEnvelope, decodeQueryEnvelope, encodeCommandOutcome, encodeQueryResult, type CommandEnvelope, type CommandOutcome, type EventRecord, type HttpRequest, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import { DEFAULT_LIMITS, WIRE_VERSION, courseView, crowdView, decideHandle, decideLearner, emptyHandleState, emptyLearnerState, evolveHandle, evolveLearner, leaderboard, learnerView, normalizeHandle, runView, transcript, type Answer, type Course, type CourseView, type Command, type Event, type HandleState, type LearnerState, type LoadedQuiz, type Query, type Quiz, type SheetTask } from "@semio-tech/course";
import { Deputy, ProctorClient, ProctorUnavailable, CourseApp, CourseSession, commandTarget, formatQuantity, localStore, newId, courseInstance, type PresenceConnect, type CourseMaterial, type StorageArea } from "@semio-tech/course-react";

beforeEach(() => window.history.replaceState(null, "", "/"));

export const text = (en: string, de: string) => ({ en, de });

export const QUIZ: Quiz = {
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

export const COURSE: Course = {
  schema: "semio.course/v1",
  id: "test-course",
  title: text("Test course", "Testkatalog"),
  introduction: { title: text("Welcome to the test course", "Willkommen im Testkatalog"), paragraphs: [text("Quizzes about everyday physics.", "Quizze über Alltagsphysik.")] },
  quizzes: ["household.json"],
  badges: [
    { id: "all-done", emoji: "🎓", label: text("All done", "Alles erledigt"), description: text("Submitted every quiz.", "Jedes Quiz abgegeben."), rule: { kind: "completed-quizzes" } },
    { id: "flawless", emoji: "💎", label: text("Flawless", "Makellos"), description: text("A perfect household quiz.", "Ein perfektes Haushaltsquiz."), rule: { kind: "perfect-quiz", quiz: "household" } },
  ],
};

const encoder = new TextEncoder();

export const decoder = new TextDecoder();

function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** ⏳️ How many reads of a fresh learner or run view the double answers "not found", like a projection catching up. */
export const PROJECTION_LAG = 2;

/** 🛂️ The proctor double: the core's deciders behind the framework wire — one handle state per handle key, one learner
 * state per learner, a registration under a handle relayed to its learner at once — idempotent by command id, with
 * views that appear only after a short projection lag and a recall that is the `handle` read. While it is `away` no
 * request reaches it at all. It serves the household course unless it is given other material, and a quiz it
 * {@link FakeProctor.revise}s is one its runs were not started against. It speaks the wire version `contract` (this
 * client's unless a test replaces it): it declares it at `GET /instance` and refuses an envelope of another version
 * as the proctor does, while it is `unreadable` every answer is a page that is no JSON, and `mangle` changes the views
 * it answers. It notes every request it gets (`requests`) and how often it was asked for its contract (`instances`). */
export class FakeProctor {
  readonly envelopes: CommandEnvelope[] = [];
  readonly requests: string[] = [];
  mangle: ((view: unknown, query: Query) => unknown) | undefined;
  readonly course: Course;
  readonly quizzes: Record<string, LoadedQuiz>;
  readonly view: CourseView;
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

  constructor(material: CourseMaterial = { course: COURSE, quizzes: [QUIZ] }) {
    this.course = material.course;
    this.quizzes = Object.fromEntries(material.quizzes.map((quiz) => [quiz.id, { quiz, revision: "0".repeat(64) }]));
    this.view = courseView(material.course, material.quizzes);
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
        : decideLearner(this.learners.get(command.learner) ?? emptyLearnerState(command.learner), command, { now: this.now, course: this.course, quizzes: this.quizzes, limits: DEFAULT_LIMITS });
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
      kind: `course.${command.type}`,
      version: this.contract,
      target: commandTarget(command, COURSE.id),
      scope: COURSE.id,
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
    return { stream: envelope.target, seq: this.seq, hlc: { millis: event.at, counter: 0 }, kind: `course.${event.type}`, payload: encoder.encode(JSON.stringify(event)) };
  }

  private async handle(request: HttpRequest): Promise<HttpResponse> {
    if (this.away) throw new ProctorUnavailable("the proctor is away");
    this.requests.push(`${request.method} ${request.path}`);
    if (this.unreadable) return { status: 200, text: async () => "<!doctype html><title>Sign in</title>", bytes: async () => encoder.encode("<!doctype html>") };
    if (request.method === "GET" && request.path === "/instance") {
      this.instances += 1;
      return reply(200, courseInstance(this.contract));
    }
    const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
    if (request.method === "POST" && request.path === "/commands") {
      const envelope = decodeCommandEnvelope(body);
      if (envelope.version !== this.contract) {
        const receipt = { commandId: envelope.commandId, actor: envelope.target, revision: this.seq, acceptedAt: envelope.clientHlc };
        return reply(200, encodeCommandOutcome({ status: "rejected", receipt, reason: { kind: "invalid", detail: `envelope-mismatch: version ${envelope.version} is not ${this.contract}` }, notices: [] }));
      }
      this.envelopes.push(envelope);
      if (this.outage && envelope.kind === "course.record-answer") return reply(503, { kind: "actorUnavailable", message: "outage" });
      const outcome = this.decide(envelope);
      if (envelope.kind === "course.submit-run") {
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
      case "course":
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

export function sloppyAnswer(task: SheetTask): Answer {
  switch (task.kind) {
    case "classification":
      return { kind: "classification", assignments: Object.fromEntries(task.items.map((item) => [item.id, task.categories[0]!.id])) };
    case "sorting":
      return { kind: "sorting", order: task.items.map((item) => item.id).reverse() };
    case "matching":
      return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item, index) => [item.id, index]))])) };
  }
}

export function seedAnonymousRival(proctor: FakeProctor, learner = "c".repeat(32), run = "d".repeat(32)): string {
  proctor.seed({ type: "identify-learner", id: newId(), learner, identity: { kind: "anonymous" } });
  proctor.seed({ type: "start-run", id: newId(), learner, run, quiz: QUIZ.id, challenge: "medium", at: proctor.now });
  const sheet = runView(proctor.runOf(run)!, run, proctor.quizzes)!.sheet;
  for (const task of sheet.tasks) proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: sloppyAnswer(task), at: proctor.now });
  const submitted = proctor.seed({ type: "submit-run", id: newId(), learner, run });
  expect(submitted.some((event) => event.type === "run-submitted")).toBe(true);
  return learner;
}

export const TIMING = { minMs: 1, maxMs: 4 };

/** 🔇️ Presence sockets that never open: the journey is about the proctor's commands and queries. */
export const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });

export function navbar(): HTMLElement {
  return screen.getByRole("navigation", { name: /^(?:Main navigation|Hauptnavigation)$/u });
}

export function app(proctor: FakeProctor, storage: StorageArea, material?: CourseMaterial) {
  return (
    <StrictMode>
      <CourseApp proctor="" tenant={COURSE.id} material={material} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={storage} languages={["de-CH", "fr"]} timing={TIMING} />
    </StrictMode>
  );
}

export async function dismissWalkthrough(user: ReturnType<typeof userEvent.setup>): Promise<void> {
  await waitFor(() => expect(document.querySelector('[data-slot="introduction-veil"]')).not.toBeNull(), { timeout: 20_000 });
  await user.keyboard("{Escape}");
  await waitFor(() => expect(document.querySelector('[data-slot="introduction-veil"]')).toBeNull());
}

export async function openQuizGrid(user: ReturnType<typeof userEvent.setup>, locale: "en" | "de"): Promise<void> {
  const card = await screen.findByRole("region", { name: locale === "en" ? "Quizzes" : "Quizze" });
  await user.click(within(card).getByRole("button", { name: locale === "en" ? "Explore" : "Entdecken" }));
}

export async function answerCurrentTask(user: ReturnType<typeof userEvent.setup>): Promise<void> {
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
        .map((item) => item.querySelector(".course-sort-label")?.textContent ?? "");
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

export function sessionOn(proctor: FakeProctor, area: StorageArea): CourseSession {
  const session = new CourseSession({ proctor: new ProctorClient(() => proctor.transport, COURSE.id), store: localStore(area, COURSE.id), timing: TIMING });
  session.start();
  return session;
}

export async function startedRun(session: CourseSession, handle = `Tab ${newId().slice(0, 6)}`): Promise<{ readonly run: string; readonly tasks: readonly SheetTask[] }> {
  const signal = new AbortController().signal;
  expect(await session.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
  expect(await session.startRun(QUIZ.id, "medium", signal)).toBeUndefined();
  const { step, runs } = session.getSnapshot().state;
  if (step.screen !== "run") throw new Error("the run did not open");
  return { run: step.run, tasks: runs[step.run]?.sheet.tasks ?? [] };
}

export function classification(tasks: readonly SheetTask[]): Extract<SheetTask, { readonly kind: "classification" }> {
  const task = tasks.find((candidate) => candidate.kind === "classification");
  if (task?.kind !== "classification") throw new Error("no classification task");
  return task;
}

export function everyItemTo(task: Extract<SheetTask, { readonly kind: "classification" }>, category: number): Answer {
  return { kind: "classification", assignments: Object.fromEntries(task.items.map((item) => [item.id, task.categories[category]!.id])) };
}

export const MATERIAL: CourseMaterial = { course: COURSE, quizzes: [QUIZ] };

/** 🫡️ A started session that holds the site's material, so its deputy decides while the proctor is away. */
export function deputisedOn(proctor: FakeProctor, area: StorageArea): CourseSession {
  const session = new CourseSession({ proctor: new ProctorClient(() => proctor.transport, COURSE.id), store: localStore(area, COURSE.id), deputy: new Deputy(MATERIAL), timing: TIMING });
  session.start();
  return session;
}

/** 💯️ The answer that scores `task` of a sheet of the household quiz 1, from the quiz's own solutions. */
export function perfectAnswer(task: SheetTask): Answer {
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
export async function playedRun(session: CourseSession): Promise<string> {
  expect(await session.startRun(QUIZ.id, "medium", new AbortController().signal)).toBeUndefined();
  const { step, runs } = session.getSnapshot().state;
  if (step.screen !== "run") throw new Error("the run did not open");
  for (const task of runs[step.run]!.sheet.tasks) session.answer(step.run, task.id, perfectAnswer(task));
  return step.run;
}

export const WHOLE_RUN = ["start-run", "record-answer", "record-answer", "record-answer", "submit-run"];

/** 🕰️ A started session whose clock is the proctor double's own, so the instants it stamps and the proctor's agree. */
export function clockedOn(proctor: FakeProctor, area: StorageArea, material?: CourseMaterial): CourseSession {
  const session = new CourseSession({ proctor: new ProctorClient(() => proctor.transport, COURSE.id), store: localStore(area, COURSE.id), ...(material === undefined ? {} : { deputy: new Deputy(material) }), timing: TIMING, now: () => proctor.now });
  session.start();
  return session;
}

/** 🎯️ The answer that scores `task` 1 where the sheet hides the keys: every true value guessed exactly. */
export function guessedAnswer(task: SheetTask): Answer {
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
export function roughAnswer(task: SheetTask): Answer {
  return task.kind === "matching" ? { kind: "matching", guesses: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item) => [item.id, 1]))])) } : sloppyAnswer(task);
}

/** 🙃️ A sorting of the household quiz in the reverse of the true order: on a ladder its outer keys miss by decades. */
export function reversedSorting(task: SheetTask): Answer {
  const source = QUIZ.tasks.find((candidate) => candidate.id === task.id);
  if (task.kind !== "sorting" || source?.kind !== "sorting") throw new Error("not the sorting");
  const value = (id: string): number => source.items.find((known) => known.id === id)!.value;
  return { kind: "sorting", order: task.items.map((item) => item.id).sort((left, right) => value(right) - value(left)) };
}

export function taskOf(session: CourseSession, run: string, kind: SheetTask["kind"]): SheetTask {
  return session.getSnapshot().state.runs[run]!.sheet.tasks.find((task) => task.kind === kind)!;
}
