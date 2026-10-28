/** 🔀️ Contract: a delta whose owners no peer touched since its base merges onto the peer's head with every owner of both sides, pseudo owners keep the union of both supports, and the stamp never covers what a peer rewrote; a peer that touched one of its owners, a stub, the environment, or the same global or membership differently refuses the merge. Every decision over seeded worlds on real fact indexes equals the independent Python model in `🐍️oracle/🐍️.py`. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {compareCodePointsV1,createFactIndexV1,rowKeyV1,type GenerationManifestV1,type GenerationRowV1} from "../../🟨️.mjs";
import {CUTOFF_POLICY_V1} from "../../✂️cutoff/🟨️.mjs";
import {decodeVerifiedScopeV1,encodeVerifiedScopeV1,readVerifiedStampV1} from "../../🔭️coverage/🟨️.mjs";
import schema from "../🧬️schema/🔣️.json";
import {MERGE_POLICY_V1,isPseudoOwnerV1,mergeGenerationV1,ownerRowsOfV1} from "../🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle/🐍️.py");
const hex=(text:string)=>createHash("sha256").update(text).digest("hex"),DERIVED=CUTOFF_POLICY_V1.ownerPrefix+"0".repeat(31)+"1";
type Side={env:string;global:string;membership:string;owners:Record<string,Record<string,unknown>>;rows:Record<string,string[]>;covered:string[]|null;at:number|null};
type Case={base:Side;head:Side;mine:Side;patch:{drop:string[];upserts:Record<string,string[]>}};
const row=(path:string):GenerationRowV1=>({kind:"file",path,fact:1});
const indexOf=(side:Side)=>createFactIndexV1(new Map(Object.entries(side.rows).map(([name,paths])=>[name,paths.map(row)])));
const manifestOf=(side:Side):GenerationManifestV1=>({version:1,generation:0,parent:null,identity:hex("i"),implementation:hex("m"),env:side.env,membership:side.membership,frontier:hex("f"+side.membership),factIndex:hex("x"),global:side.global,owners:side.owners as never,built:{kind:"delta",base:null,dirty:[]}});
const stampOf=(side:Side)=>side.covered===null?undefined:readVerifiedStampV1({atMs:side.at!,...encodeVerifiedScopeV1(Object.keys(side.owners).sort(compareCodePointsV1),new Set(side.covered))});

function decide(entry:Case){
 const found=mergeGenerationV1({base:manifestOf(entry.base),head:manifestOf(entry.head),mine:manifestOf(entry.mine),baseIndex:indexOf(entry.base),headIndex:indexOf(entry.head),patch:{drop:entry.patch.drop,upserts:new Map(Object.entries(entry.patch.upserts).map(([name,paths])=>[name,paths.map(row)])),append:new Map()},verified:stampOf(entry.mine),headVerified:stampOf(entry.head),key:rowKeyV1});
 if(found.conflict)return found.conflict==="owner"||found.conflict==="structure"?{conflict:found.conflict,owner:found.owner}:{conflict:found.conflict};
 const names=Object.keys(found.owners).sort(compareCodePointsV1);
 return{conflict:null,owners:Object.fromEntries(names.map(name=>[name,found.owners[name]])),global:found.global,membership:found.membership,
  upserts:Object.fromEntries([...found.upserts.keys()].sort().map(name=>[name,[...new Set(found.upserts.get(name)!.map(item=>item.path))].sort()])),theirs:found.theirs,ours:found.ours,
  covered:found.verified?{names:[...decodeVerifiedScopeV1(names,found.verified)].sort(),at:found.verified.atMs}:null};
}

function seeded(count:number,seed:number):Case[]{
 let state=seed;const random=()=>{state=(state*1103515245+12345)%2147483648;return state/2147483648;},chance=(p:number)=>random()<p;
 const ENV=hex("env"),owner=(name:string,version:string,env=ENV,extra:Record<string,unknown>={})=>({root:"pkg/"+name,kind:"project",fragment:hex(name+version),bytes:1,edgesEnv:env,targets:["build"],...extra});
 const out:Case[]=[];
 for(let at=0;at<count;at++){
  const names=Array.from({length:6},(_,index)=>"n"+index),base:Side={env:ENV,global:hex("g0"),membership:hex("m0"),owners:{},rows:{},covered:null,at:null};
  for(const name of names){base.owners[name]=owner(name,"0");base.rows[name]=[name+"/a",name+"/b"];}
  base.rows["@env"]=["env/a"];base.rows[DERIVED]=["d/answer","d/support"];base.rows.n0!.push("d/answer");
  const copy=(side:Side):Side=>JSON.parse(JSON.stringify(side));
  const head=copy(base),mine=copy(base),patch:Case["patch"]={drop:[],upserts:{}};
  for(const name of names){
   if(chance(0.18)){head.owners[name]=owner(name,"h");head.rows[name]=[name+"/h"];}
   else if(chance(0.05))head.rows[name]=[name+"/h2"];
   if(chance(0.04))head.owners[name]={...head.owners[name],targets:["build","test"]};
   if(chance(0.2)){mine.owners[name]=owner(name,"m");mine.rows[name]=[name+"/m"];patch.upserts[name]=[name+"/m"];}
   else if(chance(0.04)){delete mine.owners[name];delete mine.rows[name];patch.drop.push(name);}
  }
  if(chance(0.05)){head.owners.h9=owner("h9","0");head.rows.h9=["h9/a"];}
  if(chance(0.1)){mine.owners.m9=owner("m9","0");mine.rows.m9=["m9/a"];patch.upserts.m9=["m9/a"];}
  patch.upserts["@env"]=chance(0.5)?["env/a"]:["env/b"];mine.rows["@env"]=patch.upserts["@env"];
  if(chance(0.3)){head.rows["@env"]=["env/c"];}
  if(chance(0.3)){patch.upserts[DERIVED]=["d/answer","d/support2"];mine.rows[DERIVED]=patch.upserts[DERIVED];}
  if(chance(0.2))head.rows[DERIVED]=["d/answer","d/support3"];
  if(chance(0.04))head.env=hex("env2");
  if(chance(0.04)){mine.env=hex("env3");for(const name of Object.keys(mine.owners))if(chance(0.7))mine.owners[name]={...mine.owners[name],edgesEnv:mine.env};}
  if(chance(0.15))head.global=hex("gh");if(chance(0.15))mine.global=chance(0.5)?hex("gm"):head.global;
  if(chance(0.3))head.membership=hex("mh");if(chance(0.3))mine.membership=chance(0.5)?hex("mm"):head.membership;
  if(chance(0.7)){mine.at=1000+Math.floor(random()*100);mine.covered=Object.keys(mine.owners).filter(()=>chance(0.6));}
  if(chance(0.5)){head.at=1000+Math.floor(random()*100);head.covered=Object.keys(head.owners).filter(()=>chance(0.6));}
  out.push({base,head,mine,patch});
 }
 return out;
}

test("the declared policy is the one the implementation uses and names exactly the stub fields of an owner row",()=>{
 expect(MERGE_POLICY_V1).toEqual(schema.definitions.Policy.const as never);
 expect(isPseudoOwnerV1("@env")).toBe(true);expect(isPseudoOwnerV1(DERIVED)).toBe(true);expect(isPseudoOwnerV1("n0")).toBe(false);
});

test("the rows of an owner are read from its classes only",()=>{
 const index=createFactIndexV1(new Map([["a",[row("x"),row("y")]],["b",[row("y"),row("z")]],["c",[]]]));
 const rows=ownerRowsOfV1(index,new Set(["a","b","c","missing"]));
 expect(rows.get("a")!.map(item=>item.path).sort()).toEqual(["x","y"]);expect(rows.get("b")!.map(item=>item.path).sort()).toEqual(["y","z"]);
 expect(rows.get("c")).toEqual([]);expect(rows.get("missing")).toEqual([]);
});

test("a disjoint delta merges: both sides' owners survive, the pseudo owners keep both supports, the stamp drops what peers rewrote",()=>{
 const [entry]=seeded(1,7),ENV=entry!.base.env;
 const base=entry!.base,head:Side=JSON.parse(JSON.stringify(base)),mine:Side=JSON.parse(JSON.stringify(base));
 head.owners.n1={...base.owners.n1,fragment:hex("peer")};head.rows.n1=["n1/peer"];head.rows["@env"]=["env/peer"];head.covered=["n1","n2"];head.at=900;
 mine.owners.n4={...base.owners.n4,fragment:hex("mine")};mine.rows.n4=["n4/mine"];mine.covered=["n1","n4","n5"];mine.at=1000;
 const found=decide({base,head,mine,patch:{drop:[],upserts:{n4:["n4/mine"],"@env":["env/mine"]}}}) as any;
 expect(found.conflict).toBeNull();expect(found.owners.n1.fragment).toBe(hex("peer"));expect(found.owners.n4.fragment).toBe(hex("mine"));
 expect(found.upserts["@env"]).toEqual(["env/mine","env/peer"]);expect(found.upserts.n4).toEqual(["n4/mine"]);
 expect(found.covered).toEqual({names:["n1","n2","n4","n5"],at:900});expect(found.theirs).toEqual(["n1"]);expect(found.ours).toEqual(["n4"]);
 expect(ENV).toBe(found.owners.n1.edgesEnv);
});

test("an owner both sides wrote, or whose facts a peer changed, refuses the merge and names the owner",()=>{
 const [entry]=seeded(1,9),base=entry!.base;
 const head:Side=JSON.parse(JSON.stringify(base)),mine:Side=JSON.parse(JSON.stringify(base));
 head.owners.n2={...base.owners.n2,fragment:hex("peer")};mine.owners.n2={...base.owners.n2,fragment:hex("mine")};
 expect(decide({base,head,mine,patch:{drop:[],upserts:{n2:["n2/m"]}}})).toEqual({conflict:"owner",owner:"n2"});
 const quiet:Side=JSON.parse(JSON.stringify(base));quiet.rows.n3=["n3/touched"];
 expect(decide({base,head:quiet,mine:base,patch:{drop:[],upserts:{n3:["n3/x"]}}})).toEqual({conflict:"owner",owner:"n3"});
 const stub:Side=JSON.parse(JSON.stringify(base));stub.owners.n5={...base.owners.n5,targets:["build","lint"]};
 expect(decide({base,head:stub,mine:base,patch:{drop:[],upserts:{}}})).toEqual({conflict:"structure",owner:"n5"});
});

test("a writer that added, removed or re-targeted a project refuses to merge onto a peer that changed any owner, and merges onto a peer that changed none",()=>{
 const [entry]=seeded(1,11),base=entry!.base,copy=(side:Side):Side=>JSON.parse(JSON.stringify(side));
 const head=copy(base);head.owners.n1={...base.owners.n1,fragment:hex("peer")};head.rows.n1=["n1/peer"];
 const added=copy(base);added.owners.m9={...base.owners.n0,root:"pkg/m9",fragment:hex("m9")};added.rows.m9=["m9/a"];
 expect(decide({base,head,mine:added,patch:{drop:[],upserts:{m9:["m9/a"]}}})).toEqual({conflict:"structure",owner:"m9"});
 const removed=copy(base);delete removed.owners.n3;delete removed.rows.n3;
 expect(decide({base,head,mine:removed,patch:{drop:["n3"],upserts:{}}})).toEqual({conflict:"structure",owner:"n3"});
 const targeted=copy(base);targeted.owners.n4={...base.owners.n4,fragment:hex("t"),targets:["build","lint"]};
 expect(decide({base,head,mine:targeted,patch:{drop:[],upserts:{n4:["n4/t"]}}})).toEqual({conflict:"structure",owner:"n4"});
 const quiet=copy(base);quiet.rows["@env"]=["env/peer"];
 expect((decide({base,head:quiet,mine:added,patch:{drop:[],upserts:{m9:["m9/a"]}}}) as any).conflict).toBeNull();
});

test("every decision over 600 seeded worlds equals the Python model, and every outcome kind occurs",()=>{
 const cases=seeded(600,20261027),ours=cases.map(decide),kinds=new Set<string>(ours.map(found=>found.conflict??"merged"));
 for(const kind of["merged","owner","structure","environment","global","membership"])expect(kinds.has(kind)).toBe(true);
 if(python){const result=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify({cases}))});expect(result.exitCode).toBe(0);const theirs=JSON.parse(result.stdout.toString()).results;for(let at=0;at<cases.length;at++)expect(ours[at],"case "+at).toEqual(theirs[at]);}
});
