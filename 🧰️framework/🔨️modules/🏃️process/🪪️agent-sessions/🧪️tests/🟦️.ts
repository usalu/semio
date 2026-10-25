import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {chmodSync,mkdtempSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync,existsSync,readdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {
 AGENT_SESSION_POLICY_V1,AgentSessionErrorV1,adoptAgentSessionV1,agentClientProcessV1,agentSessionAddressV1,agentSessionEnvironmentV1,agentSessionIdV1,agentSessionMessageV1,agentSessionPathsV1,agentSessionSharesV1,agentSessionStateV1,agentSessionStoreV1,agentSessionWorktreeKeyV1,
 appendAgentSessionLedgerV1,closeAgentSessionV1,setAgentSessionCapabilityV1,collectAgentSessionsIfDueV1,collectAgentSessionsV1,currentAgentSessionV1,deriveAgentSessionV1,isAgentSessionV1,linuxAncestryV1,listAgentSessionsV1,openAgentSessionV1,parseAgentSessionAddressV1,parseProcStatV1,psAncestryV1,readAgentSessionTokenV1,readAgentSessionV1,
 renderAgentSessionListV1,renderAgentSessionStatusV1,resetAgentSessionsV1,systemAncestryV1,touchAgentSessionV1,windowsAncestryV1,writeAgentSessionTokenV1,WINDOWS_PROCESS_LISTING_V1,agentSessionRegistryRootV1,type AgentSeedV1,
} from "../🟦️.ts";

const scratch=mkdtempSync(join(tmpdir(),"as-core-")),python=testPythonV1(),oracle=python===null?undefined:JSON.parse(Bun.spawnSync([python,join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🧫️fixtures","🔣️.json")],{stdout:"pipe",stderr:"pipe"}).stdout.toString());
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
let counter=0;const registry=()=>join(scratch,"r"+counter++),store=(root=registry(),worktree="3c20d9163894")=>agentSessionStoreV1(root,worktree);
const derive=(row:{seed:unknown})=>deriveAgentSessionV1(row.seed as AgentSeedV1);

test("the schema, the authored policy and the closed vectors are valid under a strict third-party validator",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true});ajv.addSchema(schema,"s");
 for(const name of Object.keys(schema.definitions).filter(key=>!["Policy","Messages"].includes(key)))expect(ajv.getSchema("s#/definitions/"+name),name).toBeTruthy();
 expect(Object.keys(AGENT_SESSION_POLICY_V1)).toEqual(["version","environment","identity","resources","liveness","store"]);
 const check=ajv.getSchema("s#/definitions/AgentSession")!;
 const session={version:1,id:"a3q5dv7obc",worktree:"3c20d9163894",agent:{id:"x",client:"claude",origin:"explicit"},anchors:[{pid:3,started:"s"}],openedAtMs:1,resources:{cargo:{lane:"a3q5dv7obc"},bazel:{preference:5},nx:{namespace:"1fcf39843c24"},tests:{root:"a3q5dv7obc"}},lease:{owner:"x/a3q5dv7obc"},budget:{weight:2},capability:{scopes:["run"],projects:["p"]}};
 expect(check(session),JSON.stringify(check.errors)).toBe(true);expect(isAgentSessionV1(session)).toBe(true);
 for(const bad of[{...session,extra:1},{...session,id:"A1"},{...session,budget:{weight:17}},{...session,agent:{...session.agent,client:"vim"}},{...session,anchors:Array(9).fill({pid:1,started:""})},{...session,capability:{scopes:["root"],projects:[]}}]){expect(check(bad)).toBe(false);expect(isAgentSessionV1(bad)).toBe(false);}
});

test("every text exists in English and German with the same placeholders and every message id the code uses is authored",()=>{
 const en=schema.definitions.Messages.const.en as Record<string,string>,de=schema.definitions.Messages.const.de as Record<string,string>,holes=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(row=>row[1]!).sort();
 expect(Object.keys(en).sort()).toEqual(Object.keys(de).sort());
 for(const key of Object.keys(en))expect(holes(de[key]!),key).toEqual(holes(en[key]!));
 const source=["🪪️identity","🗄️store","🌿️current","🗣️messages"].map(name=>readFileSync(join(import.meta.dir,"..",name,"🟦️.ts"),"utf8")).join("\n");
 for(const row of source.matchAll(/(?:AgentSessionErrorV1|agentSessionMessageV1)\("([A-Za-z.]+)"/gu))expect(en[row[1]!],row[1]).toBeDefined();
 for(const state of schema.definitions.State.enum)expect(en["state."+state]).toBeDefined();
 for(const origin of schema.definitions.Origin.enum)expect(en["origin."+origin==="origin.process"?"origin.process":"origin."+origin],origin).toBeDefined();
 expect(()=>agentSessionMessageV1("nope",{},"en")).toThrow();expect(()=>agentSessionMessageV1("closed",{id:"x"},"en")).toThrow();
 expect(agentSessionMessageV1("closed",{id:"a",released:1,kept:0},"de")).toBe("Agent-Sitzung a geschlossen: 1 Ressource(n) freigegeben, 0 behalten.");
});

test("identity vectors: hand-stated fields, the pinned reference rows and the independent python derivation agree",()=>{
 for(const [index,row] of fixture.identity.entries()){
  const derived=derive(row),expected=row.expected as Record<string,any>;
  expect(derived.agent.id,row.name).toBe(expected.agent);expect(derived.agent.client,row.name).toBe(expected.client);expect(derived.agent.origin,row.name).toBe(expected.origin);
  expect(derived.id,row.name).toBe(expected.session);expect(derived.resources.cargo.lane,row.name).toBe(expected.lane);expect(derived.resources.bazel.preference,row.name).toBe(expected.bazel);expect(derived.resources.nx.namespace,row.name).toBe(expected.nx);expect(derived.resources.tests.root,row.name).toBe(expected.testsRoot);expect(derived.lease.owner,row.name).toBe(expected.leaseOwner);expect((derived.anchor??null) as unknown,row.name).toEqual((expected.anchor??null) as unknown);
  for(const [key,value] of Object.entries(row.hand as Record<string,unknown>))expect({agent:derived.agent.id,client:derived.agent.client,origin:derived.agent.origin,session:derived.id,anchor:derived.anchor}[key as "agent"],row.name+" "+key).toEqual(value as never);
  if(oracle!==undefined)expect(oracle.identity[index],row.name).toEqual(expected);
 }
 expect(()=>deriveAgentSessionV1({agent:"///",environment:{},ancestry:[]}) ).not.toThrow();
 expect(()=>deriveAgentSessionV1({agent:"",environment:{},ancestry:[]})).toThrow(AgentSessionErrorV1);
 for(const weight of[0,17,1.5,Number.NaN])expect(()=>deriveAgentSessionV1({agent:"a",environment:{},ancestry:[],weight})).toThrow(AgentSessionErrorV1);
 expect(deriveAgentSessionV1({agent:"a",environment:{},ancestry:[],weight:16}).budget.weight).toBe(16);
 const identifiers=new Set(fixture.identity.map(row=>derive(row).id));expect(identifiers.size).toBeGreaterThan(10);
 for(const id of identifiers)expect(id).toMatch(/^[a-z][a-z0-9]{0,11}$/u);
});

test("client processes are recognised by their executable, never by arguments such as shell snapshot paths",()=>{
 for(const [index,row] of fixture.clientProcess.entries()){expect(agentClientProcessV1(row.command)??null,row.command).toBe(row.client);if(oracle!==undefined)expect(oracle.clientProcess[index]).toBe(row.client);}
});

test("session state and budget shares follow the vectors and the python reference; shares always add up to the whole",()=>{
 for(const [index,row] of fixture.state.entries()){expect(agentSessionStateV1(row.facts),JSON.stringify(row.facts)).toBe(row.state);if(oracle!==undefined)expect(oracle.state[index]).toBe(row.state);}
 for(const [index,row] of fixture.shares.entries()){
  const got=agentSessionSharesV1(row.rows);expect(got,JSON.stringify(row.rows)).toEqual(row.shares as Record<string,number>);if(oracle!==undefined)expect(oracle.shares[index]).toEqual(row.shares);
  if(row.rows.length>0)expect(Object.values(got).reduce((sum,value)=>sum+value,0)).toBe(10000);
 }
 let seed=7;const next=()=>(seed=(seed*1103515245+12345)&0x7fffffff);
 for(let round=0;round<200;round++){
  const rows=Array.from({length:1+next()%9},(_,index)=>({id:"s"+index,weight:1+next()%16})),got=agentSessionSharesV1(rows);
  expect(Object.values(got).reduce((sum,value)=>sum+value,0)).toBe(10000);
  for(const row of rows){const exact=row.weight*10000/rows.reduce((sum,value)=>sum+value.weight,0);expect(Math.abs(got[row.id]!-exact)).toBeLessThan(1);}
 }
});

test("the ancestry readers keep the vectors they moved with: /proc stat, the Linux chain, the Windows listing and the POSIX ps listing",()=>{
 const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["name","text","parent","started"],properties:{name:{type:"string"},text:{type:"string"},parent:{type:["integer","null"]},started:{type:["string","null"]}}});
 for(const row of fixture.stat){expect(check(row),row.name).toBe(true);const parsed=parseProcStatV1(row.text);expect(parsed?.parent??null,row.name).toBe(row.parent);expect(parsed?.started??null,row.name).toBe(row.started);}
 const files=fixture.linuxChain.files as Record<string,string>,rows=linuxAncestryV1(path=>files[path]??null,fixture.linuxChain.first);
 expect(rows).toEqual(fixture.linuxChain.rows);expect(linuxAncestryV1(()=>null,300)).toEqual([]);expect(linuxAncestryV1(path=>files[path]??null,1)).toEqual([]);
 const other:Record<string,string>={...files,"/proc/200/stat":files["/proc/200/stat"]!.replace(" 2000 "," 2001 ")};
 const first=derive({seed:{environment:{},ancestry:rows}}),second=derive({seed:{environment:{},ancestry:linuxAncestryV1(path=>files[path]??null,fixture.linuxChain.first)}}),restarted=derive({seed:{environment:{},ancestry:linuxAncestryV1(path=>other[path]??null,fixture.linuxChain.first)}});
 expect(first.id).toBe(second.id);expect(first.id).not.toBe(restarted.id);expect(first.agent.origin).toBe("process");
 const text=JSON.stringify(fixture.windows.listing),windows=windowsAncestryV1(text,fixture.windows.first);
 expect(windows.map(row=>row.pid)).toEqual(fixture.windows.pids);expect(windowsAncestryV1(text,fixture.windows.orphanFirst).map(row=>row.pid)).toEqual(fixture.windows.orphanPids);
 expect(derive({seed:{environment:{},ancestry:windows}}).agent.client).toBe("claude");expect(derive({seed:{environment:{},ancestry:windowsAncestryV1(JSON.stringify(fixture.windows.listing.slice(0,1)),fixture.windows.first)}}).id).toBe("d");
 expect(windowsAncestryV1(JSON.stringify(fixture.windows.single),77).map(row=>row.pid)).toEqual([77]);expect(windowsAncestryV1("not json",77)).toEqual([]);expect(windowsAncestryV1("[{\"pid\":\"x\"}]",77)).toEqual([]);
 for(const word of["Win32_Process","ConvertTo-Json -Compress","ToUniversalTime","CommandLine"])expect(WINDOWS_PROCESS_LISTING_V1).toContain(word);
 const ps=psAncestryV1(fixture.psListing.text,fixture.psListing.first);expect(ps.map(row=>row.pid)).toEqual(fixture.psListing.pids);expect(ps[0]!.started).toBe("Wed Oct  7 10:06:00 2026");expect(ps[1]!.command).toBe("/bin/zsh -l");
 for(const limit of[1,2]){expect(linuxAncestryV1(path=>files[path]??null,fixture.linuxChain.first,limit)).toEqual(fixture.linuxChain.rows.slice(0,limit));expect(windowsAncestryV1(text,fixture.windows.first,limit).map(row=>row.pid)).toEqual(fixture.windows.pids.slice(0,limit));expect(psAncestryV1(fixture.psListing.text,fixture.psListing.first,limit).map(row=>row.pid)).toEqual(fixture.psListing.pids.slice(0,limit));}
 expect(psAncestryV1(fixture.psListing.text,fixture.psListing.orphanFirst).map(row=>row.pid)).toEqual([777]);expect(psAncestryV1("",5)).toEqual([]);expect(psAncestryV1("1 1 Mon Oct 20 08:00:00 2026 loop\n 2 3 Mon Oct 20 08:00:00 2026 a\n 3 2 Mon Oct 20 08:00:00 2026 b\n",2).length).toBe(2);
});
test.skipIf(process.platform==="win32")("on a real POSIX host the single ps listing starts at the parent process",()=>{
 const rows=systemAncestryV1();expect(rows[0]!.pid).toBe(process.ppid);expect(rows.every(row=>row.command.length>0)).toBe(true);
},60000);
test.skipIf(process.platform!=="win32")("on a real Windows host the CIM listing starts at the parent process",()=>{expect(systemAncestryV1()[0]!.pid).toBe(process.ppid);});

