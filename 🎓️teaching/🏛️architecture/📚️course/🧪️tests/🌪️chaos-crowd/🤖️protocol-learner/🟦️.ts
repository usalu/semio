/** 🤖️ A learner without a browser: it speaks the site's own client protocol (the quiz client's `ProctorClient` envelopes and the presence socket) straight to the proctor, plays every quiz it is given perfectly from the course's own sources, and meets the same faults a browser's network does — requests lost before they arrive, answers lost after they were decided (the retry carries the same command id, so the proctor must apply it once), commands sent twice on purpose, and a roster socket that drops and rejoins.
 * @see ../🟦️.ts — the chaos spec that mixes these learners with real browsers
 * @see ../../../🎭️e2e/🚶️learner/🟦️.ts — the sources the answers are computed from */
import { connect, type Socket } from "node:net";
import { randomBytes } from "node:crypto";
import { PRESENCE_PROTOCOL } from "../../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧬️wire-types/🟦️.ts";
import { presenceSocketUrl } from "../../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🛣️routes-and-sockets/🟦️.ts";
import { learnerTag, rosterScope, type Answer, type Challenge, type Command, type RunView, type SheetTask } from "../../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts";
import { ProctorClient, ProctorUnavailable, RETRY_TIMING, newId, proctorTransport, retryTransient, type CommandVerdict, type HttpRequest, type HttpResponse, type HttpTransport } from "../../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts";
import { PAR, itemOf, taskOf, type SourceQuiz } from "../../../🎭️e2e/🚶️learner/🟦️.ts";

/** 🎲️ How hostile the network of one protocol learner is: the chance that a request is lost before it arrives, and the chance that its answer is lost after it was decided. */
export interface Faults {
  readonly lostRequest: number;
  readonly lostAnswer: number;
}

/** 🧾️ What a protocol learner went through and what it expects of the proctor afterwards. */
export interface Account {
  readonly learner: string;
  readonly handle: string | undefined;
  readonly expectedTotal: number;
  readonly quizzes: readonly string[];
  injected: { lostRequests: number; lostAnswers: number; doubleSends: number; socketDrops: number; retried: number };
  readonly problems: string[];
}

const pause = (milliseconds: number): Promise<void> => new Promise((done) => setTimeout(done, milliseconds));
const shuffled = <T>(items: readonly T[]): T[] => items.map((item) => [Math.random(), item] as const).sort((left, right) => left[0] - right[0]).map(([, item]) => item);

/** 🌩️ A transport that loses requests and answers at random; a lost answer was decided by the proctor all the same. */
function faulty(base: string, faults: Faults, account: Account): HttpTransport {
  const connect = proctorTransport(base, 15_000);
  return {
    async send(request: HttpRequest): Promise<HttpResponse> {
      if (Math.random() < faults.lostRequest) {
        account.injected.lostRequests += 1;
        throw new ProctorUnavailable("injected: request lost");
      }
      const answered = await connect().send(request);
      if (request.method === "POST" && Math.random() < faults.lostAnswer) {
        account.injected.lostAnswers += 1;
        throw new ProctorUnavailable("injected: answer lost");
      }
      return answered;
    },
  };
}

/** 🃏️ The card of every item in one dimension, each card used once: items of equal value take the equal cards in turn. */
function distinctCards(cards: readonly number[], values: readonly (readonly [string, number])[]): Record<string, number> {
  const used = new Set<number>();
  return Object.fromEntries(values.map(([id, value]) => {
    const card = cards.findIndex((candidate, index) => candidate === value && !used.has(index));
    used.add(card);
    return [id, card];
  }));
}

/** ✅️ The perfect answer to one presented task, computed from the quiz as authored. */
export function perfectAnswer(quiz: SourceQuiz, task: SheetTask): Answer {
  const source = taskOf(quiz, task.id);
  const ids = task.items.map((item) => item.id);
  const authored = new Map(source.items.map((item, index) => [item.id, index]));
  if (task.kind === "classification") return { kind: "classification", assignments: Object.fromEntries(ids.map((id) => [id, itemOf(source, id).category!])) };
  if (task.kind === "sorting") {
    const order = [...ids].sort((left, right) => itemOf(source, left).value! - itemOf(source, right).value! || authored.get(left)! - authored.get(right)!);
    return task.keys === undefined ? { kind: "sorting", order, guesses: Object.fromEntries(ids.map((id) => [id, itemOf(source, id).value!])) } : { kind: "sorting", order };
  }
  const shown = task.dimensions.filter((dimension) => dimension.cards !== undefined);
  const hidden = task.dimensions.filter((dimension) => dimension.cards === undefined);
  const value = (id: string, dimension: string): number => itemOf(source, id).values![dimension]!;
  return {
    kind: "matching",
    ...(shown.length === 0 ? {} : { assignments: Object.fromEntries(shown.map((dimension) => [dimension.id, distinctCards(dimension.cards!, ids.map((id) => [id, value(id, dimension.id)] as const))])) }),
    ...(hidden.length === 0 ? {} : { guesses: Object.fromEntries(hidden.map((dimension) => [dimension.id, Object.fromEntries(ids.map((id) => [id, value(id, dimension.id)]))])) }),
  };
}

