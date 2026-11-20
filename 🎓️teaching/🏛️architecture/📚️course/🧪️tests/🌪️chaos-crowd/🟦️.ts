/** 🌪️ A whole class at once, with the network and the proctor misbehaving: real browser learners (as many as the machine sustains, `CHAOS_BROWSERS`) and lightweight protocol learners (`CHAOS_PROTOCOL`) that speak the site's own client protocol add up to about a hundred simultaneous learners. Every browser learner plays every quiz of the course end to end while the network drops and returns at random moments in the middle of an answer or a submission, pages reload, tabs close and reopen, the proctor is killed and started again, and the requests of the page slow down. Afterwards every learner's truth is read back from the proctor itself: each quiz was submitted and scored exactly once, the totals are exact, the leaderboard ranks everyone with the right total, and the presence counts converge on how many are online.
 * A hundred real browsers (`CHAOS_PROTOCOL=0 CHAOS_BROWSERS=100`) is a mode of this spec: `CHAOS_CONTEXTS_PER_BROWSER` spreads the learners over Chromium processes of that many contexts each, `CHAOS_ARRIVAL_S` is the time the class takes to arrive, `CHAOS_BROWSER_QUIZZES=1` gives every browser one quiz (spread evenly over the course; `CHAOS_QUIZZES` still sets how many quizzes the protocol learners play), `CHAOS_CALM=1` switches the learners' pets, icon animations and shared cursors off through the app's own preferences and asks for reduced motion, `CHAOS_VIEWPORT=WxH` makes the windows small, `CHAOS_PLAY_SLOTS=N` keeps every browser connected until the whole class has arrived and counts everyone online at once, and only then lets at most N of them play at the same moment, and `CHAOS_SAMPLE_S` is how often the memory, the connected browsers and the presence count are sampled.
 * The stack comes from the gate (`TEACHING_ARCHITECTURE_COURSE_E2E_*`); a run writes its timings and counters to `CHAOS_REPORT` (a JSON file path) when set.
 * @see ./🥒️.feature — the scenarios in words
 * @see ./🤖️protocol-learner/🟦️.ts — the learners without a browser
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner journey the browsers follow */
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { freemem, totalmem } from "node:os";
import { chromium, type Browser, type BrowserContext, type Page } from "@playwright/test";
import { ProctorClient, proctorTransport } from "../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts";
import { COURSE, PAR, QUIZZES, activeOverview, arrive, authorPreview, card, connection, count, expect, goHome, learnerId, way, openTask, answerTask, pane, playQuiz, primary, proctor, readIntroduction, scoredPoints, percent, screen, submitRun, test, PROCTOR_ORIGIN, type Challenge, type Device, type Locale, type SourceQuiz } from "../../🎭️e2e/🚶️learner/🟦️.ts";
import { learnerTag, rosterScope } from "../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts";
import { ghost, rosterAudit, type Vanish, ProtocolLearner } from "./🤖️protocol-learner/🟦️.ts";

//#region 🎚️Settings
const BROWSERS = Number(process.env.CHAOS_BROWSERS ?? 24);
const PROTOCOL = Number(process.env.CHAOS_PROTOCOL ?? 76);
const QUIZ_LIMIT = Number(process.env.CHAOS_QUIZZES ?? QUIZZES.length);
const BROWSER_QUIZZES = Number(process.env.CHAOS_BROWSER_QUIZZES ?? QUIZ_LIMIT);
const CHAOS = Number(process.env.CHAOS_INTENSITY ?? 1);
const REPORT = process.env.CHAOS_REPORT ?? "";
const CONTEXTS_PER_BROWSER = Number(process.env.CHAOS_CONTEXTS_PER_BROWSER ?? 0);
const ARRIVAL_MS = Number(process.env.CHAOS_ARRIVAL_S ?? 0) * 1_000;
const CALM = process.env.CHAOS_CALM === "1";
const MAX_BROWSERS_MB = Number(process.env.CHAOS_MAX_BROWSERS_MB ?? 0);
const RETRIES = Number(process.env.CHAOS_RETRIES ?? 8);
const PATIENCE_MS = Number(process.env.CHAOS_ADMIT_PATIENCE_S ?? 0) * 1_000;
const BOOT_SLOTS = Number(process.env.CHAOS_BOOT_SLOTS ?? 0);
const SAMPLE_MS = Number(process.env.CHAOS_SAMPLE_S ?? 30) * 1_000;
const VIEWPORT = /^(\d+)x(\d+)$/u.exec(process.env.CHAOS_VIEWPORT ?? "");
const KILLS = Number(process.env.CHAOS_KILLS ?? 3);
const PLAY_SLOTS = Number(process.env.CHAOS_PLAY_SLOTS ?? 0);
const run = promisify(execFile);
const sleep = (milliseconds: number): Promise<void> => new Promise((done) => setTimeout(done, milliseconds));
const between = (low: number, high: number): number => low + Math.random() * (high - low);
const shuffled = <T>(items: readonly T[]): T[] => items.map((item) => [Math.random(), item] as const).sort((left, right) => left[0] - right[0]).map(([, item]) => item);
const stamp = (): number => Math.round(performance.now());

const PRESENCE_CONFIG = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/👥️presence/🦀️.rs");

/** ⏱️ How long the proctor lets a presence socket stay silent before it closes it, and how often it pings: read from the proctor's own presence configuration (`idle` and `keepalive`), never assumed. */
function presenceTimeouts(): { readonly idleS: number; readonly keepaliveS: number } {
  const source = readFileSync(PRESENCE_CONFIG, "utf8");
  const idle = /idle:\s*Duration::from_secs\((\d+)\)/u.exec(source);
  const keepalive = /keepalive:\s*Duration::from_secs\((\d+)\)/u.exec(source);
  if (idle === null || keepalive === null) throw new Error("the proctor's presence configuration names no idle and keepalive timeouts");
  return { idleS: Number(idle[1]), keepaliveS: Number(keepalive[1]) };
}
const GHOSTS = Number(process.env.CHAOS_GHOSTS ?? 2);
//#endregion 🎚️Settings

