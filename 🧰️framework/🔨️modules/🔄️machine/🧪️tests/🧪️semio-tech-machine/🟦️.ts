type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { EventId, GuardId, NodeId, ROOT, init, runConformance } = dependencies;
  type Command = any;
  type GuardFn = any;
  type Machine = any;
  type MachineSpec = any;
  type NodeDef = any;
  type StatechartEvent = any;
  type TransitionDef = any;

  const { describe, expect, it } = vitest;

  interface FlatEvent extends StatechartEvent {
    readonly kind: string;
    readonly branch: number;
  }
  const FLAT_EVENT_ID = EventId(0);
  function flatEvent(kind: string, branch = 0): FlatEvent {
    return { kind, branch, eventCount: 1, eventId: () => FLAT_EVENT_ID, eventName: () => "advance" };
  }
  interface FlatSpec extends MachineSpec {
    Context: undefined;
    Event: FlatEvent;
    Input: undefined;
    Output: never;
    Effect: never;
  }
  /** 🚦 Three-state ring, one event id, no guards — the shape a flat XState-style chart compiles to. */
  function buildTrafficLight(): Machine<FlatSpec> {
    const [RED, YELLOW, GREEN] = [NodeId(1), NodeId(2), NodeId(3)];
    const nodes: readonly NodeDef[] = [
      { stableId: "root", kind: "compound", initial: RED, children: [RED, YELLOW, GREEN], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
      { stableId: "red", kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
      { stableId: "yellow", kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
      { stableId: "green", kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
    ];
    const transitions: readonly TransitionDef[] = [
      { source: RED, trigger: { kind: "event", event: FLAT_EVENT_ID }, targets: [GREEN], kind: "external", actions: [], docIndex: 0 },
      { source: GREEN, trigger: { kind: "event", event: FLAT_EVENT_ID }, targets: [YELLOW], kind: "external", actions: [], docIndex: 1 },
      { source: YELLOW, trigger: { kind: "event", event: FLAT_EVENT_ID }, targets: [RED], kind: "external", actions: [], docIndex: 2 },
    ];
    return { definition: { id: "traffic-light", nodes, transitions, contextFromInput: () => undefined, guards: [], actions: [], fingerprint: 0n, manifestJson: "{}" } };
  }

  /** 🍴 One state with two guarded outgoing rows on the same event id — branch index picks the target,
   * mirroring how a `chevrotain`/`xstate`-shaped consumer disambiguates several transitions sharing one
   * wire-level event kind by an integer branch (see `@semio-tech/cad-js`'s stately adapter). */
  function buildBranching(): Machine<FlatSpec> {
    const [A, B, C] = [NodeId(1), NodeId(2), NodeId(3)];
    const nodes: readonly NodeDef[] = [
      { stableId: "root", kind: "compound", initial: A, children: [A, B, C], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
      { stableId: "a", kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
      { stableId: "b", kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
      { stableId: "c", kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
    ];
    const guards: GuardFn<FlatSpec>[] = [
      (_ctx, event) => event?.kind === "go" && event.branch === 0,
      (_ctx, event) => event?.kind === "go" && event.branch === 1,
    ];
    const transitions: readonly TransitionDef[] = [
      { source: A, trigger: { kind: "event", event: FLAT_EVENT_ID }, guard: GuardId(0), targets: [B], kind: "external", actions: [], docIndex: 0 },
      { source: A, trigger: { kind: "event", event: FLAT_EVENT_ID }, guard: GuardId(1), targets: [C], kind: "external", actions: [], docIndex: 1 },
    ];
    return { definition: { id: "branching", nodes, transitions, contextFromInput: () => undefined, guards, actions: [], fingerprint: 0n, manifestJson: "{}" } };
  }

  describe("@semio-tech/machine", () => {
    it("flat table-driven machine advances red -> green -> yellow -> red", () => {
      const result = runConformance(buildTrafficLight(), undefined, [
        { event: flatEvent("advance"), expectActive: ["green"] },
        { event: flatEvent("advance"), expectActive: ["yellow"] },
        { event: flatEvent("advance"), expectActive: ["red"] },
      ]);
      expect(result.ok).toBe(true);
    });

    it("guarded transitions on the same event id pick the branch whose guard passes", () => {
      const branch0 = runConformance(buildBranching(), undefined, [{ event: flatEvent("go", 0), expectActive: ["b"] }]);
      expect(branch0.ok).toBe(true);
      const branch1 = runConformance(buildBranching(), undefined, [{ event: flatEvent("go", 1), expectActive: ["c"] }]);
      expect(branch1.ok).toBe(true);
    });

    it("init enters the configured initial atomic child of the root", () => {
      const sink: Command<FlatSpec>[] = [];
      const snapshot = init(buildTrafficLight(), undefined, sink);
      expect(snapshot.matches("red")).toBe(true);
      expect(snapshot.matches("green")).toBe(false);
    });
  });

}
