/** 🎭️ The abusive script of the capacity gate: it fills the roster, floods malformed commands and queries, sends oversized bodies and frames and pushes sockets into the hall's room. */
import { connect, type Socket } from "node:net";
import { type Command } from "../../../../../🧰️framework/🛍️products/📚️course/🟦️.ts";
import { commandEnvelope, newId } from "../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts";
import { PRESENCE_PROTOCOL } from "../../../../../🧰️framework/🛍️products/🖥️server/🧬️wire-types/🟦️.ts";
import { encodeCommandEnvelope } from "../../../../../🧰️framework/🛍️products/🖥️server/🔤️codec/🟦️.ts";
import { presenceSocketUrl } from "../../../../../🧰️framework/🛍️products/🖥️server/🛣️routes-and-sockets/🟦️.ts";
import { GIVE_UP_MS, SCRIPT, SIGN_UP } from "../📊️settings-and-measurements/🟦️.ts";
import { forwarded, pause } from "../🚚️transport/🟦️.ts";

//#region 😈️Script
/** 😈️ What the abusive script is told. */
export interface ScriptOrders {
  readonly progressFile?: string;
  readonly role: "script";
  readonly base: string;
  readonly tenant: string;
  readonly quiz: string;
  readonly bodyLimit: number;
  readonly socketsPerAddress: number;
}

/** 🧾️ What the abusive script saw: answers by status per attack. */
export interface ScriptReport {
  /** ⏱️ How long the script ran: what its allowances are computed over. */
  readonly seconds: number;
  /** ✍️ Registrations of new learners, anonymous and under handles, as fast as the script can ask: answers by status. */
  readonly commands: Record<string, number>;
  /** 💰️ What became of those registrations: `registered`, `refused` (a `429` naming the sign-up allowance), `throttled`
   * (a `429` of the command allowance), `rejected <detail>` or `unanswered <status>`. */
  readonly signUps: Record<string, number>;
  /** 🧨️ Commands addressed to ids no actor can have. */
  readonly malformed: Record<string, number>;
  readonly queries: Record<string, number>;
  /** 🐘️ Bodies far past the limit: `413`, `429`, or `0` when the proctor cut the connection while the body was still on its way. */
  readonly oversizedBody: Record<string, number>;
  readonly sockets: { readonly attempted: number; readonly opened: number; readonly refused: Record<string, number> };
  readonly oversizedFrame: { readonly sent: number; readonly closed: number };
}

function tally(into: Record<string, number>, key: string | number): void {
  into[String(key)] = (into[String(key)] ?? 0) + 1;
}

/** 🧵️ One connection the script keeps open and posts on, one request after the other — how a script that wants the most
 * of its address sends, and how its requests arrive behind a proxy that keeps its connections to the proctor. A raw
 * socket rather than `fetch`, whose connection pool opens a new connection for every few requests once many run at
 * once: on one machine that empties the pool of client ports the hall connects from. */
export class CapacityWire {
  private socket: Socket | undefined;
  private received = Buffer.alloc(0);
  private answered: ((status: number, body: string) => void) | undefined;
  private timer: ReturnType<typeof setTimeout> | undefined;

  constructor(private readonly base: string, private readonly timeoutMs = GIVE_UP_MS) {}

  /** 📮️ Post `body` to `path` as the proxy would forward it for the script; the status, or `0` when the connection
   * was cut before the answer was whole. */
  async post(path: string, body: string): Promise<number> {
    return (await this.asked(path, body)).status;
  }

  /** 📬️ Post like {@link post} and answer the body beside the status. */
  asked(path: string, body: string): Promise<{ readonly status: number; readonly body: string }> {
    return new Promise((answered) => {
      this.answered = (status, body) => answered({ status, body });
      this.timer = setTimeout(() => this.close(), this.timeoutMs);
      const { hostname, port, host } = new URL(this.base);
      const head = [`POST ${path} HTTP/1.1`, `Host: ${host}`, "Content-Type: application/json", `Content-Length: ${Buffer.byteLength(body)}`, ...Object.entries(forwarded(SCRIPT)).map(([name, value]) => `${name}: ${value}`), "", ""].join("\r\n");
      if (this.socket === undefined) {
        const socket = connect({ host: hostname, port: Number(port), noDelay: true });
        this.socket = socket;
        this.received = Buffer.alloc(0);
        socket.on("data", (chunk) => this.take(socket, chunk));
        socket.on("error", () => undefined);
        socket.on("close", () => this.settle(socket, 0, "", false));
      }
      this.socket.write(head + body);
    });
  }

  /** 🚪️ Close the connection. */
  close(): void {
    clearTimeout(this.timer);
    this.timer = undefined;
    const answered = this.answered;
    this.answered = undefined;
    this.socket?.destroy();
    this.socket = undefined;
    answered?.(0, "");
  }

  private take(socket: Socket, chunk: Buffer): void {
    if (socket !== this.socket) return;
    this.received = Buffer.concat([this.received, chunk]);
    const end = this.received.indexOf("\r\n\r\n");
    if (end < 0) return;
    const head = this.received.subarray(0, end).toString("latin1").toLowerCase();
    const length = Number(/\r\ncontent-length: (\d+)/u.exec(head)?.[1] ?? 0);
    if (this.received.length < end + 4 + length) return;
    const body = this.received.subarray(end + 4, end + 4 + length).toString("utf8");
    this.received = this.received.subarray(end + 4 + length);
    this.settle(socket, Number(head.slice(9, 12)), body, !head.includes("\r\nconnection: close"));
  }

  private settle(socket: Socket, status: number, body: string, keep: boolean): void {
    if (socket !== this.socket) return;
    if (!keep) {
      this.socket = undefined;
      socket.destroy();
    }
    const answered = this.answered;
    clearTimeout(this.timer);
    this.timer = undefined;
    this.answered = undefined;
    answered?.(status, body);
  }
}

