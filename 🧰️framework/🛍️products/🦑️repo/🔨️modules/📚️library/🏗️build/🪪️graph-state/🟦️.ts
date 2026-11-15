import {createHash} from "node:crypto";
import {ROSTER_RULES_V1} from "../🕸️graph/🧭️roster/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🗣️ The English and German texts of the lifecycle messages, keyed by message.
 * @see ./🧬️schema/🔣️.json */
export const GRAPH_STATE_TEXTS_V1=schema.definitions.TextCatalogue.const;

/** 💬️ Fills one message of the catalogue in one language. */
export function graphStateTextV1(key:keyof typeof GRAPH_STATE_TEXTS_V1,language:"en"|"de",fields:Readonly<Record<string,string|number>>={}):string{
 return GRAPH_STATE_TEXTS_V1[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??""));
}

/** 🪪️ The authored policy of the graph state identity.
 * @see ./🧬️schema/🔣️.json */
export const GRAPH_STATE_POLICY_V1=schema.definitions.Policy.const;
const policy=GRAPH_STATE_POLICY_V1,extensions=new Set(policy.contentExtensions),basenames=new Set(policy.contentBasenames),MASK=(1n<<256n)-1n,HEX=/^[0-9a-f]{64}$/u;

/** 🧭️ The kinds of entry the fleet exchanges: the portable graph of the build and the project graph Nx derives from it. */
export type GraphStateKindV1="portable-graph"|"nx-graph"|"project-inference";
/** 🖼️ Everything of the identity that is not a file of the shared worktree. */
export type GraphStateFrameV1=Readonly<{kind:GraphStateKindV1;tooling:string;implementation:string;platform:string;extra:Readonly<Record<string,string>>}>;
/** 🪪️ One identity of the inputs: the two order independent folds, their counts and the digest over everything. */
export type GraphStateV1=Readonly<{version:1;frame:GraphStateFrameV1;names:string;content:string;files:number;contentFiles:number;digest:string}>;
/** 📜️ One shared file as the replica or a tree walk knows it: the repository relative path with `/` and the SHA-256 of its bytes. */
export type SharedFileV1=readonly [path:string,sha256:string];
/** 🗂️ The three classes of path: invisible to the graph, visible by name only, and read by content. */
export type GraphInputClassV1="ignored"|"name"|"content";

const encoder=new TextEncoder();
const walk=ROSTER_RULES_V1.descends;
const ancestors=new Map<string,boolean>();

/** 🚶️ Whether the graph walk can reach the directory `path` (every segment passes the roster rule); memoised per directory. */
function reachable(directory:string):boolean{
 if(directory==="")return true;
 const known=ancestors.get(directory);if(known!==undefined)return known;
 const cut=directory.lastIndexOf("/"),parent=cut<0?"":directory.slice(0,cut),name=cut<0?directory:directory.slice(cut+1),answer=reachable(parent)&&walk(name,directory,parent);
 if(ancestors.size>262144)ancestors.clear();
 ancestors.set(directory,answer);return answer;
}

/** 🧭️ Classifies one repository relative path: `ignored` when the walk never enters its directory, `content` when an inference can read its bytes (the authored extensions and file names of the policy), `name` otherwise. The classification derives the walk from the same roster rules the build graph uses, so the two cannot drift. */
export function classifyGraphInputV1(path:string):GraphInputClassV1{
 const cut=path.lastIndexOf("/");
 if(!reachable(cut<0?"":path.slice(0,cut)))return "ignored";
 const base=cut<0?path:path.slice(cut+1);
 if(basenames.has(base))return "content";
 const dot=base.lastIndexOf(".");
 return dot>0&&extensions.has(base.slice(dot))?"content":"name";
}

/** 🔡️ The 256-bit value of a hex digest. */
const numberOf=(hex:string):bigint=>BigInt("0x"+hex);
/** 🔣️ A fixed-width hex form of a 256-bit value. */
const hexOf=(value:bigint):string=>value.toString(16).padStart(64,"0");
/** 🧮️ The term of one path (and value) in a fold. */
const term=(path:string,value?:string):bigint=>numberOf(createHash("sha256").update(encoder.encode(value===undefined?path:path+"\0"+value)).digest("hex"));

