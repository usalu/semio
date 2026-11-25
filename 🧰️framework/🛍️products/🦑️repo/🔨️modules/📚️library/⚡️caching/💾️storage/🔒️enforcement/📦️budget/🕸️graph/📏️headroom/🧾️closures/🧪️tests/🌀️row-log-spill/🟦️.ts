/** 🌀️ Contract: a row log is a process-private spill of the graph writer, written under the closure store as `rl/rl-<pid>-<time>-<serial>.rows`; the closure proof knows it as a spill (never read, never a member, never retired) instead of refusing the whole store with "Unknown closure reference publication", and every file shape a read writes under the store is either a known closure role or a known spill, never neither. */
import {expect,test} from "bun:test";
import {readFileSync,readdirSync,statSync} from "node:fs";
import {join,relative,sep} from "node:path";
import {CLOSURE_REPAIR_POLICY_V1,isClosureRepairReceiptNameV1} from "../../../../../../../../../🏗️build/🕸️graph/🕰️generations/🩹️closure-repair/🟨️.mjs";
import {fileURLToPath} from "node:url";
import ts from "typescript";
import {physicalSourceOriginV1} from "../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import {rowLogFileNameV1} from "../../../../../../../../../🏗️build/🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs";

const origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL);
const source=(url:string)=>readFileSync(new URL(url,origin),"utf8");
const tree=ts.createSourceFile("closures.ts",source("../../🏷️names/🟦️.ts")+"\n"+source("../../🟦️.ts"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
const declared=(name:string)=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name)!;
const load=Function(ts.transpileModule(declared("role").getText(tree)+"\n"+declared("spill").getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return{role,spill}")() as{role:(name:string)=>string;spill:(name:string)=>boolean};
const hash="8d8f21d7".repeat(8),log=rowLogFileNameV1(38258,1763470000000,0);

test("the exact refusal: a row log named by the writer is no closure role, and it is a spill in every writer directory",()=>{
 expect(log).toMatch(/^rl-38258-[0-9a-z]+-0\.rows$/u);
 const names=["inference/selections/"+hash+"/rl/"+log,"inference/contexts/"+hash+"/rl/"+log,"inference/rl/"+log];
 for(const name of names){expect(()=>load.role(name),name).toThrow("Unknown closure reference publication: "+name);expect(load.spill(name),name).toBe(true);}
});

test("the source analysis store of a dataset is a spill: sa/<shard>/<key>.json.br and its temporary file, never a role",()=>{
 const key="ff4a87193ae14f73cf3b54317132c20202427db0528e51cdd1425cd385d9ae6f";
 for(const parent of["selections","contexts"])for(const name of["inference/"+parent+"/"+hash+"/sa/ff/"+key+".json.br","inference/"+parent+"/"+hash+"/sa/ff/"+key+".json.br.4242.tmp"]){expect(()=>load.role(name),name).toThrow("Unknown closure reference publication: "+name);expect(load.spill(name),name).toBe(true);}
 for(const near of["inference/selections/"+hash+"/sa/f/"+key+".json.br","inference/selections/"+hash+"/sa/ff/"+key.slice(1)+".json.br","inference/selections/"+hash+"/sa/ff/"+key+".json","inference/selections/"+hash+"/sa/ff/sub/"+key+".json.br","inference/selections/"+hash+"/sa/ff/"+key+".json.br.tmp","sa/ff/"+key+".json.br","generations/sa/ff/"+key+".json.br"])expect(load.spill(near),near).toBe(false);
});

test("nothing near a spill is one",()=>{
 const near=["inference/selections/"+hash+"/rl/"+log+".tmp","inference/selections/"+hash+"/np/"+log,"inference/selections/short/rl/"+log,"rl/"+log,"inference/selections/"+hash+"/rl/rl-x-1-0.rows","inference/selections/"+hash+"/rl/sub/"+log,"inference/selections/"+hash+"/rl/"+log.toUpperCase(),"inference/selections/"+hash+"/rl/other.rows","generations/rl/"+log];
 for(const name of near)expect(load.spill(name),name).toBe(false);
});

test("every file name the writers of the store can produce is exactly one of a closure role or a spill",()=>{
 const roles=["inference/members.jsonl","build-input-closures/"+"a".repeat(64)+".json","snapshots/"+"a".repeat(64)+".gz","authority.json","graph.json","generations/current.json","inference/selections/"+hash+"/np/"+"a".repeat(24)+".json","inference/selections/"+hash+"/np/"+"a".repeat(24)+".json.fleet","inference/selections/"+hash+"/ob/"+"a".repeat(64)+".br"];
 for(const name of roles){expect(()=>load.role(name),name).not.toThrow();expect(load.spill(name),name).toBe(false);}
 for(const serial of[0,1,4096])for(const pid of[1,38258,4194304])expect(load.spill("inference/selections/"+hash+"/rl/"+rowLogFileNameV1(pid,Date.now(),serial))).toBe(true);
 const token="952099430c7a7f41bd3d7102",tmp=".0123456789abcdef.tmp";
 for(const name of["generations/s/"+token+".json","generations/s/"+token+".progress.json","generations/s/"+token+".ask.json","generations/s/"+token+".json"+tmp,"generations/s/"+token+".progress.json"+tmp,"generations/s/"+token+".ask.json"+tmp,"generations/j/"+hash+".json","generations/j/"+hash+".0.rows","generations/j/"+hash+".json"+tmp,"generations/withheld-revalidation.json","generations/withheld-revalidation.json"+tmp,"generations/current.json"+tmp]){
  expect(()=>load.role(name),name).toThrow("Unknown closure reference publication: "+name);
  expect(load.spill(name),name).toBe(true);
 }
 for(const near of["generations/s/short.json","generations/s/"+token+".other.json","generations/s/sub/"+token+".json","generations/j/"+token+".json","generations/current.json"])expect(load.spill(near),near).toBe(false);
});

test("no source of the build names a row log file by hand: the one name factory is the only writer of the shape",()=>{
 const found:string[]=[];
 const walk=(directory:string)=>{for(const name of readdirSync(directory)){if(name==="🧪️tests"||name==="node_modules")continue;const path=join(directory,name);let stat;try{stat=statSync(path);}catch{continue;}if(stat.isDirectory())walk(path);else if(/\.(?:ts|mjs)$/u.test(name)&&readFileSync(path,"utf8").includes('"rl-"'))found.push(name);}};
 walk(fileURLToPath(new URL("../../../../../../../../../🏗️build/",origin)));
 expect(found).toEqual(["🟨️.mjs"]);
});

test("bounded repair receipts are graph control metadata, while aliases and oversized receipts refuse the proof",()=>{
 const root=join(sep,"fixture"),folder=join(root,"generations","cr"),name="4-"+hash+".json";
 const code=ts.transpileModule(["directories","physicalRow","spill","liveSpillDirectory"].map(name=>declared(name).getText(tree)).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 for(const [basename,bytes,links,ordinary,directory] of [[name,1n,1n,true],[name+"."+"a".repeat(16)+".tmp",1n,1n,true],[name,1n,2n,false],[name,BigInt(CLOSURE_REPAIR_POLICY_V1.maximumReceiptBytes)+1n,1n,false],[name,1n,1n,false,true]] as const){
  const leaf=join(folder,basename),proof={image:{root},directories:new Map(),rows:new Map()};
  const lstatSync=(path:string)=>({isDirectory:()=>path!==leaf||directory===true,isFile:()=>path===leaf&&directory!==true,isSymbolicLink:()=>false,dev:1n,nlink:path===leaf?links:1n,size:path===leaf?bytes:0n,blocks:1n,mtimeMs:1n});
  const ports={P:{maximumPathUnits:256,maximumEntries:16,maximumFileBytes:16777216,ignoredDirectories:[],ignoredFiles:[]},join,relative,sep,stamp:()=>"same",ancestor:()=>"same",nativeImageServiceEntryV1:()=>false,check:()=>{},requireNativeStoragePathV1:(_root:string,path:string)=>path,named:(_proof:unknown,path:string)=>lstatSync(path),lstatSync,namesAt:(_proof:unknown,path:string)=>path===root?["generations"]:path===folder?[basename]:["cr"],CLOSURE_REPAIR_POLICY_V1,isClosureRepairReceiptNameV1};
  const run=Function(...Object.keys(ports),code+";return directories")(...Object.values(ports));
  if(ordinary){run(proof);expect(proof.rows.has(leaf)).toBe(false);}
  else expect(()=>run(proof)).toThrow();
 }
 for(const near of ["0-"+hash+".json","9007199254740992-"+hash+".json","01-"+hash+".json",name+".bak","sub/"+name])expect(isClosureRepairReceiptNameV1(near)).toBe(false);
});

test("the live source analysis directory is neither entered nor stamped by the proof, only its identity is kept",()=>{
 const root=join(sep,"fixture"),selection=join(root,"inference","selections",hash),analysis=join(selection,"sa"),other=join(selection,"np");
 const code=ts.transpileModule(["directories","physicalRow","spill","liveSpillDirectory"].map(name=>declared(name).getText(tree)).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 const entered:string[]=[],proof={image:{root},directories:new Map(),rows:new Map(),ancestors:new Map()};
 const lstatSync=(path:string)=>({isDirectory:()=>true,isFile:()=>false,isSymbolicLink:()=>false,dev:1n,nlink:1n,size:0n,blocks:0n,mtimeMs:1n});
 const ports={P:{maximumPathUnits:512,maximumEntries:64,maximumFileBytes:16777216,ignoredDirectories:[],ignoredFiles:[]},join,relative,sep,stamp:()=>"same",ancestor:()=>"id",nativeImageServiceEntryV1:()=>false,check:()=>{},requireNativeStoragePathV1:(_root:string,path:string)=>path,named:(_proof:unknown,path:string)=>lstatSync(path),lstatSync,CLOSURE_REPAIR_POLICY_V1:{maximumReceiptBytes:1},isClosureRepairReceiptNameV1:()=>false,
  namesAt:(_proof:unknown,path:string)=>{entered.push(path);return path===root?["inference"]:path===join(root,"inference")?["selections"]:path===join(root,"inference","selections")?[hash]:path===selection?["sa","np"]:[];}};
 Function(...Object.keys(ports),code+";return directories")(...Object.values(ports))(proof);
 expect(entered).not.toContain(analysis);
 expect(entered).toContain(other);
 expect(proof.directories.has(analysis)).toBe(false);
 expect(proof.directories.has(other)).toBe(true);
 expect(proof.ancestors.get(analysis)).toBe("id");
 for(const name of["inference/selections/"+hash+"/sa","inference/contexts/"+hash+"/sa"])expect(load.spill(name),name).toBe(false);
 for(const near of["inference/selections/"+hash+"/sa/ff","sa","inference/selections/short/sa","inference/selections/"+hash+"/np"])expect(new Function("name",ts.transpileModule(declared("liveSpillDirectory").getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return liveSpillDirectory(name)")(near),near).toBe(false);
});