//#region 📒️Ledger
/** 📒️ What the run counted: every chaos event injected, by kind, and what the learners measured. */
interface Ledger {
  readonly events: Record<string, number>;
  readonly timings: { learner: string; quiz: string; challenge: Challenge; ms: number }[];
  readonly memory: { atMs: number; freeMb: number; freePct: number; browsersMb: number; renderers: number; proctorMb: number; runnerMb: number; connected: number; presence: number }[];
  readonly latency: { arrive: number[]; quizStart: number[]; submit: number[]; submitAll: number[] };
  readonly notes: string[];
}

const ledger: Ledger = { events: {}, timings: [], memory: [], latency: { arrive: [], quizStart: [], submit: [], submitAll: [] }, notes: [] };
const stage = (name: string): void => console.log(`[chaos] stage ${name}`);
const percentile = (values: readonly number[], fraction: number): number | undefined => [...values].sort((left, right) => left - right)[Math.min(values.length - 1, Math.floor(values.length * fraction))];
const spread = (values: readonly number[]): { count: number; p50?: number; p95?: number; max?: number } => ({ count: values.length, p50: percentile(values, 0.5), p95: percentile(values, 0.95), max: percentile(values, 1) });
const inject = (kind: string): void => void (ledger.events[kind] = (ledger.events[kind] ?? 0) + 1);

/** 🌡️ The resident memory, in megabytes, of every browser process this runner started (and how many of them render pages), of the proctor, and of this runner; and the share of memory macOS still calls free. */
function processes(): { browsersMb: number; renderers: number; proctorMb: number; runnerMb: number; freePct: number } {
  if (Date.now() - lastProcesses.at >= 4_000 && !measuring) {
    measuring = true;
    void measuredProcesses().then((value) => void (lastProcesses = { at: Date.now(), value })).finally(() => void (measuring = false));
  }
  return lastProcesses.value;
}
let measuring = false;
let lastProcesses: { at: number; value: Awaited<ReturnType<typeof measuredProcesses>> } = { at: 0, value: { browsersMb: 0, renderers: 0, proctorMb: 0, runnerMb: 0, freePct: -1 } };

async function measuredProcesses(): Promise<{ browsersMb: number; renderers: number; proctorMb: number; runnerMb: number; freePct: number }> {
  const mb = (rows: readonly { rss: number }[]): number => Math.round(rows.reduce((sum, row) => sum + row.rss, 0) / 1024);
  try {
    const rows = (await run("ps", ["-axo", "pid=,ppid=,rss=,command="], { maxBuffer: 64 << 20 })).stdout.split("\n").flatMap((line) => {
      const found = /^\s*(\d+)\s+(\d+)\s+(\d+)\s+(.*)$/u.exec(line);
      return found === null ? [] : [{ pid: Number(found[1]), parent: Number(found[2]), rss: Number(found[3]), command: found[4]! }];
    });
    const mine = new Set<number>([process.pid]);
    for (let grew = true; grew;) {
      grew = false;
      for (const row of rows) if (!mine.has(row.pid) && mine.has(row.parent)) (mine.add(row.pid), (grew = true));
    }
    const browsers = rows.filter((row) => mine.has(row.pid) && row.pid !== process.pid);
    const owner = new Set((await run("lsof", ["-t", `-iTCP:${new URL(PROCTOR_ORIGIN).port}`, "-sTCP:LISTEN"]).catch(() => ({ stdout: "" }))).stdout.split("\n").filter(Boolean).map(Number));
    const pressure = process.platform === "darwin" ? /(\d+)%/u.exec((await run("memory_pressure", ["-Q"])).stdout) : null;
    return { browsersMb: mb(browsers), renderers: browsers.filter((row) => /--type=renderer/u.test(row.command)).length, proctorMb: mb(rows.filter((row) => owner.has(row.pid))), runnerMb: Math.round(process.memoryUsage().rss / 1048576), freePct: pressure === null ? -1 : Number(pressure[1]) };
  } catch {
    return { browsersMb: -1, renderers: -1, proctorMb: -1, runnerMb: -1, freePct: -1 };
  }
}

/** 🛂️ Whether the proctor is up as far as the chaos conductor knows; learners wait on it before they trust that everything is saved. */
const proctorState = { up: true, downSince: 0 };
//#endregion 📒️Ledger

//#region 🧑‍🎓️Browser learner
const OUTBOX = `semio.course.${COURSE.id}.outbox/`;

/** 🧑‍🎓️ One real browser learner: a device in the sense of the learner journey, whose network, tabs and page the chaos acts on. Console and request noise is a problem only outside a shortage — the learner's own offline window, an injected slowdown, or the proctor being away, plus a grace of a few seconds after each — while a page error and a violation of the content security policy are problems always. */
class Chaotic implements Device {
  context!: BrowserContext;
  page!: Page;
  readonly problems: string[] = [];
  readonly offlineMarks: number[] = [];
  readonly counts = { offline: 0, reload: 0, closed: 0, slow: 0, offlineAnswers: 0, offlineSubmits: 0 };
  readonly plan: SourceQuiz[];
  id = "";
  live = false;
  skipped = false;
  epoch = 0;
  private recovering = 0;
  private shortage = 0;
  private latency = 0;
  private offline = false;
  private disturbedAt = 0;
  private alive = true;
  private background: Promise<void> = Promise.resolve();
  constructor(readonly index: number, readonly locale: Locale, readonly handle: string, private readonly browser: () => Promise<Browser>, private readonly viewport: { width: number; height: number }) {
    this.plan = Array.from({ length: BROWSER_QUIZZES }, (_, place) => QUIZZES[(index + place) % QUIZZES.length]!);
  }

  get quiet(): boolean {
    return !this.tolerated() && !this.disturbed && this.recovering === 0;
  }

  expectingFailures = async (body: () => Promise<void>): Promise<number> => {
    this.shortage += 1;
    try {
      await body();
    } finally {
      this.shortage -= 1;
    }
    return 0;
  };

  /** 📣️ Failures that only a learner's own injected offline window or the proctor's restart can cause, however late a starved page reports them: the network emulation's disconnect error, and the proctor's "does not exist (yet)" answer to a view whose commands the device has not delivered yet (the page retries it through the projection lag). */
  private announced(line: string): boolean {
    return line.includes("net::ERR_INTERNET_DISCONNECTED") || (/\b404\b/u.test(line) && line.includes("/queries"));
  }

