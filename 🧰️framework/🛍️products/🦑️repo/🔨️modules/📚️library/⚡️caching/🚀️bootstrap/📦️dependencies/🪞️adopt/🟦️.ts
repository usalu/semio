import {compileJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {generationCanonicalJsonV1,generationSha256V1} from "../🧾️generation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.$defs.Protocol.const,adoptSchema=compileJsonSchemaSubset(schema.$defs.AdoptReceipt as never,schema as never);
const compareBytes=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));

/** 🪞️ Names the closed neutral contract (the schema) every adoption receipt, projection and clone follows. */
export const adoptProtocolV1=protocol;
/** 🧬️ Is the runtime a generation was installed by; a donor is only admitted for the identical runtime. */
export interface AdoptRuntimeV1{readonly bun:string;readonly platform:"darwin"|"linux"|"win32";readonly arch:string;readonly recipeSha256:string}
/** 📄️ Is one staged input file of a generation: its absolute or generation-relative posix path and exact bytes. */
export interface AdoptFileV1{readonly path:string;readonly bytes:Uint8Array}
/** 🧾️ Is the sealed claim that a complete installed generation was produced for exactly this install projection by this runtime. */
export interface AdoptReceiptV1{readonly version:1;readonly purpose:"dependency-adopt-receipt";readonly generation:string;readonly projectionSha256:string;readonly runtime:AdoptRuntimeV1;readonly installed:string;readonly sha256:string}

/** ✂️ Drops the install-inert top-level fields of a package manifest and renders it canonically; every other file stays byte exact. */
export function installProjectionBytesV1(path:string,bytes:Uint8Array):Uint8Array{
 if(path!=="package.json"&&!path.endsWith("/package.json"))return bytes;
 const document=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes))as unknown;
 if(document===null||typeof document!=="object"||Array.isArray(document))return bytes;
 const inert=new Set<string>(protocol.adopt.inertManifestFields),kept=Object.fromEntries(Object.entries(document as Record<string,unknown>).filter(([key])=>!inert.has(key)));
 return Buffer.from(generationCanonicalJsonV1(kept));
}
/** 🔑️ Digests the install projection of a staged generation: sorted paths with the digests of their projected bytes. */
export function installProjectionSha256V1(files:readonly AdoptFileV1[]):string{
 const rows=files.map(file=>[file.path,generationSha256V1(installProjectionBytesV1(file.path,file.bytes))]as const).sort((left,right)=>compareBytes(left[0],right[0]));
 if(new Set(rows.map(row=>row[0])).size!==rows.length)throw Error("Distinct staged generation files required");
 return generationSha256V1(JSON.stringify(rows));
}
/** 🔏️ Seals an adoption receipt. */
export const sealAdoptReceiptV1=(body:Omit<AdoptReceiptV1,"sha256">):AdoptReceiptV1=>Object.freeze({...body,sha256:generationSha256V1(generationCanonicalJsonV1(body))});
/** 🔓️ Admits only a schema-valid, correctly sealed adoption receipt. */
export function parseAdoptReceiptV1(bytes:Uint8Array):AdoptReceiptV1{
 if(bytes.length>protocol.adopt.maximumReceiptBytes)throw Error("Bounded adoption receipt required");
 const value=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes))as AdoptReceiptV1&Record<string,unknown>,errors=adoptSchema.validate(value);
 if(errors.length)throw Error("Closed adoption receipt required",{cause:errors.slice(0,4)});
 const{sha256,...body}=value;if(generationSha256V1(generationCanonicalJsonV1(body))!==sha256)throw Error("Sealed adoption receipt required");
 return Object.freeze(value);
}
/** ⚖️ Decides whether a receipt describes an earlier generation that may donate its installed trees to the wanted one. */
export const adoptDonorQualifiesV1=(receipt:AdoptReceiptV1,wanted:Readonly<{generation:string;projectionSha256:string;runtime:AdoptRuntimeV1}>):boolean=>
 receipt.generation!==wanted.generation&&receipt.projectionSha256===wanted.projectionSha256&&receipt.runtime.bun===wanted.runtime.bun&&receipt.runtime.platform===wanted.runtime.platform&&receipt.runtime.arch===wanted.runtime.arch&&receipt.runtime.recipeSha256===wanted.runtime.recipeSha256;
/** 🌳️ Lists the generation-relative directories the installer produces and adoption clones. */
export const adoptClonedRootsV1=(members:number):readonly string[]=>Object.freeze([protocol.adopt.clonedRoots.root,...Array.from({length:members},(_,index)=>protocol.adopt.clonedRoots.member.replace("{index}",String(index)))]);
/** 🦜️ Names the copy-on-write clone command of a platform, or undefined where the portable in-process copy applies. */
export function adoptCloneCommandV1(platform:string,source:string,target:string):readonly string[]|undefined{
 const prefix=(protocol.adopt.clone as Readonly<Record<string,readonly string[]|undefined>>)[platform];
 return prefix?Object.freeze([...prefix,source,target]):undefined;
}
