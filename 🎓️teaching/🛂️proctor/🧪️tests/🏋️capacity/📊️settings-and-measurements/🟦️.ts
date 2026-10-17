/** 📊️ The settings of one capacity run and its measurements: the options, defaults, limits and budgets, and the latency recorder with its generator lag probe. */
import { availableParallelism } from "node:os";
import { DEFAULT_LIMITS } from "../../../../../🧰️framework/🛍️products/❓️quiz/🟦️.ts";

//#region 🎚️Settings
/** 🎚️ What one capacity run plays. */
export interface CapacityOptions {
  /** 🧑‍🎓️ Learners in the hall. */
  readonly learners: number;
  /** ⏩️ How many times faster than a real lecture the hall thinks. */
  readonly compression: number;
  /** 📝️ Where the JSON report is written, when it is. */
  readonly report?: string;
  /** 📦️ A `proctor` binary to measure instead of building the release one: the binary of an image, an older build. */
  readonly executable?: string;
  /** 🏫️ Play the hall alone and skip the script: a quick look at the latencies, not the gate. */
  readonly hallOnly?: boolean;
}

/** 🏫️ The defaults: the class the limits are sized for, five times as fast as it is in life. */
export const CAPACITY_DEFAULTS: CapacityOptions = { learners: 300, compression: 5 };

/** 🌐️ The site origin the capacity proctor allows. */
export const SITE = "https://quizzes.capacity.test";
/** 🏫️ The one address the whole hall arrives from. */
export const HALL = "198.51.100.10";
/** 😈️ The address of the abusive script. */
export const SCRIPT = "203.0.113.66";
/** 🔌️ Ports other agents and dev servers use; never picked. */
export const RESERVED_PORTS = new Set([8791, 8891, 8892, 6061, 6161, 6162]);

/** ⏱️ The latency budget of the hall in milliseconds, per phase: the 95th and 99th percentile of every command and
 * query. The hall beside the script may be slower than the hall alone, and still has to feel instant. */
export const BUDGET = {
  alone: { p95: 250, p99: 500 },
  beside: { p95: 500, p99: 1500 },
} as const;

/** 🧠️ The most resident memory the proctor may reach, in MiB. */
export const MEMORY_BUDGET_MIB = 512;

/** 🔁️ The share of its requests the hall may have to send again, and of its socket joins it may have to make again,
 * because one got no answer. On one machine the hall, the script and the proctor share one pool of client ports, and a
 * script that opens connections as fast as it can empties it; the quiz client sends such a request and joins such a
 * room again after its backoff, and the wait counts as latency. */
export const RESENT_SHARE = 0.01;

/** ⌛️ How long a learner's request or join is tried again before it counts as failed. */
export const GIVE_UP_MS = 30_000;

/** 🕳️ The close code of a socket that never opened because there was no connection or the connection was cut before
 * the proctor's answer; an upgrade the proctor answered with a status closes with 1002 instead. */
export const NO_ANSWER = 1006;

/** 🚦️ The edge limits the capacity proctor is started with — its production defaults, stated to it explicitly so that
 * the verdict on the script is computed from the numbers the proctor was given. */
export const LIMITS = { commandsPerSecond: 300, commandsBurst: 900, queriesPerSecond: 600, queriesBurst: 1800, signUpsPerHour: 100, signUpsBurst: 1200, socketsPerAddress: 2048, bodyBytes: 16 * 1024 } as const;

/** 🎟️ The allowance a refused sign-up names. */
export const SIGN_UP = "sign-up";

/** 🧱️ The cap of learners the capacity proctor runs with: the default, which it is not told. */
export const ROSTER_CAP = DEFAULT_LIMITS.learners;

/** 🪶️ The share of the cap the roster may hold at the end for it to be far below the cap. */
export const ROSTER_SHARE = 0.05;

/** 🍰️ The least share of the presence bytes it is sent alone that the hall must still be sent while the script's sockets
 * sit in its rooms: the rooms' budget is dealt among addresses, so a flood of sockets from one address takes its own
 * share and leaves the hall an even split of the total — which is less than the busiest moments of a hall alone ask
 * for, and more than half of what it is sent over a lecture. */