  private tolerated(): boolean {
    return this.offline || this.shortage > 0 || this.latency > 0 || !proctorState.up || Date.now() - Math.max(this.offlineEndedAt, proctorState.downSince) < 25_000;
  }
  private offlineEndedAt = 0;
  readonly inflight = new Map<unknown, string>();
  readonly sockets: string[] = [];
  private roster = { opened: 0, welcomed: 0, states: 0, lastState: 0, refused: [] as string[] };
  private readonly bornAt = Date.now();

  /** 🧾️ What the page's latest roster socket did: when it opened and was welcomed, how many states it sent and when the last one, and what the proctor refused. */
  get rosterTrail(): string {
    const { opened, welcomed, states, lastState, refused } = this.roster;
    return `roster socket +${opened}s, welcomed ${welcomed === 0 ? "never" : `+${welcomed}s`}, ${states} states sent (last +${lastState}s), refused [${refused.join(", ")}]`;
  }

  private watch(page: Page): void {
    const report = (line: string): void => void (this.tolerated() || ((this.counts.offline > 0 || ledger.events["proctor-killed"] !== undefined) && this.announced(line)) ? (ledger.events["tolerated-noise"] = (ledger.events["tolerated-noise"] ?? 0) + 1) : this.problems.push(line));
    page.on("console", (message) => message.type() === "error" && report(`console error: ${message.text().slice(0, 300)} (${message.location().url})`));
    page.on("pageerror", (error) => this.problems.push(`page error: ${error.message}`));
    page.on("requestfailed", (request) => (request.failure()?.errorText ?? "") !== "net::ERR_ABORTED" && report(`request failed: ${request.method()} ${request.url()} ${request.failure()?.errorText}`));
    page.on("response", (response) => response.status() >= 400 && report(`request answered ${response.status()}: ${response.request().method()} ${response.url()}`));
    page.on("request", (request) => this.inflight.set(request, `${Math.round(performance.now())} ${request.method()} ${request.url().slice(-90)}`));
    page.on("requestfinished", (request) => this.inflight.delete(request));
    page.on("requestfailed", (request) => this.inflight.delete(request));
    page.on("websocket", (socket) => {
      const opened = Math.round((Date.now() - this.bornAt) / 1000);
      const scope = decodeURIComponent(new URL(socket.url()).pathname).replace(/\/presence\/ws$/u, "").slice(-30);
      this.sockets.push(`+${opened}s open ${scope}`);
      if (scope.endsWith(`/scopes/${rosterScope(COURSE.id)}`)) {
        const trail = { opened, welcomed: 0, states: 0, lastState: 0, refused: [] as string[] };
        this.roster = trail;
        const at = (): number => Math.round((Date.now() - this.bornAt) / 1000);
        socket.on("framesent", ({ payload }) => String(payload).startsWith('{"type":"state"') && ((trail.states += 1), (trail.lastState = at())));
        socket.on("framereceived", ({ payload }) => {
          const text = String(payload);
          if (text.startsWith('{"type":"welcome"')) trail.welcomed = at();
          else if (text.startsWith('{"type":"refused"')) trail.refused.push(`+${at()}s ${(JSON.parse(text) as { reason: string }).reason}`);
        });
      }
      socket.on("close", () => this.sockets.push(`+${Math.round((Date.now() - this.bornAt) / 1000)}s closed ${scope}`));
      socket.on("socketerror", (error) => this.sockets.push(`+${Math.round((Date.now() - this.bornAt) / 1000)}s error ${scope} ${String(error).slice(0, 60)}`));
    });
    page.on("crash", () => this.problems.push("page crashed"));
  }

  async start(): Promise<void> {
    const browser = await this.browser();
    this.context = await browser.newContext({ viewport: this.viewport, locale: this.locale === "en" ? "en-GB" : "de-DE", baseURL: process.env.PLAYWRIGHT_BASE_URL, ...(CALM ? { reducedMotion: "reduce" as const, serviceWorkers: "block" as const } : {}) });
    if (CALM) await this.context.addInitScript((key) => {
      try {
        const stored = JSON.parse(localStorage.getItem(key) ?? "{}") as { petsChosen?: boolean };
        if (stored.petsChosen !== true) localStorage.setItem(key, JSON.stringify({ ...stored, pets: "off", petsChosen: true, animateIcons: false, iconsChosen: true, showCursors: false }));
      } catch {}
    }, `semio.course.${COURSE.id}.preferences`);
    await this.context.exposeBinding("courseSecurityViolation", (_source, violation: string) => void this.problems.push(`content security policy: ${violation}`));
    await this.context.addInitScript(() => document.addEventListener("securitypolicyviolation", (event) => (window as unknown as { courseSecurityViolation(violation: string): void }).courseSecurityViolation(`${event.effectiveDirective} blocked ${event.blockedURI || "inline code"}`)));
    await this.context.route(/\/(commands|queries)$/u, async (route) => {
      if (this.latency > 0) await sleep(this.latency * between(0.5, 1.5));
      await route.continue().catch(() => undefined);
    });
    this.page = await this.context.newPage();
    this.watch(this.page);
    const arriving = performance.now();
    await arrive(this, 120_000);
    ledger.latency.arrive.push(Math.round(performance.now() - arriving));
    await readIntroduction(this);
    const form = screen(this.page, "identity");
    await expect(async () => {
      await form.locator('input[type="radio"][value="pseudonym"]').check({ timeout: 10_000 });
      await form.locator('input[type="text"]').fill(this.handle, { timeout: 10_000 });
    }).toPass({ timeout: 150_000, intervals: [500, 1_000, 2_000] }).catch(async (error: unknown) => {
      const state = await this.page.evaluate(() => {
        const radio = document.querySelector<HTMLInputElement>('input[type="radio"][value="pseudonym"]');
        const box = radio?.getBoundingClientRect();
        const top = box === undefined ? null : document.elementFromPoint(box.x + box.width / 2, box.y + box.height / 2);
        return JSON.stringify({ disabled: radio?.disabled, checked: radio?.checked, box: box === undefined ? null : [box.x, box.y, box.width, box.height], top: top === null ? null : `${top.tagName}.${top.className}`.slice(0, 120), visibility: document.visibilityState, focus: document.hasFocus(), text: document.body.innerText.slice(0, 160), dialogs: document.querySelectorAll("[role=dialog]").length });
      }, undefined, { timeout: 5_000 }).catch((failure: unknown) => `page unresponsive: ${String(failure).slice(0, 80)}`);
      throw new Error(`${this.handle} identity step: ${state} | ${error instanceof Error ? error.message.split("\n")[0] : ""}`);
    });
    await primary(form).click({ noWaitAfter: true });
    await expect(activeOverview(this.page)).toBeVisible();
    await expect.poll(() => learnerId(this), { message: "the identified learner is kept on the device", timeout: 120_000 }).not.toBe("");
    this.id = await learnerId(this);
    await authorPreview(this);
    this.live = true;
    if (PLAY_SLOTS === 0) this.disturb();
  }

