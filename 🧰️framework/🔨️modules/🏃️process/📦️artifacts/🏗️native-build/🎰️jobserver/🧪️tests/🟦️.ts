import { expect, test } from "bun:test";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { registeredToolExecutableV1 } from "../../../../🧰️tools/🏃️run/🟦️.ts";
import { admitJobserverConfigV1, createJobserverV1, defaultJobserverStyleV1, jobserverAuthFromEnvironmentV1, jobserverMakeflagsV1, withoutJobserverEnvironmentV1, type JobserverStyleV1 } from "../🟦️.ts";
import { schedulerBinaries } from "../../🧪️tests/🦀️cargo/🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
mkdirSync(artifacts, { recursive: true });
const pool = (tokens: number) => createJobserverV1({ version: 1, tokens, style: defaultJobserverStyleV1() }, mkdtempSync(join(artifacts, "j-")));

/** 🐍️ Borrows a native transport token through an independent process. */
function borrowToken(environment: Readonly<Record<string, string>>, mode: "abandon" | "return"): string {
  const python = registeredToolExecutableV1("python"), oracle = join(import.meta.dir, "🧫️fixtures", "🐍️.py");
  const result = spawnSync(python, [oracle, mode], { env: { ...process.env, ...environment }, encoding: "utf8", timeout: 3000 });
  expect(result.status, result.stderr).toBe(0);
  return result.stdout.trim();
}

/** 🐃️ A GNU make whose fifo jobserver style is the protocol oracle (4.4 or newer), `null` when none is installed. */
function gnuMake(): string | null {
  for (const candidate of ["gmake", "make"]) {
    const version = /GNU Make (\d+)\.(\d+)/u.exec(spawnSync(candidate, ["--version"], { encoding: "utf8" }).stdout ?? "");
    const capability = spawnSync(candidate, ["-s", "-j2", "--jobserver-style=fifo", "--eval=flags:;@echo supported", "flags"], { encoding: "utf8" });
    if (capability.status === 0 && version && (Number(version[1]) > fixture.gnuMake.minimumVersion[0]! || Number(version[1]) === fixture.gnuMake.minimumVersion[0] && Number(version[2]) >= fixture.gnuMake.minimumVersion[1]!)) return candidate;
  }
  return null;
}

test("the pool configuration is schema-first and agrees with Ajv", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const config of fixture.configs.valid) { expect(validate(config)).toBe(true); expect(admitJobserverConfigV1(config)).toEqual(config as never); }
  for (const config of fixture.configs.invalid) { expect(validate(config)).toBe(false); expect(() => admitJobserverConfigV1(config)).toThrow(/Invalid jobserver configuration/u); }
});

test("MAKEFLAGS and --jobserver-auth follow the vectors and Cargo's precedence", () => {
  for (const row of fixture.makeflags) expect(jobserverMakeflagsV1(row.tokens, row.endpoint, row.style as JobserverStyleV1)).toBe(row.makeflags);
  for (const row of fixture.auth) {
    expect(jobserverAuthFromEnvironmentV1(row.environment)).toBe(row.auth);
    expect(withoutJobserverEnvironmentV1({ ...row.environment, KEEP: "value" })).toEqual({ KEEP: "value" });
  }
  expect(defaultJobserverStyleV1("win32")).toBe("semaphore");
  expect(defaultJobserverStyleV1("darwin")).toBe("fifo");
});

test("the pool grants exactly its tokens first come first served, and a cancelled waiter leaves no token behind", async () => {
  const jobserver = await pool(fixture.pool.tokens);
  try {
    await jobserver.withClient(async environment => {
      expect(jobserverAuthFromEnvironmentV1(environment)).toBe(`${process.platform === "win32" ? "" : "fifo:"}${jobserver.endpoint}`);
    });
    const held = await Promise.all(Array.from({ length: fixture.pool.tokens }, () => jobserver.acquire()));
    expect(held.filter((slot) => slot.implicit)).toHaveLength(1);
    expect(jobserver.held()).toBe(fixture.pool.tokens);
    const order: number[] = [], waiters = [0, 1].map((index) => jobserver.acquire().then((slot) => { order.push(index); return slot; }));
    const cancelled = new AbortController(), doomed = jobserver.acquire(cancelled.signal);
    await Bun.sleep(20);
    expect(jobserver.waiting()).toBe(3);
    expect(order).toEqual([]);
    cancelled.abort(Error("cancelled"));
    await expect(doomed).rejects.toThrow("cancelled");
    held[0]!.release();
    held[1]!.release();
    held[0]!.release();
    const granted = await Promise.all(waiters);
    expect(order).toEqual([0, 1]);
    for (const slot of [...held.slice(2), ...granted]) slot.release();
    expect(jobserver.held()).toBe(0);
    const again = await Promise.all(Array.from({ length: fixture.pool.tokens }, () => jobserver.acquire(AbortSignal.timeout(1000))));
    await expect(jobserver.acquire(AbortSignal.timeout(100))).rejects.toThrow();
    for (const slot of again) slot.release();
  } finally { jobserver.close(); }
});

