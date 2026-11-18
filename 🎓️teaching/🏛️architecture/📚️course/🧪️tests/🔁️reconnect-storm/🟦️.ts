/** 🔁️ A reconnect storm: ten real browser learners are signed in — some in a run, some on the overview and the leaderboard — while the one proctor of the stack is killed and started again and again, with down-times from a tenth of a second to ninety seconds, frozen (a black hole: sockets stay open, nothing answers) and thawed or killed while frozen, killed during a socket handshake and during an in-flight command or submission, with the network of some learners dropping and returning in every order around the kill, and tabs going hidden and visible meanwhile. After every cycle each page must show its presence footer with everyone online and its connection indicator calm again, within a bound derived from the client's own backoff cap; the worst recovery of the run is reported.
 * The stack comes from the gate (`TEACHING_ARCHITECTURE_COURSE_E2E_*`) and kills its proctor by process id, one at a time, so no two proctors ever contend for the port. `RECONNECT_LEARNERS`, `RECONNECT_CYCLES`, `RECONNECT_SEED` and `RECONNECT_REPORT` (a JSON file path) tune a run.
 * @see ./🥒️.feature — the scenarios in words
 * @see ../🔌️connection-shortage/🟦️.ts — one learner through one shortage
 * @see ../🌪️chaos-crowd/🟦️.ts — a whole class through chaos
 * @see ../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👥️presence/🔌️socket/🟦️.ts — the room that rejoins */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import type { Page, Request } from "@playwright/test";
import { PRESENCE_HEARTBEAT_MS, PRESENCE_SILENCE_FACTOR } from "../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧬️wire-types/🟦️.ts";
import { RETRY_TIMING } from "../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts";
import { platform } from "node:os";
import { answerRun, answerTask, card, enter, expect, handle, openTask, playQuiz, primary, proctor, quizOf, screen, test, type Device, type Locale } from "../../🎭️e2e/🚶️learner/🟦️.ts";

//#region 🎚️Settings
const LEARNERS = Number(process.env.RECONNECT_LEARNERS ?? 10);
const CYCLES = Number(process.env.RECONNECT_CYCLES ?? 16);
const SEED = Number(process.env.RECONNECT_SEED ?? Date.now() % 2_147_483_647);
const REPORT = process.env.RECONNECT_REPORT ?? "";
const DOWNTIMES_MS = [100, 300, 1_000, 3_000, 8_000, 20_000, 45_000, 90_000] as const;
const CONNECT_AND_ANSWER_MS = 10_000;

/** 🕳️ How long a joined socket may stay silent before the client closes it: {@link PRESENCE_SILENCE_FACTOR} heartbeats. A freeze of the proctor must outlast it for the client to notice. */
const SILENCE_MS = PRESENCE_HEARTBEAT_MS * PRESENCE_SILENCE_FACTOR;

/** ⏱️ The longest a page may take to be whole again once the proctor is back, the network is back and the tab is visible: it waits out one backoff of at most the cap, tries again, and waits out another, and its last request may take as long as the client lets a request take. */
export const RECOVERY_BOUND_MS = 2 * RETRY_TIMING.maxMs + CONNECT_AND_ANSWER_MS;

/** 🕰️ Extra time the browser and the machine may take on top of the bound before a page counts as stuck. */
const SLACK_MS = 5_000;
//#endregion 🎚️Settings

//#region 🎲️Randomness
function generator(seed: number): () => number {
  let state = seed >>> 0;
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let mixed = Math.imul(state ^ (state >>> 15), state | 1);
    mixed ^= mixed + Math.imul(mixed ^ (mixed >>> 7), mixed | 61);
    return ((mixed ^ (mixed >>> 14)) >>> 0) / 4_294_967_296;
  };
}
const sleep = (milliseconds: number): Promise<void> => new Promise((done) => setTimeout(done, milliseconds));
//#endregion 🎲️Randomness

//#region 🕳️Black hole
const CONTROL = process.env.TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL ?? "";

/** 🕳️ Whether the platform can freeze a process (`SIGSTOP`), which the black-hole cycles need. */
const FREEZING = platform() !== "win32";

/** 🧊️ Freezes (`pause`) or thaws (`resume`) the stack's proctor: its accepted sockets stay open and nothing answers, as with a network that swallows packets without a reset. */
async function freeze(action: "pause" | "resume"): Promise<number> {
  return (await fetch(`${CONTROL}/proctor/${action}`, { method: "POST" })).status;
}
//#endregion 🕳️Black hole

//#region 🧑‍🎓️Learners
type Place = "run" | "run-idle" | "board" | "home";

interface Learner {
  readonly index: number;
  readonly device: Device;
  readonly place: Place;
  readonly sockets: string[];
  hidden: boolean;
  offline: boolean;
}