  /** ⛈️ Starts this learner's own disturbances (see {@link chaos}). */
  disturb(): void {
    this.background = this.chaos();
  }

  /** 🌩️ The random disturbances of this learner's own network, tabs and page, one after the other, until it stops. */
  private async chaos(): Promise<void> {
    await sleep(between(10_000, 30_000) / CHAOS);
    while (this.alive) {
      const roll = Math.random();
      try {
        if (roll < 0.5) await this.goOffline(between(2_000, 12_000));
        else if (roll < 0.75) await this.slowDown(between(8_000, 20_000), between(400, 2_500));
        else if (roll < 0.9) await this.reload();
        else await this.reopenTab();
      } catch (error) {
        if (this.alive) ledger.notes.push(`${this.handle}: chaos step failed: ${error instanceof Error ? error.message.split("\n")[0] : String(error)}`);
      }
      await sleep(between(15_000, 50_000) / CHAOS);
    }
  }

  async goOffline(forMs: number): Promise<void> {
    this.offline = true;
    this.counts.offline += 1;
    inject("offline");
    try {
      await this.context.setOffline(true);
      await sleep(forMs);
    } finally {
      this.offline = false;
      this.offlineEndedAt = Date.now();
      await this.context.setOffline(false).catch(() => undefined);
    }
  }

  private async slowDown(forMs: number, latency: number): Promise<void> {
    this.latency = latency;
    this.counts.slow += 1;
    inject("slow-network");
    await sleep(forMs);
    this.latency = 0;
    this.offlineEndedAt = Date.now();
  }

  private async reload(): Promise<void> {
    if (this.offline || this.recovering > 0) return;
    this.counts.reload += 1;
    inject("reload");
    this.epoch += 1;
    this.disturbedAt = Date.now();
    await this.page.reload({ waitUntil: "domcontentloaded" }).catch(() => undefined);
  }

  private async reopenTab(): Promise<void> {
    if (this.offline || this.recovering > 0) return;
    this.counts.closed += 1;
    inject("tab-closed-reopened");
    this.epoch += 1;
    this.disturbedAt = Date.now();
    const gone = this.page;
    this.page = await this.context.newPage();
    this.watch(this.page);
    await gone.close({ runBeforeUnload: false }).catch(() => undefined);
    await sleep(between(500, 3_000));
    await this.page.goto(`/?preview=1`, { waitUntil: "domcontentloaded" }).catch(() => undefined);
  }

  get isOffline(): boolean {
    return this.offline;
  }

  get disturbed(): boolean {
    return Date.now() - this.disturbedAt < 45_000;
  }

  /** 💾️ How many commands wait on the device for the proctor. */
  pending(): Promise<number> {
    return this.page.evaluate((prefix) => Object.keys(localStorage).filter((key) => key.startsWith(prefix)).length, OUTBOX).catch(() => -1);
  }

  /** 🔌️ Waits until the network is back, the proctor is up, and the connection indicator says everything is saved; returns how long that took. */
  async settle(limit = 150_000): Promise<number> {
    const started = Date.now();
    for (let waited = 0; (this.offline || !proctorState.up) && waited < limit; waited += 250) await sleep(250);
    await expect(connection(this)).toHaveAttribute("data-tone", "calm", { timeout: limit });
    await expect.poll(() => this.pending(), { timeout: limit }).toBe(0);
    return Date.now() - started;
  }

  /** 🏠️ Gets the device back onto the overview after a reload or a reopened tab. */
  async recover(): Promise<void> {
    this.recovering += 1;
    try {
      for (let attempt = 1; ; attempt++) {
        for (let waited = 0; this.offline && waited < 60_000; waited += 250) await sleep(250);
        if (this.page.isClosed()) {
          this.page = await this.context.newPage();
          this.watch(this.page);
        }
        await this.page.goto("/?preview=1", { waitUntil: "domcontentloaded" }).catch(() => undefined);
        const shown = await activeOverview(this.page).waitFor({ state: "visible", timeout: 120_000 }).then(() => true, () => false);
        if (shown) return;
        const state = `${this.handle} did not boot in attempt ${attempt} (offline ${this.offline}, proctor up ${proctorState.up}, waiting on ${[...this.inflight.values()].slice(0, 8).join(" | ")}; page: ${(await this.page.evaluate(() => document.body.innerText.slice(0, 200)).catch(() => "?")).replace(/\n/gu, " / ")})`;
        ledger.notes.push(state);
        if (attempt >= 4) throw new Error(state);
      }
    } finally {
      this.recovering -= 1;
    }
  }

  /** 🔍️ What the page says right now: its connection tone, pending commands, and the first words on it. */
  async describe(): Promise<string> {
    const tone = await connection(this).getAttribute("data-tone", { timeout: 2_000 }).catch(() => "?");
    const text = await this.page.evaluate(() => document.body.innerText.replace(/\s+/gu, " ").slice(0, 400)).catch(() => "?");
    return `${this.handle} sockets(last 12)=[${this.sockets.slice(-12).join("; ")}] ${this.rosterTrail} offline=${this.offline} proctorUp=${proctorState.up} tone=${tone} pending=${await this.pending()} inflight=[${[...this.inflight.values()].slice(0, 5).join(" | ")}] text="${text}"`;
  }

  async stop(): Promise<void> {
    this.alive = false;
    await Promise.race([this.background, sleep(5_000)]);
    await this.goOnline();
  }

  private async goOnline(): Promise<void> {
    this.offline = false;
    this.offlineEndedAt = Date.now();
    await this.context.setOffline(false).catch(() => undefined);
  }

  async close(): Promise<void> {
    this.live = false;
    await this.context?.close().catch(() => undefined);
  }
}
//#endregion 🧑‍🎓️Browser learner

//#region 🎬️Journey
const challengeOf = (learner: Chaotic, place: number): Challenge => (learner.index % 2 === 1 && place % 3 === 0 ? "easy" : "medium");

