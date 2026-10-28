/** ✏️️ Contract (derived facts included: a stale support or a named derived fact is settled by asking the derived question again, served only when the answer is the recorded one and the new support reads nothing new): a sealed memo whose journal names a relevant change re-checks exactly the facts that read the changed paths with the planner's own checks and serves when they all hold (an edit that keeps a file's imports, an atomic save, a mode change, a touched directory), and plans for every change the planner would see (an added import, a rename, a raw content change, a derived or TypeScript text fact, a new member or walked directory) and for changes it cannot prove (a path that became a symbolic link, a directory renamed over another). Oracles: the planner's own verdict on the same tree, i.e. the engine's verification of every fact (`observationsHold`) plus the membership advance of the frontier (`advanceFrontierV1`), and, on a real generation store, the planner itself (`refreshClosure` builds or not); the same lookup without the edit ports (the base behaviour) refuses every touched case; a settlement that trusts the rows or skips the frontier is wrong somewhere.
 * @see ../../🟦️.ts `settleMemoEditsV1`
 * @see ../../📤️relevance/🟦️.ts `buildGraphMemoRelevanceV1` */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {appendFileSync,chmodSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,realpathSync,renameSync,rmSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import * as observation from "../../../../../👁️observation/🟨️.mjs";
import {advanceFrontierV1,walkFrontierV1} from "../../../../../🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs";
import {GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,createFactIndexV1,createGenerationStoreV1,createNodeFilePortV1,packFactIndexV1,type FactIndexV1,type GenerationSealedV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {derivedOwnerNameV1} from "../../../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";
import {encodeGenerationPinV1} from "../../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
import {createHolds,createWorld,seed} from "../../../../../🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts";
import type {ChangeCursorV1,ChangeRowV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {readChangeStateV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1,type ChangeAnswerV1,type ChangeJournalClientV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {startChangeJournalServiceV1} from "../../../../../🏡️resident/📜️journal/🟦️.ts";
import {GRAPH_MEMO_POLICY_V1,collectGraphMemoV1,collectMemoChangesV1,openGraphMemoV1,packMemoRelevanceV1,settleMemoEditsV1,unpackMemoEditIndexV1,unpackMemoRelevanceV1,type GraphMemoEditPortsV1,type MemoFrontierV1,type MemoRowV1} from "../../🟦️.ts";
import {buildGraphMemoRelevanceV1,writeGraphMemoSidecarFilesV1,writeGraphMemoSliceSidecarV1,type MemoBuiltV1} from "../../📤️relevance/🟦️.ts";
import {memoFactsHoldV1} from "../../🚪️command/🟦️.ts";
import {answerGraphMemoDerivationV1,deriveGraphMemoSuspectsV1,graphMemoDerivationHeldV1,graphMemoDeriveAnswerFileV1,graphMemoSuspectsV1,readGraphMemoDeriveFileV1,settleGraphMemoDerivationV1,writeGraphMemoDeriveFileV1,type MemoDeriverV1} from "../../🧮️derive/🟦️.ts";

const EPOCH="5e5e5e5e5e5e5e5e",NAMES=new Set(["node_modules",".git"]),insensitive=process.platform==="darwin"||process.platform==="win32",LATER=60000;
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-edit-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
const excluded=(relative:string):boolean=>relative.split("/").some(part=>NAMES.has(part));
const imports=observation.registerSourceAnalysisV1({id:"memo-edit-law:v1",revision:"2".repeat(64),analyze:(_path:string,bytes:Buffer)=>({imports:[...bytes.toString("utf8").matchAll(/^import ["']([^"']+)["'];?$/gmu)].map(match=>match[1]!).sort()})});
const sha=(text:string|Buffer)=>createHash("sha256").update(text).digest("hex");
const descend=(name:string)=>!NAMES.has(name),member=(path:string)=>path.slice(path.lastIndexOf("/")+1)==="project.json";

/** 🧪️ The workspace of the engine law: an analysed file and its import, a probed file, an unread sibling, a file read whole, a file a derived fact names, a file read as TypeScript text, two members, a walked directory no fact lists, an excluded region. Directories are settled one minute in the past, so the frontier can stamp them. */
function fixture():{root:string;outside:string}{
 const root=join(sandbox,"w"+(counter++)),outside=root+".ext";
 for(const directory of ["src/sub","lib","docs","node_modules/pkg"])mkdirSync(join(root,directory),{recursive:true});
 mkdirSync(outside,{recursive:true});
 const files:Record<string,string>={"project.json":"{}\n","src/a.ts":"import './b.ts';\nexport const a=1;\n","src/b.ts":"export const b=2;\n","src/notes.md":"notes\n","src/sub/c.ts":"c\n","src/sub/notes.txt":"n\n","src/sub/project.json":"{\"name\":\"sub\"}\n","lib/d.ts":"import './x.ts';\nd\n","lib/x.ts":"x\n","lib/y.ts":"y\n","lib/e.ts":"e\n","docs/readme.md":"docs\n","node_modules/pkg/index.js":"module.exports=1;\n"};
 for(const [path,text] of Object.entries(files))writeFileSync(join(root,path),text);
 writeFileSync(join(outside,"a.ts"),files["src/a.ts"]!);
 const settled=new Date(Date.now()-LATER);for(const directory of [".","src","src/sub","lib","docs"])utimesSync(join(root,directory),settled,settled);
 return{root,outside};
}
const modelImports=(text:string):string[]=>[...text.matchAll(/^import ["']([^"']+)["'];?$/gmu)].map(match=>match[1]!);
/** 🧮️ The law's derived question, asked like the planner's deriver asks one (its own observation capture): how many imports of `lib/d.ts` name an existing file; it reads the file as TypeScript text and probes every import target, and fails on a `FAIL` marker. */
const deriveModel:MemoDeriverV1=async row=>{
 const [,file]=JSON.parse(row.path) as [string,string];
 const captured=await observation.captureObservedSourceInputsV1(()=>{const text=(observation.observedTypeScriptHost({readFile:(path:string)=>readFileSync(path,"utf8")} as never) as {readFile(path:string):string|undefined}).readFile(file)??"";if(text.includes("FAIL"))throw Error("derivation failed");return modelImports(text).filter(spec=>observation.observedFs.existsSync(join(dirname(file),spec))).length;},undefined,false);
 return{fact:captured.value,rows:captured.observations as unknown as MemoRowV1[]};
};
/** 🔮️ The independent answer of the derived question on the tree as it is now (plain file reads, no observation layer); `undefined` when the question fails. */
const modelAnswer=(root:string):number|undefined=>{try{const text=readFileSync(join(root,"lib/d.ts"),"utf8");return text.includes("FAIL")?undefined:modelImports(text).filter(spec=>{try{lstatSync(join(root,"lib",spec));return true;}catch{return false;}}).length;}catch{return undefined;}};
/** 📸️ The facts of one reading session, taken with the real observation module, and one derived fact (how many imports of `lib/d.ts` name an existing file) held by the project and supported by its pseudo owner with the rows its derivation read, all in a real fact index (`rows` are its rows, so a position is an id). */
async function harvest(root:string,derive:MemoDeriverV1=deriveModel):Promise<{rows:readonly MemoRowV1[];raw:readonly MemoRowV1[];index:FactIndexV1}>{
 const at=(path:string)=>join(root,path),captured=await observation.captureObservedSourceInputsV1(()=>{
  const fs=observation.observedFs;
  observation.observedSourceAnalysisV1(imports,at("src/a.ts"));fs.existsSync(at("src/a.ts"));fs.existsSync(at("src/b.ts"));fs.lstatSync(at("src/b.ts"));fs.existsSync(at("src/missing.ts"));
  fs.readdirSync(at("src"),{withFileTypes:true});fs.readdirSync(at("src/sub"));fs.readFileSync(at("src/sub/c.ts"),"utf8");fs.readFileSync(at("project.json"),"utf8");
  observation.observedRequireResolve(at("src/a.ts"),"./b.ts");
  (observation.observedTypeScriptHost({readFile:(path:string)=>readFileSync(path,"utf8")} as never) as {readFile(path:string):string|undefined}).readFile(at("lib/e.ts"));
 },undefined,false);
 const raw=captured.observations as unknown as MemoRowV1[],question={kind:"derived",path:JSON.stringify(["memo-edit-law:imports",at("lib/d.ts")]),fact:null as unknown},answer=await derive(question),derived={...question,fact:answer.fact};
 const index=createFactIndexV1(new Map([["proj",[...raw,derived]],[derivedOwnerNameV1(derived),[derived,...answer.rows]]]) as never) as FactIndexV1;
 return{raw,rows:index.rows as readonly MemoRowV1[],index};
}
const ports=(root:string)=>({
 list:async(directory:string)=>{try{return readdirSync(join(root,directory),{withFileTypes:true}).map(entry=>({name:String(entry.name),kind:(entry.isSymbolicLink()?"other":entry.isDirectory()?"directory":entry.isFile()?"file":"other") as "directory"|"file"|"other"}));}catch{return undefined;}},
 stat:async(path:string)=>{try{const status=lstatSync(join(root,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}},
 descend:(name:string)=>descend(name),member,
});
const walk=async(root:string)=>({...await walkFrontierV1(ports(root)),capturedAt:Date.now()});
/** 🧭️ The planner's membership verdict: the advance of the frontier answers the same directories and members (with their modes). */
async function membershipKept(root:string,frontier:Awaited<ReturnType<typeof walk>>):Promise<boolean>{
 const advanced=await advanceFrontierV1({...ports(root),frontier});
 if(!advanced)return false;
 if("unchanged" in advanced)return true;
 const key=(rows:readonly (readonly unknown[])[])=>JSON.stringify(rows.map(row=>row.length===2?row:[row[0]]).sort());
 return key(advanced.members)===key(frontier.members)&&key(advanced.directories)===key(frontier.directories);
}

/** 🪞️ The model journal: every entry whose identity changed between two snapshots (ctime included), minus the excluded regions; `advisory` reports every row as 'modified' and drops directory rows, as a coalescing watcher may. */
type Snapshot=Map<string,string>;
function snapshot(root:string):Snapshot{
 const out:Snapshot=new Map(),visit=(directory:string)=>{
  for(const name of readdirSync(join(root,directory))){
   const path=directory?directory+"/"+name:name,status=lstatSync(join(root,path));
   out.set(path,[status.isDirectory()?"d":status.isSymbolicLink()?"l"+readlinkSync(join(root,path)):"f",status.size,status.mtimeMs,status.ctimeMs,status.ino,status.mode].join("|"));
   if(status.isDirectory())visit(path);
  }
 };
 visit("");return out;
}
function journalRows(before:Snapshot,after:Snapshot,advisory:boolean):ChangeRowV1[]{
 const rows:ChangeRowV1[]=[];
 for(const path of new Set([...before.keys(),...after.keys()])){
  if(excluded(path))continue;
  const was=before.get(path),now=after.get(path);if(was===now)continue;
  const directory=(now??was)!.startsWith("d");if(advisory&&directory)continue;
  rows.push([advisory?"m":was===undefined?"a":now===undefined?"r":"m",directory?1:0,path]);
 }
 return rows;
}
class ScriptedJournal implements ChangeJournalClientV1{
 rows:ChangeRowV1[]=[];
 excluder(){return excluded;}
 async cursor(){return{epoch:EPOCH,seq:this.rows.length};}
 async since(from:ChangeCursorV1):Promise<ChangeAnswerV1>{return from.epoch===EPOCH?{exact:true,cursor:{epoch:EPOCH,seq:this.rows.length},rows:this.rows.slice(from.seq)}:{exact:false,reason:"epoch"};}
}

type Mutation={name:string;serve:boolean;apply:(root:string,outside:string)=>void;advisory?:boolean;transient?:readonly string[]};
const atomic=(root:string,path:string,text:string)=>{writeFileSync(join(root,path+".tmp"),text);renameSync(join(root,path+".tmp"),join(root,path));};
const mutations:Mutation[]=[
 {name:"content-only edit of an analysed file (its imports are kept)",serve:true,apply:root=>appendFileSync(join(root,"src/a.ts"),"// a note\n")},
 {name:"edit that adds an import",serve:false,apply:root=>appendFileSync(join(root,"src/a.ts"),"import './sub/c.ts';\n")},
 {name:"atomic save keeping the imports",serve:true,transient:["src/a.ts.tmp"],apply:root=>atomic(root,"src/a.ts","import './b.ts';\nexport const a=2;\n")},
 {name:"atomic save adding an import",serve:false,transient:["src/a.ts.tmp"],apply:root=>atomic(root,"src/a.ts","import './b.ts';\nimport './sub/c.ts';\nexport const a=1;\n")},
 {name:"atomic save keeping the imports, reported only as 'modified'",serve:true,advisory:true,transient:["src/a.ts.tmp"],apply:root=>atomic(root,"src/a.ts","import './b.ts';\nexport const a=3;\n")},
 {name:"rename the analysed file",serve:false,apply:root=>renameSync(join(root,"src/a.ts"),join(root,"src/a2.ts"))},
 {name:"delete the analysed file",serve:false,apply:root=>unlinkSync(join(root,"src/a.ts"))},
 {name:"rename an unread sibling in a listed directory",serve:false,apply:root=>renameSync(join(root,"src/notes.md"),join(root,"src/notes2.md"))},
 {name:"content edit of an unread sibling",serve:true,apply:root=>appendFileSync(join(root,"src/notes.md"),"more\n")},
 {name:"content-only edit and an unread sibling edit together",serve:true,apply:root=>{appendFileSync(join(root,"src/a.ts"),"// b note\n");appendFileSync(join(root,"src/notes.md"),"more\n");}},
 {name:"content edit of a file read whole",serve:false,apply:root=>appendFileSync(join(root,"src/sub/c.ts"),"more\n")},
 {name:"content-only edit of a file a derived fact reads (the answer is kept)",serve:true,apply:root=>appendFileSync(join(root,"lib/d.ts"),"more\n")},
 {name:"atomic save of a derived fact's input keeping the answer",serve:true,transient:["lib/d.ts.tmp"],apply:root=>atomic(root,"lib/d.ts","import './x.ts';\nd2\n")},
 {name:"edit that changes a derived answer (an import added to its input)",serve:false,apply:root=>appendFileSync(join(root,"lib/d.ts"),"import './y.ts';\n")},
 {name:"a derived answer kept, but its question now reads another file",serve:false,apply:root=>writeFileSync(join(root,"lib/d.ts"),"import './y.ts';\nd\n")},
 {name:"a derived question that now fails",serve:false,apply:root=>appendFileSync(join(root,"lib/d.ts"),"// FAIL\n")},
 {name:"content edit of a file a derived question probes and the project does not read",serve:true,apply:root=>appendFileSync(join(root,"lib/x.ts"),"more\n")},
 {name:"delete a file a derived question probes",serve:false,apply:root=>unlinkSync(join(root,"lib/x.ts"))},
 {name:"edit of a file read as TypeScript text",serve:false,apply:root=>appendFileSync(join(root,"lib/e.ts"),"more\n")},
 {name:"a new member in a walked directory no fact lists",serve:false,apply:root=>writeFileSync(join(root,"docs/project.json"),"{}\n")},
 {name:"a new walked directory",serve:false,apply:root=>mkdirSync(join(root,"docs/more"))},
 {name:"a creation reported only as 'modified'",serve:false,advisory:true,apply:root=>writeFileSync(join(root,"src/ghost.ts"),"g\n")},
 {name:"create the file a probe found missing",serve:false,apply:root=>writeFileSync(join(root,"src/missing.ts"),"m\n")},
 {name:"replace the analysed file by a symbolic link to an identical copy",serve:false,apply:(root,outside)=>{unlinkSync(join(root,"src/a.ts"));symlinkSync(join(outside,"a.ts"),join(root,"src/a.ts"));}},
 {name:"replace a probed file by a symbolic link",serve:false,apply:(root,outside)=>{unlinkSync(join(root,"src/b.ts"));symlinkSync(join(outside,"a.ts"),join(root,"src/b.ts"));}},
 {name:"touch a walked directory",serve:true,apply:root=>{const now=new Date();utimesSync(join(root,"src/sub"),now,new Date(now.getTime()+7000));}},
 {name:"change the mode of the analysed file",serve:true,apply:root=>chmodSync(join(root,"src/a.ts"),0o755)},
 {name:"change the mode of a member",serve:false,apply:root=>chmodSync(join(root,"src/sub/project.json"),0o755)},
 {name:"rename a walked directory",serve:false,apply:root=>renameSync(join(root,"src/sub"),join(root,"src/sub2"))},
 {name:"recreate a walked directory with the same contents (another inode)",serve:false,apply:root=>{rmSync(join(root,"docs"),{recursive:true});mkdirSync(join(root,"docs"));writeFileSync(join(root,"docs/readme.md"),"docs\n");}},
 {name:"nothing",serve:true,apply:()=>{}},
];

const POINTER_MANIFEST="a".repeat(64),INDEX="b".repeat(64),FRONTIER="c".repeat(64);
/** 🏗️ One sealed memo over a fixture: relevance and edit index from the real builder, a record sealed by the real memo, ports that answer rows by position and the walked frontier with the walk predicates. */
async function sealed(root:string,derive:MemoDeriverV1=deriveModel){
 const {rows,raw,index}=await harvest(root,derive),frontier=await walk(root);
 const built=await buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier,excluded,isLink:async relative=>{try{return lstatSync(join(root,relative)).isSymbolicLink();}catch{return false;}}});
 const journal=new ScriptedJournal(),directory=join(sandbox,"memo"+counter++),stem=join(sandbox,"side"+counter++),pointerFile=join(sandbox,"cur"+counter+++".json");
 writeFileSync(pointerFile,JSON.stringify({generation:1,manifest:POINTER_MANIFEST}));
 writeGraphMemoSidecarFilesV1(stem,{file:pointerFile,generation:1,manifest:POINTER_MANIFEST},built,insensitive,{index:INDEX,frontier:FRONTIER});
 const edit:GraphMemoEditPortsV1={rows:async(digest,ids)=>digest===INDEX?ids.map(id=>rows[id]!):undefined,frontier:async digest=>digest===FRONTIER?{frontier,descend:(name:string)=>descend(name),member}:undefined,derive:async({stale,derived})=>settleGraphMemoDerivationV1({index,stale,derived,ask:suspects=>deriveGraphMemoSuspectsV1({index,suspects,derive})})};
 const pin=JSON.stringify({version:1,owner:1,generation:1,manifest:POINTER_MANIFEST,covered:{atMs:1,count:0,owners:""}}),plan="{\"projects\":{}}";
 const open=(extra:Record<string,unknown>={})=>openGraphMemoV1({root,directory,client:journal,key:"e".repeat(64),insensitive,now:()=>Date.now()+LATER,holds:list=>observation.observationsHold(list as never,64,new Set(),true),edit,...extra});
 expect(await (await open()).seal({plan,pin,projects:0})).toBe("light");
 expect(await (await open()).seal({plan,pin,projects:0,sidecar:stem+".json"})).toBe("sealed");
 return{rows,raw,index,frontier,built,journal,directory,open,edit};
}
/** ⚖️ The planner's verdict on the tree: every fact the project reads holds by the engine, the membership advance keeps the frontier, and the derived question answers what it recorded (the planner's cutoff: a stale support whose question answers the same keeps the project). */
async function planner(root:string,raw:readonly MemoRowV1[],frontier:Awaited<ReturnType<typeof walk>>,d:number|undefined,answer:(root:string)=>number|undefined=modelAnswer):Promise<boolean>{
 const same=d!==undefined&&answer(root)===d;
 return await observation.observationsHold(raw as never,64,new Set(),true)&&await membershipKept(root,frontier)&&same;
}

test("the edit index names every row by the entries it reads and survives packing",async()=>{
 const {root}=fixture(),{rows}=await harvest(root),frontier=await walk(root);
 const built=await buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier,excluded,isLink:async()=>false});
 expect(built.unsealable).toBeNull();
 const stem=join(sandbox,"idx"+counter++);writeGraphMemoSidecarFilesV1(stem,{file:"/x",generation:1,manifest:POINTER_MANIFEST},built,insensitive,{index:INDEX,frontier:FRONTIER,ids:rows.map((_,at)=>at*10+3)});
 const edits=unpackMemoEditIndexV1(readFileSync(stem+GRAPH_MEMO_POLICY_V1.editSuffix))!;
 expect(edits.insensitive).toBe(insensitive);
 const position=(kind:string,path:string)=>rows.findIndex(row=>row.kind===kind&&row.path===join(root,path));
 expect(edits.named("src/a.ts")).toEqual(expect.arrayContaining([position("analysis","src/a.ts")*10+3,position("exists","src/a.ts")*10+3]));
 expect(edits.inside("src"),"the listing of src reads its children").toContain(position("readdir","src")*10+3);
 expect(edits.above("src")&&edits.above("")&&!edits.above("src/a.ts")).toBe(true);
 expect(edits.frontier("docs")&&edits.frontier("src/sub")&&!edits.frontier("node_modules")).toBe(true);
 expect(edits.member("src/sub/project.json")&&!edits.member("src/a.ts")).toBe(true);
 expect(edits.named("lib/d.ts").length,"the derived fact and the TypeScript text its derivation read name its input").toBe(2);
 expect(JSON.parse(readFileSync(stem+".json","utf8")).edit).toEqual({index:INDEX,frontier:FRONTIER});
 expect(unpackMemoEditIndexV1(Buffer.concat([readFileSync(stem+GRAPH_MEMO_POLICY_V1.editSuffix),Buffer.from([0])]))).toBeUndefined();
});

test("soundness and precision against the planner's verdict: each adversarial change is served exactly when the facts that read it hold and the membership is kept, the base behaviour refuses every touched case, and a trusting settlement, one that reads the frontier of the present tree and one that accepts every derivation are caught",async()=>{
 let served=0,refused=0,trustingWrong=0,blindWrong=0,acceptingWrong=0,baseDerived=0;
 for(const mutation of mutations){
  const {root,outside}=fixture(),memo=await sealed(root),d=modelAnswer(root);
  expect(await planner(root,memo.raw,memo.frontier,d),mutation.name+" (fresh)").toBe(true);
  const before=snapshot(root);mutation.apply(root,outside);
  const rows=[...(mutation.transient??[]).flatMap(path=>mutation.advisory?[["m",0,path] as ChangeRowV1]:[["a",0,path] as ChangeRowV1,["r",0,path] as ChangeRowV1]),...journalRows(before,snapshot(root),mutation.advisory===true)];
  memo.journal.rows.push(...rows);
  const verdict=await planner(root,memo.raw,memo.frontier,d),base=await (await memo.open({edit:undefined})).lookup(),{derive:_derive,...without}=memo.edit,withoutDerive=mutation.name.includes("derived")&&mutation.serve?await (await memo.open({edit:without})).lookup():undefined,answer=await (await memo.open()).lookup();
  if(answer.hit)expect(verdict,"the memo served what the planner would plan: "+mutation.name).toBe(true);
  expect(answer.hit,mutation.name).toBe(mutation.serve);
  if(rows.some(row=>unpackMemoRelevanceV1(packMemoRelevanceV1(memo.built.directories,insensitive,memo.built.strict,memo.built.stamped))!.holds(row[2])))expect(base.hit,"the base memo refuses a touched entry: "+mutation.name).toBe(false);
  if(answer.hit&&rows.length&&mutation.name.startsWith("content-only"))expect(answer.edited?.rows,mutation.name).toBeGreaterThan(0);
  answer.hit?served++:refused++;
  const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(memo.built.directories,insensitive,memo.built.strict,memo.built.stamped))!,collected=collectMemoChangesV1(relevance,rows),edits=unpackMemoEditIndexV1(readFileSync(join(memo.directory,(JSON.parse(readFileSync(join(memo.directory,"e".repeat(40)+".json"),"utf8")) as {relevance:{edit:{id:string}}}).relevance.edit.id+GRAPH_MEMO_POLICY_V1.editSuffix)))!;
  const listed=new Map([...collected.pending].map(([directory,row])=>[directory,row[2]]));
  const settle=(over:Partial<Parameters<typeof settleMemoEditsV1>[0]>)=>settleMemoEditsV1({root,relevance,edits,changed:collected.changed,listed,index:INDEX,frontier:FRONTIER,ports:memo.edit,holds:list=>observation.observationsHold(list as never,64,new Set(),true),now:()=>Date.now()+LATER,...over});
  if((await settle({holds:async()=>true})).held&&!verdict)trustingWrong++;
  const blind:GraphMemoEditPortsV1={...memo.edit,frontier:async()=>({frontier:await walk(root),descend:(name:string)=>descend(name),member})};
  if((await settle({ports:blind})).held&&!verdict)blindWrong++;
  if((await settle({ports:{...memo.edit,derive:async()=>({held:true,questions:1})}})).held&&!verdict)acceptingWrong++;
  if(mutation.name.includes("derived")&&mutation.serve){expect(withoutDerive?.hit,"the landed memo (no derivation) plans: "+mutation.name).toBe(false);baseDerived++;if(answer.hit)expect(answer.edited?.questions,mutation.name).toBe(1);}
 }
 expect(served).toBeGreaterThanOrEqual(10);expect(refused).toBeGreaterThanOrEqual(19);expect(baseDerived).toBe(3);
 expect(acceptingWrong,"a settlement that accepts every derivation must be wrong somewhere").toBeGreaterThan(0);
 expect(trustingWrong,"a settlement that does not re-check the rows must be wrong somewhere").toBeGreaterThan(0);
 expect(blindWrong,"a settlement that proves membership against the present tree instead of the sealed frontier must be wrong somewhere").toBeGreaterThan(0);
},180000);

test("a served edit re-seals: the stamp of the directory it listed again is renewed, so the next sibling edit there is settled by the stamp alone, and the journal cursor moves past the edit",async()=>{
 const {root}=fixture(),memo=await sealed(root),recordFile=join(memo.directory,"e".repeat(40)+".json"),record=()=>JSON.parse(readFileSync(recordFile,"utf8")) as {cursor:string;relevance:{stamps:string}};
 const before=snapshot(root),stamps=record().relevance.stamps;atomic(root,"src/sub/c.ts","c\n");
 memo.journal.rows.push(["a",0,"src/sub/c.ts.tmp"],["r",0,"src/sub/c.ts.tmp"],...journalRows(before,snapshot(root),false));
 const first=await (await memo.open()).lookup();
 expect(first.hit,"an atomic save with the same bytes in a stamped directory").toBe(true);
 if(first.hit)expect(first.edited?.rows).toBeGreaterThan(0);
 expect(record().relevance.stamps,"the stamp table was renewed").not.toBe(stamps);
 expect(record().cursor.endsWith(":"+memo.journal.rows.length)||record().cursor.endsWith(String(memo.journal.rows.length))).toBe(true);
 appendFileSync(join(root,"src/sub/notes.txt"),"x\n");memo.journal.rows.push(["m",0,"src/sub/notes.txt"]);
 const second=await (await memo.open()).lookup();
 expect(second.hit).toBe(true);
 if(second.hit)expect(second.edited,"settled by the renewed stamp, no listing again").toBeUndefined();
 collectGraphMemoV1(memo.directory);
 expect(readdirSync(memo.directory).some(name=>name.endsWith(GRAPH_MEMO_POLICY_V1.editSuffix)),"retention keeps the edit index a record names").toBe(true);
},60000);

test("fail closed: a tampered edit index, a record without one, a retired generation, a missing frontier and too many paths or rows refuse",async()=>{
 const {root}=fixture(),memo=await sealed(root),recordFile=join(memo.directory,"e".repeat(40)+".json"),record=JSON.parse(readFileSync(recordFile,"utf8")) as {relevance:{edit:{id:string}}},indexFile=join(memo.directory,record.relevance.edit.id+GRAPH_MEMO_POLICY_V1.editSuffix),saved=readFileSync(indexFile);
 appendFileSync(join(root,"src/a.ts"),"// edit\n");memo.journal.rows.push(["m",0,"src/a.ts"]);
 writeFileSync(indexFile,Buffer.concat([saved,Buffer.from([1])]));expect(await (await memo.open()).lookup()).toEqual({hit:false,reason:"generation"});writeFileSync(indexFile,saved);
 expect(await (await memo.open({edit:{rows:async()=>undefined,frontier:memo.edit.frontier}})).lookup(),"a retired fact index").toEqual({hit:false,reason:"changed",detail:"src/a.ts"});
 const {edit:_drop,...without}=record.relevance as Record<string,unknown>;writeFileSync(recordFile,JSON.stringify({...record,relevance:without}));
 expect(await (await memo.open()).lookup(),"a record sealed without an edit index plans as before").toEqual({hit:false,reason:"changed",detail:"src/a.ts"});
 writeFileSync(recordFile,JSON.stringify(record));
 expect((await (await memo.open()).lookup()).hit).toBe(true);
 const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(memo.built.directories,insensitive,memo.built.strict,memo.built.stamped))!,edits=unpackMemoEditIndexV1(saved)!,base={root,relevance,edits,listed:new Map<string,string>(),index:INDEX,frontier:FRONTIER as string|null,ports:memo.edit,holds:async()=>true,now:()=>Date.now()};
 expect(await settleMemoEditsV1({...base,changed:Array.from({length:GRAPH_MEMO_POLICY_V1.maximumEditPaths+1},(_,at)=>["m",0,"src/a.ts"+at] as ChangeRowV1)})).toEqual({held:false,path:"src/a.ts0"});
 expect(await settleMemoEditsV1({...base,edits:{...edits,named:()=>Array.from({length:GRAPH_MEMO_POLICY_V1.maximumEditRows+1},(_,at)=>at)},changed:[["m",0,"src/a.ts"]]})).toEqual({held:false,path:"src/a.ts"});
 expect(await settleMemoEditsV1({...base,frontier:null,changed:[["m",1,"src/sub"]]}),"a generation without a frontier proves no directory").toEqual({held:false,path:"src/sub"});
 expect(await settleMemoEditsV1({...base,changed:[["m",0,"src/a.ts"]]})).toMatchObject({held:true});
},60000);