test("the store opens a session once, merges anchors, updates the weight, writes nothing when nothing changed and refuses foreign or malformed records",()=>{
 const s=store(),derived=derive({seed:{agent:"alpha",environment:{},ancestry:[]}}),alive=(pid:number)=>pid<1000;
 const first=openAgentSessionV1(s,derived,{now:1000,alive});expect(first.created).toBe(true);expect(first.session.worktree).toBe(s.worktree);expect(first.session.anchors).toEqual([]);
 const record=join(s.directory,derived.id+".json"),before=statSync(record).mtimeMs;
 const again=openAgentSessionV1(s,derived,{now:2000,alive});expect(again.created).toBe(false);expect(again.session).toEqual(first.session);expect(statSync(record).mtimeMs).toBe(before);
 const anchored=openAgentSessionV1(s,{...derived,anchor:{pid:10,started:"a"}},{now:3000,alive});expect(anchored.session.anchors).toEqual([{pid:10,started:"a"}]);
 expect(openAgentSessionV1(s,{...derived,anchor:{pid:10,started:"b"}},{now:3000,alive}).session.anchors).toEqual([{pid:10,started:"b"}]);
 for(let pid=11;pid<11+12;pid++)openAgentSessionV1(s,{...derived,anchor:{pid,started:""}},{now:3000,alive:p=>p!==15});
 const merged=readAgentSessionV1(s,derived.id)!.anchors;expect(merged.length).toBe(8);expect(merged.at(-1)!.pid).toBe(22);expect(merged.map(row=>row.pid)).not.toContain(15);
 expect(openAgentSessionV1(s,derived,{now:4000,alive,weight:5}).session.budget.weight).toBe(5);expect(openAgentSessionV1(s,derived,{now:4000,alive}).session.budget.weight).toBe(5);
 writeFileSync(join(s.directory,"zzz.json"),"{not json");writeFileSync(join(s.directory,derived.id+"x.json"),JSON.stringify({...first.session,id:derived.id+"x",worktree:"ffffffffffff"}));
 expect(readAgentSessionV1(s,"zzz")).toBeUndefined();expect(readAgentSessionV1(s,derived.id+"x")).toBeUndefined();expect(listAgentSessionsV1(s,{now:4000,alive}).map(row=>row.session.id)).toEqual([derived.id]);
 const ledger=readFileSync(join(s.directory,"ledger.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));expect(ledger.map(row=>row.event)).toEqual(["open"]);
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"s");for(const row of ledger)expect(ajv.getSchema("s#/definitions/LedgerEvent")!(row)).toBe(true);
});

test("sessions are listed with state, last use and the budget shared by weight between the live ones",()=>{
 const s=store(),now=10_000_000,mk=(agent:string,weight:number)=>openAgentSessionV1(s,derive({seed:{agent,environment:{},ancestry:[]}}),{now:0,weight}).session;
 const a=mk("a1",1),b=mk("b1",3),c=mk("c1",1),d=openAgentSessionV1(s,deriveAgentSessionV1({environment:{},ancestry:[]}),{now:0}).session;
 touchAgentSessionV1(s,c.id,now-1000,true);touchAgentSessionV1(s,b.id,now-2_000_000,true);
 const rows=listAgentSessionsV1(s,{now,alive:()=>false}),by=Object.fromEntries(rows.map(row=>[row.session.agent.id,row]));
 expect(by.a1!.state).toBe("gone");expect(by.b1!.state).toBe("idle");expect(by.c1!.state).toBe("live");expect(by.default!.state).toBe("live");
 expect(by.c1!.shareBasisPoints+by.default!.shareBasisPoints).toBe(10000);expect(by.a1!.shareBasisPoints).toBe(0);expect(by.b1!.shareBasisPoints).toBe(0);
 expect(d.id).toBe("d");expect(b.budget.weight).toBe(3);expect(a.id).not.toBe(c.id);
 const text=renderAgentSessionListV1(rows,now,"en");expect(text[0]).toContain("SESSION");expect(text.length).toBe(5);expect(text.join("\n")).toContain("live");
 expect(renderAgentSessionListV1([],0,"de")).toEqual(["Keine Agent-Sitzungen in diesem Arbeitsbaum."]);
 const lines=renderAgentSessionStatusV1({row:by.c1!,live:2,nowMs:now,paths:agentSessionPathsV1(c,join(scratch,"st"))},"de");
 expect(lines.join("\n")).toContain("Budgetanteil:");expect(lines.join("\n")).toContain(join(scratch,"st",c.resources.tests.root,"t"));expect(lines.find(row=>row.startsWith("Kapazitätszusagen"))).toBeUndefined();
});

test("collection reclaims the resources of gone sessions first, keeps a session whose resources are still used, never collects the default session and runs at most once per interval",()=>{
 const s=store(),mk=(agent:string)=>openAgentSessionV1(s,derive({seed:{agent,environment:{},ancestry:[]}}),{now:0}).session;
 const gone=mk("gone"),busy=mk("busy"),fresh=mk("fresh"),base=openAgentSessionV1(s,deriveAgentSessionV1({environment:{},ancestry:[]}),{now:0}).session;
 const now=10_000_000;touchAgentSessionV1(s,fresh.id,now-1000,true);const seen:string[]=[];
 const reclaim=(session:{id:string})=>{seen.push(session.id);return session.id===busy.id?{released:1,kept:1}:{released:2,kept:0};};
 expect(collectAgentSessionsV1(s,reclaim,{only:[base.id],now,alive:()=>false})).toEqual([]);expect(readAgentSessionV1(s,base.id)).toBeDefined();
 const dry=collectAgentSessionsV1(s,reclaim,{now,alive:()=>false,dryRun:true});expect(dry.map(row=>row.session.id).sort()).toEqual([busy.id,gone.id].sort());expect(seen).toEqual([]);
 const rows=collectAgentSessionsV1(s,reclaim,{now,alive:()=>false});
 expect(Object.fromEntries(rows.map(row=>[row.session.agent.id,[row.released,row.kept,row.removed]]))).toEqual({gone:[2,0,true],busy:[1,1,false]});
 expect(readAgentSessionV1(s,gone.id)).toBeUndefined();expect(readAgentSessionV1(s,busy.id)).toBeDefined();expect(readAgentSessionV1(s,fresh.id)).toBeDefined();expect(readAgentSessionV1(s,base.id)).toBeDefined();
 const events=readFileSync(join(s.directory,"ledger.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line)).filter(row=>row.event!=="open");expect(events.map(row=>row.event)).toEqual(["close","collect"]);
 const t=store(),late=openAgentSessionV1(t,derive({seed:{agent:"late",environment:{},ancestry:[]}}),{now:0}).session;let calls=0;
 expect(collectAgentSessionsIfDueV1(t,()=>{calls++;return {released:0,kept:0};},{now:Date.now()+10_000_000_000,alive:()=>false}).length).toBe(1);
 expect(collectAgentSessionsIfDueV1(t,()=>{calls++;return {released:0,kept:0};},{now:Date.now()+10_000_000_000,alive:()=>false}).length).toBe(0);expect(calls).toBe(1);expect(readAgentSessionV1(t,late.id)).toBeUndefined();
 expect(closeAgentSessionV1(t,"zz","x")).toBe(false);
 const u=store(),one=openAgentSessionV1(u,derive({seed:{agent:"one",environment:{},ancestry:[]}}),{now:0}).session;
 expect(collectAgentSessionsV1(u,()=>({released:0,kept:0}),{only:[one.id],reason:"close"}).map(row=>row.removed)).toEqual([true]);
});

test("capability tokens live in a private file beside the record, never in the record",()=>{
 const s=store(),session=openAgentSessionV1(s,derive({seed:{agent:"tok",environment:{},ancestry:[]}})).session;
 expect(readAgentSessionTokenV1(s,session.id)).toBeUndefined();writeAgentSessionTokenV1(s,session.id,"v1.payload.mac");
 expect(readAgentSessionTokenV1(s,session.id)).toBe("v1.payload.mac");if(process.platform!=="win32")expect(statSync(join(s.directory,session.id+".token")).mode&0o777).toBe(0o600);
 expect(readFileSync(join(s.directory,session.id+".json"),"utf8")).not.toContain("payload");closeAgentSessionV1(s,session.id,"test");expect(readAgentSessionTokenV1(s,session.id)).toBeUndefined();
 appendAgentSessionLedgerV1(s,{event:"open",session:"x",agent:"y",reason:"r".repeat(5)});
});

test("the issued capability is recorded in the session record without the token and refuses an open schema",()=>{
 const s=store(),session=openAgentSessionV1(s,derive({seed:{agent:"cap",environment:{},ancestry:[]}})).session;
 const next=setAgentSessionCapabilityV1(s,session.id,{scopes:["run","status"],projects:["lib-a"],expiresAtMs:5000})!;
 expect(next.capability).toEqual({scopes:["run","status"],projects:["lib-a"],expiresAtMs:5000});expect(readAgentSessionV1(s,session.id)!.capability).toEqual(next.capability);
 expect(()=>setAgentSessionCapabilityV1(s,session.id,{scopes:["root"] as never,projects:[]})).toThrow();expect(setAgentSessionCapabilityV1(s,"zz",{scopes:[],projects:[]})).toBeUndefined();
});

test("the current session resolves the environment address first, then the explicit name, the client variable, the connection, the client process and the default, and a process resolves each identity once",()=>{
 resetAgentSessionsV1();const root=registry(),common={registry:root,worktree:"/work/tree",alive:()=>true};let spawned=0;
 const ancestry=()=>{spawned++;return [{pid:200,started:"Mon Oct 20 10:00:00 2026",command:"claude --resume"}];};
 const byProcess=currentAgentSessionV1({...common,environment:{},ancestry});expect(byProcess.agent.origin).toBe("process");expect(byProcess.anchors.map(row=>row.pid)).toEqual([200]);
 expect(currentAgentSessionV1({...common,environment:{},ancestry}).id).toBe(byProcess.id);expect(spawned).toBe(1);
 const viaEnvironment=currentAgentSessionV1({...common,environment:{CLAUDE_CODE_SESSION_ID:"uuid-1"},ancestry});expect(viaEnvironment.agent).toEqual({id:"uuid-1",client:"claude",origin:"environment"});expect(spawned).toBe(1);
 const address=agentSessionAddressV1(viaEnvironment);expect(address).toMatch(/^[0-9a-f]{12}\.[a-z][a-z0-9]{0,11}$/u);expect(parseAgentSessionAddressV1(address)).toEqual({worktree:viaEnvironment.worktree,id:viaEnvironment.id});expect(parseAgentSessionAddressV1("x.y")).toBeUndefined();expect(parseAgentSessionAddressV1(undefined)).toBeUndefined();
 resetAgentSessionsV1();const attached=currentAgentSessionV1({registry:root,environment:agentSessionEnvironmentV1(viaEnvironment),ancestry:()=>{throw Error("no ancestry needed");}});expect(attached.id).toBe(viaEnvironment.id);
 resetAgentSessionsV1();const explicit=currentAgentSessionV1({...common,environment:agentSessionEnvironmentV1(viaEnvironment),agent:"named",client:"codex-cli"});expect(explicit.agent).toEqual({id:"named",client:"codex",origin:"explicit"});expect(explicit.id).not.toBe(viaEnvironment.id);
 const viaConnection=currentAgentSessionV1({...common,environment:{},connection:{client:"claude-code",clientProcess:5,process:6},ownerPid:6});expect(viaConnection.agent.origin).toBe("connection");expect(viaConnection.anchors.map(row=>row.pid)).toEqual([6]);
 const service=currentAgentSessionV1({...common,environment:{},agent:"lib-a",origin:"service"});expect(service.agent.client).toBe("service");
 const stale=currentAgentSessionV1({...common,registry:registry(),environment:{SEMIO_AGENT_SESSION:"3c20d9163894.a0000000000"},ancestry:()=>[]});expect(stale.agent.origin).toBe("default");expect(stale.id).toBe("d");
 const env:NodeJS.ProcessEnv={};resetAgentSessionsV1();const adopted=adoptAgentSessionV1(env,{...common,ancestry:()=>[]});expect(env.SEMIO_AGENT_SESSION).toBe(agentSessionAddressV1(adopted));
 for(const name of Object.keys(env))expect(name).toBe("SEMIO_AGENT_SESSION");
 expect(agentSessionRegistryRootV1({SEMIO_AGENT_SESSION_DIRECTORY:"/x"})).toBe("/x");expect(agentSessionRegistryRootV1({},"darwin","/h")).toBe("/h/Library/Caches/semio/agent-sessions");expect(agentSessionRegistryRootV1({LOCALAPPDATA:"C:/L"},"win32","/h")).toContain("semio");expect(agentSessionRegistryRootV1({XDG_CACHE_HOME:"/c"},"linux","/h")).toBe("/c/semio/agent-sessions");
 expect(agentSessionWorktreeKeyV1(undefined)).toBe("000000000000");expect(agentSessionWorktreeKeyV1("/Work/Tree","darwin")).toBe(agentSessionWorktreeKeyV1("/work/tree","darwin"));expect(agentSessionWorktreeKeyV1("/work/tree","linux")).not.toBe(agentSessionWorktreeKeyV1("/work/other","linux"));
 expect(agentSessionIdV1("default")).toBe("d");
});

test("source law: no module outside the tests reads a removed per-tool identity variable",()=>{
 const roots=["🔒️leases","🧪️testing","🏗️build"].map(name=>join(import.meta.dir,"..","..",name));
 const removed=["SEMIO_AGENT_ID","SEMIO_CARGO_LANE","SEMIO_SESSION\\b"],found:string[]=[];
 const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){const path=join(directory,entry.name);if(entry.isDirectory()){if(!["node_modules","🧪️tests","🧫️fixtures"].includes(entry.name))walk(path);}else if(entry.name.endsWith(".ts")){const text=readFileSync(path,"utf8");for(const name of removed)if(new RegExp(name,"u").test(text))found.push(path+" "+name);}}};
 for(const root of roots)if(existsSync(root))walk(root);
 expect(found).toEqual([]);
});
