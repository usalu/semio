/** 🪃️ The host's ONE continuation scheduler: the single owner of "run this again, soon" for every
 * evaluation/continuation loop that crosses the JS event loop — the guest's `Effect::DispatchAction`
 * re-arm (`scheduleDispatchAction`), the plugin-UI continuation yield (`yieldPluginUiContinuation`)
 * and the typed-operation drain wake.
 *
 * 🛑️ Why it exists: a `setTimeout(fn, 0)` chain is NOT "next tick" in a browser. A hidden, unfocused
 * or headless renderer clamps nested timers to roughly one tick per second (once per minute under
 * Chrome's intensive throttling), so a guest that converges in 5 re-arms natively in 0.58 s took
 * 112 s in a headless page, in two silent ~11-13 s gaps per extension hop with zero logging in
 * between (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-extension-hop-latency-2026-09-12.md`).
 * A `MessageChannel` message is a macrotask from a different task source that page visibility never
 * throttles, so a zero-delay continuation costs one event-loop turn whether the tab is focused,
 * hidden or headless.
 *
 * 📐️ The contract, in one line each:
 * - `delayMs <= 0` is a CONTINUATION: an unthrottled macrotask, never a timer. It runs before any
 *   timer this scheduler still holds, and a chain of them advances the wall clock by nothing.
 * - `delayMs > 0` is a DEADLINE: a real timer, but ONE host timer for the whole scheduler (armed at
 *   the earliest deadline and re-armed on drain) instead of one per request.
 * - every request is cancellable, and requests sharing a `key` coalesce to a single run that keeps
 *   the EARLIEST deadline asked for and the NEWEST callback supplied.
 * - order is preserved: within a lane, ties run in enqueue order.
 *
 * 🙈️ What the browser still throttles, and what we therefore do NOT build on: `requestAnimationFrame`
 * does not fire at all in a hidden tab, and `requestIdleCallback` is deferred indefinitely. Neither
 * is on this path — evaluation progress must not depend on the page being painted, so a hidden tab
 * keeps converging (CLAUDE.md: "support short connection-shortages and not freeze the app"). Only
 * PAINT is allowed to stop when nobody is looking. Timers (`delayMs > 0`) stay throttled by design:
 * a deadline that fires late is safe, a continuation that fires late is not.
 *
 * 🇩🇪 Der einzige Fortsetzungs-Scheduler des Hosts. `delayMs <= 0` ist eine Fortsetzung (ungedrosselter
 * Makrotask über `MessageChannel`), `delayMs > 0` eine Frist (ein einziger Host-Timer). Alles ist
 * abbrechbar, gleichnamige Anforderungen werden zusammengefasst, die Reihenfolge bleibt erhalten.
 *
 * @see https://developer.chrome.com/blog/timer-throttling-in-chrome-88
 * @see https://html.spec.whatwg.org/multipage/web-messaging.html#message-channels
 */

//#region 🔌️Ports
/** 🔌️ Everything this scheduler needs from a host, so the whole contract is testable on a virtual
 * clock with no real timers — and so a worker/node host can supply its own primitives. */
export type ContinuationPorts = Readonly<{
  /** 📮️ Posts ONE macrotask that the host must not clamp by page visibility. */
  postMacrotask: (run: () => void) => void;
  /** ⏰️ Arms one host timer and returns its handle. At most one is ever live per scheduler. */
  setTimer: (run: () => void, delayMs: number) => unknown;
  /** 🧹️ Cancels a handle returned by {@link ContinuationPorts.setTimer}. */
  clearTimer: (handle: unknown) => void;
  /** 🕰️ Milliseconds on a clock that only moves forward. */
  now: () => number;
}>;

/** ⏱️ How early a host timer is allowed to fire and still count as due. Real timers undershoot by
 * well under a millisecond; a virtual clock lands exactly on the deadline. */
const TIMER_EPSILON_MS = 1;

let sharedMacrotaskPost: ((run: () => void) => void) | null = null;