test("the planner's own check: environment rows by the environment rule, every other row by the engine, a fact the planner settles by another computation never holds",async()=>{
 const {root}=fixture(),{raw}=await harvest(root);
 expect(await memoFactsHoldV1(root,raw)).toBe(await observation.observationsHold(raw as never,64,new Set(),true));
 writeFileSync(join(root,"src/sub/project.json"),JSON.stringify({name:"sub"}));
 const field={kind:"field",path:"src/sub/project.json",fact:{extract:"project-json",value:{name:"sub",nativeRoot:null}}},text=readFileSync(join(root,"lib/e.ts")),status=lstatSync(join(root,"lib/e.ts")),digest={kind:"digest",path:"lib/e.ts",fact:{sha256:sha(text),size:status.size,mtimeMs:status.mtimeMs}};
 expect(await memoFactsHoldV1(root,[field,digest])).toBe(true);
 writeFileSync(join(root,"src/sub/project.json"),JSON.stringify({name:"other"}));
 expect(await memoFactsHoldV1(root,[field])).toBe(false);
 appendFileSync(join(root,"lib/e.ts"),"x\n");expect(await memoFactsHoldV1(root,[digest])).toBe(false);
 for(const kind of ["derived","closure","stamp","registry"])expect(await memoFactsHoldV1(root,[{kind,path:join(root,"lib/d.ts"),fact:"x"}]),kind).toBe(false);
 expect(await memoFactsHoldV1(root,[])).toBe(true);
},60000);

