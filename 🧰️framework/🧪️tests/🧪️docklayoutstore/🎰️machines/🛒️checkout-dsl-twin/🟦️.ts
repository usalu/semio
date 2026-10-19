/** 🛒️ The checkout DSL twin integration machine tests. */
import type { EventId as EventIdBrand, Command, Machine, MachineSpec, PersistedSnapshot, StatechartEvent } from "@semio-tech/framework";
import type { MachineTestKit } from "../🧰️machine-test-kit/🟦️.ts";

/** 🔗 Registers the machine checkout DSL twin integration tests. */
export function registerCheckoutDslTwinTests(kit: MachineTestKit): void {
  const { ActionId, ActorSystem, EventId, GuardId, InvokeId, Model, NodeId, TestHost, TimerId, TraceInspector, describe, expect, explore, init, it, macrostep, persist, restore, start, step, timerElapsed } = kit;
  type CheckoutContext = { attempts: number; methodSet: boolean };
  type Receipt = { readonly attempts: number };
  interface CheckoutSpec extends MachineSpec {
    readonly Context: CheckoutContext;
    readonly Event: CheckoutEvent;
    readonly Input: void;
    readonly Output: Receipt;
    readonly Effect: void;
  }
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