/** ✂️ Runs `step`, giving it up as soon as a reload or a reopened tab took its page away, instead of letting it wait out its long timeouts on a page that no longer shows what it acts on. */
async function untilReloaded<T>(learner: Chaotic, step: () => Promise<T>): Promise<T> {
  const epoch = learner.epoch;
  let timer: ReturnType<typeof setInterval> | undefined;
  const taken = new Promise<never>((_, reject) => void (timer = setInterval(() => learner.epoch !== epoch && reject(new Error("a reload or a reopened tab took the page away")), 500)));
  const running = step();
  running.catch(() => undefined);
  try {
    return await Promise.race([running, taken]);
  } finally {
    clearInterval(timer);
  }
}

/** 🔁️ Runs `step` until it passes, recovering the page after a disturbance that interrupted it; a failure with no disturbance behind it is real. */
async function resilient<T>(learner: Chaotic, what: string, step: () => Promise<T>, recover: () => Promise<void>): Promise<T> {
  for (let attempt = 1; ; attempt++) {
    try {
      return await untilReloaded(learner, step);
    } catch (error) {
      if (attempt >= RETRIES || !(learner.disturbed || learner.isOffline || learner.counts.offline > 0 || !proctorState.up)) throw new Error(`${what}: ${error instanceof Error ? error.message.split("\n")[0] : String(error)} | ${await learner.describe()}`);
      inject("step-recovered");
      await learner.settle().catch(() => undefined);
      await recover().catch((failure: unknown) => void ledger.notes.push(`${learner.handle}: ${what} recovery interrupted: ${String(failure).split("\n")[0]}`));
    }
  }
}

/** 🏠️ Goes home unless the overview already shows and no page is open over it (the navbar's way home is disabled there). */
async function homeSafe(learner: Chaotic): Promise<void> {
  const already = (await way(learner.page, "overview").getAttribute("aria-disabled")) === "true" && (await learner.page.locator("[data-layered-pane][data-opened]").count()) === 0;
  if (!already) await goHome(learner);
  await expect(activeOverview(learner.page)).toBeVisible();
}

/** 🏃️ One quiz from the overview to the results, answered perfectly under chaos. */
async function playOne(learner: Chaotic, truth: ProctorClient, quiz: SourceQuiz, challenge: Challenge): Promise<void> {
  const started = performance.now();
  const resume = async (): Promise<void> => {
    await learner.recover();
    await playQuiz(learner, quiz.id, challenge);
  };
  const opening = performance.now();
  const quietBefore = learner.quiet;
  await resilient(learner, `${quiz.id} start`, () => playQuiz(learner, quiz.id, challenge), resume);
  if (quietBefore && learner.quiet) ledger.latency.quizStart.push(Math.round(performance.now() - opening));
  for (let index = 0; index < quiz.tasks.length; index++) {
    await resilient(learner, `${quiz.id} task ${index + 1}`, async () => {
      if (!(await screen(learner.page, "run").isVisible())) await resume();
      await openTask(learner, index);
      const offline = await learner.page.evaluate(() => !navigator.onLine);
      await answerTask(learner, quiz, "perfect");
      if (offline && (await learner.page.evaluate(() => !navigator.onLine))) {
        learner.counts.offlineAnswers += 1;
        expect(await learner.pending(), `${learner.handle}: an answer made offline waits on the device`).toBeGreaterThan(0);
      }
      await expect(screen(learner.page, "run").locator("nav button").nth(index).locator("[data-complete]")).toBeVisible();
    }, resume);
  }
  const submittedByTruth = async () => (await truth.learner(learner.id)).runs.find((run) => run.quiz === quiz.id && run.status === "submitted");
  let results: { score: number; points: number } | undefined;
  for (let attempt = 1; results === undefined; attempt++) {
    try {
      if (!(await screen(learner.page, "run").isVisible())) await resume();
      if (Math.random() < 0.35 && !learner.disturbed) {
        learner.counts.offlineSubmits += 1;
        inject("offline-mid-submit");
        void learner.goOffline(between(3_000, 9_000));
        await sleep(300);
      }
      const quietSubmit = attempt === 1 && learner.quiet;
      const submitting = performance.now();
      await submitRun(learner);
      const took = Math.round(performance.now() - submitting);
      ledger.latency.submitAll.push(took);
      if (quietSubmit && learner.quiet) ledger.latency.submit.push(took);
      results = { score: percent(await screen(learner.page, "results").locator("p").first().innerText()), points: scoredPoints(learner, await screen(learner.page, "results").locator("p").nth(1).innerText()).points };
    } catch (error) {
      if (attempt >= 5) throw new Error(`${quiz.id} submit: ${error instanceof Error ? error.message : String(error)}`);
      inject("submit-recovered");
      const drained = await learner.settle(300_000).then(() => true, () => false);
      const view = await submittedByTruth().catch(() => undefined);
      if (view !== undefined) {
        results = { score: Math.round((view.score ?? 0) * 100), points: view.points ?? 0 };
      } else if (!drained && performance.now() - started < 1_800_000) {
        attempt -= 1;
      } else await resume().catch(() => undefined);
    }
  }
  if (results.points !== PAR[challenge]) {
    const shown = async (): Promise<number> => ((await screen(learner.page, "results").isVisible()) ? scoredPoints(learner, await screen(learner.page, "results").locator("p").nth(1).innerText()).points : (await submittedByTruth())?.points ?? -1);
    const first = results.points;
    await expect.poll(() => shown().catch(() => -1), { timeout: 90_000, intervals: [1_000, 2_000, 5_000], message: `${learner.handle}: ${quiz.id} earns every point of ${challenge} (first shown ${first})` }).toBe(PAR[challenge]);
    ledger.notes.push(`${learner.handle}: ${quiz.id} first showed ${first} points of ${PAR[challenge]} and caught up`);
    results.points = PAR[challenge];
  }
  expect(results.points, `${learner.handle}: ${quiz.id} earns every point of ${challenge}`).toBe(PAR[challenge]);
  ledger.timings.push({ learner: learner.handle, quiz: quiz.id, challenge, ms: Math.round(performance.now() - started) });
  await resilient(learner, `${quiz.id} home`, () => homeSafe(learner), () => learner.recover());
}