const PLACES: readonly Place[] = ["run", "run", "run-idle", "board", "board", "board", "home", "home", "home", "home"];
const QUIZ = quizOf("cooling");

async function visibility(page: Page, state: "hidden" | "visible"): Promise<void> {
  await page.evaluate((shown) => {
    Object.defineProperty(document, "visibilityState", { configurable: true, get: () => shown });
    Object.defineProperty(document, "hidden", { configurable: true, get: () => shown === "hidden" });
    document.dispatchEvent(new Event("visibilitychange"));
  }, state);
}

async function arrange(index: number, device: Device, started: number): Promise<Learner> {
  const place = PLACES[index % PLACES.length]!;
  const sockets: string[] = [];
  device.page.on("websocket", (socket) => {
    const at = (): number => Math.round((Date.now() - started) / 1000);
    const scope = decodeURIComponent(new URL(socket.url()).pathname).replace(/\/presence\/ws$/u, "").slice(-24);
    sockets.push(`+${at()}s open ${scope}`);
    socket.on("close", () => sockets.push(`+${at()}s closed ${scope}`));
    socket.on("socketerror", (error) => sockets.push(`+${at()}s error ${scope} ${String(error).slice(0, 40)}`));
  });
  await enter(device, { kind: "pseudonym", handle: handle(`Storm${index}`) });
  if (place === "run" || place === "run-idle") {
    await playQuiz(device, QUIZ.id);
    await openTask(device, 0);
  } else if (place === "board") await primary(card(device.page, "board")).click();
  return { index, device, place, sockets, hidden: false, offline: false };
}
//#endregion 🧑‍🎓️Learners

//#region 🌪️Storm
/** 📒️ What one cycle did and how long every page took to be whole again. */
interface Cycle {
  readonly index: number;
  readonly kind: string;
  readonly downMs: number;
  readonly presenceMs: number[];
  readonly calmMs: number[];
  readonly failures: string[];
}

interface Observation {
  readonly footer: boolean;
  readonly online: number;
  readonly tone: string | null;
  readonly visible: string;
  readonly onLine: boolean;
}

async function observe(page: Page): Promise<Observation> {
  return page.evaluate(() => ({
    footer: document.querySelector("[data-presence-footer] [data-presence-status]") !== null,
    online: Number(/\d+/u.exec(document.querySelector("[data-presence-footer] [data-presence-status]")?.textContent ?? "")?.[0] ?? -1),
    tone: document.querySelector("header [data-tone]")?.getAttribute("data-tone") ?? null,
    visible: document.visibilityState,
    onLine: navigator.onLine,
  }));
}

class Storm {
  readonly cycles: Cycle[] = [];
  readonly counts: Record<string, number> = {};
  private pending: string[] = [];
  constructor(private readonly learners: readonly Learner[], private readonly random: () => number) {}

  private count(kind: string): void {
    this.counts[kind] = (this.counts[kind] ?? 0) + 1;
  }

  private pick<T>(items: readonly T[]): T {
    return items[Math.floor(this.random() * items.length)]!;
  }

  private some(fraction: number): Learner[] {
    return this.learners.filter(() => this.random() < fraction);
  }

  private async expectSilentSocketsClosed(): Promise<void> {
    const seen = await Promise.all(this.learners.map(async (learner) => [learner, await observe(learner.device.page)] as const));
    for (const [learner, last] of seen) {
      if (last.footer) this.pending.push(`learner ${learner.index} (${learner.place}) still showed its presence footer after ${SILENCE_MS / 1000} s of a frozen proctor: the silent socket was not closed`);
      else this.count("silent-socket-closed");
    }
  }

  private async network(learners: readonly Learner[], offline: boolean): Promise<void> {
    await Promise.all(learners.map(async (learner) => {
      learner.offline = offline;
      await learner.device.context.setOffline(offline);
    }));
  }

  private async tabs(learners: readonly Learner[], hidden: boolean): Promise<void> {
    await Promise.all(learners.map(async (learner) => {
      learner.hidden = hidden;
      await visibility(learner.device.page, hidden ? "hidden" : "visible");
    }));
  }

  private async kill(): Promise<void> {
    this.count("kill");
    await proctor("stop");
  }

  private async revive(): Promise<void> {
    this.count("start");
    await proctor("start");
  }

  private async killOnNextSocket(): Promise<void> {
    const seen = new Promise<void>((done) => {
      const listeners = this.learners.map((learner) => {
        const listener = (): void => (cleanup(), done());
        learner.device.page.on("websocket", listener);
        return [learner.device.page, listener] as const;
      });
      const timer = setTimeout(() => (cleanup(), done()), 25_000);
      function cleanup(): void {
        clearTimeout(timer);
        for (const [page, listener] of listeners) page.off("websocket", listener);
      }
    });
    await seen;
    await this.kill();
    this.count("killed-in-handshake");
  }

