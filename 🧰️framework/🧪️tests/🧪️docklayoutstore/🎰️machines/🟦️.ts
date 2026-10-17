/** 🎰️ The statechart machines of the framework in-source suites: the fixture machines, the test host, the trace inspector, the kernel, persistence, the actor system, model testing, bit sets and the checkout DSL twin. */
import type { EventId as EventIdBrand, Command, ConformanceStep, Invariant, Machine, MachineSpec, Migration, NodeDef, PersistedSnapshot, StatechartEvent, TransitionDef } from "@semio-tech/framework";

export async function registerMachineTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework")): Promise<void> {
  const { ActionId, ActorId, ActorSystem, BitSet, EventId, GuardId, InvokeId, Model, NodeId, NullInspector, TestHost, TimerId, TraceInspector, checkInvariants, explore, init, macrostep, persist, restore, runConformance, start, step, timerElapsed } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🔄️MachineFixtures
  class UnitFlipEvent implements StatechartEvent {
    readonly eventCount = 1;
    eventId(): EventIdBrand {
      return EventId(0);
    }
    eventName(): string {
      return "Flip";
    }
  }
  const UNIT_FLIP = new UnitFlipEvent();

  interface UnitToggleSpec extends MachineSpec {
    readonly Context: { readonly count: number };
    readonly Event: UnitFlipEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }

  const UNIT_TOGGLE_NODES: readonly NodeDef[] = [
    { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
    { stableId: "off", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
    { stableId: "on", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
  ];
  const UNIT_TOGGLE_TRANSITIONS: readonly TransitionDef[] = [
    { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
    { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 1 },
  ];
  const UNIT_TOGGLE_MACHINE: Machine<UnitToggleSpec> = {
    definition: { id: "unit_toggle", nodes: UNIT_TOGGLE_NODES, transitions: UNIT_TOGGLE_TRANSITIONS, contextFromInput: () => ({ count: 0 }), guards: [], actions: [], fingerprint: 42n, manifestJson: "{}" },
  };

  interface ToggleSpec extends MachineSpec {
    readonly Context: { count: number; allow: boolean };
    readonly Event: UnitFlipEvent;
    readonly Input: boolean;
    readonly Output: void;
    readonly Effect: void;
  }
  const TOGGLE_MACHINE: Machine<ToggleSpec> = {
    definition: {
      id: "toggle",
      nodes: UNIT_TOGGLE_NODES,
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [ActionId(0)], docIndex: 0 },
        { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, guard: GuardId(0), targets: [NodeId(1)], kind: "external", actions: [ActionId(0)], docIndex: 1 },
      ],
      contextFromInput: (allow) => ({ count: 0, allow }),
      guards: [(ctx) => ctx.allow],
      actions: [(ctx) => (ctx.count += 1)],
      fingerprint: 1n,
      manifestJson: "{}",
    },
  };

  class PlayerEvent implements StatechartEvent {
    private static readonly IDS = { open: 0, pause: 1, play: 2, stop: 3, resume: 4 } as const;
    private static readonly NAMES = ["Open", "Pause", "Play", "Stop", "Resume"];
    readonly eventCount = 5;
    readonly type: keyof typeof PlayerEvent.IDS;
    constructor(type: keyof typeof PlayerEvent.IDS) {
      this.type = type;
    }
    eventId(): EventIdBrand {
      return EventId(PlayerEvent.IDS[this.type]);
    }
    eventName(id: EventIdBrand): string {
      return PlayerEvent.NAMES[id] ?? "?";
    }
  }
  interface PlayerSpec extends MachineSpec {
    readonly Context: Record<string, never>;
    readonly Event: PlayerEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }
  const PLAYER_MACHINE: Machine<PlayerSpec> = {
    definition: {
      id: "player",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(3)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "closed", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "playing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "open", kind: "compound", parent: NodeId(0), initial: NodeId(2), children: [NodeId(2), NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "paused", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
        { stableId: "open.history", kind: "historyShallow", parent: NodeId(3), initial: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(3)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(2), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 2 },
        { source: NodeId(3), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 },
        { source: NodeId(1), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 },
      ],
      contextFromInput: () => ({}),
      guards: [],
      actions: [],
      fingerprint: 2n,
      manifestJson: "{}",
    },
  };

  class RecorderEvent implements StatechartEvent {
    private static readonly IDS = { start: 0, audioStop: 1, videoStop: 2 } as const;
    private static readonly NAMES = ["Start", "AudioStop", "VideoStop"];
    readonly eventCount = 3;
    readonly type: keyof typeof RecorderEvent.IDS;
    constructor(type: keyof typeof RecorderEvent.IDS) {
      this.type = type;
    }
    eventId(): EventIdBrand {
      return EventId(RecorderEvent.IDS[this.type]);
    }
    eventName(id: EventIdBrand): string {
      return RecorderEvent.NAMES[id] ?? "?";
    }
  }
  interface RecorderSpec extends MachineSpec {
    readonly Context: Record<string, never>;
    readonly Event: RecorderEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }
  const RECORDER_MACHINE: Machine<RecorderSpec> = {
    definition: {
      id: "recorder",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "idle", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "recording", kind: "parallel", parent: NodeId(0), children: [NodeId(3), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "audio", kind: "compound", parent: NodeId(2), initial: NodeId(4), children: [NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "audio.capturing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
        { stableId: "audio.done", kind: "final", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
        { stableId: "video", kind: "compound", parent: NodeId(2), initial: NodeId(7), children: [NodeId(7), NodeId(8)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
        { stableId: "video.capturing", kind: "atomic", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
        { stableId: "video.done", kind: "final", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 },
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(7), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(8)], kind: "external", actions: [], docIndex: 2 },
        { source: NodeId(2), trigger: { kind: "done", node: NodeId(2) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 },
      ],
      contextFromInput: () => ({}),
      guards: [],
      actions: [],
      fingerprint: 3n,
      manifestJson: "{}",
    },
  };

  class CheckoutEvent implements StatechartEvent {
    private static readonly IDS = { confirm: 0, selectMethod: 1, paymentSucceeded: 2, paymentFailed: 3, retry: 4, cancel: 5, resume: 6, shipDone: 7, invoiceDone: 8 } as const;
    private static readonly NAMES = ["Confirm", "SelectMethod", "PaymentSucceeded", "PaymentFailed", "Retry", "Cancel", "Resume", "ShipDone", "InvoiceDone"];
    readonly eventCount = 9;
    readonly type: keyof typeof CheckoutEvent.IDS;
    constructor(type: keyof typeof CheckoutEvent.IDS) {
      this.type = type;
    }
    eventId(): EventIdBrand {
      return EventId(CheckoutEvent.IDS[this.type]);
    }
    eventName(id: EventIdBrand): string {
      return CheckoutEvent.NAMES[id] ?? "?";
    }
  }
  type CheckoutContext = { attempts: number; methodSet: boolean };
  type Receipt = { readonly attempts: number };
  interface CheckoutSpec extends MachineSpec {
    readonly Context: CheckoutContext;
    readonly Event: CheckoutEvent;
    readonly Input: void;
    readonly Output: Receipt;
    readonly Effect: void;
  }
  // 🧾️ Hand-compiled TS twin of the Rust `checkout_integration` module's `statechart!` DSL machine —
  // TS has no `statechart!`/`export_wasm_machine!` derive macros, so its dense node/transition tables
  // are authored directly (see the twin's WasmBridge decision in the ticket report). Node ids: 0 root,
  // 1 cart, 2 payment, 3 selecting, 4 processing, 5 failed, 6 payment_history, 7 fulfilment,
  // 8 shipping, 9 ship_pending, 10 ship_done, 11 invoicing, 12 invoice_pending, 13 invoice_done, 14 done.
  const CHECKOUT_MACHINE: Machine<CheckoutSpec> = {
    definition: {
      id: "checkout",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2), NodeId(7), NodeId(14)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "cart", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "payment", kind: "compound", parent: NodeId(0), initial: NodeId(3), children: [NodeId(3), NodeId(4), NodeId(5), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "selecting", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "processing", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [InvokeId(0)], timers: [[TimerId(0), 5000]], docIndex: 4 },
        { stableId: "failed", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
        { stableId: "payment_history", kind: "historyShallow", parent: NodeId(2), initial: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
        { stableId: "fulfilment", kind: "parallel", parent: NodeId(0), children: [NodeId(8), NodeId(11)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
        { stableId: "shipping", kind: "compound", parent: NodeId(7), initial: NodeId(9), children: [NodeId(9), NodeId(10)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 },
        { stableId: "ship_pending", kind: "atomic", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 9 },
        { stableId: "ship_done", kind: "final", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 10 },
        { stableId: "invoicing", kind: "compound", parent: NodeId(7), initial: NodeId(12), children: [NodeId(12), NodeId(13)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 11 },
        { stableId: "invoice_pending", kind: "atomic", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 12 },
        { stableId: "invoice_done", kind: "final", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 13 },
        { stableId: "done", kind: "final", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 14 },
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(1), trigger: { kind: "event", event: EventId(6) }, targets: [NodeId(6)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(3), trigger: { kind: "event", event: EventId(1) }, guard: GuardId(0), targets: [NodeId(4)], kind: "external", actions: [ActionId(0)], docIndex: 2 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(7)], kind: "external", actions: [], docIndex: 3 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(5) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 5 },
        { source: NodeId(4), trigger: { kind: "timer", timer: TimerId(0) }, targets: [NodeId(5)], kind: "external", actions: [ActionId(1)], docIndex: 6 },
        { source: NodeId(5), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 7 },
        { source: NodeId(9), trigger: { kind: "event", event: EventId(7) }, targets: [NodeId(10)], kind: "external", actions: [], docIndex: 8 },
        { source: NodeId(12), trigger: { kind: "event", event: EventId(8) }, targets: [NodeId(13)], kind: "external", actions: [], docIndex: 9 },
        { source: NodeId(7), trigger: { kind: "done", node: NodeId(7) }, targets: [NodeId(14)], kind: "external", actions: [], docIndex: 10 },
      ],
      contextFromInput: () => ({ attempts: 0, methodSet: false }),
      makeOutput: (ctx) => ({ attempts: ctx.attempts }),
      guards: [(ctx) => ctx.attempts < 3],
      actions: [(ctx) => (ctx.methodSet = true), (ctx) => (ctx.attempts += 1)],
      fingerprint: 100n,
      manifestJson: "{}",
    },
  };

  //#endregion 🔄️MachineFixtures
  describe("machine: TestHost", () => {
    it("advance fires due timers only", () => {
      const host = new TestHost<UnitToggleSpec>();
      host.schedule(ActorId(0), TimerId(0), 100);
      host.schedule(ActorId(0), TimerId(1), 300);
      expect(host.advance(150)).toEqual([[0, 0]]);
      expect(host.advance(200)).toEqual([[0, 1]]);
    });

    it("cancelTimer removes pending", () => {
      const host = new TestHost<UnitToggleSpec>();
      host.schedule(ActorId(0), TimerId(0), 100);
      host.cancelTimer(ActorId(0), TimerId(0));
      expect(host.advance(200)).toEqual([]);
    });

    it("records effects and task lifecycle", () => {
      const host = new TestHost<{ Context: void; Event: UnitFlipEvent; Input: void; Output: void; Effect: string }>();
      host.executeEffect(ActorId(0), "audit");
      expect(host.effects()).toEqual([[0, "audit"]]);
      host.startTask(ActorId(0), InvokeId(0));
      expect(host.startedTasks()).toEqual([[0, 0]]);
      host.cancelTask(ActorId(0), InvokeId(0));
      expect(host.startedTasks()).toEqual([]);
      expect(host.cancelledTasks()).toEqual([[0, 0]]);
    });
  });

  describe("machine: TraceInspector", () => {
    it("records one microstep per transition", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const inspector = new TraceInspector<UnitToggleSpec>();
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(inspector.entries.length).toBe(2);
      expect(inspector.entries[0]!.exited).toEqual([NodeId(1)]);
      expect(inspector.entries[0]!.entered).toEqual([NodeId(2)]);
      expect(inspector.entries[1]!.exited).toEqual([NodeId(2)]);
      expect(inspector.entries[1]!.entered).toEqual([NodeId(1)]);
    });
  });

  describe("machine: kernel", () => {
    it("flat machine toggles and counts", () => {
      const sink: Command<ToggleSpec>[] = [];
      const snapshot = init(TOGGLE_MACHINE, true, sink);
      expect(snapshot.matches("off")).toBe(true);
      const inspector = new NullInspector<ToggleSpec>();
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      expect(snapshot.context.count).toBe(1);
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("off")).toBe(true);
      expect(snapshot.context.count).toBe(2);
    });

    it("guard blocks transition when false", () => {
      const sink: Command<ToggleSpec>[] = [];
      const snapshot = init(TOGGLE_MACHINE, false, sink);
      const inspector = new NullInspector<ToggleSpec>();
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      expect(snapshot.context.count).toBe(1);
    });

    it("hierarchical machine enters default descendant", () => {
      const sink: Command<PlayerSpec>[] = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      expect(snapshot.matches("closed")).toBe(true);
      expect(snapshot.matches("open")).toBe(false);
    });

    it("hierarchical machine transitions into compound default", () => {
      const sink: Command<PlayerSpec>[] = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, new NullInspector());
      expect(snapshot.matches("open")).toBe(true);
      expect(snapshot.matches("playing")).toBe(true);
    });

    it("shallow history restores last active child", () => {
      const sink: Command<PlayerSpec>[] = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      const inspector = new NullInspector<PlayerSpec>();
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, inspector);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("pause"), sink, inspector);
      expect(snapshot.matches("paused")).toBe(true);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("stop"), sink, inspector);
      expect(snapshot.matches("closed")).toBe(true);
      expect(snapshot.matches("open")).toBe(false);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("resume"), sink, inspector);
      expect(snapshot.matches("open")).toBe(true);
      expect(snapshot.matches("paused")).toBe(true);
      expect(snapshot.matches("playing")).toBe(false);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("play"), sink, inspector);
      expect(snapshot.matches("playing")).toBe(true);
      expect(snapshot.matches("paused")).toBe(false);
    });

    it("parallel regions enter together", () => {
      const sink: Command<RecorderSpec>[] = [];
      const snapshot = init(RECORDER_MACHINE, undefined, sink);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, new NullInspector());
      expect(snapshot.matches("recording")).toBe(true);
      expect(snapshot.matches("audio.capturing")).toBe(true);
      expect(snapshot.matches("video.capturing")).toBe(true);
    });

    it("parallel done bubbles only once every region finishes", () => {
      const sink: Command<RecorderSpec>[] = [];
      const snapshot = init(RECORDER_MACHINE, undefined, sink);
      const inspector = new NullInspector<RecorderSpec>();
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, inspector);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("audioStop"), sink, inspector);
      expect(snapshot.matches("audio.done")).toBe(true);
      expect(snapshot.matches("recording")).toBe(true);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("videoStop"), sink, inspector);
      expect(snapshot.matches("idle")).toBe(true);
      expect(snapshot.matches("recording")).toBe(false);
    });
  });

  describe("machine: persist/restore", () => {
    it("persist then restore round-trips active state", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, new NullInspector());
      expect(snapshot.matches("on")).toBe(true);

      const persisted = persist(UNIT_TOGGLE_MACHINE, snapshot);
      expect(persisted.fingerprint).toBe(UNIT_TOGGLE_MACHINE.definition.fingerprint);
      expect(persisted.states).toContain("on");

      const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("on")).toBe(true);
    });

    it("restore rejects fingerprint mismatch without migration", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const persisted: PersistedSnapshot = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
      const result = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
      expect(result).toEqual({ ok: false, error: { kind: "fingerprintMismatch" } });
    });

    it("restore applies migration chain until fingerprint matches", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const persisted: PersistedSnapshot = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
      const migration: Migration = { sourceFingerprint: 9999n, migrate: (s) => ({ ...s, fingerprint: UNIT_TOGGLE_MACHINE.definition.fingerprint }) };
      const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, [migration]);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("off")).toBe(true);
    });
  });

  describe("machine: ActorSystem", () => {
    it("drains sent events through one macrostep each", () => {
      const system = new ActorSystem<UnitToggleSpec>(new TestHost<UnitToggleSpec>(), UNIT_TOGGLE_MACHINE);
      const root = system.spawnRoot(undefined);
      expect(system.snapshot(root)!.matches("off")).toBe(true);

      system.send(root, UNIT_FLIP);
      const reports = system.drain();
      expect(reports.length).toBe(1);
      expect(system.snapshot(root)!.matches("on")).toBe(true);

      system.send(root, UNIT_FLIP);
      system.drain();
      expect(system.snapshot(root)!.matches("off")).toBe(true);
      expect(system.snapshot(root)!.context).toEqual({ count: 0 });
    });
  });

  describe("machine: testing (Model/Coverage/Invariant/Conformance)", () => {
    it("explore reaches both toggle states", () => {
      const model = new Model<UnitToggleSpec>([UNIT_FLIP]);
      const coverage = explore(UNIT_TOGGLE_MACHINE, model, undefined);
      expect(coverage.reachedStableIds).toContain("off");
      expect(coverage.reachedStableIds).toContain("on");
      expect(coverage.visitedConfigurations).toBe(2);
    });

    it("conformance fixture passes for matching sequence", () => {
      const steps: ConformanceStep<UnitToggleSpec>[] = [
        { event: UNIT_FLIP, expectActive: ["on"] },
        { event: UNIT_FLIP, expectActive: ["off"] },
      ];
      expect(runConformance(UNIT_TOGGLE_MACHINE, undefined, steps).ok).toBe(true);
    });

    it("conformance fixture fails with a descriptive message", () => {
      const steps: ConformanceStep<UnitToggleSpec>[] = [{ event: UNIT_FLIP, expectActive: ["off"] }];
      const result = runConformance(UNIT_TOGGLE_MACHINE, undefined, steps);
      expect(result.ok).toBe(false);
      expect(!result.ok && result.error.message).toContain("step 0");
      expect(!result.ok && result.error.message).toContain("off");
    });

    it("invariant reports violation by name", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const invariants: Invariant<UnitToggleSpec>[] = [{ name: "never off", check: (s) => (s.matches("off") ? { ok: false, error: { kind: "violation", message: "was off" } } : { ok: true }) }];
      expect(checkInvariants(snapshot, invariants)).toEqual(["never off: was off"]);
    });
  });

  describe("machine: BitSet", () => {
    it("set/clear/contains", () => {
      const bits = new BitSet();
      expect(bits.contains(NodeId(3))).toBe(false);
      bits.set(NodeId(3));
      expect(bits.contains(NodeId(3))).toBe(true);
      bits.clear(NodeId(3));
      expect(bits.contains(NodeId(3))).toBe(false);
    });

    it("iterOnes ascends regardless of insertion order", () => {
      const bits = new BitSet();
      bits.set(NodeId(100));
      bits.set(NodeId(0));
      bits.set(NodeId(64));
      bits.set(NodeId(63));
      expect([...bits.iterOnes()]).toEqual([0, 63, 64, 100]);
    });

    it("clearAll and isEmpty", () => {
      const bits = new BitSet();
      expect(bits.isEmpty()).toBe(true);
      bits.set(NodeId(5));
      expect(bits.isEmpty()).toBe(false);
      bits.clearAll();
      expect(bits.isEmpty()).toBe(true);
    });
  });

  // 🎫️ End-to-end proof that the hand-compiled `checkout` DSL twin → kernel → runtime → `TestHost`
  // timers/invoke → persist/restore → inspection trace → model coverage all compose over one real
  // machine, mirroring Rust's `checkout_integration` module.
  describe("machine: checkout DSL twin (integration)", () => {
    it("walks cart to receipt", () => {
      const host = new TestHost<CheckoutSpec>();
      const system = new ActorSystem<CheckoutSpec>(host, CHECKOUT_MACHINE);
      const root = system.spawnRoot(undefined);
      expect(system.snapshot(root)!.matches("cart")).toBe(true);

      system.send(root, new CheckoutEvent("confirm"));
      system.drain();
      expect(system.snapshot(root)!.matches("selecting")).toBe(true);

      system.send(root, new CheckoutEvent("selectMethod"));
      system.drain();
      expect(system.snapshot(root)!.matches("processing")).toBe(true);
      expect(system.snapshot(root)!.context.methodSet).toBe(true);
      expect(host.startedTasks()).toEqual([[root, 0]]);

      system.send(root, new CheckoutEvent("paymentSucceeded"));
      system.drain();
      expect(system.snapshot(root)!.matches("ship_pending")).toBe(true);
      expect(system.snapshot(root)!.matches("invoice_pending")).toBe(true);
      expect(host.cancelledTasks()).toEqual([[root, 0]]);

      system.send(root, new CheckoutEvent("shipDone"));
      system.drain();
      expect(system.snapshot(root)!.matches("ship_done")).toBe(true);
      expect(system.snapshot(root)!.matches("invoice_pending")).toBe(true);
      system.send(root, new CheckoutEvent("invoiceDone"));
      system.drain();

      const finalStatus = system.snapshot(root)!.status;
      expect(finalStatus.kind).toBe("done");
      expect(finalStatus.kind === "done" && finalStatus.output.attempts).toBe(0);
    });

    it("cancel/resume round-trips via shallow history", () => {
      const sink: Command<CheckoutSpec>[] = [];
      const snapshot = init(CHECKOUT_MACHINE, undefined, sink);
      const inspector = new TraceInspector<CheckoutSpec>();

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("confirm"), sink, inspector);
      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("selectMethod"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("cancel"), sink, inspector);
      expect(snapshot.matches("cart")).toBe(true);
      expect(snapshot.matches("payment")).toBe(false);

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("resume"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);
      expect(snapshot.matches("selecting")).toBe(false);
      expect(inspector.entries.length).toBeGreaterThan(0);

      const fired = timerElapsed(CHECKOUT_MACHINE, snapshot, TimerId(0), sink, inspector);
      expect(fired.microsteps).toBe(1);
      expect(snapshot.matches("failed")).toBe(true);
      expect(snapshot.context.attempts).toBe(1);

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("retry"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);

      const persisted = persist(CHECKOUT_MACHINE, snapshot);
      expect(persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
      const restored = restore(CHECKOUT_MACHINE, persisted, { ...snapshot.context }, []);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("processing")).toBe(true);
    });

    it("model coverage reaches every declared state", () => {
      const model = new Model<CheckoutSpec>(
        (["confirm", "selectMethod", "paymentSucceeded", "paymentFailed", "retry", "cancel", "resume", "shipDone", "invoiceDone"] as const).map((type) => new CheckoutEvent(type)),
      );
      const coverage = explore(CHECKOUT_MACHINE, model, undefined);
      for (const expected of ["cart", "selecting", "processing", "failed", "ship_pending", "ship_done", "invoice_pending", "invoice_done", "done"]) {
        expect(coverage.reachedStableIds).toContain(expected);
      }
    });

    //#region 🔖️StepTests
    it("start produces a persistable initial configuration", () => {
      const initial = start(CHECKOUT_MACHINE, undefined);
      expect(initial.isActive("cart")).toBe(true);
      expect(initial.entered).toEqual([]);
      expect(initial.persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
    });

    it("step round-trips through persisted state only", () => {
      let carried = start(CHECKOUT_MACHINE, undefined).persisted;
      let context: CheckoutContext = { attempts: 0, methodSet: false };
      for (const [type, expected] of [
        ["confirm", "selecting"],
        ["selectMethod", "processing"],
      ] as const) {
        const outcome = step(CHECKOUT_MACHINE, carried, context, new CheckoutEvent(type), []);
        expect(outcome.ok).toBe(true);
        if (!outcome.ok) continue;
        expect(outcome.step.isActive(expected)).toBe(true);
        context = { ...context, methodSet: true };
        carried = outcome.step.persisted;
      }
    });

    it("step reports entered and exited states", () => {
      const initial = start(CHECKOUT_MACHINE, undefined).persisted;
      const outcome = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(outcome.ok).toBe(true);
      if (!outcome.ok) return;
      expect(outcome.step.exited).toContain("cart");
      expect(outcome.step.entered).toContain("payment");
      expect(outcome.step.entered).toContain("selecting");
      expect(outcome.step.isActive("cart")).toBe(false);
    });

    it("step with a blocked guard leaves the configuration untouched", () => {
      const initial = start(CHECKOUT_MACHINE, undefined).persisted;
      const confirmed = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(confirmed.ok).toBe(true);
      if (!confirmed.ok) return;
      const blocked = step(CHECKOUT_MACHINE, confirmed.step.persisted, { attempts: 3, methodSet: false }, new CheckoutEvent("selectMethod"), []);
      expect(blocked.ok).toBe(true);
      if (!blocked.ok) return;
      expect(blocked.step.isActive("selecting")).toBe(true);
      expect(blocked.step.entered).toEqual([]);
    });

    it("step rejects a persisted snapshot from another machine shape", () => {
      const foreign: PersistedSnapshot = { ...start(CHECKOUT_MACHINE, undefined).persisted };
      const mismatched: PersistedSnapshot = { ...foreign, fingerprint: foreign.fingerprint ^ 0xffffffffn };
      const outcome = step(CHECKOUT_MACHINE, mismatched, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(outcome.ok).toBe(false);
    });
    //#endregion 🔖️StepTests
  });
}