/** 🧑‍🎓️ The whole course of one browser learner. */
async function attendBrowser(learner: Chaotic, truth: ProctorClient, finished: () => void): Promise<void> {
  for (const [place, quiz] of learner.plan.entries()) {
    await playOne(learner, truth, quiz, challengeOf(learner, place));
    finished();
  }
  await learner.stop();
}
//#endregion 🎬️Journey

//#region 🧮️Truth
const shuffledPlan = (ids: readonly string[]): SourceQuiz[] => ids.map((id) => QUIZZES.find((quiz) => quiz.id === id)!);

/** 🏆️ What a browser shows of the learner's own leaderboard row. */
async function boardTotal(learner: Chaotic): Promise<number> {
  await recoverToOverview(learner);
  await primary(card(learner.page, "board")).click();
  const board = pane(learner.page, "board");
  await expect(board).toHaveAttribute("data-opened", "");
  const own = board.locator('tr[aria-current="true"]').first();
  await expect(own).toBeVisible({ timeout: 60_000 });
  return Number((await own.locator("td, th").nth(2).innerText()).replace(/\D/gu, ""));
}

async function recoverToOverview(learner: Chaotic): Promise<void> {
  await goHome(learner).catch(() => learner.recover());
}

/** 👥️ How many learners the device counts online. */
async function online(learner: Chaotic): Promise<number> {
  const status = await learner.page.locator("[data-presence-status]").innerText({ timeout: 5_000 }).catch(() => "");
  return status === "" ? -1 : count(status);
}

/** 🎟️ At most `count` holders at once; {@link acquire} waits for a free slot, in arrival order. */
function slots(count: number): { readonly acquire: () => Promise<void>; readonly release: () => void } {
  let free = count;
  const waiting: (() => void)[] = [];
  return {
    acquire: () => (free > 0 ? ((free -= 1), Promise.resolve()) : new Promise<void>((enter) => waiting.push(enter))),
    release: () => {
      const next = waiting.shift();
      if (next === undefined) free += 1;
      else next();
    },
  };
}

/** 🔭️ Asks the proctor itself who is on the roster; set once the crowd exists. */
let audit: () => Promise<string> = async () => "";

/** 👪️ Every browser learner of the crowd; set once the crowd exists, so a failure can describe the learners the proctor misses. */
let everyBrowser: readonly Chaotic[] = [];

/** 🔬️ Polls how many learners `learner`'s device counts online until it is `wanted`, and names the device's whole state when it never is. */
async function expectOnline(learner: Chaotic, wanted: number, what: string, timeout = 90_000): Promise<void> {
  try {
    await expect.poll(() => online(learner), { timeout }).toBe(wanted);
  } catch (error) {
    const roster = await audit().catch((failure) => `roster audit failed: ${String(failure)}`);
    const missing = everyBrowser.filter((other) => other !== learner && roster.includes(other.handle));
    const theirs = await Promise.all(missing.map(async (other) => `missing ${await other.describe()} counts ${await online(other)}`));
    throw new Error(`${learner.handle}: ${what}: counts ${await online(learner)}, wanted ${wanted} | ${await learner.describe()} | ${roster} | ${theirs.join(" | ")} | ${error instanceof Error ? error.message.split("\n")[0] : ""}`);
  }
}
//#endregion 🧮️Truth