test("on a real generation store the memo serves an edit exactly when the planner itself verifies the slice without building, and finds the re-checked rows by their position in the generation's fact index",async()=>{
 const dir=join(sandbox,"world"+counter++),journal=new ScriptedJournal(),generations=join(dir,".gen"),exclude={rooted:[".gen",".leases"],names:["node_modules"]};
 await seed(dir,{"pkg/a/project.json":JSON.stringify({name:"a"}),"pkg/a/src/index.ts":"export const a=1;\n","pkg/a/src/util.ts":"export const u=1;\n","pkg/b/project.json":JSON.stringify({name:"b",uses:["pkg/b/data.txt"]}),"pkg/b/data.txt":"one\n","pkg/b/src/index.ts":"export const b=2;\n"});
 const settled=new Date(Date.now()-LATER);for(const directory of ["pkg","pkg/a","pkg/a/src","pkg/b","pkg/b/src"])utimesSync(join(dir,directory),settled,settled);
 const world=createWorld(dir),slice=new Set(["a"]),directory=join(sandbox,"wm"+counter++);
 const store=createGenerationStoreV1({root:generations,fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"law"}});
 const edit:GraphMemoEditPortsV1={rows:async(index,ids)=>{const view=await store.readIndexView(index);return view?ids.map(id=>view.rows[id] as MemoRowV1):undefined;},frontier:async digest=>{const frontier=await store.readFrontier(digest);return frontier?{frontier,descend:()=>true,member:(path:string)=>path.endsWith("/project.json")||path.endsWith(".ts")} as MemoFrontierV1:undefined;}};
 const holds=createHolds(dir),open=(withEdit=true)=>openGraphMemoV1({root:dir,directory,client:journal,key:"w".repeat(64),insensitive,now:()=>Date.now()+LATER,holds:list=>holds(list as never),...(withEdit?{edit}:{})});
 const pinOf=(current:GenerationSealedV1)=>encodeGenerationPinV1({owner:1,generation:current.pointer.generation,manifest:current.pointer.manifest!,owners:Object.keys(current.manifest!.owners).sort(compareCodePointsV1),covered:slice,atMs:Date.now()});
 const g1=(await world.refreshClosure(["a"])).current,plan=canonicalJsonV1(await world.graph(g1,["a"])),pin=pinOf(g1),stem=join(sandbox,"ws"+counter++);
 expect(await (await open()).seal({plan,pin,projects:1})).toBe("light");
 await writeGraphMemoSliceSidecarV1({root:dir,generations,exclude,pin,stem,expected:undefined,client:journal,insensitive});
 expect(JSON.parse(readFileSync(stem+".json","utf8")).edit).toEqual({index:g1.manifest!.factIndex,frontier:g1.manifest!.frontier});
 expect(await (await open()).seal({plan,pin,projects:1,sidecar:stem+".json"})).toBe("sealed");
 const touch=(path:string)=>{const later=new Date(Date.now()+3000);utimesSync(join(dir,path),later,later);journal.rows.push(["m",0,path]);};

 touch("pkg/a/src/index.ts");
 expect(await (await open(false)).lookup(),"base: a touched source refuses").toEqual({hit:false,reason:"changed",detail:"pkg/a/src/index.ts"});
 const touched=await (await open()).lookup();
 expect(touched,"a touched source whose facts hold is served").toMatchObject({hit:true,edited:{paths:1}});
 const oracle1=await world.refreshClosure(["a"]);expect(oracle1.built,"oracle: the planner verifies the slice without building").toBe(false);

 const before=snapshot(dir);writeFileSync(join(dir,"pkg/a/src/util.ts.tmp"),"export const u=1;\n");renameSync(join(dir,"pkg/a/src/util.ts.tmp"),join(dir,"pkg/a/src/util.ts"));
 journal.rows.push(["a",0,"pkg/a/src/util.ts.tmp"],["r",0,"pkg/a/src/util.ts.tmp"],...journalRows(before,snapshot(dir),false).filter(row=>!row[2].startsWith(".")));
 expect((await (await open()).lookup()).hit,"an atomic save with the same bytes is served").toBe(true);
 const oracle2=await world.refreshClosure(["a"]);expect(oracle2.built,"oracle: the planner keeps the generation").toBe(false);

 writeFileSync(join(dir,"pkg/a/src/index.ts"),"import \"@syn/b\";\nexport const a=1;\n");journal.rows.push(["m",0,"pkg/a/src/index.ts"]);
 expect(await (await open()).lookup(),"an edit that adds an import is planned").toEqual({hit:false,reason:"changed",detail:"pkg/a/src/index.ts"});
 const oracle3=await world.refreshClosure(["a"]);expect([oracle3.built,oracle3.computed],"oracle: the planner re-infers the owner").toEqual([true,1]);

 writeFileSync(join(dir,"pkg/b/data.txt"),"two\n");journal.rows.push(["m",0,"pkg/b/data.txt"]);
 const g3=oracle3.current,plan3=canonicalJsonV1(await world.graph(g3,["a"])),pin3=pinOf(g3),stem3=join(sandbox,"ws"+counter++),memo3=await open();
 await writeGraphMemoSliceSidecarV1({root:dir,generations,exclude,pin:pin3,stem:stem3,expected:undefined,client:journal,insensitive});
 expect(await memo3.seal({plan:plan3,pin:pin3,projects:1,sidecar:stem3+".json"})).toBe("sealed");
 writeFileSync(join(dir,"pkg/a/src/new.ts"),"export const n=1;\n");journal.rows.push(["a",0,"pkg/a/src/new.ts"]);
 const created=await (await open()).lookup();expect(created.hit,"a new source in a listed directory is planned").toBe(false);
 const oracle4=await world.refreshClosure(["a"]);expect(oracle4.built,"oracle: the planner builds").toBe(true);
},120000);

