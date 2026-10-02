/** 🪐️ Artifact and snapshot share the same literal persisted Space fields. */
import{parseSSpaceSnapshot,type SSpaceSnapshot}from"./📸️snapshot/🟦️.ts";
export{parseSpaceTimestamp,parseSpaceArtifactDialect,parseSpaceArtifactRow}from"./📸️snapshot/🟦️.ts";
export type{SSpaceSnapshot,SpaceArtifactDialect,SpaceArtifactRow}from"./📸️snapshot/🟦️.ts";
export type{SSpaceDiff}from"./🔺️diff/🟦️.ts";
export type{SSpaceMutation}from"./🧬️mutations/🟦️.ts";
export interface SSpaceArtifact extends SSpaceSnapshot{}
/** 📇️ Preserve the owning artifact's full persisted metadata. */
export function parseSSpaceArtifact(value:unknown,at="$"):SSpaceArtifact{return parseSSpaceSnapshot(value,at);}
