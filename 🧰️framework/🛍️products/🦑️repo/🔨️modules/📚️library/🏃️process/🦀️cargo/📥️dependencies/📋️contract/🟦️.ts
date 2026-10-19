/** 📋️ Repository Cargo dependency inputs: the contract the readiness storage and the dependency owner share, held apart so neither imports the other.
 * @see ../🟦️.ts — the owner that admits and consumes the inputs
 */
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {CargoProfileConsumptionV1} from "../../../../🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {NativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";

/** 🌐️ Retains the original caller's explicit offline grant without ambient network inference. */
export interface CargoDependencyNetworkV1 {readonly offline:boolean}
/** 📥️ Names one bounded registry owner and the exact original manifest selection. */
export interface RepositoryCargoInputsV1 {readonly invocation:ScriptInvocation;readonly consumption:CargoProfileConsumptionV1;readonly workspace:string;readonly cwd:string;readonly manifest:string;readonly args:readonly string[];readonly additionalManifests?:readonly Readonly<{manifest:string;cwd:string;args:readonly string[]}>[];readonly network:CargoDependencyNetworkV1;readonly control:ResourceLeaseControl;readonly storage:NativeStorageReservationV1;readonly environment:Readonly<Record<string,string|undefined>>}