const watcher=process.env.SEMIO_REPO_WATCH_BINARY??"";
test.skipIf(!watcher)("with a REAL file watcher: every served edit is one the planner keeps, and an edit that keeps the imports, an atomic save and a touched directory are served",async()=>{
 const wanted=new Set(["content-only edit of an analysed file (its imports are kept)","edit that adds an import","atomic save keeping the imports","rename the analysed file","content edit of a file read whole","a new member in a walked directory no fact lists","replace the analysed file by a symbolic link to an identical copy","touch a walked directory","change the mode of the analysed file","rename a walked directory","content-only edit of a file a derived fact reads (the answer is kept)","edit that changes a derived answer (an import added to its input)"]),must=new Set(["content-only edit of a file a derived fact reads (the answer is kept)","content-only edit of an analysed file (its imports are kept)","atomic save keeping the imports","touch a walked directory"]);
 let served=0,refused=0;
 for(const mutation of mutations.filter(item=>wanted.has(item.name))){
  const {root,outside}=fixture(),journalDirectory=root+".journal",environment={...process.env,SEMIO_REPO_WATCH_BINARY:watcher,SEMIO_REPO_CHANGES:"on"};
  try{
   const state:{rows:readonly MemoRowV1[];index?:FactIndexV1;frontier?:Awaited<ReturnType<typeof walk>>}={rows:[]},stem=join(sandbox,"real"+counter++),pointerFile=join(sandbox,"rc"+counter++);
   const edit:GraphMemoEditPortsV1={rows:async(_index,ids)=>ids.map(id=>state.rows[id]!),frontier:async()=>state.frontier&&{frontier:state.frontier,descend:(name:string)=>descend(name),member},derive:async({stale,derived})=>settleGraphMemoDerivationV1({index:state.index!,stale,derived,ask:suspects=>deriveGraphMemoSuspectsV1({index:state.index!,suspects,derive:deriveModel})})};
   const client=createChangeJournalClientV1({directory:journalDirectory,root,environment,start:()=>startChangeJournalServiceV1({directory:journalDirectory,root,environment})}),open=()=>openGraphMemoV1({root,directory:join(journalDirectory,"memo"),client,key:"r".repeat(64),insensitive,now:()=>Date.now()+LATER,holds:list=>observation.observationsHold(list as never,64,new Set(),true),edit});
   const pin=JSON.stringify({version:1,owner:1,generation:1,manifest:POINTER_MANIFEST,covered:{atMs:1,count:0,owners:""}}),first=await open();expect(first.cursor,mutation.name).toBeDefined();
   expect(await first.seal({plan:"{\"projects\":{}}",pin,projects:0}),mutation.name).toBe("light");
   const sealing=await open(),{rows,raw,index}=await harvest(root),frontier=await walk(root),d=modelAnswer(root);state.rows=rows;state.index=index;state.frontier=frontier;
   const built:MemoBuiltV1=await buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier,excluded,isLink:async relative=>{try{return lstatSync(join(root,relative)).isSymbolicLink();}catch{return false;}}});
   writeFileSync(pointerFile,JSON.stringify({generation:1,manifest:POINTER_MANIFEST}));writeGraphMemoSidecarFilesV1(stem,{file:pointerFile,generation:1,manifest:POINTER_MANIFEST},built,insensitive,{index:INDEX,frontier:FRONTIER});
   expect(await sealing.seal({plan:"{\"projects\":{}}",pin,projects:0,sidecar:stem+".json"}),mutation.name).toBe("sealed");
   mutation.apply(root,outside);
   const verdict=await planner(root,raw,frontier,d),answer=await (await open()).lookup();
   if(answer.hit){expect(verdict,"the memo served what the planner would plan: "+mutation.name).toBe(true);served++;}else refused++;
   if(must.has(mutation.name))expect(answer.hit,"served with real journal rows: "+mutation.name).toBe(true);
  }finally{const pid=readChangeStateV1(journalDirectory)?.pid;if(pid)try{process.kill(pid,"SIGTERM");}catch{}rmSync(journalDirectory,{recursive:true,force:true});}
 }
 expect(served).toBeGreaterThanOrEqual(3);expect(refused).toBeGreaterThanOrEqual(5);
},300000);