/** 📮️ The process-wide unthrottled macrotask port. One `MessageChannel` is enough for every
 * scheduler: a post carries no payload, it only asks the event loop for a turn. Built lazily so
 * importing this module costs nothing, and `unref`'d where the host offers it (node keeps a started
 * port ref'd, which would hold a script open forever). */
const macrotaskPort = (): ((run: () => void) => void) => {
  if (sharedMacrotaskPost) return sharedMacrotaskPost;
  const channel = typeof MessageChannel === "function" ? new MessageChannel() : null;
  if (!channel) {
    sharedMacrotaskPost = (run) => {
      setTimeout(run, 0);
    };
    return sharedMacrotaskPost;
  }
  const waiting: (() => void)[] = [];
  channel.port1.onmessage = () => {
    for (const run of waiting.splice(0)) run();
  };
  (channel.port1 as { unref?: () => void }).unref?.();
  (channel.port2 as { unref?: () => void }).unref?.();
  sharedMacrotaskPost = (run) => {
    waiting.push(run);
    channel.port2.postMessage(null);
  };
  return sharedMacrotaskPost;
};

/** 🌐️ The real host's ports: `MessageChannel` for continuations, `setTimeout` for deadlines. */
export const defaultContinuationPorts = (): ContinuationPorts => ({
  postMacrotask: (run) => macrotaskPort()(run),
  setTimer: (run, delayMs) => setTimeout(run, delayMs),
  clearTimer: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>),
  now: typeof performance === "object" && typeof performance.now === "function" ? () => performance.now() : () => Date.now(),
});
//#endregion 🔌️Ports

//#region 🪃️Scheduler
/** 🧹️ Undoes one {@link ContinuationScheduler.schedule}. Idempotent, and a no-op once the callback ran. */
export type ContinuationCancel = () => void;

export type ContinuationScheduler = Readonly<{
  /** 🪃️ Runs `run` after `delayMs` (`<= 0` = the next unthrottled macrotask). A `key` coalesces this
   * request with any still-pending request sharing it: one run, at the earliest deadline either asked
   * for, with the newest callback. */
  schedule: (run: () => void, delayMs: number, key?: string) => ContinuationCancel;
  /** ⏭️ Awaits one unthrottled macrotask — the yield every guest-driving loop uses to let the host
   * breathe without handing the browser an excuse to clamp it. */
  yieldContinuation: () => Promise<void>;
  /** 🧹️ Cancels whatever is still pending under `key`. */
  cancelKey: (key: string) => void;
  /** 📏️ Requests still pending, both lanes. Diagnostics and tests only. */
  pending: () => number;
  /** 🛑️ Drops every pending request and disarms the host timer. */
  dispose: () => void;
}>;

type ContinuationEntry = { readonly seq: number; readonly key: string | null; readonly runAtMs: number; run: (() => void) | null };

const insertByDeadline = (queue: ContinuationEntry[], entry: ContinuationEntry): void => {
  let low = 0;
  let high = queue.length;
  while (low < high) {
    const mid = (low + high) >> 1;
    const other = queue[mid]!;
    if (other.runAtMs < entry.runAtMs || (other.runAtMs === entry.runAtMs && other.seq < entry.seq)) low = mid + 1;
    else high = mid;
  }
  queue.splice(low, 0, entry);
};

const dropEntry = (queue: ContinuationEntry[], entry: ContinuationEntry): boolean => {
  const index = queue.indexOf(entry);
  if (index < 0) return false;
  queue.splice(index, 1);
  return true;
};

/** 🏭️ Builds a scheduler over `ports`. Production takes the default ports; tests and the node twin
 * hand it {@link createVirtualContinuationHost}'s. */
