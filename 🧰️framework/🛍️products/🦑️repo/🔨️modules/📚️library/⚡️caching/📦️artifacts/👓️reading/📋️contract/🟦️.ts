import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {NativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 👓️ Describes exact publication bytes only inside an accepted consumer's live control. */
export interface RepositoryArtifactV1{readonly path:string;readonly owner:string;readonly files:readonly string[];readonly control:ResourceLeaseControl;recheck():Promise<void>}
/** 📋️ Bounds an exact publication without granting reader custody through path data. */
export interface RepositoryArtifactRequestV1{readonly version:1;readonly path:string;readonly owner:string;readonly maximumBytes:number;readonly maximumFiles:number}
/** 🎟️ Captures actual storage ownership separately from its carried consumer phase. */
export interface RepositoryArtifactReaderOptionsV1{readonly workspace:string;readonly invocation:ScriptInvocation;readonly storage:NativeStorageReservationV1;readonly control:ResourceLeaseControl;readonly phaseControl?:ResourceLeaseControl;readonly request:RepositoryArtifactRequestV1}