/** 🧧️ What became of one posted registration, from its answer: `registered` (accepted with its event), `refused` (a
 * `429` naming the sign-up allowance), `throttled` (any other `429`), `rejected <detail>` (decided against, or the
 * replay of one), `unanswered <status>` otherwise. */
export function signUpFate(status: number, body: string): string {
  let answer: { status?: string; events?: unknown[]; reason?: { detail?: string }; allowance?: string } = {};
  try {
    answer = JSON.parse(body) as typeof answer;
  } catch {}
  if (status === 429) return answer.allowance === SIGN_UP ? "refused" : "throttled";
  if (status !== 200) return `unanswered ${status}`;
  return answer.status === "accepted" && (answer.events?.length ?? 0) > 0 ? "registered" : `rejected ${answer.reason?.detail ?? answer.status ?? "?"}`;
}

/** 😈️ Run every attack beside the hall until told to stop; report what was answered. */
export async function script(orders: ScriptOrders, stopped: () => boolean): Promise<ScriptReport> {
  const started = performance.now();
  const report: Omit<ScriptReport, "seconds"> = { commands: {}, signUps: {}, malformed: {}, queries: {}, oversizedBody: {}, sockets: { attempted: 0, opened: 0, refused: {} }, oversizedFrame: { sent: 0, closed: 0 } };
  const wires: CapacityWire[] = [];
  const wire = (): CapacityWire => {
    const opened = new CapacityWire(orders.base);
    wires.push(opened);
    return opened;
  };
  const envelope = (command: Command): string => JSON.stringify(encodeCommandEnvelope(commandEnvelope(command, orders.tenant, Date.now())));
  const flood = async (claiming: boolean): Promise<void> => {
    const posting = wire();
    while (!stopped()) {
      const learner = newId();
      const answer = await posting.asked("/commands", envelope({ type: "identify-learner", id: newId(), learner, identity: claiming ? { kind: "pseudonym", handle: `Script ${learner.slice(0, 16)}` } : { kind: "anonymous" } }));
      tally(report.commands, answer.status);
      tally(report.signUps, signUpFate(answer.status, answer.body));
    }
  };
  const malformed = async (): Promise<void> => {
    const posting = wire();
    while (!stopped()) {
      const sent = JSON.parse(envelope({ type: "start-run", id: newId(), learner: newId(), run: newId(), quiz: orders.quiz, challenge: "medium", at: Date.now() })) as { target: { id: string } };
      sent.target.id = `no-such-learner-${Math.random()}`;
      tally(report.malformed, await posting.post("/commands", JSON.stringify(sent)));
    }
  };
  const queries = async (): Promise<void> => {
    const posting = wire();
    const query = JSON.stringify({ queryId: newId(), kind: "course.leaderboard", version: 1, scope: orders.tenant, principal: { kind: "anonymous" }, arguments: Array.from(new TextEncoder().encode(JSON.stringify({ type: "leaderboard", period: "all-time" }))), consistency: { kind: "authority" }, cursor: null });
    while (!stopped()) tally(report.queries, await posting.post("/queries", query));
  };
  const oversized = async (): Promise<void> => {
    const posting = wire();
    const body = JSON.stringify({ padding: "x".repeat(orders.bodyLimit * 64) });
    while (!stopped()) {
      tally(report.oversizedBody, await posting.post("/commands", body));
      await pause(50);
    }
  };
  const open = (scope: string): Promise<WebSocket | number> =>
    new Promise((settled) => {
      const socket = new WebSocket(presenceSocketUrl(orders.base, scope, "home"), { headers: forwarded(SCRIPT), protocols: [PRESENCE_PROTOCOL] } as unknown as string[]);
      const timer = setTimeout(() => { socket.close(); settled(0); }, GIVE_UP_MS);
      socket.addEventListener("open", () => { clearTimeout(timer); settled(socket); });
      socket.addEventListener("close", (event) => { clearTimeout(timer); settled(event.code); });
      socket.addEventListener("error", () => { clearTimeout(timer); settled(0); });
    });
  const held: WebSocket[] = [];
  const sockets = async (): Promise<void> => {
    while (!stopped() && report.sockets.attempted < orders.socketsPerAddress + 512) {
      const wave = await Promise.all(Array.from({ length: 64 }, () => open(orders.tenant)));
      for (const opened of wave) {
        (report.sockets as { attempted: number }).attempted += 1;
        if (typeof opened === "number") tally(report.sockets.refused, opened);
        else {
          (report.sockets as { opened: number }).opened += 1;
          held.push(opened);
        }
      }
    }
  };
  const frames = async (): Promise<void> => {
    while (!stopped()) {
      const opened = await open(`${orders.tenant}/home`);
      if (typeof opened !== "number") {
        const closed = new Promise<void>((done) => opened.addEventListener("close", () => done()));
        opened.send(JSON.stringify({ type: "state", state: { padding: "x".repeat(64 * 1024) } }));
        (report.oversizedFrame as { sent: number }).sent += 1;
        if ((await Promise.race([closed.then(() => true), pause(5_000).then(() => false)])) === true) (report.oversizedFrame as { closed: number }).closed += 1;
        else opened.close();
      }
      await pause(250);
    }
  };
  await Promise.all([...Array.from({ length: 24 }, (_, wire) => flood(wire % 2 === 1)), ...Array.from({ length: 8 }, malformed), ...Array.from({ length: 64 }, queries), oversized(), sockets(), frames()]);
  const seconds = (performance.now() - started) / 1000;
  for (const socket of held) socket.close();
  for (const opened of wires) opened.close();
  return { ...report, seconds };
}
//#endregion 😈️Script
