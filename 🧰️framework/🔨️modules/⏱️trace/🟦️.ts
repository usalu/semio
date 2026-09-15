// #region 🧲️Header
/** @emoji ⏱️ Browser twin of this module's Rust span vocabulary (`⏱️trace/🦀️.rs`), scoped to the ONE
 * thing the native tracer cannot see: the renderer's per-hop round trip. A "hop" is one
 * guest-command dispatch and everything the host does with its answer — encode, worker crossing,
 * reply decode, ui refresh, React commit, and the arming of the next dispatch.
 *
 * 🩺️ Spans are published as User Timing `performance.measure` entries under
 * {@link HOP_TRACE_MEASURE_PREFIX} so a CDP-attached probe reads them straight out of the live
 * isolate (`🐍️react-hop-cost-probe.mjs`) with no console parsing, and mirrored into a bounded ring
 * the laws read without a browser. Both halves are pure accounting: a tracer must never change what
 * a hop settles or the order it settles in, only say what it cost.
 *
 * 🪶️ Cost is a `performance.now()` pair plus one `measure` call per span — single-digit microseconds
 * against hops measured in hundreds of milliseconds — so this is always armed rather than gated
 * behind a flag nobody remembers to set before the run that mattered.
 */
// #endregion 🧲️Header

//#region 🔖️Vocabulary
/** 🏷️ The one prefix every hop measure carries, so a probe can select ours out of a page whose
 * libraries publish their own User Timing entries. */
export const HOP_TRACE_MEASURE_PREFIX = "semio.hop.";

/** 🧱️ The declared stages of ONE hop, in the order a hop passes through them. The language-agnostic
 * twin is `🧫️fixtures/🪃️hop-stages/🔣️.json`; a probe that renders a breakdown table and a law that
 * checks ordering both read the same list rather than two hand-kept copies. */
export const HOP_TRACE_STAGES = ["encode", "channel", "decode", "invoke", "refresh.guest", "refresh.turn", "refresh.project", "refresh.slots", "refresh.apply", "refresh", "commit", "arm", "mesh.decode", "surface.paint", "patch.install", "patch.paint", "turn.accept", "turn.decide", "turn.yield", "worker.turn", "worker.receive", "worker.decode", "worker.guest", "worker.reply"] as const;

export type HopTraceStage = (typeof HOP_TRACE_STAGES)[number];

/** 🧾️ Scalars only: a detail record crosses `structuredClone` into the probe and is rendered into a
 * table, so it may carry what a row is keyed and grouped by and nothing else. */
export type HopTraceDetail = Readonly<Record<string, string | number | boolean | null>>;

/** 📐️ One published span: the stage, when it opened on the page's own monotonic clock, how long it
 * held, and the detail that says which hop it belongs to. */
export type HopTraceSpan = Readonly<{
  readonly stage: HopTraceStage;
  readonly startMs: number;
  readonly durationMs: number;
  readonly detail: HopTraceDetail;
}>;

/** 🔌️ The two host facilities a tracer needs, injected so the laws drive a real tracer over a fake
 * clock and a fake User Timing sink instead of a browser. */
export type HopTracePorts = Readonly<{
  readonly now: () => number;
  readonly measure: (name: string, startMs: number, endMs: number, detail: HopTraceDetail) => void;
}>;

/** 🚧️ Ring capacity. A journey run publishes ~10 spans per hop and a long run is a few thousand
 * hops, so this holds a whole run; past it the OLDEST spans drop, because a probe that reads at the
 * end of a step cares about that step. */
export const HOP_TRACE_RING_CAPACITY = 20_000;
//#endregion 🔖️Vocabulary

//#region 🔖️Ports
/** 🌐️ Real browser ports: `performance.now` and a User Timing `measure` with a `detail`. Every reader
 * is wrapped because a sandboxed or Node host has neither, and a tracer may never be the reason a
 * hop throws. */
export function defaultHopTracePorts(): HopTracePorts {
  const clock = typeof performance === "object" && typeof performance.now === "function" ? () => performance.now() : () => Date.now();
  return {
    now: clock,
    measure: (name, startMs, endMs, detail) => performance.measure(name, { start: startMs, end: endMs, detail } as PerformanceMeasureOptions),
  };
}
/** 🌍️ The one clock two realms share. A Worker's `performance.now()` counts from ITS OWN
 * `timeOrigin`, so a duration measured inside the shard worker is comparable and an instant is not —
 * `timeOrigin + now()` is the same Unix-epoch millisecond on both sides, at sub-millisecond
 * resolution, which is what lets the crossing itself (post → receive, reply → receive) be measured
 * rather than inferred by subtracting the parts from the whole. */
export function hopTraceEpochNowMs(): number {
  if (typeof performance !== "object" || typeof performance.now !== "function") return Date.now();
  return (typeof performance.timeOrigin === "number" ? performance.timeOrigin : 0) + performance.now();
}

/** 🧭️ Places a shared-epoch instant on THIS realm's `performance` timeline, where every other hop
 * span already lives, so one probe read returns a breakdown that adds up. */
