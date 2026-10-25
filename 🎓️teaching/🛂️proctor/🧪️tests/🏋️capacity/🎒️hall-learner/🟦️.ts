/** 🎒️ One simulated learner of the lecture hall: enrolment, rooms, polling, two quizzes at two challenges with revisions, submission and results, and the Wi-Fi drop. */
import { CHALLENGES, challengeRules, learnerTag, rosterScope, thinkingAnswer, thinkingScope, type Answer, type Command, type IdentityClaim, type PresenceState, type RunView, type SheetTask, type ThinkingAnswer } from "../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts";
import { ProctorClient, RETRY_TIMING, newId, retryTransient, type CommandVerdict } from "../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts";
import { CLOCK_MARGIN_MS, CURSOR, capacityProgress, fail, GIVE_UP_MS, HALL, LAG_PROBE_MS, measurements, type Measurements, REAL, record } from "../📊️settings-and-measurements/🟦️.ts";
import { between, pause, Room, transport } from "../🚚️transport/🟦️.ts";

//#region 🧑‍🎓️Learner
/** 🏫️ What every learner of one phase shares. */
interface Hall {
  readonly base: string;
  readonly tenant: string;
  readonly quizzes: readonly string[];
  readonly scale: number;
  readonly measured: () => Measurements;
  readonly rooms: Set<Room>;
}

function shuffled<T>(items: readonly T[]): T[] {
  const order = [...items];
  for (let index = order.length - 1; index > 0; index -= 1) {
    const other = Math.floor(Math.random() * (index + 1));
    [order[index], order[other]] = [order[other], order[index]];
  }
  return order;
}

/** 🎲️ A number a learner types where the keys are hidden: three digits somewhere between a tenth and a million,
 * positive on either scale. */
const guessed = (): number => Number((10 ** between(-1, 6)).toPrecision(3));

/** ✍️ A valid answer to one presented task — the keys assigned where the sheet shows them, a guess per item (and
 * dimension) where it hides them, the guessed items standing in the order of their guesses — and the draft a learner
 * shares of it. */
function answerOf(task: SheetTask): { readonly answer: Answer; readonly draft: ThinkingAnswer | undefined } {
  const items = task.items.map((item) => item.id);
  const drafted = (answer: Answer) => ({ answer, draft: thinkingAnswer(task, answer) });
  if (task.kind === "classification") return drafted({ kind: "classification", assignments: Object.fromEntries(items.map((item) => [item, task.categories[Math.floor(Math.random() * task.categories.length)].id])) });
  if (task.kind === "sorting") {
    if (task.keys !== undefined) return drafted({ kind: "sorting", order: shuffled(items) });
    const guesses = Object.fromEntries(items.map((item) => [item, guessed()]));
    return drafted({ kind: "sorting", order: [...items].sort((left, right) => guesses[left] - guesses[right]), guesses });
  }
  const shown = task.dimensions.flatMap((dimension) => (dimension.cards === undefined ? [] : [[dimension.id, shuffled(dimension.cards.map((_, card) => card))] as const]));
  const hidden = task.dimensions.filter((dimension) => dimension.cards === undefined);
  return drafted({
    kind: "matching",
    ...(shown.length === 0 ? {} : { assignments: Object.fromEntries(shown.map(([dimension, order]) => [dimension, Object.fromEntries(items.map((item, index) => [item, order[index]]))])) }),
    ...(hidden.length === 0 ? {} : { guesses: Object.fromEntries(hidden.map((dimension) => [dimension.id, Object.fromEntries(items.map((item) => [item, guessed()]))])) }),
  });
}