/** 🚪️ The roster socket of a protocol learner: joins the course's roster room, shares its place, and rejoins with its latest state after a drop. */
class Roster {
  private socket: WebSocket | undefined;
  private latest: unknown;
  private gone = false;
  private readonly opened = new Set<WebSocket>();
  constructor(private readonly base: string, private readonly tenant: string) {}
  async join(): Promise<void> {
    for (let attempt = 0; attempt < 20 && !this.gone; attempt++) {
      const opened = await new Promise<boolean>((done) => {
        const socket = new WebSocket(presenceSocketUrl(this.base, rosterScope(this.tenant), "home"), [PRESENCE_PROTOCOL]);
        this.socket = socket;
        this.opened.add(socket);
        let greeted = false;
        const timer = setTimeout(() => (socket.close(), done(false)), 10_000);
        socket.addEventListener("message", () => (clearTimeout(timer), (greeted = true), done(true)), { once: true });
        socket.addEventListener("close", () => (this.opened.delete(socket), clearTimeout(timer), done(false), greeted && this.socket === socket && !this.gone && ((this.socket = undefined), setTimeout(() => void this.join(), 500 + Math.random() * 2500))), { once: true });
        socket.addEventListener("error", () => undefined);
      });
      if (opened) break;
      await pause(500 + attempt * 250);
    }
    if (this.latest !== undefined) this.send(this.latest);
  }
  share(state: unknown): void {
    this.latest = state;
    this.send(state);
  }
  async drop(afterMs: number): Promise<void> {
    const dropped = this.socket;
    this.socket = undefined;
    dropped?.close();
    await pause(afterMs);
    if (!this.gone) await this.join();
  }
  leave(): void {
    this.gone = true;
    this.socket?.close();
    for (const socket of this.opened) socket.close();
  }
  private send(state: unknown): void {
    if (this.socket?.readyState === WebSocket.OPEN) this.socket.send(JSON.stringify({ type: "state", state }));
  }
}

/** 🧑‍🎓️ One protocol learner's whole course: it registers, joins the roster, plays every quiz (start, answer every task — some of them twice under one command id —, submit), and returns once it is done; the roster socket stays open until {@link ProtocolLearner.leave}. */
export class ProtocolLearner {
  readonly account: Account;
  private readonly client: ProctorClient;
  private readonly roster: Roster;
  private readonly identity: { kind: "anonymous" } | { kind: "pseudonym" | "name"; handle: string };

  constructor(private readonly base: string, private readonly tenant: string, readonly index: number, private readonly quizzes: readonly SourceQuiz[], private readonly faults: Faults, private readonly give: AbortSignal) {
    const learner = newId();
    const challenges = quizzes.map((_, place) => (["easy", "medium", "hard"] as const)[(index + place) % 3]!);
    this.identity = index % 3 === 0 ? { kind: "anonymous" } : { kind: index % 3 === 1 ? "pseudonym" : "name", handle: `Bot ${index} ${learner.slice(0, 8)}` };
    this.account = { learner, handle: this.identity.kind === "anonymous" ? undefined : this.identity.handle, expectedTotal: challenges.reduce((sum, challenge) => sum + PAR[challenge], 0), quizzes: quizzes.map((quiz) => quiz.id), injected: { lostRequests: 0, lostAnswers: 0, doubleSends: 0, socketDrops: 0, retried: 0 }, problems: [] };
    this.client = new ProctorClient(() => faulty(base, faults, this.account), tenant);
    this.roster = new Roster(base, tenant);
    this.challenges = challenges;
  }
  private readonly challenges: readonly Challenge[];

  private async send(command: Command): Promise<CommandVerdict> {
    let attempts = 0;
    const verdict = await retryTransient(() => (attempts += 1, this.client.command(command)), RETRY_TIMING, AbortSignal.any([this.give, AbortSignal.timeout(180_000)]));
    this.account.injected.retried += Math.max(0, attempts - 1);
    if (Math.random() < 0.15) {
      this.account.injected.doubleSends += 1;
      const again = await retryTransient(() => this.client.command(command), RETRY_TIMING, AbortSignal.any([this.give, AbortSignal.timeout(180_000)]));
      if (again.kind !== verdict.kind) this.account.problems.push(`${command.type}: a repeated command id answered ${again.kind} after ${verdict.kind}`);
    }
    return verdict;
  }

  private async accept(command: Command): Promise<void> {
    const verdict = await this.send(command);
    if (verdict.kind !== "accepted") this.account.problems.push(`${command.type}: ${verdict.kind === "rejected" ? JSON.stringify(verdict.rejection) : verdict.detail} ${JSON.stringify(command).slice(0, 700)}`);
  }

  finished = false;

  async play(): Promise<void> {
    try {
      await this.playAll();
    } finally {
      this.finished = true;
    }
  }