export const createContinuationScheduler = (ports: ContinuationPorts = defaultContinuationPorts()): ContinuationScheduler => {
  const immediate: ContinuationEntry[] = [];
  const timed: ContinuationEntry[] = [];
  const byKey = new Map<string, ContinuationEntry>();
  let seq = 0;
  let immediateArmed = false;
  let timerHandle: unknown = null;
  let timerDeadlineMs = Number.POSITIVE_INFINITY;
  let disposed = false;

  const forgetKey = (entry: ContinuationEntry): void => {
    if (entry.key !== null && byKey.get(entry.key) === entry) byKey.delete(entry.key);
  };

  const armTimer = (): void => {
    const head = timed[0];
    if (!head) {
      if (timerHandle !== null) ports.clearTimer(timerHandle);
      timerHandle = null;
      timerDeadlineMs = Number.POSITIVE_INFINITY;
      return;
    }
    if (timerHandle !== null && timerDeadlineMs <= head.runAtMs) return;
    if (timerHandle !== null) ports.clearTimer(timerHandle);
    timerDeadlineMs = head.runAtMs;
    timerHandle = ports.setTimer(fireTimer, Math.max(0, head.runAtMs - ports.now()));
  };

  function fireTimer(): void {
    timerHandle = null;
    timerDeadlineMs = Number.POSITIVE_INFINITY;
    const nowMs = ports.now();
    const due: ContinuationEntry[] = [];
    while (timed.length > 0 && timed[0]!.runAtMs - nowMs <= TIMER_EPSILON_MS) due.push(timed.shift()!);
    armTimer();
    for (const entry of due) {
      forgetKey(entry);
      const run = entry.run;
      entry.run = null;
      run?.();
    }
  }

  const drainImmediate = (): void => {
    immediateArmed = false;
    const batch = immediate.splice(0);
    for (const entry of batch) {
      forgetKey(entry);
      const run = entry.run;
      entry.run = null;
      run?.();
    }
  };

  const schedule = (run: () => void, delayMs: number, key?: string): ContinuationCancel => {
    if (disposed) return () => {};
    const nowMs = ports.now();
    let runAtMs = nowMs + (Number.isFinite(delayMs) && delayMs > 0 ? delayMs : 0);
    if (key !== undefined) {
      const existing = byKey.get(key);
      if (existing) {
        runAtMs = Math.min(runAtMs, existing.runAtMs);
        existing.run = null;
        byKey.delete(key);
        if (!dropEntry(immediate, existing) && dropEntry(timed, existing)) armTimer();
      }
    }
    const entry: ContinuationEntry = { seq: (seq += 1), key: key ?? null, runAtMs, run };
    if (key !== undefined) byKey.set(key, entry);
    if (runAtMs <= nowMs) {
      immediate.push(entry);
      if (!immediateArmed) {
        immediateArmed = true;
        ports.postMacrotask(drainImmediate);
      }
    } else {
      insertByDeadline(timed, entry);
      armTimer();
    }
    return () => {
      if (entry.run === null) return;
      entry.run = null;
      forgetKey(entry);
      if (!dropEntry(immediate, entry) && dropEntry(timed, entry)) armTimer();
    };
  };

  return Object.freeze({
    schedule,
    yieldContinuation: () =>
      new Promise<void>((resolve) => {
        schedule(resolve, 0);
      }),
    cancelKey: (key: string) => {
      const entry = byKey.get(key);
      if (!entry) return;
      entry.run = null;
      byKey.delete(key);
      if (!dropEntry(immediate, entry) && dropEntry(timed, entry)) armTimer();
    },
    pending: () => immediate.length + timed.length,
    dispose: () => {
      disposed = true;
      immediate.length = 0;
      timed.length = 0;
      byKey.clear();
      if (timerHandle !== null) ports.clearTimer(timerHandle);
      timerHandle = null;
      timerDeadlineMs = Number.POSITIVE_INFINITY;
    },
  });
};

/** 🌐️ The host process's scheduler. Every renderer-side continuation goes through THIS instance so
 * "one implementation, not two" is a fact about the object graph and not only about the source. */
export const hostContinuations: ContinuationScheduler = createContinuationScheduler();
//#endregion 🪃️Scheduler

//#region 🧪️VirtualHost
/** 🕰️ A deterministic event loop: a macrotask FIFO, a timer wheel and a clock that only advances when
 * nothing else can run. It models the ONE property the browser gives us and we depend on — a posted
 * macrotask runs before a timer whose deadline has not arrived — without any real waiting. */