  async killOnNextCommand(trigger: () => Promise<unknown>, only?: string): Promise<void> {
    const seen = new Promise<void>((done) => {
      const listeners = this.learners.map((learner) => {
        const listener = (request: Request): void => {
          if (request.method() === "POST" && request.url().endsWith("/commands") && (only === undefined || (request.postData() ?? "").includes(only))) (cleanup(), done());
        };
        learner.device.page.on("request", listener);
        return [learner.device.page, listener] as const;
      });
      const timer = setTimeout(() => (cleanup(), done()), 20_000);
      function cleanup(): void {
        clearTimeout(timer);
        for (const [page, listener] of listeners) page.off("request", listener);
      }
    });
    void trigger().catch(() => undefined);
    await seen;
    await this.kill();
    this.count("killed-in-flight");
  }

  private traffic(): Promise<unknown> {
    const runner = this.learners.find((learner) => learner.place === "run");
    this.count("answer");
    return runner === undefined ? Promise.resolve() : answerTask(runner.device, QUIZ, this.random() < 0.5 ? "perfect" : "flawed");
  }

  async cycle(index: number, downMs: number): Promise<void> {
    const kinds = ["blip", "handshake", "inflight", "offline-around", "offline-after", "hidden", "fuzz", ...(FREEZING ? (["black-hole", "long-black-hole", "freeze-then-kill"] as const) : [])] as const;
    const kind = kinds[index % kinds.length]!;
    this.count(kind);
    switch (kind) {
      case "blip":
        await this.kill();
        await sleep(downMs);
        break;
      case "handshake":
        await this.kill();
        await sleep(Math.min(downMs, 1_000));
        await this.revive();
        await this.killOnNextSocket();
        await sleep(downMs);
        break;
      case "inflight":
        await this.killOnNextCommand(() => this.traffic());
        await sleep(downMs);
        break;
      case "offline-around": {
        const some = this.some(0.6);
        await this.network(some, true);
        await this.kill();
        await sleep(downMs);
        await this.network(some, false);
        break;
      }
      case "offline-after": {
        const some = this.some(0.6);
        await this.kill();
        await sleep(downMs / 2);
        await this.network(some, true);
        await sleep(downMs / 2);
        await this.revive();
        await sleep(this.random() * 400);
        await this.network(some, false);
        break;
      }
      case "hidden": {
        const some = this.some(0.7);
        await this.tabs(some, true);
        await this.kill();
        await sleep(downMs);
        await this.revive();
        await sleep(this.random() * 3_000);
        break;
      }
      case "black-hole":
        this.count("frozen");
        await freeze("pause");
        await sleep(downMs);
        break;
      case "long-black-hole":
        this.count("frozen");
        await freeze("pause");
        await sleep(Math.max(downMs, SILENCE_MS + 15_000));
        await this.expectSilentSocketsClosed();
        break;
      case "freeze-then-kill":
        this.count("frozen");
        await freeze("pause");
        await sleep(Math.max(downMs, SILENCE_MS + 5_000));
        await this.expectSilentSocketsClosed();
        await this.kill();
        await sleep(downMs / 2);
        break;
      case "fuzz":
        for (let step = 0; step < 6; step++) {
          const action = this.pick(["kill", "revive", "offline", "online", "hide", "show", "sleep", "sleep", "traffic", ...(FREEZING ? (["pause", "resume"] as const) : [])] as const);
          if (action === "pause" || action === "resume") await freeze(action);
          else if (action === "kill") await this.kill();
          else if (action === "revive") await this.revive();
          else if (action === "offline") await this.network(this.some(0.5), true);
          else if (action === "online") await this.network(this.learners, false);
          else if (action === "hide") await this.tabs(this.some(0.5), true);
          else if (action === "show") await this.tabs(this.learners, false);
          else if (action === "traffic") void this.traffic().catch(() => undefined);
          else await sleep(this.random() * Math.min(downMs, 4_000));
        }
        await this.kill();
        await sleep(downMs);
        break;
    }
    await this.settle(index, kind, downMs);
  }