export function hopTraceEpochToTimeline(epochMs: number): number {
  return Math.max(0, epochMs - (typeof performance === "object" && typeof performance.timeOrigin === "number" ? performance.timeOrigin : 0));
}
//#endregion 🔖️Ports

//#region 🔖️Tracer
/** 🧵️ Closes one open span. Idempotent: a second call is dropped, so a caller may close in both a
 * success path and a `finally` without publishing twice. */
export type HopTraceClose = (extraDetail?: HopTraceDetail) => void;

export type HopTracer = Readonly<{
  /** ⏱️ Opens `stage` now and returns its close. */
  open: (stage: HopTraceStage, detail?: HopTraceDetail) => HopTraceClose;
  /** 📮️ Publishes a span measured somewhere this realm's clock never ran — the shard worker's own
   * stages, handed back on the reply and placed on THIS timeline by {@link hopTraceEpochToTimeline}. */
  record: (stage: HopTraceStage, startMs: number, durationMs: number, detail?: HopTraceDetail) => void;
  /** ⏱️ Times a synchronous body, publishing `stage` whether it returns or throws. */
  time: <T>(stage: HopTraceStage, detail: HopTraceDetail | undefined, run: () => T) => T;
  /** ⏱️ Times an awaited body, publishing `stage` whether it resolves or rejects. */
  timeAsync: <T>(stage: HopTraceStage, detail: HopTraceDetail | undefined, run: () => Promise<T>) => Promise<T>;
  /** 📖️ Everything still in the ring, oldest first. */
  spans: () => readonly HopTraceSpan[];
  /** 🧹️ Empties the ring. Published User Timing entries are the page's, not ours, and stay. */
  reset: () => void;
}>;

/** 🏭️ Builds a tracer over `ports`. Production takes the browser ports; the laws hand it a virtual
 * clock and collect the measures. */
export function createHopTracer(ports: HopTracePorts = defaultHopTracePorts()): HopTracer {
  const ring: HopTraceSpan[] = [];
  const publish = (stage: HopTraceStage, startMs: number, endMs: number, detail: HopTraceDetail): void => {
    const span: HopTraceSpan = { stage, startMs, durationMs: Math.max(0, endMs - startMs), detail };
    ring.push(span);
    if (ring.length > HOP_TRACE_RING_CAPACITY) ring.splice(0, ring.length - HOP_TRACE_RING_CAPACITY);
    try {
      ports.measure(`${HOP_TRACE_MEASURE_PREFIX}${stage}`, startMs, endMs, detail);
    } catch {
      /* 🛡️ A tracer is never the reason a hop fails: a host without User Timing keeps the ring. */
    }
  };
  const open: HopTracer["open"] = (stage, detail) => {
    const startMs = ports.now();
    let closed = false;
    return (extraDetail) => {
      if (closed) return;
      closed = true;
      publish(stage, startMs, ports.now(), { ...detail, ...extraDetail });
    };
  };
  return {
    open,
    record: (stage, startMs, durationMs, detail) => publish(stage, startMs, startMs + Math.max(0, durationMs), { ...detail }),
    time: (stage, detail, run) => {
      const close = open(stage, detail);
      try {
        return run();
      } finally {
        close();
      }
    },
    timeAsync: async (stage, detail, run) => {
      const close = open(stage, detail);
      try {
        return await run();
      } finally {
        close();
      }
    },
    spans: () => ring,
    reset: () => {
      ring.length = 0;
    },
  };
}

/** 🌍️ The one tracer of a renderer realm — the hop crosses three modules (`PluginRuntime`,
 * `ShellHost`, `ShellHelpers`) and they must publish onto ONE timeline for the breakdown to add up. */
export const hopTrace: HopTracer = createHopTracer();
//#endregion 🔖️Tracer

//#region 🔖️Breakdown
/** 🧮️ Sums a span set per stage — the shape both the probe's table and the laws assert on. Spans of
 * a stage that never ran are absent rather than zero, so a missing stage reads as missing. */
export function hopTraceTotalsByStage(spans: readonly HopTraceSpan[]): Readonly<Partial<Record<HopTraceStage, { readonly count: number; readonly totalMs: number }>>> {
  const totals: Partial<Record<HopTraceStage, { count: number; totalMs: number }>> = {};
  for (const span of spans) {
    const entry = totals[span.stage] ?? { count: 0, totalMs: 0 };
    entry.count += 1;
    entry.totalMs += span.durationMs;
    totals[span.stage] = entry;
  }
  return totals;
}

/** 🪃️ The gap between consecutive hops: from the end of one `invoke` to the start of the next. This
 * is the term no single span can hold — it is the host's own latency between a settled dispatch and
 * the dispatch it arms next — and it is the one the `arm` stage is measured against. */
export function hopTraceInterHopGapsMs(spans: readonly HopTraceSpan[]): readonly number[] {
  const invocations = spans.filter((span) => span.stage === "invoke").slice().sort((left, right) => left.startMs - right.startMs);
  const gaps: number[] = [];
  for (let index = 1; index < invocations.length; index += 1) {
    const previous = invocations[index - 1]!;
    gaps.push(Math.max(0, invocations[index]!.startMs - (previous.startMs + previous.durationMs)));
  }
  return gaps;
}
//#endregion 🔖️Breakdown