test("the derivation child: a stale row a project holds itself, an unsupported derived fact, no or too many questions refuse at once; a request asked through the child entry against a real generation store answers what the in-process derivation answers; a retired index answers missing; the runtime and the graph loader wire the child",async()=>{
 const {root}=fixture(),{index}=await harvest(root),project=(kind:string,path:string)=>index.rows.findIndex(row=>row.kind===kind&&row.path===join(root,path)),support=project("typescriptText","lib/d.ts"),own=project("file","src/sub/c.ts"),derived=index.rows.find(row=>row.kind==="derived") as MemoRowV1;
 expect(support).toBeGreaterThanOrEqual(0);expect(own).toBeGreaterThanOrEqual(0);
 expect(graphMemoSuspectsV1(index,[support],[]).suspects).toEqual(new Set([derivedOwnerNameV1(derived)]));
 const never=async()=>{throw Error("asked");};
 expect(await settleGraphMemoDerivationV1({index,stale:[support,own],derived:[],ask:never})).toEqual({held:false,reason:"direct:proj"});
 expect(await settleGraphMemoDerivationV1({index,stale:[],derived:[{...derived,fact:7}],ask:never})).toEqual({held:false,reason:"unsupported"});
 expect(await settleGraphMemoDerivationV1({index,stale:[],derived:[],ask:never})).toEqual({held:false,reason:"questions"});
 const suspects=new Set([derivedOwnerNameV1(derived)]),inProcess=await deriveGraphMemoSuspectsV1({index,suspects,derive:deriveModel});
 expect(inProcess).toEqual({version:1,held:[...suspects],changed:[],failed:0,uncovered:[],missing:false});
 expect(graphMemoDerivationHeldV1(suspects,inProcess)).toEqual({held:true,questions:1});
 for(const [answer,reason] of [[undefined,"missing"],[{...inProcess,missing:true},"missing"],[{...inProcess,changed:["x"]},"changed"],[{...inProcess,failed:1},"changed"],[{...inProcess,uncovered:["x"]},"uncovered"],[{...inProcess,held:[]},"unanswered"]] as const)expect(graphMemoDerivationHeldV1(suspects,answer as never),reason).toEqual({held:false,reason});
 const generations=join(sandbox,"gen"+counter++),{bytes}=packFactIndexV1(index as never,GENERATION_POLICY_V1.maximumIndexBytes),digest=sha(Buffer.from(bytes));
 mkdirSync(join(generations,GENERATION_POLICY_V1.indexes),{recursive:true});writeFileSync(join(generations,GENERATION_POLICY_V1.indexes,digest+".facts.br"),bytes);
 const request=join(sandbox,"req"+counter++)+".json",environment={[GRAPH_MEMO_POLICY_V1.derive]:request};
 writeGraphMemoDeriveFileV1(request,{version:1,index:digest,suspects:[...suspects]});
 expect(readGraphMemoDeriveFileV1(request,"request")).toEqual({version:1,index:digest,suspects:[...suspects]});
 await answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations);
 expect(readGraphMemoDeriveFileV1(graphMemoDeriveAnswerFileV1(request),"answer"),"the child answers what the in-process derivation answers").toEqual(inProcess);
 appendFileSync(join(root,"lib/d.ts"),"import './y.ts';\n");await answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations);
 expect(readGraphMemoDeriveFileV1(graphMemoDeriveAnswerFileV1(request),"answer")).toMatchObject({held:[],changed:[...suspects]});
 writeGraphMemoDeriveFileV1(request,{version:1,index:"9".repeat(64),suspects:[...suspects]});await answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations);
 expect(readGraphMemoDeriveFileV1(graphMemoDeriveAnswerFileV1(request),"answer")).toMatchObject({missing:true});
 writeFileSync(request,JSON.stringify({version:1,index:digest,suspects:[...suspects],extra:1}));await expect(answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations)).rejects.toThrow();
 const here=import.meta.dir,runtime=readFileSync(join(here,"../../../../../⌨️command/⚙️runtime/🟦️.ts"),"utf8"),loader=readFileSync(join(here,"../../../🟦️.ts"),"utf8");
 expect(runtime.includes(`process.env.${GRAPH_MEMO_POLICY_V1.derive}!==undefined`)&&runtime.indexOf(`process.env.${GRAPH_MEMO_POLICY_V1.derive}`)<runtime.indexOf("prepareBuildToolStorageV1"),"the runtime answers a derivation before any tool or graph work").toBe(true);
 expect(loader.includes(`child("build:graph-derive",environment)`),"the graph loader hands the memo its derivation child").toBe(true);
},120000);