export type VirtualContinuationHost = Readonly<{
  ports: ContinuationPorts;
  nowMs: () => number;
  /** ⏩️ Advances the virtual clock to `untilMs` (default: until nothing is left), running everything
   * that comes due. Returns the number of callbacks run. Throws past `stepBudget` so a runaway chain
   * is a test failure rather than a hang. */
  drain: (untilMs?: number, stepBudget?: number) => number;
  /** 📏️ Whether anything at all is still queued. */
  idle: () => boolean;
}>;

/** 🏭️ Builds a {@link VirtualContinuationHost}. */
export const createVirtualContinuationHost = (): VirtualContinuationHost => {
  let nowMs = 0;
  let handleSeq = 0;
  const macrotasks: (() => void)[] = [];
  const timers = new Map<number, { readonly runAtMs: number; readonly run: () => void }>();
  const earliestTimerMs = (): number => {
    let earliest = Number.POSITIVE_INFINITY;
    for (const timer of timers.values()) if (timer.runAtMs < earliest) earliest = timer.runAtMs;
    return earliest;
  };
  const ports: ContinuationPorts = {
    postMacrotask: (run) => {
      macrotasks.push(run);
    },
    setTimer: (run, delayMs) => {
      handleSeq += 1;
      timers.set(handleSeq, { runAtMs: nowMs + Math.max(0, delayMs), run });
      return handleSeq;
    },
    clearTimer: (handle) => {
      timers.delete(handle as number);
    },
    now: () => nowMs,
  };
  return Object.freeze({
    ports,
    nowMs: () => nowMs,
    idle: () => macrotasks.length === 0 && timers.size === 0,
    drain: (untilMs = Number.POSITIVE_INFINITY, stepBudget = 100_000) => {
      let steps = 0;
      for (;;) {
        if (steps > stepBudget) throw new Error(`[DEBUG] virtual continuation host exceeded its ${stepBudget}-step budget at ${nowMs} ms`);
        if (macrotasks.length > 0) {
          steps += 1;
          macrotasks.shift()!();
          continue;
        }
        const nextMs = earliestTimerMs();
        if (!Number.isFinite(nextMs) || nextMs > untilMs) {
          if (Number.isFinite(untilMs) && untilMs > nowMs) nowMs = untilMs;
          return steps;
        }
        nowMs = Math.max(nowMs, nextMs);
        for (const [handle, timer] of [...timers]) {
          if (timer.runAtMs > nowMs) continue;
          timers.delete(handle);
          steps += 1;
          timer.run();
          break;
        }
      }
    },
  });
};
//#endregion 🧪️VirtualHost

//#region 🧫️Fixture
/** 🧫️ One scheduling request in a fixture case. `repeat` models a self-re-arming continuation chain —
 * exactly the guest's `Effect::DispatchAction { delay_ms: 0 }` re-arm — by re-scheduling itself with
 * the same delay until it has run `repeat` times; each run is recorded as `label#n`. */
export type ContinuationRequestCase = Readonly<{
  label: string;
  atMs: number;
  delayMs: number;
  key?: string;
  repeat?: number;
  cancelAtMs?: number;
}>;

/** 🧫️ One language-agnostic scheduling law: what was asked for, in what order it must run, and at
 * what millisecond the last callback must land. */
export type ContinuationCase = Readonly<{
  name: string;
  law: string;
  requests: readonly ContinuationRequestCase[];
  expectedOrder: readonly string[];
  expectedCompletionMs: number;
}>;

export type ContinuationSuite = Readonly<{ law: string; provenance: string; cases: readonly ContinuationCase[] }>;

export type ContinuationCaseRun = Readonly<{ order: readonly string[]; completionMs: number }>;

/** ▶️ Runs one case on a virtual clock — no real timers, so the result is a pure function of the
 * fixture. This is the oracle the vitest suite and the node twin both compare against. */
