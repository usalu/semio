/** 🏛️ Retains exact catalog declarations independently from installed codec capabilities. */
import type {DocumentOpenArtifactV1,DocumentOpenPackageV1,DocumentOpenSurfaceV1,DocumentOpenGrantV1} from "../../../📇️directory/🧬️schema/🟦️.ts";
import type {DocumentOpenBrowserActorV1} from "../../../📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts";
import {parseDocumentOpenBrowserActorV1} from "../../../📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts";
import {DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 as channelVersion} from "../../../📇️directory/🧬️schema/🟦️.ts";
export type ArtifactCatalogNativeIdentity={kind:"typed";owner:string}|{kind:"guest";pluginId:string;packageHash:string;schema:string};
export type ArtifactCatalogCapability={kind:"unlinkedGuest"}|{kind:"linked";nativeIdentity:ArtifactCatalogNativeIdentity;sqliteSchema:string|null;factory:string|null};
export interface ArtifactCatalogTarget{parentDialect:string;surface:DocumentOpenSurfaceV1;grant:DocumentOpenGrantV1;browserActor:DocumentOpenBrowserActorV1}
export interface ArtifactCatalogBinding{contributor:DocumentOpenPackageV1;owner:DocumentOpenPackageV1;artifact:DocumentOpenArtifactV1;target:ArtifactCatalogTarget|null;capability:ArtifactCatalogCapability}
/** 🗝️ Keys original declaration and exact selected surface without hiding ownership conflicts. */
export function catalogBindingKey(row:ArtifactCatalogBinding):string {
 const p=row.contributor,t=row.target;
 return JSON.stringify([p.pluginId,p.packageId,p.version,row.artifact.kind,row.artifact.schema,...(t?[t.parentDialect,t.surface.surfaceId,t.surface.appId,t.surface.windowKindId,t.surface.role,t.surface.rendererTarget]:[])]);
}
function equal(left:unknown,right:unknown):boolean{
 if(left===right)return true;
 if(!left||!right||typeof left!=="object"||typeof right!=="object"||Array.isArray(left)!==Array.isArray(right))return false;
 const a=left as Record<string,unknown>,b=right as Record<string,unknown>,keys=Object.keys(a);
 return keys.length===Object.keys(b).length&&keys.every(key=>Object.hasOwn(b,key)&&equal(a[key],b[key]));
}
function closed(value:unknown,keys:readonly string[]):value is Record<string,unknown>{return !!value&&typeof value==="object"&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));}
function text(value:unknown):value is string{return typeof value==="string"&&value.length>0&&!/[\u0000-\u001f\u007f]/u.test(value);}
function digest(value:unknown):boolean{return typeof value==="string"&&/^[0-9a-f]{64}$/u.test(value)&&value!=="0".repeat(64);}
function packageValid(value:unknown):boolean{return closed(value,["pluginId","packageId","version","componentSha256","componentBlake3","descriptorByteSha256","executionProtocol"])&&[value.pluginId,value.packageId,value.version].every(text)&&[value.componentSha256,value.componentBlake3,value.descriptorByteSha256].every(digest)&&closed(value.executionProtocol,["appChannelVersion"])&&value.executionProtocol.appChannelVersion===channelVersion;}
function valid(row:ArtifactCatalogBinding):boolean{
 if(!closed(row,["contributor","owner","artifact","target","capability"])||!packageValid(row.contributor)||!packageValid(row.owner)||!closed(row.artifact,["kind","schema","packSchemaHash"])||![row.artifact.kind,row.artifact.schema].every(text)||!digest(row.artifact.packSchemaHash))return false;
 if(row.target!==null){const t=row.target;if(!closed(t,["parentDialect","surface","grant","browserActor"])||!text(t.parentDialect)||t.parentDialect.split("@").length!==2||t.parentDialect.split("@")[1]!.split("/").length!==2||t.parentDialect.split(/[@/]/u).some(part=>!part)||!closed(t.surface,["surfaceId","appId","windowKindId","role","rendererTarget"])||![t.surface.surfaceId,t.surface.appId,t.surface.windowKindId].every(text)||t.surface.surfaceId!==t.surface.appId||!["viewer","editor"].includes(t.surface.role)||!["react","wgpu","wasm"].includes(t.surface.rendererTarget)||!closed(t.grant,["read","write","observe"])||![t.grant.read,t.grant.write,t.grant.observe].every(value=>typeof value==="boolean"))return false;try{parseDocumentOpenBrowserActorV1(t.browserActor,row.contributor,t.surface.rendererTarget);}catch{return false;}}
 const c=row.capability;if(c.kind==="unlinkedGuest")return closed(c,["kind"]);
 if(!closed(c,["kind","nativeIdentity","sqliteSchema","factory"])||c.kind!=="linked"||(c.sqliteSchema!==null&&!text(c.sqliteSchema))||(c.factory!==null&&!text(c.factory)))return false;
 const identity=c.nativeIdentity;return identity.kind==="typed"?closed(identity,["kind","owner"])&&text(identity.owner):identity.kind==="guest"&&closed(identity,["kind","pluginId","packageHash","schema"])&&identity.pluginId===row.owner.pluginId&&digest(identity.packageHash)&&identity.packageHash===row.owner.componentBlake3&&identity.schema===row.artifact.schema;
}
/** 🔐️ Refuses duplicate inputs and changed provenance before any publication occurs. */
export function proposeCatalogBindings(existing:ReadonlyMap<string,ArtifactCatalogBinding>,rows:readonly ArtifactCatalogBinding[]):Map<string,ArtifactCatalogBinding>{
 const proposed=new Map<string,ArtifactCatalogBinding>();
 for(const row of rows){if(!valid(row))throw Error("invalid original catalog declaration");const key=catalogBindingKey(row);if(proposed.has(key))throw Error("duplicate original catalog declaration");const installed=existing.get(key);if(installed&&!equal(installed,row))throw Error("conflicting original catalog provenance");proposed.set(key,structuredClone(row));}
 return proposed;
}
