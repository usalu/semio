/** 🔎️ Statechart inspection: the inspection events, the `Inspector` port, the null inspector and the microstep trace recorder. TypeScript twin of `🔎️inspect/🦀️.rs`. */

import { type Command, type MachineSpec, NodeId } from "../🟦️.ts";

//#region 🔎️Inspection
/** 🔎 One structured observation emitted while a macrostep runs to completion. */
export type InspectionEvent<M extends MachineSpec> =
  | { readonly kind: "macrostepStart" }
  | { readonly kind: "microstep"; readonly exited: readonly NodeId[]; readonly entered: readonly NodeId[] }
  | { readonly kind: "commandIssued"; readonly command: Command<M> }
  | { readonly kind: "settled"; readonly microsteps: number };

/** 🔎 Observer of {@link InspectionEvent}s — implemented by hosts/tooling that need microstep visibility. */
export interface Inspector<M extends MachineSpec> {
  observe(event: InspectionEvent<M>): void;
}

/** 🔎 An {@link Inspector} that discards every event — the default for callers that don't need tracing. */
export class NullInspector<M extends MachineSpec> implements Inspector<M> {
  observe(): void {}
}

/** 🔎 One recorded microstep — the exited/entered node sets, in kernel-execution order. */
export interface MicrostepTrace {
  readonly exited: readonly NodeId[];
  readonly entered: readonly NodeId[];
}

/** 🔎 An {@link Inspector} that records every microstep for later assertion/replay. */
export class TraceInspector<M extends MachineSpec> implements Inspector<M> {
  readonly entries: MicrostepTrace[] = [];

  observe(event: InspectionEvent<M>): void {
    if (event.kind === "microstep") this.entries.push({ exited: event.exited, entered: event.entered });
  }
}
//#endregion 🔎️Inspection
