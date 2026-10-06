import type {Binary64} from "../../../../../../../../🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🏃️ Authored persisted run outcomes. */
export type RunStatus="pending"|"running"|"succeeded"|"failed"|"canceled";
/** 🚦️ Authored per-node run outcomes. */
export type RunNodeStatus="computed"|"cacheHit"|"failed";
/** 🎬️ Exact mutually exclusive trigger ownership. */
export type RunTrigger={kind:"manual";actor:string}|{kind:"automation";automationRef:string;eventFingerprint:string};
/** 🔑️ Literal fingerprint carried by one ordered port. */
export interface RunPortFingerprint{portId:string;fingerprint:string}
/** 📤️ Literal artifact destination carried by one ordered output. */
export interface RunOutputArtifact{portId:string;artifactId:string;path:string}
/** 📇️ Complete persisted node ownership with exact duration. */
export interface RunNodeRecord{nodeId:string;status:RunNodeStatus;documentFingerprint:string;configFingerprint:string;inputFingerprints:RunPortFingerprint[];outputFingerprints:RunPortFingerprint[];outputs:RunOutputArtifact[];durationMs:Binary64}
/** 📜️ One ordered persisted log with unrestricted literal values. */
export interface RunLogLine{nodeId:string;level:string;message:string;at:string}
/** 🧬️ Complete run snapshot source domain at its authored scalar widths. */
export interface RunSnapshot{schema:string;workflowRef:string;workflowCheckpointId:string;inputCollectionRef:string;inputSnapshotId:string;parameterValues:{parameterId:string;value:string}[];outputCollectionRef:string;status:RunStatus;trigger:RunTrigger;nodeRecords:RunNodeRecord[];logs:RunLogLine[];startedAt:string;finishedAt?:string;sealed:boolean}