/** 🧑‍🎓️ One learner's lecture: enrol, look around, play `plays` quizzes, leave. */
async function attend(hall: Hall, index: number, plays: number): Promise<void> {
  const learner = newId();
  const tag = learnerTag(learner);
  const client = new ProctorClient(() => transport(hall.base, HALL, (status) => (status === 429 || status === 503 ? (hall.measured().throttled += 1) : undefined)), hall.tenant);
  /** ⏱️ One operation as the quiz client performs it: a call that got no answer is made again after the client's own
   * backoff, and the time until it is answered — the waits included — is what the learner waited. */
  const timed = async <T>(operation: string, call: () => Promise<T>): Promise<T | undefined> => {
    const started = performance.now();
    let attempts = 0;
    try {
      const value = await retryTransient(
        () => {
          attempts += 1;
          return call();
        },
        RETRY_TIMING,
        AbortSignal.timeout(GIVE_UP_MS),
      );
      record(hall.measured(), operation, performance.now() - started);
      return value;
    } catch (error) {
      fail(hall.measured(), `${operation}: ${error instanceof Error ? `${error.name}: ${error.message}` : String(error)}`);
      return undefined;
    } finally {
      hall.measured().requests += 1;
      hall.measured().resent += Math.max(0, attempts - 1);
    }
  };
  const command = async (sent: Command): Promise<boolean> => {
    const verdict: CommandVerdict | undefined = await timed(sent.type, () => client.command(sent));
    if (verdict !== undefined && verdict.kind !== "accepted") fail(hall.measured(), `${sent.type}: ${verdict.kind === "rejected" ? verdict.rejection : verdict.detail}`);
    return verdict?.kind === "accepted";
  };
  const room = (scope: string, surface: string): Room => {
    const joined = new Room(hall.base, HALL, scope, surface, hall.measured);
    hall.rooms.add(joined);
    return joined;
  };
  const leave = (joined: Room): void => {
    hall.rooms.delete(joined);
    joined.leave();
  };
  /** 🖱️ Spend `milliseconds` at `anchor`, moving the pointer some of the time. */
  const dwell = async (milliseconds: number, page: Room, anchor: string): Promise<void> => {
    const until = performance.now() + milliseconds;
    while (performance.now() < until) {
      if (Math.random() < CURSOR.share) page.share({ tag, cursor: { anchor, x: Math.round(Math.random() * 1000) / 1000, y: Math.round(Math.random() * 1000) / 1000 }, focus: anchor });
      await pause(Math.min(CURSOR.intervalMs, Math.max(0, until - performance.now())));
    }
  };

  await pause(Math.random() * REAL.arrival * hall.scale);
  const identity: IdentityClaim = index % 3 === 0 ? { kind: "anonymous" } : { kind: index % 3 === 1 ? "pseudonym" : "name", handle: `Hall ${index} ${learner.slice(0, 10)}` };
  if (!(await command({ type: "identify-learner", id: newId(), learner, identity }))) return;
  const here = (place: PresenceState["place"]): PresenceState => ({ tag, identity: identity.kind === "anonymous" ? identity : { kind: identity.kind, handle: identity.handle }, place, active: true });

  const roster = room(rosterScope(hall.tenant), "home");
  let page = room(`${hall.tenant}/home`, "home");
  await Promise.all([roster.join(), page.join()]);
  roster.share(here({ screen: "home" }));
  page.watch([`${hall.tenant}/introduction`, `${hall.tenant}/leaderboard`, `${hall.tenant}/badges`, ...hall.quizzes.flatMap((quiz) => [`${hall.tenant}/quiz/${quiz}`, thinkingScope(hall.tenant, quiz)])].slice(0, 16));
  await Promise.all([timed("query course", () => client.course(learner)), timed("query learner", () => client.learner(learner)), timed("query leaderboard", () => client.leaderboard({ period: "all-time" }, learner))]);

  let attending = true;
  const polling = (async () => {
    while (attending) {
      await pause(REAL.poll * hall.scale * between(0.8, 1.2));
      if (attending) await timed("query leaderboard", () => client.leaderboard({ period: "all-time" }, learner));
    }
  })();

  const quizzes = [hall.quizzes[0], ...shuffled(hall.quizzes.slice(1))].slice(0, plays);
  for (const [played, quiz] of quizzes.entries()) {
    await dwell(REAL.home * hall.scale * between(0.5, 1.5), page, `card:${quiz}`);
    const run = newId();
    const challenge = CHALLENGES[(index + played) % CHALLENGES.length];
    if (!(await command({ type: "start-run", id: newId(), learner, run, quiz, challenge, at: Date.now() }))) break;
    const view: RunView | undefined = await timed("query run", () => client.run(run, learner));
    if (view === undefined) break;
    leave(page);
    page = room(`${hall.tenant}/quiz/${quiz}`, "run");
    const thinking = room(thinkingScope(hall.tenant, quiz), "run");
    await Promise.all([page.join(), thinking.join()]);
    const drafts: Record<string, ThinkingAnswer> = {};
    for (const task of view.sheet.tasks) {
      roster.share(here({ screen: "run", quiz, task: task.id }));
      const opened = Date.now();
      if (task.seconds !== undefined && !(await command({ type: "open-task", id: newId(), learner, run, task: task.id, at: opened }))) continue;
      const deadline = task.seconds === undefined ? Number.POSITIVE_INFINITY : opened + task.seconds * 1000 - CLOCK_MARGIN_MS;
      for (let revision = 0, revisions = 2 + Math.floor(Math.random() * 3); revision < revisions; revision += 1) {
        await dwell(between(REAL.revise[0], REAL.revise[1]) * hall.scale, page, `task:${task.id}`);
        if (Date.now() > deadline) break;
        const { answer, draft } = answerOf(task);
        if (draft !== undefined) drafts[task.id] = draft;
        thinking.share({ tag, answers: { ...drafts } });
        if ((await command({ type: "record-answer", id: newId(), learner, run, task: task.id, answer, at: Date.now() })) && challengeRules(challenge).hints) await timed("query run", () => client.run(run, learner));
      }
    }
    await command({ type: "submit-run", id: newId(), learner, run });
    roster.share(here({ screen: "results", quiz }));
    await Promise.all([timed("query run", () => client.run(run, learner)), timed("query crowd", () => client.crowd(quiz, learner)), timed("query leaderboard", () => client.leaderboard({ period: "all-time" }, learner)), timed("query learner", () => client.learner(learner))]);
    await dwell(REAL.read * hall.scale, page, "card:results");
    leave(thinking);
    leave(page);
    page = room(`${hall.tenant}/home`, "home");
    await page.join();
    roster.share(here({ screen: "home" }));
  }
  attending = false;
  await polling;
  leave(page);
  leave(roster);
}

