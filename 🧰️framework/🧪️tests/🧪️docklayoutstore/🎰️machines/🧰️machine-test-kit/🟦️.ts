/** 🧰️ The shared statechart machine test kit: the destructured framework statechart API and the unit toggle, toggle, player, recorder and checkout fixture machines. */


/** 🏭 Creates the shared machine test kit of one registration from the injected framework. */
export function createMachineTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework")) {
  const { ActionId, ActorId, ActorSystem, BitSet, EventId, GuardId, InvokeId, Model, NodeId, NullInspector, TestHost, TimerId, TraceInspector, checkInvariants, explore, init, macrostep, persist, restore, runConformance, start, step, timerElapsed } = dependencies;
  const { describe, expect, it } = vitest;
  return { ActionId, ActorId, ActorSystem, BitSet, EventId, GuardId, InvokeId, Model, NodeId, NullInspector, TestHost, TimerId, TraceInspector, checkInvariants, describe, expect, explore, init, it, macrostep, persist, restore, runConformance, start, step, timerElapsed };
}
export type MachineTestKit = ReturnType<typeof createMachineTestKit>;
