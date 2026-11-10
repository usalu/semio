import schema from "./🔣️.json" with {type: "json"};

/** 🧊️ The bounds of staging and pinning (single source: the sibling schema document, definition `Policy`). */
export type RevisionPolicyV1=Readonly<{version:1;concurrency:number;maximumAttempts:number;chunkBytes:number;pinTtlMs:number;renewIntervalMs:number;maximumPaths:number;progressMs:number}>;
export const REVISION_POLICY_V1:RevisionPolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as RevisionPolicyV1);
/** 🧾️ A content address: lowercase sha256 hex. */
export type DigestV1=string;
export type EntryModeV1="file"|"executable"|"symlink";
/** 📄️ One input of a manifest: workspace-relative POSIX path, mode, content digest, byte size. */
export type ManifestEntryV1=Readonly<{path:string;mode:EntryModeV1;digest:DigestV1;size:number}>;
export type TreeEntryV1=Readonly<{digest:DigestV1;kind:EntryModeV1|"tree";name:string;size:number}>;
/** 🌳️ A tree object rooted at a workspace-relative prefix. */
export type TreeRefV1=Readonly<{path:string;tree:DigestV1;size:number}>;
/** 🧊️ An immutable snapshot of an input closure: its id is the store address of its canonical bytes. */
export type RevisionBodyV1=Readonly<{journalCursor:string;parent?:DigestV1;roots:readonly TreeRefV1[];workspace:string}>;
export type RevisionV1=RevisionBodyV1&Readonly<{id:DigestV1}>;
/** 📋️ The flat, sorted view of a revision. `digest` is the root tree digest. */
export type InputManifestV1=Readonly<{revision:DigestV1;digest:DigestV1;entries:readonly ManifestEntryV1[]}>;
export type CasRefV1=Readonly<{hash:DigestV1;size:number}>;
export type StageRequestV1=Readonly<{revision:DigestV1;paths:readonly string[]}>;
export type StageReplyV1=Readonly<{revision:DigestV1;refs:readonly CasRefV1[]}>;
/** 📌️ A holder's claim on a revision; the store keeps every object of the revision while any live pin exists. */
export type PinV1=Readonly<{version:1;revision:DigestV1;holder:string;expiresMs:number}>;
export type RevisionErrorCodeV1="unsafe-path"|"content-evicted"|"too-many-paths"|"unknown-revision"|"unsupported-entry"|"invalid-revision";
/** 🚫️ A typed, language-neutral failure; render it with `revisionTextV1`. */
export interface RevisionErrorV1 extends Error{readonly code:RevisionErrorCodeV1;readonly fields:Readonly<Record<string,string|number>>}
/** 🗣️ The EN+DE texts; the schema document is their single source. */
export const REVISION_MESSAGES_V1=Object.freeze(schema.definitions.Messages.const);

/** 🧮️ Fills a message template with values. */
export function revisionTextV1(key:keyof typeof REVISION_MESSAGES_V1,language:"en"|"de",values:Readonly<Record<string,string|number>>={}):string{
 return REVISION_MESSAGES_V1[key][language].replace(/\{(\w+)\}/gu,(_all,name:string)=>String(values[name]??""));
}

/** 🚫️ Builds a typed failure. */
export function revisionErrorV1(code:RevisionErrorCodeV1,fields:Readonly<Record<string,string|number>>={}):RevisionErrorV1{
 return Object.assign(Error(`${REVISION_MESSAGES_V1[code].en} / ${REVISION_MESSAGES_V1[code].de}`.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""))),{code,fields}) as RevisionErrorV1;
}

const PATH=new RegExp(schema.definitions.Path.pattern,"u"),DIGEST=new RegExp(schema.definitions.Digest.pattern,"u");
/** ✅️ Whether a string is a safe workspace-relative POSIX path (no `..`, no absolute, no backslash). */
export const isSafeRevisionPathV1=(path:string):boolean=>path.length>0&&path.length<=1024&&PATH.test(path);
/** ✅️ Whether a string is a sha256 hex digest. */
export const isRevisionDigestV1=(value:string):boolean=>DIGEST.test(value);