export const runContinuationCase = (testCase: ContinuationCase): ContinuationCaseRun => {
  const host = createVirtualContinuationHost();
  const scheduler = createContinuationScheduler(host.ports);
  const order: string[] = [];
  let completionMs = 0;
  const cancels = new Map<string, ContinuationCancel>();
  const submit = (request: ContinuationRequestCase): void => {
    const total = Math.max(1, request.repeat ?? 1);
    let run = 0;
    const step = (): void => {
      run += 1;
      order.push(total === 1 ? request.label : `${request.label}#${run}`);
      completionMs = host.nowMs();
      if (run < total) cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
    };
    cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
  };
  const milestones = [...new Set(testCase.requests.flatMap((request) => [request.atMs, ...(request.cancelAtMs === undefined ? [] : [request.cancelAtMs])]))].sort((a, b) => a - b);
  for (const milestone of milestones) {
    host.drain(milestone);
    for (const request of testCase.requests) if (request.atMs === milestone) submit(request);
    for (const request of testCase.requests) if (request.cancelAtMs === milestone) cancels.get(request.label)?.();
  }
  host.drain();
  return { order, completionMs };
};

/** ▶️ The same case on the REAL host event loop — the platform's own `MessageChannel` and
 * `setTimeout`, no virtual clock anywhere. It is the independent oracle for
 * {@link runContinuationCase}: if the browser/node event loop and our virtual model disagree about a
 * declared law, the law is wrong. Resolves once every expected callback has run, or rejects at
 * `timeoutMs`. */
export const runContinuationCaseLive = async (testCase: ContinuationCase, timeoutMs = 5_000): Promise<ContinuationCaseRun & { readonly elapsedMs: number }> => {
  const scheduler = createContinuationScheduler();
  const order: string[] = [];
  const startedAtMs = Date.now();
  const cancels = new Map<string, ContinuationCancel>();
  const expected = testCase.expectedOrder.length;
  let settle: (() => void) | null = null;
  const done = new Promise<void>((resolve) => {
    settle = resolve;
  });
  const record = (label: string): void => {
    order.push(label);
    if (order.length >= expected) settle?.();
  };
  const submit = (request: ContinuationRequestCase): void => {
    const total = Math.max(1, request.repeat ?? 1);
    let run = 0;
    const step = (): void => {
      run += 1;
      record(total === 1 ? request.label : `${request.label}#${run}`);
      if (run < total) cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
    };
    cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
  };
  const at = (delayMs: number, run: () => void): void => {
    if (delayMs <= 0) run();
    else setTimeout(run, delayMs);
  };
  for (const request of testCase.requests) at(request.atMs, () => submit(request));
  for (const request of testCase.requests) if (request.cancelAtMs !== undefined) at(request.cancelAtMs, () => cancels.get(request.label)?.());
  const guard = setTimeout(() => settle?.(), timeoutMs);
  await done;
  clearTimeout(guard);
  // 🩺 One extra unthrottled turn so a callback that should NOT have run gets its chance to prove it did.
  await scheduler.yieldContinuation();
  scheduler.dispose();
  return { order, completionMs: Date.now() - startedAtMs, elapsedMs: Date.now() - startedAtMs };
};

/** 🧪️ Checks one case both ways and returns one human-readable fault per violation, empty when the
 * law holds. Shared by the vitest suite and the node twin so the two cannot drift. */
export const continuationCaseFaults = (testCase: ContinuationCase, run: ContinuationCaseRun, toleranceMs = 0): readonly string[] => {
  const faults: string[] = [];
  if (run.order.join(" → ") !== testCase.expectedOrder.join(" → ")) faults.push(`${testCase.name}: ran [${run.order.join(", ")}], expected [${testCase.expectedOrder.join(", ")}]`);
  if (run.completionMs < testCase.expectedCompletionMs - toleranceMs || run.completionMs > testCase.expectedCompletionMs + toleranceMs)
    faults.push(`${testCase.name}: finished at ${Math.round(run.completionMs)} ms, expected ${testCase.expectedCompletionMs} ms ±${toleranceMs}`);
  return faults;
};
//#endregion 🧫️Fixture