/** 📶️ The Wi-Fi drop: every open socket of the hall drops and rejoins within the (compressed) storm window. */
async function storm(hall: Hall): Promise<number> {
  const rooms = [...hall.rooms];
  await Promise.all(rooms.map((joined) => joined.reconnect(Math.random() * REAL.storm).then(() => undefined, () => undefined)));
  return rooms.length;
}

/** 📜️ What one thread of the hall is told: the learners of one lecture it plays. */
export interface HallOrders {
  readonly progressFile?: string;
  readonly role: "hall";
  readonly base: string;
  readonly tenant: string;
  readonly quizzes: readonly string[];
  readonly scale: number;
  readonly learners: readonly number[];
}

/** 🧾️ What one thread of the hall measured. */
export interface HallSlice {
  readonly latencies: Record<string, number[]>;
  readonly errors: string[];
  readonly throttled: number;
  readonly requests: number;
  readonly resent: number;
  readonly sockets: Measurements["sockets"];
  readonly reconnected: number;
  /** 🐌️ How late this thread's own timers fired, in milliseconds, probed every {@link LAG_PROBE_MS}. */
  readonly lag: number[];
}

/** 🎤️ One thread's part of a lecture: its learners attend, and when `dropped` resolves its sockets drop once. */
export async function lecture(orders: HallOrders, dropped: Promise<void>): Promise<HallSlice> {
  const measured = measurements();
  const hall: Hall = { base: orders.base, tenant: orders.tenant, quizzes: orders.quizzes, scale: orders.scale, measured: () => measured, rooms: new Set() };
  const lag: number[] = [];
  let due = performance.now() + LAG_PROBE_MS;
  const probe = setInterval(() => {
    const now = performance.now();
    lag.push(Math.max(0, now - due));
    due = now + LAG_PROBE_MS;
  }, LAG_PROBE_MS);
  let attending = true;
  let reconnected = 0;
  let storming: Promise<void> = Promise.resolve();
  void dropped.then(() => {
    capacityProgress(orders.progressFile, `hall worker ${orders.learners[0]} received drop`);
    if (attending) storming = storm(hall).then((count) => void (reconnected = count));
  });
  await Promise.all(orders.learners.map((index) => attend(hall, index, 2)));
  capacityProgress(orders.progressFile, `hall worker ${orders.learners[0]} learner work complete`);
  attending = false;
  await storming;
  capacityProgress(orders.progressFile, `hall worker ${orders.learners[0]} storm complete: ${reconnected}`);
  clearInterval(probe);
  return { latencies: Object.fromEntries(measured.latencies), errors: measured.errors, throttled: measured.throttled, requests: measured.requests, resent: measured.resent, sockets: measured.sockets, reconnected, lag };
}
//#endregion 🧑‍🎓️Learner
