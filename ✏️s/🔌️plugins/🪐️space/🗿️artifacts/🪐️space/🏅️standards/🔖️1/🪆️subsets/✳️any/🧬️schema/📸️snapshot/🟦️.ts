/** 🪐️ Persisted Space metadata and full native unsigned64 timestamps. */
import{parseSchemaRecord}from"../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
export interface SpaceArtifactDialect{artifactKind:string;standard:string;subset:string}
export interface SpaceArtifactRow{id:string;name:string;kindId:string;schema:string;dialect:SpaceArtifactDialect;createdAtMs:bigint;createdBy:string;updatedAtMs:bigint;updatedBy:string}
export interface SSpaceSnapshot{schema:string;spaceId:string;artifacts:SpaceArtifactRow[]}
function text(value:unknown,at:string):string{if(typeof value!=="string")throw Error(at+": string required");return value;}
/** 🔢️ Bind the exact persisted unsigned64 timestamp domain. */
export function parseSpaceTimestamp(value:unknown,at="$"):bigint{if(typeof value!=="bigint"||value<0n||value>18446744073709551615n)throw Error(at+": unsigned64 bigint required");return value;}
/** 🧭️ A literal native dialect may contain empty or unregistered strings. */
export function parseSpaceArtifactDialect(value:unknown,at="$"):SpaceArtifactDialect{const row=parseSchemaRecord(value,["artifactKind","standard","subset"],at);return{artifactKind:text(row.artifactKind,at+".artifactKind"),standard:text(row.standard,at+".standard"),subset:text(row.subset,at+".subset")};}
/** 📇️ Preserve one metadata occurrence without deduplicating its business ID. */
export function parseSpaceArtifactRow(value:unknown,at="$"):SpaceArtifactRow{const row=parseSchemaRecord(value,["id","name","kindId","schema","dialect","createdAtMs","createdBy","updatedAtMs","updatedBy"],at);return{id:text(row.id,at+".id"),name:text(row.name,at+".name"),kindId:text(row.kindId,at+".kindId"),schema:text(row.schema,at+".schema"),dialect:parseSpaceArtifactDialect(row.dialect,at+".dialect"),createdAtMs:parseSpaceTimestamp(row.createdAtMs,at+".createdAtMs"),createdBy:text(row.createdBy,at+".createdBy"),updatedAtMs:parseSpaceTimestamp(row.updatedAtMs,at+".updatedAtMs"),updatedBy:text(row.updatedBy,at+".updatedBy")};}
/** 📸️ Bind the complete persisted Space index. */
export function parseSSpaceSnapshot(value:unknown,at="$"):SSpaceSnapshot{const row=parseSchemaRecord(value,["schema","spaceId","artifacts"],at);if(!Array.isArray(row.artifacts))throw Error(at+".artifacts: array required");return{schema:text(row.schema,at+".schema"),spaceId:text(row.spaceId,at+".spaceId"),artifacts:row.artifacts.map((value,index)=>parseSpaceArtifactRow(value,at+".artifacts["+index+"]"))};}