/** ➕ An order independent fold: the sum modulo 2^256 of the SHA-256 terms. A change of one entry is one subtraction and one addition, so the fold of a new state follows from the fold of the old one and the list of changed paths. */
export class GraphStateFoldV1{
 private sum=0n;
 /** 📊️ How many terms the fold holds. */
 count=0;
 /** ➕ Adds one path, or one path with its value. */
 add(path:string,value?:string):void{this.sum=(this.sum+term(path,value))&MASK;this.count++;}
 /** ➖ Removes a term added earlier. */
 remove(path:string,value?:string):void{this.sum=(this.sum-term(path,value))&MASK;this.count--;}
 /** 🏷️ The current fold as 64 hex digits. */
 hex():string{return hexOf(this.sum);}
}

/** 🛡️ Checks the frame once and returns its canonical form (keys of `extra` sorted). */
export function graphStateFrameV1(frame:GraphStateFrameV1):GraphStateFrameV1{
 if(frame.kind!=="portable-graph"&&frame.kind!=="nx-graph"&&frame.kind!=="project-inference")throw Error("Graph state kind required / Art des Graph-Zustands erforderlich");
 if(!HEX.test(frame.tooling)||!HEX.test(frame.implementation))throw Error("Tooling and implementation digests required / Digests von Werkzeug und Implementierung erforderlich");
 if(!/^(neutral|[a-z0-9]+-[a-z0-9_]+)$/u.test(frame.platform))throw Error("Platform tag required / Plattform-Kennung erforderlich");
 const extra:Record<string,string>={};
 for(const key of Object.keys(frame.extra).sort()){if(!HEX.test(frame.extra[key]!))throw Error("Extra digest must be hex: "+key);extra[key]=frame.extra[key]!;}
 return Object.freeze({kind:frame.kind,tooling:frame.tooling,implementation:frame.implementation,platform:frame.platform,extra:Object.freeze(extra)});
}

/** 🧮️ Seals the two folds and the frame into one state. */
export function sealGraphStateV1(frame:GraphStateFrameV1,names:GraphStateFoldV1,content:GraphStateFoldV1):GraphStateV1{
 const canonical=graphStateFrameV1(frame),digest=createHash("sha256").update(JSON.stringify([policy.domain,canonical.kind,canonical.tooling,canonical.implementation,canonical.platform,Object.entries(canonical.extra),names.hex(),content.hex(),names.count,content.count])).digest("hex");
 return Object.freeze({version:1,frame:canonical,names:names.hex(),content:content.hex(),files:names.count,contentFiles:content.count,digest});
}

/** 🪪️ Computes the state of a list of shared files: every non-ignored path enters the name fold, every content path enters the content fold with its SHA-256. The list needs no order, no deduplication beyond the caller's (a path listed twice is counted twice) and no file system access. */
export function graphStateOfV1(frame:GraphStateFrameV1,files:Iterable<SharedFileV1>):GraphStateV1{
 const names=new GraphStateFoldV1(),content=new GraphStateFoldV1();
 for(const [path,sha] of files){
  const kind=classifyGraphInputV1(path);
  if(kind==="ignored")continue;
  names.add(path);
  if(kind==="content"){if(!HEX.test(sha))throw Error("Digest of a content file required: "+path);content.add(path,sha);}
 }
 return sealGraphStateV1(frame,names,content);
}

/** 🔁️ The state after a few files changed, without a pass over the unchanged ones. `before` and `after` list the changed paths with their old and new digest (`undefined` for absent). The result equals `graphStateOfV1` over the whole new list. */
export function graphStateAfterV1(previous:Readonly<{frame:GraphStateFrameV1;names:GraphStateFoldV1;content:GraphStateFoldV1}>,changes:Iterable<Readonly<{path:string;before:string|undefined;after:string|undefined}>>):GraphStateV1{
 const names=previous.names,content=previous.content;
 for(const change of changes){
  const kind=classifyGraphInputV1(change.path);
  if(kind==="ignored")continue;
  if(change.before!==undefined){names.remove(change.path);if(kind==="content")content.remove(change.path,change.before);}
  if(change.after!==undefined){names.add(change.path);if(kind==="content")content.add(change.path,change.after);}
 }
 return sealGraphStateV1(previous.frame,names,content);
}

/** 🏷️ The platform tag of this process: `<os>-<arch>` in lower case, as the build names native targets. */
export function graphPlatformV1(platform:string=process.platform,arch:string=process.arch):string{return `${platform}-${arch}`.toLowerCase();}

/** 🔐️ The slot a graph entry belongs to: one per kind, platform and tooling, so a newer state supersedes the older ones of its slot in the retention of the cache server. */
export function graphStateSlotParts(frame:GraphStateFrameV1):readonly string[]{return [frame.kind,frame.platform,frame.tooling];}