test("tryAcquire takes a free slot without waiting, never jumps a waiter and answers null on an empty pool", async () => {
  const jobs = await pool(2);
  try {
    const first = jobs.tryAcquire(), second = jobs.tryAcquire();
    expect([first?.implicit, second?.implicit]).toEqual([true, false]);
    expect(jobs.tryAcquire()).toBeNull();
    const waiting = jobs.acquire();
    first!.release();
    expect(jobs.tryAcquire()).toBeNull();
    const granted = await waiting;
    expect(jobs.held()).toBe(2);
    granted.release(); second!.release();
    expect(jobs.held()).toBe(0);
  } finally { jobs.close(); }
});

test("a retired client cannot strand an exclusive waiter or replace an active client's channel", async () => {
  const row = fixture.retirement, jobs = await pool(row.tokens);
  try {
    const held = await Promise.all(Array.from({ length: row.driverSlots }, () => jobs.acquire()));
    const endpoint = jobs.endpoint;
    const exclusive = await jobs.withClient(async (active) => {
      await jobs.withClient(async (abandoning) => {
        expect(borrowToken(abandoning, "abandon")).toBe(row.abandonedBorrow);
      });
      expect(jobs.endpoint).toBe(endpoint);
      expect(borrowToken(active, "return")).toBe(row.whileClientActive);
      const waiting = jobs.acquireAll(AbortSignal.timeout(1000));
      expect(jobs.waiting()).toBe(1);
      return { waiting };
    });
    expect(jobs.endpoint).not.toBe(endpoint);
    const replacement = jobs.endpoint;
    for (const slot of held) slot.release();
    const restored = await exclusive.waiting;
    expect(restored).toHaveLength(row.tokens);
    expect(jobs.tryAcquire()).toBeNull();
    for (const slot of restored) slot.release();
    const healthySlot = await jobs.acquire();
    await jobs.withClient(async (healthy) => expect(borrowToken(healthy, "return")).toBe(row.afterRetirement));
    expect(jobs.endpoint).toBe(replacement);
    healthySlot.release();
    expect(jobs.held()).toBe(0);
  } finally { jobs.close(); }
});

test("a whole-pool probe returns partial slots while an external token is unavailable", async () => {
  const row = fixture.retirement, jobs = await pool(row.tokens);
  try {
    const { exclusive } = await jobs.withClient(async environment => {
      expect(borrowToken(environment, "abandon")).toBe(row.abandonedBorrow);
      const exclusive = jobs.acquireAll(AbortSignal.timeout(3000));
      void exclusive.catch(() => {});
      expect(jobs.held()).toBe(0);
      expect(jobs.waiting()).toBe(1);
      expect(borrowToken(environment, "return")).toBe(row.afterPartialProbe);
      expect(jobs.held()).toBe(0);
      return { exclusive };
    });
    const slots = await exclusive;
    expect(slots).toHaveLength(row.tokens);
    expect(jobs.tryAcquire()).toBeNull();
    for (const slot of slots) slot.release();
  } finally { jobs.close(); }
});

test("the jobserver crate, the client Cargo and rustc embed, never holds more slots than the pool and returns every token", async () => {
  const oracle = (await schedulerBinaries()).get("jobserver-oracle")!, jobserver = await pool(fixture.oracle.tokens);
  try {
    const slot = await jobserver.acquire();
    const result = await jobserver.withClient(async environment => spawnSync(oracle, [fixture.oracle.workers, fixture.oracle.holdMs, fixture.oracle.rounds].map(String), { env: { ...process.env, ...environment }, encoding: "utf8" }));
    slot.release();
    expect(result.status, result.stderr).toBe(0);
    expect(JSON.parse(result.stdout)).toEqual({ maxConcurrent: fixture.oracle.maxConcurrent, acquired: (fixture.oracle.workers - 1) * fixture.oracle.rounds });
    const all = await Promise.all(Array.from({ length: fixture.oracle.tokens }, () => jobserver.acquire(AbortSignal.timeout(1000))));
    for (const held of all) held.release();
  } finally { jobserver.close(); }
}, 600_000);