  async settle(index: number, kind: string, downMs: number): Promise<void> {
    if (FREEZING) await freeze("resume");
    await this.revive();
    await this.network(this.learners, false);
    await this.tabs(this.learners, false);
    const since = Date.now();
    const limit = RECOVERY_BOUND_MS + SLACK_MS;
    const cycle: Cycle = { index, kind, downMs, presenceMs: [], calmMs: [], failures: this.pending.splice(0) };
    await Promise.all(this.learners.map(async (learner) => {
      let presence: number | undefined;
      let calm: number | undefined;
      let last: Observation | undefined;
      while (Date.now() - since < limit && (presence === undefined || calm === undefined)) {
        last = await observe(learner.device.page);
        const elapsed = Date.now() - since;
        if (presence === undefined && last.footer && last.online === this.learners.length) presence = elapsed;
        if (calm === undefined && last.tone === "calm") calm = elapsed;
        if (presence === undefined || calm === undefined) await sleep(150);
      }
      if (presence !== undefined) cycle.presenceMs.push(presence);
      if (calm !== undefined) cycle.calmMs.push(calm);
      if (presence === undefined || calm === undefined) cycle.failures.push(`learner ${learner.index} (${learner.place}) stuck after ${Math.round((Date.now() - since) / 1000)} s: presence ${presence ?? "never"}, calm ${calm ?? "never"}, last ${JSON.stringify(last)}, sockets ${JSON.stringify(learner.sockets.slice(-8))}`);
    }));
    this.cycles.push(cycle);
    console.log(`[reconnect] cycle ${index} ${kind} down ${downMs} ms: presence ${Math.max(0, ...cycle.presenceMs)} ms, calm ${Math.max(0, ...cycle.calmMs)} ms, stuck ${cycle.failures.length}`);
  }
}
//#endregion 🌪️Storm

test("every page gets its presence footer and a calm connection indicator back after every proctor kill", async ({ device }, info) => {
  test.setTimeout(3 * 60 * 60_000);
  const started = Date.now();
  const random = generator(SEED);
  const kinds: Locale[] = ["en", "de"];
  const learners = await Promise.all(Array.from({ length: LEARNERS }, async (_, index) => arrange(index, await device(kinds[index % 2]!), started)));
  for (const learner of learners) {
    await expect(learner.device.page.locator("[data-presence-footer] [data-presence-status]")).toBeVisible({ timeout: 30_000 });
    await expect(learner.device.page.locator("header [data-tone]")).toHaveAttribute("data-tone", "calm", { timeout: 30_000 });
  }

  const storm = new Storm(learners, random);
  const offset = Math.floor(random() * DOWNTIMES_MS.length);
  const downs = Array.from({ length: CYCLES }, (_, index) => DOWNTIMES_MS[(index + offset) % DOWNTIMES_MS.length]!);
  const runner = learners.find((learner) => learner.place === "run-idle");
  const conduct = async (): Promise<void> => {
    for (const [index, downMs] of downs.entries()) await storm.cycle(index, downMs);
    if (runner === undefined) return;
    await answerRun(runner.device, QUIZ, "perfect");
    await expect(primary(screen(runner.device.page, "run"))).toBeEnabled();
    await primary(screen(runner.device.page, "run")).click();
    const confirm = primary(runner.device.page.getByRole("alertdialog"));
    await expect(confirm).toBeVisible();
    await storm.killOnNextCommand(() => confirm.click(), "submit-run");
    await sleep(2_000);
    await storm.settle(CYCLES, "submit", 2_000);
    await expect(screen(runner.device.page, "results")).toBeVisible({ timeout: RECOVERY_BOUND_MS * 2 });
  };
  const worst = (pick: (cycle: Cycle) => number[]): number => Math.max(0, ...storm.cycles.flatMap(pick));
  const write = (): void => {
    const report = { seed: SEED, learners: LEARNERS, boundMs: RECOVERY_BOUND_MS, counts: storm.counts, worstPresenceMs: worst((cycle) => cycle.presenceMs), worstCalmMs: worst((cycle) => cycle.calmMs), cycles: storm.cycles };
    if (REPORT !== "") {
      mkdirSync(dirname(REPORT), { recursive: true });
      writeFileSync(REPORT, JSON.stringify(report, null, 1));
    }
    info.annotations.push({ type: "worst", description: `presence ${report.worstPresenceMs} ms, calm ${report.worstCalmMs} ms of bound ${RECOVERY_BOUND_MS} ms` });
    console.log(`[reconnect] seed ${SEED}: worst presence ${report.worstPresenceMs} ms, worst calm ${report.worstCalmMs} ms, bound ${RECOVERY_BOUND_MS} ms, counts ${JSON.stringify(storm.counts)}`);
  };
  try {
    await learners.map((learner) => learner.device).reduceRight<() => Promise<void>>((inner, tolerant) => async () => void (await tolerant.expectingFailures(inner)), conduct)();
  } finally {
    write();
  }
  expect(storm.cycles.flatMap((cycle) => cycle.failures.map((failure) => `cycle ${cycle.index} ${cycle.kind} down ${cycle.downMs} ms: ${failure}`))).toEqual([]);
});
