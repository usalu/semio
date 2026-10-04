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
 *   scheduler deadline that is not yet due; a virtual chain adds no artificial clock delay.
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

/** 📮️ The process-wide unthrottled macrotask port. Native hosts use their immediate queue so
 * continuously posted messages cannot starve timers; browsers share one `MessageChannel`. Built lazily so
 * importing this module costs nothing. Only message ports are unref'd where supported; native
 * immediates keep an awaited continuation alive until it runs. */
const macrotaskPort = (): ((run: () => void) => void) => {
  if (sharedMacrotaskPost) return sharedMacrotaskPost;
  const host = globalThis as { process?: { versions?: { node?: string; bun?: string } }; setImmediate?: (run: () => void) => unknown };
  const immediate = host.setImmediate;
  if ((host.process?.versions?.node || host.process?.versions?.bun) && typeof immediate === "function") {
    sharedMacrotaskPost = (run) => { immediate(run); };
    return sharedMacrotaskPost;
  }
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

/** 🌐️ The real host's ports: native immediate queue or browser messages for continuations, timers for deadlines. */
const defaultContinuationPorts = (): ContinuationPorts => ({
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