export const PRESENCE_SHARE = 0.5;

/** 🐌️ How late (p99) the load generator's own timers may fire for a latency over the budget to say something about
 * the proctor: lateness of the generator is counted into every latency, so it can only make the hall look slower. A
 * hall inside its budget although the generator ran late is inside its budget. */
export const GENERATOR_LAG_MS = 100;

/** 🧊️ How late (p99) the load generator may run before the hall it plays is no longer the hall: its learners then act
 * so far off their schedule that the phase measures nothing, whatever the latencies. */
export const GENERATOR_STALL_MS = 1_000;

/** 🧮️ Real think times in milliseconds, before compression. */
export const REAL = { arrival: 60_000, home: 15_000, revise: [4_000, 12_000] as const, read: 5_000, poll: 10_000, storm: 2_000 };
/** 🖱️ How often a learner who moves the pointer shares it, and how much of the time one does. */
export const CURSOR = { intervalMs: 100, share: 0.3 };
/** ⏲️ How long before the clock of an expert task runs out a learner stops answering it: the learner acts on the
 * device's clock, so a margin keeps the last answer inside the time however late it is delivered. */
export const CLOCK_MARGIN_MS = 1_000;
/** 👀️ The interval a home page watches the other rooms at. */
export const WATCH_INTERVAL_MS = 250;
/** 🐌️ How often a thread of the load generator probes its own lateness. */
export const LAG_PROBE_MS = 20;
/** 🧵️ The threads the hall is played on, so that the load generator's own queueing stays out of the hall's latencies. */
export const HALL_THREADS = Math.max(2, Math.min(8, Math.floor(availableParallelism() / 2)));

//#endregion 🎚️Settings
//#region 📊️Measurements
/** 📊️ What one phase measured of the hall. */
export interface Measurements {
  readonly latencies: Map<string, number[]>;
  readonly errors: string[];
  throttled: number;
  /** 📮️ Commands and queries the hall made, and how many more times it had to send one that got no answer. */
  requests: number;
  resent: number;
  /** 🔌️ Presence sockets of the hall: joined (welcomed), lost, refused a frame, and how many more times a join that got
   * no answer had to be made. */
  sockets: { opened: number; dropped: number; refused: number; rejoined: number; frames: number; bytes: number };
}

export function measurements(): Measurements {
  return { latencies: new Map(), errors: [], throttled: 0, requests: 0, resent: 0, sockets: { opened: 0, dropped: 0, refused: 0, rejoined: 0, frames: 0, bytes: 0 } };
}

export function record(into: Measurements, operation: string, milliseconds: number): void {
  const samples = into.latencies.get(operation);
  if (samples === undefined) into.latencies.set(operation, [milliseconds]);
  else samples.push(milliseconds);
}

export function fail(into: Measurements, message: string): void {
  if (into.errors.length < 200) into.errors.push(message);
  else if (into.errors.length === 200) into.errors.push("… further errors are not listed");
}

/** 📐️ The nearest-rank percentile of `samples` (milliseconds, rounded to a tenth). */
export function percentile(samples: readonly number[], rank: number): number {
  if (samples.length === 0) return 0;
  const sorted = [...samples].sort((left, right) => left - right);
  return Math.round(sorted[Math.min(sorted.length - 1, Math.max(0, Math.ceil((rank / 100) * sorted.length) - 1))] * 10) / 10;
}

/** 🧾️ One row of the report: an operation's count and percentiles. */
export interface LatencyRow {
  readonly operation: string;
  readonly count: number;
  readonly p50: number;
  readonly p95: number;
  readonly p99: number;
  readonly max: number;
}

export function rows(of: Measurements): LatencyRow[] {
  return [...of.latencies.entries()].sort(([left], [right]) => left.localeCompare(right)).map(([operation, samples]) => ({ operation, count: samples.length, p50: percentile(samples, 50), p95: percentile(samples, 95), p99: percentile(samples, 99), max: percentile(samples, 100) }));
}
//#endregion 📊️Measurements
