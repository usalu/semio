import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {INTERFACE_POLICY_V1,maskRustSourceV1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔭️interface/🟦️.ts";

/** 🧿️ The closed policy of the interface closure: which task keys a dependency crate by its interface, which files make a closure an interface closure and carry its implementation.
 * @see ./🧬️schema/🔣️.json */
export const INTERFACE_CLOSURE_POLICY_V1=schema.definitions.Policy.const;
const policy=INTERFACE_CLOSURE_POLICY_V1,sha=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");

/** 🧫️ What a type check observes of one `.rs` file, in a form small enough to keep for a whole batch: the digest of its text without the unobservable function bodies and the two reasons a file stops the whole closure from being masked. */
export type InterfaceFactsV1=Readonly<{masked:string;bodies:number;void:boolean;splice:boolean}>;

/** 🧬️ One closure member after reading: its name, the digest of its bytes (`null` when absent) and, for a present `.rs` file, its facts. */
export type InterfaceRowV1=Readonly<{name:string;sha256:string|null;facts:InterfaceFactsV1|null}>;

/** 🔬️ Reads the interface facts of one source file from its bytes; the masker is the one of the unit cache, so a type check and a unit identity agree on what a body is. */
export function interfaceFileFactsV1(bytes:Uint8Array):InterfaceFactsV1{
 const analysis=maskRustSourceV1(Buffer.from(bytes).toString("utf8"));
 return Object.freeze({masked:sha(analysis.masked),bodies:analysis.bodies,void:analysis.void,splice:analysis.splice});
}

/** 🏷️ Whether a closure roster is an interface closure: it lists the marker file. */
export function isInterfaceClosureV1(files:readonly string[]):boolean{return files.includes(policy.marker);}

/** 🧮️ The digest of an interface closure: a `.rs` file by its text without the unobservable bodies, any other file by its bytes, an absent file by its absence. One file that cannot be masked, or one that splices other files into itself, makes every `.rs` file count by its bytes. The rule is the one of {@link interfaceDigestV1}; the namespace separates the two protocols. */
export function interfaceClosureDigestV1(rows:readonly InterfaceRowV1[]):string{
 const whole=rows.some(row=>row.facts!==null&&(row.facts.void||row.facts.splice));
 const listed=rows.map(row=>{
  if(row.sha256===null)return[row.name,"missing",""];
  if(row.facts!==null&&!whole)return[row.name,"masked",row.facts.masked];
  return[row.name,"bytes",row.sha256];
 }).sort((a,b)=>a[0]!<b[0]!?-1:a[0]!>b[0]!?1:0);
 return sha(JSON.stringify([policy.namespace,INTERFACE_POLICY_V1.namespace,whole,listed]));
}

/** 🗂️ The roster of an interface closure: the exact files of the crate and the helpers of every compacted group, plus the members that carry the implementation. */
export function interfaceClosureFilesV1(files:readonly string[]):readonly string[]{return [...new Set([...files,...policy.members])].sort();}

/** 🎯️ Whether a target is keyed by the interface of its dependency crates: a `check` of production scope. A test or a build links code, so it keeps the bytes. */
export function interfaceTargetV1(name:string,target:Readonly<{metadata?:{semio?:{execution?:{scope?:string}}}}>):boolean{
 return policy.targets.names.includes(name)&&target.metadata?.semio?.execution?.scope===policy.targets.scope;
}

/** 🪢️ The dependency input of one `^nativeSources` or `^nativeTestSources` entry of a native target: the interface group of the dependency projects for the `^nativeSources` of an interface target, the source group otherwise, and nothing for a crate without path dependencies. */
export function interfaceDependencyInputV1(name:string,target:Readonly<{metadata?:{semio?:{execution?:{scope?:string}}}}>,input:unknown,dependencies:readonly string[]):readonly unknown[]{
 if(!dependencies.length)return[];
 const group=input===policy.targets.dependencyInput&&interfaceTargetV1(name,target)?policy.group:policy.targets.sourceGroup;
 return[{input:group,projects:dependencies}];
}
