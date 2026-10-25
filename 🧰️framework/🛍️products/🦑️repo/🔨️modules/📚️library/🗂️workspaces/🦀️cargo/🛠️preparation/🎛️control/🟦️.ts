/** 🦀️ Cargo preparation control: the progress and cancellation contract a controlled workspace or preparation phase runs under, held apart so custody and the owner share it without importing each other.
 * @see ../🟦️.ts — the preparation owner
 */
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {OwnedCommandOptions} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 📈️ One bounded unit of Cargo workspace work the control is told about. */
export type CargoWorkspaceProgressV1=Readonly<{operation:"state"|"presence"|"list"|"text"|"recipe";path:string;completedOperations:number}>;
/** 🎛️ Progress and cancellation of a controlled Cargo workspace operation. */
export interface CargoWorkspaceControlV1 {readonly signal:AbortSignal;advance(progress:CargoWorkspaceProgressV1):Promise<void>;}
/** 🛠️ The control of a Cargo preparation phase: the lease it runs under and the budget and options of its child processes. */
export interface CargoPreparationControlV1 extends CargoWorkspaceControlV1 {readonly leaseControl:ResourceLeaseControl;childBudget(requestedMilliseconds:number):number;childOptions(environment:Readonly<Record<string,string|undefined>>):OwnedCommandOptions&{readonly signal:AbortSignal;readonly output:NonNullable<OwnedCommandOptions["output"]>};}
