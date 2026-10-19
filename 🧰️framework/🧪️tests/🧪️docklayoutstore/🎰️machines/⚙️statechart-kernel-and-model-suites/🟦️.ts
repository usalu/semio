/** ⚙️ The TestHost, TraceInspector, kernel, persistence, ActorSystem, model testing and BitSet machine suites with their unit toggle, toggle, player and recorder fixture machines. */
import type { EventId as EventIdBrand, Command, ConformanceStep, Invariant, Machine, MachineSpec, Migration, NodeDef, PersistedSnapshot, StatechartEvent, TransitionDef } from "@semio-tech/framework";
import type { MachineTestKit } from "../🧰️machine-test-kit/🟦️.ts";

/** 🧪 Registers the machine TestHost, TraceInspector, kernel, persist and restore, ActorSystem, model testing and BitSet suites. */
export function registerStatechartKernelAndModelSuites(kit: MachineTestKit): void {
  const { ActionId, ActorId, ActorSystem, BitSet, EventId, GuardId, InvokeId, Model, NodeId, NullInspector, TestHost, TimerId, TraceInspector, checkInvariants, describe, expect, explore, init, it, macrostep, persist, restore, runConformance } = kit;
  interface UnitToggleSpec extends MachineSpec {
    readonly Context: { readonly count: number };
    readonly Event: UnitFlipEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }
  interface ToggleSpec extends MachineSpec {
    readonly Context: { count: number; allow: boolean };
    readonly Event: UnitFlipEvent;
    readonly Input: boolean;
    readonly Output: void;
    readonly Effect: void;
  }
  interface PlayerSpec extends MachineSpec {
    readonly Context: Record<string, never>;
    readonly Event: PlayerEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }
  interface RecorderSpec extends MachineSpec {
    readonly Context: Record<string, never>;
    readonly Event: RecorderEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }
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
}