test("whole-pool reservations leave native tokens available, serialize successors, and cancel cleanly", async () => {
  const row = fixture.exclusive, jobs = await pool(row.tokens);
  try {
    const running = await jobs.acquire(), controller = new AbortController();
    const cancelled = jobs.acquireAll(controller.signal);
    const successor = jobs.acquire();
    void cancelled.catch(() => {});
    void successor.catch(() => {});
    await jobs.withClient(async environment => expect(borrowToken(environment, "return")).toBe(row.externalBorrow));
    expect(jobs.held()).toBe(row.waitingHeld);
    expect(jobs.waiting()).toBe(2);
    expect(jobs.tryAcquire()).toBeNull();
    controller.abort(Error("exclusive cancelled"));
    await expect(cancelled).rejects.toThrow("exclusive cancelled");
    (await successor).release();
    expect(jobs.held()).toBe(row.waitingHeld);
    const exclusive = jobs.acquireAll(), following = jobs.acquire();
    expect(jobs.held()).toBe(row.waitingHeld);
    running.release();
    const reserved = await exclusive;
    expect(reserved).toHaveLength(row.tokens);
    expect(jobs.held()).toBe(row.tokens);
    expect(jobs.waiting()).toBe(1);
    for (const slot of reserved) slot.release();
    (await following).release();
    expect(jobs.held()).toBe(0);
  } finally { jobs.close(); }
});

test("an external jobserver client can use the entire pool after a whole-pool reservation", async () => {
  const row = fixture.exclusive, jobs = await pool(row.tokens);
  try {
    for (const slot of await jobs.acquireAll()) slot.release();
    const oracle = (await schedulerBinaries()).get("jobserver-oracle")!, slot = await jobs.acquire();
    const result = await jobs.withClient(async environment => spawnSync(oracle, [row.workers, row.holdMs, row.rounds].map(String), { env: { ...process.env, ...environment }, encoding: "utf8" }));
    slot.release();
    expect(result.status, result.stderr).toBe(0);
    expect(JSON.parse(result.stdout)).toEqual({ maxConcurrent: row.maxConcurrent, acquired: (row.workers - 1) * row.rounds });
    console.log(`[jobserver] exclusive ${row.tokens} workers; third-party client peak=${row.maxConcurrent}`);
  } finally { jobs.close(); }
}, 600_000);

const make = gnuMake();
test.skipIf(make === null)(`GNU make ≥ ${fixture.gnuMake.minimumVersion.join(".")} --jobserver-style=fifo: its MAKEFLAGS parse here, and as a client of this pool it never exceeds the tokens`, async () => {
  const directory = mkdtempSync(join(artifacts, "m-")), log = join(directory, "spans.log"), driver = join(directory, "📜️script.ts"), probe = `"${process.execPath}" "${driver}" "${log}" ${fixture.gnuMake.recipeMs}`;
  writeFileSync(driver, `import { appendFileSync } from "node:fs"; appendFileSync(process.argv[2], Date.now() + " start\\n"); setTimeout(() => appendFileSync(process.argv[2], Date.now() + " end\\n"), Number(process.argv[3]));`);
  writeFileSync(join(directory, "Makefile"), `all: ${Array.from({ length: fixture.gnuMake.recipes }, (_, index) => `t${index}`).join(" ")}\nflags:\n\t+@node -e "console.log(process.env.MAKEFLAGS)"\nt%:\n\t@${probe}\n`);
  const served = spawnSync(make!, ["-s", "-j3", "--jobserver-style=fifo", "flags"], { cwd: directory, encoding: "utf8" });
  expect(jobserverAuthFromEnvironmentV1({ MAKEFLAGS: served.stdout.trim() })).toMatch(/^fifo:/u);
  const jobserver = await pool(fixture.gnuMake.tokens);
  try {
    const slot = await jobserver.acquire();
    await jobserver.withClient(async environment => {
      expect(spawnSync(make!, ["-s", "all"], { cwd: directory, env: { ...process.env, MAKEFLAGS: environment.MAKEFLAGS }, encoding: "utf8" }).status).toBe(0);
    });
    slot.release();
    let running = 0, peak = 0;
    for (const line of readFileSync(log, "utf8").trim().split("\n").map((row) => row.split(" ")).sort((left, right) => Number(left[0]) - Number(right[0]) || (left[1] === "end" ? -1 : 1))) { running += line[1] === "start" ? 1 : -1; peak = Math.max(peak, running); }
    expect(peak).toBeLessThanOrEqual(fixture.gnuMake.tokens);
  } finally { jobserver.close(); }
});