/** ⏱️ A second derived question, whose support reads the whole text of every import target and whose row names only its input (the targets lie in another directory, so a change there does not name the question): how many imports of `lib/d.ts` name a file whose text contains `yes`. A target can therefore be restored to its sealed bytes without touching any row that names the question itself. */
const deriveTexts:MemoDeriverV1=async row=>{
 const [,file]=JSON.parse(row.path) as [string,string];
 const captured=await observation.captureObservedSourceInputsV1(()=>{const host=observation.observedTypeScriptHost({readFile:(path:string)=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}} as never) as {readFile(path:string):string|undefined};return modelImports(host.readFile(file)??"").filter(spec=>(host.readFile(join(dirname(file),spec))??"").includes("yes")).length;},undefined,false);
 return{fact:captured.value,rows:captured.observations as unknown as MemoRowV1[]};
};
/** 👯️ The independent answer of {@link deriveTexts} on the tree as it is now (plain reads). */
const textsAnswer=(root:string):number|undefined=>{const read=(path:string)=>{try{return readFileSync(join(root,path),"utf8");}catch{return "";}};return modelImports(read("lib/d.ts")).filter(spec=>read(join("lib",spec)).includes("yes")).length;};

test("a derived serve leaves its stale support behind: the record carries those rows, so a later change that restores one of their paths alone is settled against the tree the question was last asked on, never against the sealed rows (two-step sequences, oracle = the planner's verdict on the final tree)",async()=>{
 const write=(path:string,text:string)=>(root:string)=>writeFileSync(join(root,path),text),both=(...steps:((root:string)=>void)[])=>(root:string)=>{for(const step of steps)step(root);},swap=both(write("docs/x.ts","no\n"),write("docs/z.ts","yes\n"));
 const sequences:readonly Readonly<{name:string;first:(root:string)=>void;second:(root:string)=>void;serve:boolean}>[]=[
  {name:"the two targets swap their marker, then the first is restored (the answer grows to 2)",first:swap,second:write("docs/x.ts","yes\n"),serve:false},
  {name:"the two targets swap their marker, then the second is restored (the answer drops to 0)",first:swap,second:write("docs/z.ts","no\n"),serve:false},
  {name:"the two targets swap their marker, then the swap is undone (the sealed tree again)",first:swap,second:both(write("docs/x.ts","yes\n"),write("docs/z.ts","no\n")),serve:true},
  {name:"a target edited keeping its marker, then restored (the answer stays 1)",first:write("docs/x.ts","yes again\n"),second:write("docs/x.ts","yes\n"),serve:true},
 ];
 let carriedWrong=0;
 for(const sequence of sequences){
  const {root}=fixture();for(const [path,text] of [["lib/d.ts","import '../docs/x.ts';\nimport '../docs/z.ts';\nd\n"],["docs/x.ts","yes\n"],["docs/z.ts","no\n"]] as const)writeFileSync(join(root,path),text);
  const settled=new Date(Date.now()-LATER);for(const directory of ["lib","docs"])utimesSync(join(root,directory),settled,settled);
  const memo=await sealed(root,deriveTexts),d=textsAnswer(root);
  expect(d,"one of the two targets carries the marker").toBe(1);
  let before=snapshot(root);sequence.first(root);memo.journal.rows.push(...journalRows(before,snapshot(root),false));
  const first=await (await memo.open()).lookup();
  expect(textsAnswer(root),sequence.name+": the first step keeps the answer").toBe(d);
  expect(first.hit?first.edited?.questions:first.detail,sequence.name+": the first step is served by asking the question again").toBe(1);
  before=snapshot(root);sequence.second(root);memo.journal.rows.push(...journalRows(before,snapshot(root),false));
  const verdict=await planner(root,memo.raw,memo.frontier,d,textsAnswer),second=await (await memo.open()).lookup();
  if(second.hit)expect(verdict,"the memo served what the planner would plan: "+sequence.name).toBe(true);
  expect(second.hit,sequence.name).toBe(sequence.serve);
  expect(verdict,sequence.name+" (planner)").toBe(sequence.serve);
  const recordFile=join(memo.directory,"e".repeat(40)+".json"),record=JSON.parse(readFileSync(recordFile,"utf8")) as {relevance:Record<string,unknown>};
  if(sequence.name.includes("undone"))expect(record.relevance.carried,"the sealed tree again: nothing left to carry").toBeUndefined();
  const {carried:_carried,...forgotten}=record.relevance;writeFileSync(recordFile,JSON.stringify({...record,relevance:forgotten}));
  memo.journal.rows.push(["m",0,"docs/x.ts"]);
  if((await (await memo.open()).lookup()).hit&&!await planner(root,memo.raw,memo.frontier,d,textsAnswer))carriedWrong++;
 }
 expect(carriedWrong,"a record that forgets the carried rows must be wrong somewhere").toBeGreaterThan(0);
},120000);
