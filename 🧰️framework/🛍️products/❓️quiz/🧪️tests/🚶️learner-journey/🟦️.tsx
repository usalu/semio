/** 🚶️ The whole learner journey against a proctor double speaking the framework wire (design §9a) and deciding with
 * the core's own pure deciders: introduction → identity → home → run (keyboard answers, a short outage) → submit →
 * results → leaderboard, then a reload that resumes locally in the chosen language.
 */

import { StrictMode } from "react";
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { decodeCommandEnvelope, decodeQueryEnvelope, encodeCommandOutcome, encodeQueryResult, type CommandEnvelope, type CommandOutcome, type EventRecord, type HttpRequest, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import {
  catalogView,
  crowdView,
  decideLearner,
  decideRoster,
  emptyLearnerState,
  emptyRosterState,
  evolveLearner,
  evolveRoster,
  leaderboard,
  learnerTag,
  learnerView,
  runView,
  type Answer,
  type Catalog,
  type Command,
  type Event,
  type LearnerState,
  type LoadedQuiz,
  type Query,
  type Quiz,
  type RecordAnswerCommand,
  type SheetTask,
} from "@semio-tech/quiz";
import {
  ProctorClient,
  ProctorUnavailable,
  QuizApp,
  QuizSession,
  formatQuantity,
  lastSubmittedRunOf,
  learnerName,
  localStore,
  memoryStorageOrigin,
  newId,
  openRunOf,
  quizText,
  type PresenceConnect,
  type PresenceSocket,
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
      quantity: { label: text("Mass", "Masse"), unit: "g", scale: "logarithmic", prefixed: true },
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
      dimensions: [{ id: "power", quantity: { label: text("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true } }],
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

/** 🛂️ The proctor double: the core's deciders behind the framework wire, idempotent by command id, with views that
 * appear only after a short projection lag. */
class FakeProctor {
  readonly envelopes: CommandEnvelope[] = [];
  readonly quizzes: Readonly<Record<string, LoadedQuiz>> = { household: { quiz: QUIZ, revision: "0".repeat(64) } };
  readonly view = catalogView(CATALOG, [QUIZ]);
  outage = false;
  lagged = 0;
  runQueries = 0;
  submissions = 0;
  private held: Promise<void> | undefined;
  private releaseHeld: () => void = () => undefined;
  private readonly lagging = new Map<string, number>();
  private roster = emptyRosterState();
  private readonly learners = new Map<string, LearnerState>();
  private readonly outcomes = new Map<string, CommandOutcome>();
  private seq = 0;
  private now = 1_760_000_000_000;

  readonly transport: HttpTransport = { send: async (request) => this.handle(request) };

  decide(envelope: CommandEnvelope): CommandOutcome {
    const known = this.outcomes.get(envelope.idempotencyKey ?? envelope.commandId);
    if (known !== undefined) return known;
    const command = JSON.parse(decoder.decode(envelope.payload)) as Command;
    this.now += 1000;
    const decision =
      command.type === "identify-learner" ? decideRoster(this.roster, command, this.now) : decideLearner(this.learners.get(command.learner) ?? emptyLearnerState(command.learner), command, { now: this.now, catalog: CATALOG, quizzes: this.quizzes });
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
    const target = command.type === "identify-learner" ? { tenant: CATALOG.id, kind: "quiz-roster", id: "roster" } : { tenant: CATALOG.id, kind: "quiz-learner", id: command.learner };
    return {
      commandId: command.id,
      kind: `quiz.${command.type}`,
      version: 1,
      target,
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
    this.roster = evolveRoster(this.roster, event);
    this.learners.set(event.learner, evolveLearner(this.learners.get(event.learner) ?? emptyLearnerState(event.learner), event));
    this.seq += 1;
    return { stream: envelope.target, seq: this.seq, hlc: { millis: event.at, counter: 0 }, kind: `quiz.${event.type}`, payload: encoder.encode(JSON.stringify(event)) };
  }

  private async handle(request: HttpRequest): Promise<HttpResponse> {
    const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
    if (request.method === "POST" && request.path === "/commands") {
      const envelope = decodeCommandEnvelope(body);
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
      const query = JSON.parse(decoder.decode(envelope.arguments)) as Query;
      if (query.type === "run") this.runQueries += 1;
      const value = this.answer(query);
      if (value === undefined) return reply(404, { kind: "notFound", message: envelope.kind });
      return reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(value)), frontier: null }));
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
        return leaderboard([...this.learners.values()], this.view);
      case "crowd":
        return crowdView(
          QUIZ,
          [...this.learners.values()].flatMap((state) => state.runs.flatMap((run) => (run.result === undefined ? [] : [run.result]))),
        );
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

function seedAnonymousRival(proctor: FakeProctor): string {
  const learner = "c".repeat(32);
  const run = "d".repeat(32);
  proctor.seed({ type: "identify-learner", id: newId(), learner, identity: { kind: "anonymous" } });
  proctor.seed({ type: "start-run", id: newId(), learner, run, quiz: QUIZ.id });
  const sheet = runView(proctor.runOf(run)!, run, proctor.quizzes)!.sheet;
  for (const task of sheet.tasks) proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: sloppyAnswer(task) });
  const submitted = proctor.seed({ type: "submit-run", id: newId(), learner, run });
  expect(submitted.some((event) => event.type === "run-submitted")).toBe(true);
  return learner;
}

const TIMING = { minMs: 1, maxMs: 4 };

/** 🔇️ Presence sockets that never open: the journey is about the proctor's commands and queries. */
const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });

function navbar(): HTMLElement {
  return screen.getByRole("navigation", { name: /^(?:Main navigation|Hauptnavigation)$/u });
}

function app(proctor: FakeProctor, storage: StorageArea) {
  return (
    <StrictMode>
      <QuizApp proctor="" tenant={CATALOG.id} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={storage} languages={["de-CH", "fr"]} timing={TIMING} />
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
        .map((item) => {
          const label = item.cloneNode(true) as HTMLElement;
          label.querySelector("[data-crowd-item]")?.remove();
          return label.textContent?.replace(/[⠿↑↓\d]/gu, "") ?? "";
        });
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
    expect(screen.getByText(/No password is needed/u)).toBeTruthy();
    await user.click(screen.getByRole("radio", { name: "Pseudonym" }));
    await user.type(screen.getByRole("textbox", { name: "Your pseudonym" }), "  Ada   Lovelace ");
    await user.click(screen.getByRole("button", { name: "Continue" }));

    const learnerCard = await screen.findByRole("region", { name: "Ada Lovelace" });
    expect(within(learnerCard).getByText("0 points")).toBeTruthy();
    expect(within(learnerCard).getByText("0 of 1 quizzes played")).toBeTruthy();
    const identify = proctor.envelopes.find((envelope) => envelope.kind === "quiz.identify-learner")!;
    expect(identify.principal).toEqual({ kind: "anonymous" });
    expect(identify.target).toEqual({ tenant: CATALOG.id, kind: "quiz-roster", id: "roster" });
    expect(JSON.parse(decoder.decode(identify.payload)).identity).toEqual({ kind: "pseudonym", handle: "Ada Lovelace" });
    const quizCard = screen.getByRole("region", { name: "Household physics" });
    expect(within(quizCard).getByText("Not attempted yet")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("0 of 2 badges earned")).toBeTruthy();

    await user.click(within(quizCard).getByRole("button", { name: "Start quiz" }));
    await screen.findByRole("heading", { level: 1, name: "Household physics" });
    await waitFor(() => expect(proctor.lagged).toBe(2 * PROJECTION_LAG));
    expect(screen.queryByRole("alert")).toBeNull();
    expect(screen.getByText("0 of 3 tasks complete")).toBeTruthy();
    const submit = screen.getByRole("button", { name: "Submit quiz" });
    expect((submit as HTMLButtonElement).disabled).toBe(true);

    const taskButtons = within(screen.getByRole("navigation", { name: "Tasks" })).getAllByRole("button");
    expect(taskButtons).toHaveLength(3);
    for (const [index, button] of taskButtons.entries()) {
      await user.click(button);
      expect(button.getAttribute("aria-current")).toBe("step");
      if (index === 2) proctor.outage = true;
      await answerCurrentTask(user);
    }
    await screen.findByText(/Connection lost – retrying \(\d answers kept on this device\)/u);
    proctor.outage = false;
    await screen.findByText("All answers saved", {}, { timeout: 5000 });
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

    expect((screen.getByRole("button", { name: "Submit quiz" }) as HTMLButtonElement).disabled).toBe(false);
    await user.click(screen.getByRole("button", { name: "Submit quiz" }));
    const dialog = screen.getByRole("alertdialog", { name: "Submit this quiz?" });
    expect(document.activeElement).toBe(within(dialog).getByRole("button", { name: "Submit now" }));
    await user.click(within(dialog).getByRole("button", { name: "Submit now" }));

    await screen.findByRole("heading", { level: 1, name: "Results: Household physics" });
    expect(screen.getByText("Your score: 100%").closest("p")?.textContent).toBe("Your score: 100%");
    const newBadges = screen.getByRole("region", { name: "New badges" });
    expect(within(newBadges).getByText("All done")).toBeTruthy();
    expect(within(newBadges).getByText("Flawless")).toBeTruthy();
    const climates = screen.getByRole("region", { name: /Climates/u });
    const desertRow = within(climates).getByRole("rowheader", { name: "Desert" }).closest("tr")!;
    expect(
      within(desertRow)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toEqual(["Hot and dry", "Hot and dry", "✓ Correct (100%)", "All runs on Desert: Hot and dry 2×👥Hot and dry 2×", "Little rain, much sun."]);
    expect(
      within(climates)
        .getAllByRole("columnheader")
        .map((head) => head.textContent),
    ).toContain("Everyone");
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
    const description = screen.getByText(/^All learners with a submitted quiz/u);
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
    await user.click(screen.getByRole("button", { name: "Overview" }));
    await screen.findByRole("region", { name: "Household physics" });
    expect(window.location.hash).toBe("");

    cleanup();
    render(app(proctor, origin.tab()));
    await screen.findByRole("region", { name: "Ada Lovelace" });
    expect(screen.getByRole("heading", { level: 1, name: "Quizzes" })).toBeTruthy();
    const card = screen.getByRole("region", { name: "Household physics" });
    await within(card).findByText("Best score: 100%");
    expect(within(card).getByRole("button", { name: "Start again" })).toBeTruthy();
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
});

function sessionOn(proctor: FakeProctor, area: StorageArea): QuizSession {
  const session = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store: localStore(area, CATALOG.id), timing: TIMING });
  session.start();
  return session;
}

async function startedRun(session: QuizSession, handle = `Tab ${newId().slice(0, 6)}`): Promise<{ readonly run: string; readonly tasks: readonly SheetTask[] }> {
  const signal = new AbortController().signal;
  expect(await session.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
  expect(await session.startRun(QUIZ.id, signal)).toBeUndefined();
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
    proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: everyItemTo(task, 1) });
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
    proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: everyItemTo(task, 1) });
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
    expect(screen.getByText("Connection lost – retrying")).toBeTruthy();
    expect(screen.queryByText("All answers saved")).toBeNull();
    up = true;
    await user.click(screen.getByRole("button", { name: "Try again now" }));
    await screen.findByRole("heading", { level: 1, name: "Welcome to the test catalog" });
    expect(screen.getByText("All answers saved")).toBeTruthy();
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
    expect(stylesheet).toMatch(/\.quiz-home-grid \{[^}]*grid-template-columns: minmax\(0, 1fr\);/u);
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
    const stale: RecordAnswerCommand = { type: "record-answer", id: newId(), learner, run, task: tasks[1]!.id, answer: sloppyAnswer(tasks[1]!) };
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
    expect(within(card).queryByRole("button", { name: "Resume quiz" })).toBeNull();
    expect(within(card).queryByText("In progress")).toBeNull();
    expect(within(card).getByRole("button", { name: "Start again" })).toBeTruthy();
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
    await waitFor(() => expect(document.querySelector(`[data-layered-pane="${QUIZ.id}"] [data-crowd-source="live"]`)?.textContent).toBe("👥What others think now· Thinking along: 1"));
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

    await user.click(within(screen.getByRole("region", { name: "Household physics" })).getByRole("button", { name: "Start quiz" }));
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
    await waitFor(() => expect(within(task).getByText("What others think now").closest("[data-crowd-source]")?.getAttribute("data-crowd-source")).toBe("live"));
    const sentences = [...task.querySelectorAll("[data-crowd-item] .sr-only")].map((sentence) => sentence.textContent);
    expect(sentences.length).toBeGreaterThan(0);
    for (const sentence of sentences) expect(sentence).toMatch(/^(The others on (Desert|Fjord): Hot and dry 1×|On average the others put (Mouse|Cat|Horse) at place [123] of 3|The others on (LED bulb|Floodlight): (8\sW|2\skW) 1×)$/u);
  });
});