  private async playAll(): Promise<void> {
    const { learner } = this.account;
    await pause(Math.random() * 3000);
    await this.accept({ type: "identify-learner", id: newId(), learner, identity: this.identity });
    await this.roster.join();
    const tag = learnerTag(learner);
    const here = (place: unknown): unknown => ({ tag, identity: this.identity, place, active: true });
    this.roster.share(here({ screen: "home" }));
    for (const [place, quiz] of this.quizzes.entries()) {
      if (this.give.aborted) return;
      const run = newId();
      await this.accept({ type: "start-run", id: newId(), learner, run, quiz: quiz.id, challenge: this.challenges[place]!, at: Date.now() });
      const view = await this.view(run);
      if (view === undefined) return;
      for (const task of view.sheet.tasks) {
        this.roster.share(here({ screen: "run", quiz: quiz.id, task: task.id }));
        if (task.seconds !== undefined) await this.accept({ type: "open-task", id: newId(), learner, run, task: task.id, at: Date.now() });
        await pause(100 + Math.random() * 400);
        await this.accept({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: perfectAnswer(quiz, task), at: Date.now() });
        if (Math.random() < 0.05) {
          this.account.injected.socketDrops += 1;
          void this.roster.drop(Math.random() * 3000);
        }
      }
      await this.accept({ type: "submit-run", id: newId(), learner, run });
      this.roster.share(here({ screen: "results", quiz: quiz.id }));
      await pause(Math.random() * 1000);
      this.roster.share(here({ screen: "home" }));
    }
  }

  private async view(run: string): Promise<RunView | undefined> {
    try {
      return await retryTransient(() => this.client.run(run, this.account.learner), RETRY_TIMING, AbortSignal.any([this.give, AbortSignal.timeout(180_000)]));
    } catch (error) {
      this.account.problems.push(`run view: ${error instanceof Error ? error.message : String(error)}`);
      return undefined;
    }
  }

  /** 🔎️ What the proctor holds of this learner now. */
  learnerView(): ReturnType<ProctorClient["learner"]> {
    return retryTransient(() => this.client.learner(this.account.learner), RETRY_TIMING, AbortSignal.timeout(180_000));
  }

  leave(): void {
    this.roster.leave();
  }
}

/** 👻️ How a presence socket vanishes: its TCP connection ends without a close frame (`closed`), is reset (`reset`), or stays open while its peer stops answering, as a closed laptop lid or a Wi-Fi that is gone does (`silent`). */
export type Vanish = "closed" | "reset" | "silent";

/** 🔌️ One masked client text frame, as a browser would send it. */
function frame(text: string): Buffer {
  const payload = Buffer.from(text);
  const mask = randomBytes(4);
  const head = payload.length < 126 ? Buffer.from([0x81, 0x80 | payload.length]) : Buffer.from([0x81, 0x80 | 126, payload.length >> 8, payload.length & 255]);
  return Buffer.concat([head, mask, Buffer.from(payload.map((byte, index) => byte ^ mask[index % 4]!))]);
}

/** 👻️ A learner on the course roster that vanishes without a close frame: a raw TCP client that upgrades to the presence socket by hand, shares one state, and then ends, resets or goes silent (never answering a ping, never reading again). Resolves once the proctor has welcomed it and the state is sent; the returned function destroys the socket for good. */
export async function ghost(base: string, tenant: string, how: Vanish, origin: string): Promise<() => void> {
  const url = new URL(presenceSocketUrl(base, rosterScope(tenant), "home"));
  const socket: Socket = connect({ host: url.hostname, port: Number(url.port) });
  socket.on("error", () => undefined);
  await new Promise<void>((ready) => socket.once("connect", () => ready()));
  socket.write([`GET ${url.pathname}${url.search} HTTP/1.1`, `Host: ${url.host}`, "Upgrade: websocket", "Connection: Upgrade", `Sec-WebSocket-Key: ${randomBytes(16).toString("base64")}`, "Sec-WebSocket-Version: 13", `Sec-WebSocket-Protocol: ${PRESENCE_PROTOCOL}`, `Origin: ${origin}`, "", ""].join("\r\n"));
  await new Promise<void>((welcomed, failed) => {
    let seen = "";
    const timer = setTimeout(() => failed(new Error("no welcome for the ghost")), 20_000);
    socket.on("data", (chunk) => {
      seen += chunk.toString("latin1");
      if (!seen.includes("101") ) return failed(new Error(`ghost upgrade refused: ${seen.slice(0, 120)}`));
      if (seen.includes('"welcome"')) (clearTimeout(timer), welcomed());
    });
  });
  const id = randomBytes(16).toString("hex");
  socket.write(frame(JSON.stringify({ type: "state", state: { tag: learnerTag(id), identity: { kind: "anonymous" }, place: { screen: "home" }, active: true } })));
  await pause(300);
  socket.removeAllListeners("data");
  if (how === "silent") socket.pause();
  else if (how === "reset") socket.resetAndDestroy();
  else socket.destroy();
  return () => socket.destroy();
}