test("a crowd of about a hundred learners plays every quiz while the network and the proctor fail", async ({ browser }, info) => {
  test.setTimeout(4 * 60 * 60_000);
  const viewport = VIEWPORT === null ? info.project.use.viewport ?? { width: 1440, height: 900 } : { width: Number(VIEWPORT[1]), height: Number(VIEWPORT[2]) };
  const launched: Promise<Browser>[] = [];
  const browserFor = (index: number): (() => Promise<Browser>) => async () => {
    if (CONTEXTS_PER_BROWSER <= 0) return browser;
    const slot = Math.floor(index / CONTEXTS_PER_BROWSER);
    launched[slot] ??= chromium.launch({ headless: true, args: ["--disable-gpu", "--disable-dev-shm-usage", "--mute-audio", "--no-first-run"] });
    return launched[slot]!;
  };
  const truth = new ProctorClient(proctorTransport(PROCTOR_ORIGIN), COURSE.id);
  const quizzes = QUIZZES.slice(0, QUIZ_LIMIT);
  const before = await truth.leaderboard({ period: "all-time" }, undefined);
  const base = { learners: before.learners, submissions: before.submissions };
  const give = new AbortController();
  const started = Date.now();

  const learners = Array.from({ length: BROWSERS }, (_, index) => new Chaotic(index, index % 2 === 0 ? "en" : "de", `Chaos ${index} ${Math.random().toString(36).slice(2, 7)}`, browserFor(index), viewport));
  const bots = Array.from({ length: PROTOCOL }, (_, index) => new ProtocolLearner(PROCTOR_ORIGIN, COURSE.id, index, shuffled(quizzes), { lostRequest: 0.06, lostAnswer: 0.06 }, give.signal));
  everyBrowser = learners;
  audit = () => rosterAudit(PROCTOR_ORIGIN, COURSE.id, new Map([...learners.filter((learner) => learner.id !== "").map((learner): [string, string] => [learnerTag(learner.id), learner.handle]), ...bots.map((bot): [string, string] => [learnerTag(bot.account.learner), `bot ${bot.index}`])]));
  let presenceSeen = 0;
  let booting = 0;
  const admit = async (index: number): Promise<(() => void) | undefined> => {
    const waitedFrom = Date.now();
    for (;;) {
      if (PATIENCE_MS > 0 && Date.now() - waitedFrom > PATIENCE_MS) return undefined;
      const slotFree = BOOT_SLOTS <= 0 || booting < BOOT_SLOTS;
      const memoryFree = MAX_BROWSERS_MB <= 0 || processes().browsersMb < MAX_BROWSERS_MB || learners.every((learner) => !learner.live);
      if (slotFree && memoryFree) break;
      await sleep(1_000 + Math.random() * 1_000);
    }
    booting += 1;
    if (Date.now() - waitedFrom > 4_000) ledger.notes.push(`learner ${index} waited ${Math.round((Date.now() - waitedFrom) / 1000)} s for a boot slot (${BOOT_SLOTS}) and browsers below ${MAX_BROWSERS_MB} MB, with ${learners.filter((learner) => learner.live).length} connected`);
    return () => void (booting -= 1);
  };
  const sampler = setInterval(() => {
    const connected = learners.filter((learner) => learner.live).length;
    const sample = { atMs: Date.now() - started, freeMb: Math.round(freemem() / 1048576), ...processes(), connected, presence: presenceSeen };
    ledger.memory.push(sample);
    console.log(`[chaos] t=${Math.round(sample.atMs / 1000)}s connected ${connected}; browser quizzes done ${done}/${total}; bots done ${bots.filter((bot) => bot.finished).length}/${PROTOCOL}; offline now ${learners.filter((learner) => learner.isOffline).length}; proctor up ${proctorState.up}; presence seen ${presenceSeen}; free ${sample.freePct}%; browsers ${sample.browsersMb} MB in ${sample.renderers} renderers; proctor ${sample.proctorMb} MB`);
  }, SAMPLE_MS);
  const watcherLoop = setInterval(() => {
    const watcher = learners.find((learner) => learner.live && !learner.isOffline);
    if (watcher !== undefined) void online(watcher).then((seen) => void (presenceSeen = Math.max(presenceSeen, seen)), () => undefined);
  }, 5_000);

  let done = 0;
  let arrivals = 0;
  let arrived!: () => void;
  const everyoneArrived = new Promise<void>((accept) => {
    arrived = () => void (++arrivals >= BROWSERS && accept());
    if (BROWSERS === 0) accept();
  });
  const playing = slots(PLAY_SLOTS);
  const assembled = PLAY_SLOTS > 0 ? everyoneArrived.then(async () => {
    const attending = learners.filter((learner) => !learner.skipped);
    stage(`${attending.length} of ${BROWSERS} browsers arrived`);
    await Promise.all(attending.map((learner) => expectOnline(learner, attending.length + PROTOCOL, "presence counts the whole class online at once", 300_000)));
    stage(`all ${attending.length + PROTOCOL} online at once`);
    ledger.notes.push(`all ${attending.length} browsers and ${PROTOCOL} protocol learners were online at once, every browser counting them`);
  }) : Promise.resolve();
  const total = BROWSERS * BROWSER_QUIZZES;
  let crowdDone = false;
  const conductor = (async () => {
    for (let kill = 1; kill <= KILLS; kill++) {
      const threshold = Math.ceil((total * kill) / (KILLS + 1));
      while (done < threshold && !give.signal.aborted && !crowdDone) await sleep(500);
      if (crowdDone) return;
      if (give.signal.aborted) return;
      inject("proctor-killed");
      proctorState.up = false;
      proctorState.downSince = Date.now();
      await proctor("stop");
      await sleep(kill === 2 ? 1_000 : between(6_000, 14_000));
      await proctor("start");
      proctorState.up = true;
      proctorState.downSince = Date.now();
    }
  })();

  try {
    await Promise.all([
      ...learners.map(async (learner, index) => {
        await sleep(index * (ARRIVAL_MS > 0 ? ARRIVAL_MS / Math.max(1, BROWSERS) : 1_500));
        const booted = await admit(index);
        if (booted === undefined) {
          learner.skipped = true;
          inject("not-admitted-memory-ceiling");
          return arrived();
        }
        await (async () => {
          for (let attempt = 1; ; attempt++) {
            try {
              return await learner.start();
            } catch (error) {
              await learner.close();
              if (attempt >= 3) throw error;
              ledger.notes.push(`${learner.handle} did not arrive in attempt ${attempt} and arrives again: ${error instanceof Error ? error.message.split("\n")[0] : String(error)}`);
            }
          }
        })().finally(booted);
        arrived();
        if (PLAY_SLOTS > 0) {
          await assembled;
          await playing.acquire();
          learner.disturb();
        }
        try {
          await attendBrowser(learner, truth, () => void (done += 1));
        } finally {
          if (PLAY_SLOTS > 0) playing.release();
        }
      }),
      ...bots.map(async (bot) => {
        await Promise.race([everyoneArrived, sleep(300_000)]);
        await bot.play();
      }),
    ]);
    crowdDone = true;
    await conductor;
    const attending = learners.filter((learner) => !learner.skipped).length;
    learners.splice(0, learners.length, ...learners.filter((learner) => !learner.skipped));
    stage(`all done, ${attending} of ${BROWSERS} browsers attended`);
    ledger.notes.push(`all learners finished after ${Math.round((Date.now() - started) / 1000)} s`);

    for (const bot of bots) expect(bot.account.problems, `bot ${bot.index}`).toEqual([]);
    const everyone = [...learners.map((learner) => ({ id: "", name: learner.handle, expected: 0, plan: learner.plan })), ...bots.map((bot) => ({ id: bot.account.learner, name: `bot ${bot.index}`, expected: bot.account.expectedTotal, plan: shuffledPlan(bot.account.quizzes) }))];
    const expectedBrowser = (learner: Chaotic): number => learner.plan.reduce((sum, _quiz, place) => sum + PAR[challengeOf(learner, place)], 0);
    await Promise.all(learners.map((learner) => learner.settle()));
    stage("settled");
    for (const [position, learner] of learners.entries()) {
      everyone[position]!.id = learner.id;
      everyone[position]!.expected = expectedBrowser(learner);
    }

    const failures: string[] = [];
    for (const who of everyone) {
      const view = await truth.learner(who.id);
      const submitted = view.runs.filter((run) => run.status === "submitted");
      if (submitted.length !== who.plan.length) failures.push(`${who.name}: ${submitted.length} submitted runs, wanted ${who.plan.length}`);
      for (const quiz of who.plan) if (submitted.filter((run) => run.quiz === quiz.id).length !== 1) failures.push(`${who.name}: quiz ${quiz.id} was submitted ${submitted.filter((run) => run.quiz === quiz.id).length} times`);
      if (view.runs.length !== who.plan.length) failures.push(`${who.name}: ${view.runs.length} runs in all (${view.runs.map((run) => `${run.quiz}:${run.status}`).join(", ")}), wanted ${who.plan.length}`);
      if (view.total !== who.expected) failures.push(`${who.name}: total ${view.total}, wanted ${who.expected}`);
      const row = (await truth.leaderboard({ period: "all-time" }, who.id)).own;
      if (row === undefined || row.total !== who.expected) failures.push(`${who.name}: leaderboard row ${row?.total}, wanted ${who.expected}`);
    }
    const board = await truth.leaderboard({ period: "all-time" }, undefined);
    if (board.learners - base.learners !== attending + PROTOCOL) failures.push(`the leaderboard ranks ${board.learners - base.learners} new learners, wanted ${attending + PROTOCOL}`);
    const submissionsWanted = learners.reduce((sum, learner) => sum + learner.plan.length, 0) + PROTOCOL * quizzes.length;
    if (board.submissions - base.submissions !== submissionsWanted) failures.push(`the leaderboard counts ${board.submissions - base.submissions} new submissions, wanted ${submissionsWanted}`);
    ledger.notes.push(...failures);
    stage("proctor truth checked");
    expect(failures, "what the proctor holds").toEqual([]);

    await Promise.all(learners.map(async (learner) => expect(await boardTotal(learner), `${learner.handle}: the leaderboard page shows the own total`).toBe(expectedBrowser(learner))));
    stage("leaderboard pages checked");
    await Promise.all(learners.map(async (learner) => expect(connection(learner), `${learner.handle}: the connection indicator recovered | ${await learner.describe()}`).toHaveAttribute("data-tone", "calm", { timeout: 60_000 })));
    stage("connections calm");
    await Promise.all(learners.map((learner) => expectOnline(learner, attending + PROTOCOL, "presence converges on everyone online")));
    stage("presence converged");
    for (const bot of bots) bot.leave();
    await Promise.all(learners.map((learner) => expectOnline(learner, attending, "presence converges on the browsers left")));

    stage("protocol learners left");
    const { idleS, keepaliveS } = presenceTimeouts();
    const origin = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1";
    const watcher = learners[0]!;
    const vanishing: readonly Vanish[] = ["closed", "reset", "silent"];
    const release: (() => void)[] = [];
    const began = Date.now();
    for (const how of vanishing) for (let each = 0; each < GHOSTS; each++) release.push(await ghost(PROCTOR_ORIGIN, COURSE.id, how, origin));
    const silent = GHOSTS;
    await expectOnline(watcher, attending + silent, "ghosts that ended or reset their connection leave at once; the silent ones are still counted", 30_000);
    const abruptS = Math.round((Date.now() - began) / 100) / 10;
    await expectOnline(watcher, attending, `silent sockets leave within the proctor's idle timeout (${idleS} s, ping every ${keepaliveS} s)`, (idleS + keepaliveS + 30) * 1000);
    const silentS = Math.round((Date.now() - began) / 100) / 10;
    ledger.notes.push(`ghost sockets: ${GHOSTS} closed, ${GHOSTS} reset, ${GHOSTS} silent; ended/reset gone after ${abruptS} s, silent gone after ${silentS} s (proctor idle ${idleS} s, keepalive ${keepaliveS} s)`);
    ledger.events["ghost-closed-silent-converged-s"] = silentS;
    await Promise.all(learners.map((learner) => expectOnline(learner, attending, "presence holds the true count after the ghosts", 30_000)));
    release.forEach((end) => end());

    for (const learner of learners) await learner.stop();
    expect(learners.flatMap((learner) => learner.problems.map((problem) => `${learner.handle}: ${problem}`)), "page errors and unannounced failures").toEqual([]);
  } finally {
    give.abort();
    clearInterval(sampler);
    clearInterval(watcherLoop);
    await conductor.catch(() => undefined);
    if (!proctorState.up) await proctor("start").catch(() => undefined);
    for (const bot of bots) bot.leave();
    const durations = ledger.timings.map((entry) => entry.ms).sort((left, right) => left - right);
    const summary = {
      browsers: BROWSERS,
      attended: learners.length,
      protocol: PROTOCOL,
      quizzes: quizzes.length,
      runsCompleted: ledger.timings.length + PROTOCOL * quizzes.length,
      browserRuns: ledger.timings.length,
      chaos: ledger.events,
      perLearnerCounts: learners.map((learner) => ({ handle: learner.handle, ...learner.counts, problems: learner.problems.length })),
      botFaults: bots.reduce((sum, bot) => ({ lostRequests: sum.lostRequests + bot.account.injected.lostRequests, lostAnswers: sum.lostAnswers + bot.account.injected.lostAnswers, doubleSends: sum.doubleSends + bot.account.injected.doubleSends, socketDrops: sum.socketDrops + bot.account.injected.socketDrops, retried: sum.retried + bot.account.injected.retried }), { lostRequests: 0, lostAnswers: 0, doubleSends: 0, socketDrops: 0, retried: 0 }),
      quizMs: { p50: durations[Math.floor(durations.length * 0.5)], p95: durations[Math.floor(durations.length * 0.95)], max: durations[durations.length - 1] },
      memory: { totalMb: Math.round(totalmem() / 1048576), minFreeMb: Math.min(...ledger.memory.map((sample) => sample.freeMb), Infinity), minFreePct: Math.min(...ledger.memory.map((sample) => sample.freePct), Infinity), peakBrowsersMb: Math.max(...ledger.memory.map((sample) => sample.browsersMb), 0), peakRenderers: Math.max(...ledger.memory.map((sample) => sample.renderers), 0), peakProctorMb: Math.max(...ledger.memory.map((sample) => sample.proctorMb), 0), peakRunnerMb: Math.max(...ledger.memory.map((sample) => sample.runnerMb), 0) },
      peak: { connectedBrowsers: Math.max(...ledger.memory.map((sample) => sample.connected), 0), presenceSeen, browserProcesses: launched.length || 1, contextsPerBrowser: CONTEXTS_PER_BROWSER },
      latencyMs: { arrive: spread(ledger.latency.arrive), quizStart: spread(ledger.latency.quizStart), submitQuiet: spread(ledger.latency.submit), submitAll: spread(ledger.latency.submitAll) },
      elapsedS: Math.round((Date.now() - started) / 1000),
      notes: ledger.notes,
      timings: ledger.timings,
      memorySamples: ledger.memory,
    };
    if (REPORT !== "") {
      mkdirSync(dirname(REPORT), { recursive: true });
      writeFileSync(REPORT, JSON.stringify(summary, null, 1));
    }
    await Promise.all(learners.map((learner) => learner.close()));
    await Promise.all(launched.map((pending) => pending.then((started) => started.close(), () => undefined)));
    info.annotations.push({ type: "chaos", description: JSON.stringify({ ...summary, timings: undefined, memorySamples: undefined, perLearnerCounts: undefined }) });
    console.log(`[chaos] ${JSON.stringify({ ...summary, timings: undefined, memorySamples: undefined, perLearnerCounts: undefined })}`);
  }
});
