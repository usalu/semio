import {afterAll,describe,expect,spyOn,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {createHmac} from "node:crypto";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import callerSchema from "../../../🧬️schema/📥️caller.json";
import catalog from "../../../⚙️operations/🔣️catalog.json";
import birth from "../../🧪️tests/📥️birth/🔣️.json";
import corpus from "./🔣️.json";
import {activeServiceTicketV1,createServiceAdmissionV1,createServiceAuditV1,createServiceGrantV1,declaredCapabilitiesV1,holdServiceInvocationV1,readServiceGrantV1,ServiceRefusalV1,serviceInvocationPolicyV1 as policy,type ServiceGrantV1,type ServiceAuditRowV1,type ServiceRequestV1} from "../🟦️.ts";
import {attenuateCapabilityV1,canonicalJsonV1,canonicalRootV1,capabilityIdV1,createRevocationRegistryV1,createRootKeyV1,endpointsV1,keyDigestV1,mintCapabilityV1,worktreeIdV1} from "../🔑️session/🟨️.mjs";
import {createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1,readOwnedBuildInvocationV1} from "../../🟦️.ts";
import {projectOwnedNxProcessEnvelopeV1} from "../../../🔧️tools/🧶️nx/📨️envelope/🟦️.ts";
import {createBuildBootstrapCapabilitiesV1} from "../../📤️child/🟦️.ts";
import {createScriptProcessEnvelope,readScriptPolicy,withScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";

const ajv=new Ajv({strict:false,allErrors:true});
const compile=(name:string)=>ajv.compile({...(schema.definitions as Record<string,object>)[name],definitions:schema.definitions});
const base=mkdtempSync(join(tmpdir(),"svc-core-")),shortBase=mkdtempSync(join(process.platform==="win32"||tmpdir().length<=40?tmpdir():"/tmp","sc-"));
const root=realpathSync((mkdirSync(join(base,"w1/pkgs/a"),{recursive:true}),join(base,"w1")));
const rootTwo=realpathSync((mkdirSync(join(base,"w2/pkgs/a"),{recursive:true}),join(base,"w2")));
mkdirSync(join(base,"outside"),{recursive:true});
symlinkSync(join(base,"outside"),join(root,"link"));
const cwd=join(root,"pkgs/a");
const key=createRootKeyV1();
const realRoot=realpathSync((()=>{for(let place=import.meta.dir;;place=dirname(place)){if(existsSync(join(place,"bunfig.toml"))&&existsSync(join(place,"nx.json")))return place;if(dirname(place)===place)throw Error("Workspace root not found");}})());
let at=1_000_000;
const now=()=>at;
const worktree=worktreeIdV1(root);
const grantFor=(extra:Partial<ServiceGrantV1>={}):ServiceGrantV1=>({version:1,worktree,keyDigest:keyDigestV1(key),operations:["run","run-many","affected"],maximumRequests:100,maximumConcurrent:2,...extra});
const caveatsFor=(extra:object[]=[],drop:string[]=[])=>[...[{k:"scope",v:["run"]},{k:"worktree",v:worktree},{k:"agent",v:"codex-1"},{k:"notAfter",v:at+60_000}].filter(caveat=>!drop.includes(caveat.k)),...extra];
const tokenFor=(extra:object[]=[],drop:string[]=[],withKey:Uint8Array=key)=>mintCapabilityV1(withKey,caveatsFor(extra,drop) as never);
const requestFor=(token:string,args:string[]=["run","app:build"],extra:Partial<ServiceRequestV1>={}):ServiceRequestV1=>({token,agent:"codex-1",cwd,args,...extra});

function harness(options:{grant?:ServiceGrantV1;root?:string;withKey?:Uint8Array;closure?:(named:readonly string[])=>Promise<ReadonlySet<string>>;failAudit?:{value:boolean}}={}){
 const rows:ServiceAuditRowV1[]=[],declared:string[][]=[],registry=createRevocationRegistryV1(join(base,"revoked-"+Math.random().toString(16).slice(2)+".jsonl"),now),validate=compile("Audit");
 const audit={record(row:ServiceAuditRowV1){if(options.failAudit?.value)throw new Error("disk full");expect(validate(row)).toBe(true);rows.push(row);}};
 const declare=async(args:readonly string[])=>{declared.push([...args]);return[["repo",...args],["nx","run",args[1]!,"--output-style=stream"]];};
 const admission=createServiceAdmissionV1({root:options.root??root,grant:options.grant??grantFor(),tool:"nx",key:options.withKey??key,revocations:registry,audit,declare,closure:options.closure,now});
 return{admission,rows,declared,registry};
}
const capabilities=(extra:object={})=>({workspaceRoot:root,launches:[["repo","service","executor"]],service:grantFor(),...extra});
const refused=async(promise:Promise<unknown>)=>{try{await promise;}catch(error){expect(error).toBeInstanceOf(ServiceRefusalV1);return(error as ServiceRefusalV1).code;}throw new Error("expected a refusal");};

describe("a genuine request",()=>{
 test("declares exactly the launches of the one-shot door for the duration of its command and records admit and finish",async()=>{
  const{admission,rows,declared}=harness(),held=capabilities(),token=tokenFor();
  expect(declaredCapabilitiesV1(held) as object).toEqual({workspaceRoot:root,launches:[["repo","service","executor"]]});
  const exit=await admission.run(requestFor(token),async ticket=>{
   expect(activeServiceTicketV1()).toBe(ticket);expect(ticket.session).toBe(capabilityIdV1(token)!);
   const view=declaredCapabilitiesV1(held) as {launches:string[][];service?:unknown};
   expect(view.launches).toEqual([["repo","service","executor"],["repo","run","app:build"],["nx","run","app:build","--output-style=stream"]]);expect("service" in view).toBe(false);
   return 7;
  });
  expect(exit).toBe(7);expect(activeServiceTicketV1()).toBeUndefined();expect(declaredCapabilitiesV1(held).launches).toEqual([["repo","service","executor"]]);
  expect(declared).toEqual([["run","app:build"]]);
  expect(rows.map(row=>[row.event,row.operation,row.agent,row.session,row.launches??null,row.exit??null])).toEqual([["admit","run","codex-1",capabilityIdV1(token)!,2,null],["finish","run","codex-1",capabilityIdV1(token)!,2,0]]);
  expect(JSON.stringify(rows)).not.toContain(token);
 });

 test("a failing command is recorded with exit 1 and releases its slot",async()=>{
  const{admission,rows}=harness({grant:grantFor({maximumConcurrent:1})}),token=tokenFor();
  await expect(admission.run(requestFor(token),async()=>{throw new Error("boom");})).rejects.toThrow("boom");
  await admission.run(requestFor(token),async()=>undefined);
  expect(rows.map(row=>row.event+":"+(row.exit??""))).toEqual(["admit:","finish:1","admit:","finish:0"]);
 });

 test("concurrent commands of different sessions each see only their own declaration",async()=>{
  const{admission}=harness(),held=capabilities(),seen:Record<string,string[]>={};
  const one=(name:string)=>admission.run(requestFor(tokenFor(),["run",name]),async()=>{await Bun.sleep(20);seen[name]=(declaredCapabilitiesV1(held) as {launches:string[][]}).launches.slice(1).map(launch=>launch[2]!);await Bun.sleep(20);seen[name+"/later"]=(declaredCapabilitiesV1(held) as {launches:string[][]}).launches.slice(1).map(launch=>launch[2]!);});
  await Promise.all([one("app:build"),one("lib:build")]);
  expect(seen).toEqual({"app:build":["app:build","app:build"],"app:build/later":["app:build","app:build"],"lib:build":["lib:build","lib:build"],"lib:build/later":["lib:build","lib:build"]});
 });
});

describe("the admission corpus (language-agnostic; an independent Python reference decides every row)",()=>{
 const symbols=(value:unknown):unknown=>{
  if(typeof value==="string")return value.replaceAll("@other-worktree","0".repeat(12)).replaceAll("@worktree",worktree).replaceAll("@root",root);
  if(Array.isArray(value))return value.map(symbols);
  if(value&&typeof value==="object"){const record=value as Record<string,unknown>;return"@now"in record?at+(record["@now"] as number):Object.fromEntries(Object.entries(record).map(([name,entry])=>[name,symbols(entry)]));}
  return value;
 };
 type Spec={kind?:string;drop?:string[];extra?:object[];attenuate?:object[]};
 const tokenOf=(spec:Spec):string=>{
  const caveats=[...(caveatsFor([],spec.drop??[])),...((symbols(spec.extra??[]) as object[]))] as never;
  const genuine=()=>(symbols(spec.attenuate??[]) as object[]).reduce((token:string,caveat)=>attenuateCapabilityV1(token,caveat as never),mintCapabilityV1(key,caveats));
  switch(spec.kind??"genuine"){
   case"empty":return"";
   case"garbage":return"v1.x.y";
   case"other-key":return mintCapabilityV1(createRootKeyV1(),caveats);
   case"forged-mac":return genuine().split(".").slice(0,2).join(".")+"."+Buffer.alloc(32).toString("base64url");
   case"tampered-scope":case"tampered-expiry":{
    const[version,body,mac]=genuine().split(".") as [string,string,string],payload=JSON.parse(Buffer.from(body,"base64url").toString());
    if(spec.kind==="tampered-scope")payload.caveats[0]={k:"scope",v:["run","admin"]};else payload.caveats=payload.caveats.filter((caveat:{k:string})=>caveat.k!=="notAfter");
    return[version,Buffer.from(JSON.stringify(payload)).toString("base64url"),mac].join(".");
   }
   default:return genuine();
  }
 };
 const places:Record<string,string>={inside:cwd,outside:join(base,"outside"),"other-worktree":join(rootTwo,"pkgs/a"),"symlink-out":join(root,"link"),missing:join(root,"nowhere")};
 const oracle=(()=>{
  const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json"),join(import.meta.dir,"../🧬️schema/🔣️.json"),join(import.meta.dir,"../../../⚙️operations/🔣️catalog.json")]);
  if(run.exitCode!==0)throw new Error(run.stderr.toString());
  return JSON.parse(run.stdout.toString()) as {requests:string[];arguments:{code:string;named:string[]|null}[]};
 })();

 test("the corpus and its schema are as authored",()=>{
  expect(corpus.clock).toBe(at);expect(new Set(corpus.requests.map(row=>row.name)).size).toBe(corpus.requests.length);
  expect(corpus.requests.length).toBeGreaterThanOrEqual(35);expect(corpus.arguments.length).toBeGreaterThanOrEqual(45);
  expect(oracle.requests.length).toBe(corpus.requests.length);expect(oracle.arguments.length).toBe(corpus.arguments.length);
 });

 corpus.requests.forEach((row:any,index:number)=>test(row.name+" is decided as "+row.expect,async()=>{
  const{admission,rows,declared,registry}=harness({grant:grantFor(row.grantOperations?{operations:row.grantOperations}:{})}),token=tokenOf(row.token),request=requestFor(token,row.args??["run","app:build"],{agent:row.agent??"codex-1",cwd:places[row.cwd??"inside"]!});
  if(row.revoked==="session")registry.revoke(capabilityIdV1(token)!);if(row.revoked==="agent")registry.revokeAgent(request.agent);
  let ran=false,code="ok";
  try{await admission.run(request,async()=>{ran=true;});}catch(error){expect(error).toBeInstanceOf(ServiceRefusalV1);code=(error as ServiceRefusalV1).code;}
  expect(code).toBe(row.expect);expect(code).toBe(oracle.requests[index]);
  expect(ran).toBe(row.expect==="ok");expect(declared.length).toBe(row.expect==="ok"?1:0);
  expect(rows.map(entry=>entry.event)).toEqual(row.expect==="ok"?["admit","finish"]:["refuse"]);
  if(row.expect!=="ok")expect(rows[0]!.code).toBe(row.expect);
  expect(JSON.stringify(rows)).not.toMatch(/v1\./u);
 }));

 corpus.arguments.forEach((row:any,index:number)=>test(row.args.slice(0,4).join(" ")+(row.args.length>4?" …":"")+" is decided as "+row.expect,async()=>{
  const{admission,declared}=harness({grant:grantFor(row.grantOperations?{operations:row.grantOperations}:{})}),token=tokenOf(row.token??{});
  let code="ok",named:readonly string[]|undefined;
  try{named=(await admission.admit(requestFor(token,row.args))).named;}catch(error){expect(error).toBeInstanceOf(ServiceRefusalV1);code=(error as ServiceRefusalV1).code;}
  expect(code).toBe(row.expect);expect(code).toBe(oracle.arguments[index]!.code);
  if(code==="ok"){expect(named).toEqual(row.named);expect(named).toEqual(oracle.arguments[index]!.named as string[]);expect(declared).toEqual([row.args]);}else expect(declared).toEqual([]);
 }));

 test("a request that is not closed data is refused",async()=>{
  const{admission}=harness();
  for(const request of[{...requestFor(tokenFor()),extra:1},{...requestFor(tokenFor()),cwd:""},{...requestFor(tokenFor()),args:[]},{...requestFor(tokenFor()),agent:"a b"},{...requestFor(tokenFor()),args:["run","a\u0000:build"]}])expect(await refused(admission.admit(request as never))).toBe("arguments");
 });
});

describe("cross worktree",()=>{
 test("a token, a grant or a key of one worktree never admits a request of another",async()=>{
  const worktreeTwo=worktreeIdV1(rootTwo),viaTwo=(extra:{grant?:ServiceGrantV1;withKey?:Uint8Array}={})=>harness({root:rootTwo,grant:extra.grant??grantFor({worktree:worktreeTwo}),withKey:extra.withKey});
  const request=(token:string,place=join(rootTwo,"pkgs/a"))=>requestFor(token,["run","app:build"],{cwd:place});
  expect(await refused(viaTwo().admission.admit(request(tokenFor())))).toBe("worktree");
  expect(await refused(viaTwo().admission.admit(request(tokenFor(),cwd)))).toBe("worktree");
  expect(await refused(viaTwo({grant:grantFor()}).admission.admit(request(tokenFor([{k:"worktree",v:worktreeTwo}],["worktree"]))))).toBe("grant");
  const wrongKey=createRootKeyV1();
  expect(await refused(viaTwo({withKey:wrongKey}).admission.admit(request(tokenFor([{k:"worktree",v:worktreeTwo}],["worktree"],wrongKey))))).toBe("grant");
  const ok=await viaTwo().admission.admit(request(tokenFor([{k:"worktree",v:worktreeTwo}],["worktree"])));expect(ok.operation).toBe("run");
 });
});

describe("grants narrower than the catalogue",()=>{
 test("every operation a grant may carry is a catalogue operation and every flag it takes exists in the catalogue",()=>{
  for(const operation of policy.operations){
   const entry=catalog.operations.find(candidate=>candidate.name===operation)!;expect(entry).toBeDefined();
   for(const flag of(entry as {flags?:string[]}).flags??[])expect((catalog.flags as {name:string}[]).some(candidate=>candidate.name===flag)).toBe(true);
  }
  expect(policy.forbiddenFlags.every(flag=>(catalog.flags as {name:string}[]).some(candidate=>candidate.name===flag)||flag==="--graph")).toBe(true);
 });
});

describe("dependencies of the named projects",()=>{
 const request=(closure?:(named:readonly string[])=>Promise<ReadonlySet<string>>)=>harness({closure});
 test("a dependency task is admitted only when the graph says it is a dependency of what the client named",async()=>{
  const named=["c"],token=attenuateCapabilityV1(tokenFor(),{k:"projects",v:["c"]}),dependency=requestFor(token,["run","b:build"],{named});
  expect((await request(async()=>new Set(["c","b","a"])).admission.admit(dependency)).args).toEqual(["run","b:build"]);
  expect(await refused(request(async()=>new Set(["c"])).admission.admit(dependency))).toBe("closure");
  expect(await refused(request().admission.admit(dependency))).toBe("closure");
  expect(await refused(request(async()=>new Set(["b"])).admission.admit(requestFor(token,["run","b:build"])))).toBe("scope");
  expect((await request().admission.admit(requestFor(token,["run","c:build"],{named}))).args).toEqual(["run","c:build"]);
 });
});

describe("revocation, expiry and limits",()=>{
 test("a revoked session or agent is refused, and a session revoked while its command runs loses its declaration at once",async()=>{
  const first=harness(),token=tokenFor(),held=capabilities();
  first.registry.revoke(capabilityIdV1(token)!);
  expect(await refused(first.admission.admit(requestFor(token)))).toBe("revoked");
  const second=harness();second.registry.revokeAgent("codex-1");
  expect(await refused(second.admission.admit(requestFor(tokenFor())))).toBe("revoked");
  const third=harness(),live=tokenFor();
  await expect(third.admission.run(requestFor(live),async ticket=>{
   expect((declaredCapabilitiesV1(held) as {launches:string[][]}).launches.length).toBe(3);
   third.registry.revoke(capabilityIdV1(live)!);
   expect(()=>ticket.check()).toThrow(/revoked/u);
   expect(()=>declaredCapabilitiesV1(held)).toThrow(/revoked/u);
   throw new ServiceRefusalV1("revoked");
  })).rejects.toThrow(/revoked/u);
  expect(third.rows.map(row=>row.event)).toEqual(["admit","finish"]);
  expect(await refused(third.admission.admit(requestFor(live)))).toBe("revoked");
  const other=tokenFor();expect((await third.admission.admit(requestFor(other))).session).toBe(capabilityIdV1(other)!);
 });

 test("a token that expires while its command runs loses its declaration",async()=>{
  const{admission}=harness(),held=capabilities();at=2_000_000;
  try{await admission.run(requestFor(tokenFor()),async()=>{expect((declaredCapabilitiesV1(held) as {launches:string[][]}).launches.length).toBe(3);at+=60_001;expect(()=>declaredCapabilitiesV1(held)).toThrow(/Service request refused \(expired\)/u);});}finally{at=1_000_000;}
 });

 test("the request and concurrency budgets are per session",async()=>{
  const{admission}=harness({grant:grantFor({maximumRequests:3,maximumConcurrent:2})}),token=tokenFor(),gate=Promise.withResolvers<void>();
  const running=[admission.run(requestFor(token),()=>gate.promise),admission.run(requestFor(token),()=>gate.promise)];
  await Bun.sleep(10);
  expect(await refused(admission.admit(requestFor(token)))).toBe("busy");
  const otherSession=admission.run(requestFor(tokenFor()),async()=>undefined);
  gate.resolve();await Promise.all([...running,otherSession]);
  await admission.run(requestFor(token),async()=>undefined);
  expect(await refused(admission.admit(requestFor(token)))).toBe("busy");
 });

 test("a request whose audit row cannot be written is refused and leaves no slot behind",async()=>{
  const failing={value:true},{admission,rows}=harness({failAudit:failing,grant:grantFor({maximumConcurrent:1})}),token=tokenFor();
  let ran=false;expect(await refused(admission.run(requestFor(token),async()=>{ran=true;}))).toBe("audit");expect(ran).toBe(false);
  failing.value=false;await admission.run(requestFor(token),async()=>undefined);expect(rows.map(row=>row.event)).toEqual(["admit","finish"]);
 });
});

describe("the declaration view",()=>{
 test("capabilities without a grant are returned as they are; a grant without a matching live ticket only loses the grant",async()=>{
  const plain={workspaceRoot:root,launches:[["repo","x"]]};expect(declaredCapabilitiesV1(plain)).toBe(plain);
  const{admission}=harness();
  await admission.run(requestFor(tokenFor()),async()=>{
   expect((declaredCapabilitiesV1(capabilities({service:grantFor({maximumRequests:5})})) as {launches:string[][]}).launches).toEqual([["repo","service","executor"]]);
   expect((declaredCapabilitiesV1(capabilities({workspaceRoot:rootTwo})) as {launches:string[][]}).launches).toEqual([["repo","service","executor"]]);
   expect(declaredCapabilitiesV1(plain)).toBe(plain);
  });
 });
});

describe("schema and policy",()=>{
 test("the grant is closed data (Ajv and the library validator agree)",()=>{
  const validate=compile("Grant"),good=grantFor();
  expect(validate(good)).toBe(true);expect(readServiceGrantV1(good)).toEqual(good);
  for(const bad of[{...good,extra:1},{...good,version:2},{...good,worktree:"ABC"},{...good,keyDigest:"0".repeat(63)},{...good,operations:[]},{...good,operations:["run","run"]},{...good,operations:["exec"]},{...good,maximumRequests:0},{...good,maximumConcurrent:65},{version:1}]){
   expect(validate(bad)).toBe(false);expect(()=>readServiceGrantV1(bad)).toThrow(/Closed service grant/u);
  }
 });
 test("the caller schema repeats the grant exactly",()=>{
  const inline=JSON.parse(JSON.stringify(schema.definitions.Grant));inline.properties.operations.items={enum:schema.definitions.Operation.enum};
  expect((callerSchema.properties as Record<string,unknown>).service).toEqual(inline);
  expect(callerSchema.required).not.toContain("service");
 });
 test("every refusal code has an English and a German text with the same placeholders and a wire code the protocol knows",()=>{
  const codes=schema.definitions.Refusal.enum,placeholders=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
  const messages=policy.messages as Record<string,Record<string,string>>,wire=policy.wire as Record<string,string>;
  expect(Object.keys(messages.en!).sort()).toEqual([...codes].sort());expect(Object.keys(messages.de!).sort()).toEqual([...codes].sort());expect(Object.keys(wire).sort()).toEqual([...codes].sort());
  for(const code of codes){expect(placeholders(messages.en![code]!)).toEqual(placeholders(messages.de![code]!));expect(messages.de![code]).not.toBe(messages.en![code]);expect(["capability","scope","busy","internal"]).toContain(wire[code]);}
 });
 test("the audit row schema is closed and never takes a token",()=>{
  const validate=compile("Audit"),row={v:1,at:1,event:"admit",session:"0123456789abcdef",agent:"a",worktree,operation:"run",args:["run","a:build"],launches:2};
  expect(validate(row)).toBe(true);
  for(const bad of[{...row,token:"v1.a.b"},{...row,session:"v1.a.b"},{...row,event:"other"},{...row,code:"nope"}])expect(validate(bad)).toBe(false);
 });
 test("the token MAC chain equals an independent HMAC computation",()=>{
  const caveats=caveatsFor(),token=mintCapabilityV1(key,caveats as never,"0123456789abcdef");
  let mac=createHmac("sha256",key).update("0123456789abcdef").digest();
  for(const caveat of caveats)mac=createHmac("sha256",mac).update(canonicalJsonV1(caveat)).digest();
  expect(token.split(".")[2]).toBe(mac.toString("base64url"));
 });
 test("the audit sink appends schema-valid rows to a private file and publishes them through the held invocation",()=>{
  const published:object[]=[],file=join(base,"audit.jsonl"),sink=createServiceAuditV1({policy:{owner:"o"},control:{publish:(value:object)=>{published.push(value);}}} as never,file);
  const row:ServiceAuditRowV1={v:1,at:5,event:"finish",session:"0123456789abcdef",agent:"a",worktree,operation:"run",args:["run","a:build"],exit:0};
  sink.record(row);
  expect(readFileSync(file,"utf8")).toBe(JSON.stringify(row)+"\n");expect(published).toEqual([{owner:"o",command:"service:finish:0123456789abcdef",stage:"complete"}]);
  expect(()=>sink.record({...row,args:"x"} as never)).toThrow(/Closed service audit row/u);
 });
});

describe("the real launch checks of the authority core",()=>{
 const args=birth.cases[0]!.args.filter(argument=>argument!=="--tool=nx"),mode=birth.cases[0]!.mode as "release"|"prototype",environment={SEMIO_BUILD_MODE:mode};
 const policyOf=()=>readScriptPolicy({version:1,owner:"service-invocation-test",maximumElapsedMilliseconds:60000});
 const realKey=createRootKeyV1();
 const runtime=(()=>{const directory=join(base,"runtime");mkdirSync(directory,{recursive:true,mode:0o700});chmodSync(directory,0o700);writeFileSync(join(directory,"key"),realKey,{mode:0o600});return directory;})();

 test("a launch is declared by a plain task command only inside an admitted request, exactly as the one-shot door declares it",async()=>{
  const door=await createBuildBootstrapCapabilitiesV1(realRoot,["repo",...args],environment) as {launches:(readonly string[])[];native:object;tool:string;buildMode:string};
  const caller=createNativeBuildCallerCapabilitiesV1(realRoot,["service","executor"],"nx",{...environment,SEMIO_SCRIPT_CAPABILITIES:undefined} as never);
  const grant={...grantFor(),worktree:worktreeIdV1(realRoot),keyDigest:keyDigestV1(realKey)};
  const held={...caller,buildMode:door.buildMode,service:grant};
  const registry=createRevocationRegistryV1(join(runtime,"revoked.jsonl"),now),rows:ServiceAuditRowV1[]=[];
  await withScriptProcessEnvelope(createScriptProcessEnvelope(policyOf(),held,Date.now()),async invocation=>{
   const admission=holdServiceInvocationV1(realRoot,invocation,{key:realKey,revocations:registry,audit:{record:row=>{rows.push(row);}},bootstrap:createBuildBootstrapCapabilitiesV1,now,environment:()=>({...environment})});
   const selection=["nx",...door.launches.find(launch=>launch[0]==="nx")!.slice(1)];
   expect(()=>readOwnedBuildInvocationV1(realRoot,invocation,args)).toThrow("Undeclared original Repo build selection");
   expect(()=>projectOwnedNxProcessEnvelopeV1(realRoot,invocation,selection.slice(1))).toThrow("build.nx.child-selection-undeclared");
   const token=mintCapabilityV1(realKey,[{k:"scope",v:["run"]},{k:"worktree",v:worktreeIdV1(realRoot)},{k:"agent",v:"codex-1"},{k:"notAfter",v:at+60_000}]);
   const{readBuildGraphAncestorSelectionV1}=await import("../../../🧭️invocation/🟦️.ts"),carried=()=>JSON.parse(issueBuildTaskEnvironmentV1(realRoot,invocation,{} as never,args,{...environment}).SEMIO_SCRIPT_PROCESS_INVOCATION!).capabilities as object;
   expect(()=>readBuildGraphAncestorSelectionV1(realRoot,invocation.capabilities)).not.toThrow();
   expect(Object.hasOwn(carried(),"service")).toBe(false);
   await admission.run({token,agent:"codex-1",cwd:realRoot,args},async ticket=>{
    expect(()=>readBuildGraphAncestorSelectionV1(realRoot,invocation.capabilities)).not.toThrow();
    expect(Object.hasOwn(carried(),"service")).toBe(false);
    expect(ticket.launches).toEqual(door.launches);
    expect(readOwnedBuildInvocationV1(realRoot,invocation,args)).toMatchObject({tool:"nx",workspaceRoot:realRoot});
    expect(projectOwnedNxProcessEnvelopeV1(realRoot,invocation,selection.slice(1)).capabilities).toMatchObject({selection});
    expect(()=>readOwnedBuildInvocationV1(realRoot,invocation,["run","other:build"])).toThrow("Undeclared original Repo build selection");
    expect(()=>readOwnedBuildInvocationV1(realRoot,invocation,["exec","app:build"])).toThrow("Undeclared original Repo build selection");
    expect(()=>projectOwnedNxProcessEnvelopeV1(realRoot,invocation,["run","other:build"])).toThrow("build.nx.child-selection-undeclared");
   });
   expect(()=>readOwnedBuildInvocationV1(realRoot,invocation,args)).toThrow("Undeclared original Repo build selection");
   expect(await refused(admission.admit({token,agent:"codex-1",cwd:realRoot,args:[...args,"--tool=bazel"]}))).toBe("tool");
   expect(rows.map(row=>row.event)).toEqual(["admit","finish","refuse"]);
  });
 },60000);

 test("a door that answers for another tool, mode, resources or with a grant of its own is refused as tool",async()=>{
  const child=await import("../../📤️child/🟦️.ts"),caller=createNativeBuildCallerCapabilitiesV1(realRoot,["service","executor"],"nx",{...environment,SEMIO_SCRIPT_CAPABILITIES:undefined} as never);
  const held={...caller,buildMode:mode,service:{...grantFor(),worktree:worktreeIdV1(realRoot),keyDigest:keyDigestV1(realKey)}},genuine={...caller,buildMode:mode,launches:[["repo",...args]]};
  const variants:object[]=[{...genuine,tool:"bazel"},{...genuine,buildMode:mode==="release"?"prototype":"release"},{...genuine,native:{...caller.native,artifactDirectory:"/elsewhere"}},{...genuine,service:held.service}];
  await withScriptProcessEnvelope(createScriptProcessEnvelope(policyOf(),held,Date.now()),async invocation=>{
   const admission=holdServiceInvocationV1(realRoot,invocation,{key:realKey,revocations:createRevocationRegistryV1(join(runtime,"r3.jsonl")),audit:{record(){}},bootstrap:(...call)=>child.createBuildBootstrapCapabilitiesV1(...call),now,environment:()=>({...environment})});
   const token=mintCapabilityV1(realKey,[{k:"scope",v:["run"]},{k:"worktree",v:worktreeIdV1(realRoot)},{k:"agent",v:"codex-1"},{k:"notAfter",v:at+60_000}]);
   for(const answer of variants){
    const spy=spyOn(child,"createBuildBootstrapCapabilitiesV1").mockResolvedValue(answer as never);
    try{expect(await refused(admission.admit({token,agent:"codex-1",cwd:realRoot,args}))).toBe("tool");}finally{spy.mockRestore();}
   }
   expect((await admission.admit({token,agent:"codex-1",cwd:realRoot,args})).launches.length).toBeGreaterThan(0);
  });
 });

 test("an invocation without a grant cannot be held, and a forged request never reaches the declaration",async()=>{
  const caller=createNativeBuildCallerCapabilitiesV1(realRoot,["service","executor"],"nx",{...environment,SEMIO_SCRIPT_CAPABILITIES:undefined} as never);
  await withScriptProcessEnvelope(createScriptProcessEnvelope(policyOf(),caller,Date.now()),async invocation=>{
   expect(()=>holdServiceInvocationV1(realRoot,invocation,{key:realKey,revocations:createRevocationRegistryV1(join(runtime,"r2.jsonl")),audit:{record(){}},bootstrap:createBuildBootstrapCapabilitiesV1})).toThrow(/Service request refused \(grant\)/u);
   expect(()=>readOwnedBuildInvocationV1(realRoot,invocation,args)).toThrow("Undeclared original Repo build selection");
  });
 });

 test("the door writes the grant only for the executor and only with a key the service already wrote",async()=>{
  const place=join(shortBase,"door");mkdirSync(place,{recursive:true,mode:0o700});
  const outside=await createBuildBootstrapCapabilitiesV1(realRoot,["repo",...args],environment) as {service?:unknown};
  expect(outside.service).toBeUndefined();
  expect(()=>createServiceGrantV1(realRoot,{XDG_RUNTIME_DIR:place} as never)).toThrow();
  const directory=endpointsV1(realRoot,{XDG_RUNTIME_DIR:place},process.platform).directory;
  mkdirSync(directory,{recursive:true,mode:0o700});chmodSync(directory,0o700);writeFileSync(endpointsV1(realRoot,{XDG_RUNTIME_DIR:place},process.platform).key,realKey,{mode:0o600});
  const grant=createServiceGrantV1(realRoot,{XDG_RUNTIME_DIR:place} as never);
  expect(grant).toMatchObject({version:1,worktree:worktreeIdV1(realRoot),keyDigest:keyDigestV1(realKey),operations:["run","run-many","affected"],...policy.defaults});
  const executor=await createBuildBootstrapCapabilitiesV1(realRoot,["repo","service","executor"],{...environment,XDG_RUNTIME_DIR:place}) as {service?:unknown;launches:(readonly string[])[]};
  expect(executor.service).toEqual(grant);expect(executor.launches).toContainEqual(["repo","service","executor"]);
 },60000);
});

describe("the build owner registry",()=>{
 test("is parsed once per file state in a resident process, and never trusted while it is still being written",async()=>{
  const{readSelectedBuildTargetModesV1}=await import("../../../🧭️invocation/🟦️.ts"),place=mkdtempSync(join(tmpdir(),"reg-")),file=join(place,"📋️build-owners.json");
  const text=readFileSync(join(realRoot,"📋️build-owners.json"),"utf8"),original=JSON.parse;let parsed=0;
  const put=(content:string,ageMilliseconds:number)=>{writeFileSync(file,content);const when=new Date(Date.now()-ageMilliseconds);utimesSync(file,when,when);};
  const read=()=>readSelectedBuildTargetModesV1(place,["run","nobody:build"]);
  JSON.parse=((value:string,...rest:never[])=>{if(typeof value==="string"&&value.length>1000)parsed++;return original(value,...rest);}) as typeof JSON.parse;
  try{
   put(text,10_000);read();read();read();expect(parsed).toBe(1);
   put(text+"\n",10_000);read();read();expect(parsed).toBe(2);
   put(text,0);read();read();expect(parsed).toBe(4);
   put(text,10_000);read();expect(parsed).toBe(5);read();expect(parsed).toBe(5);
   readSelectedBuildTargetModesV1(place,["run","nobody:build"],()=>text);readSelectedBuildTargetModesV1(place,["run","nobody:build"],()=>text);expect(parsed).toBe(7);
   expect(read()).toEqual([]);
  }finally{JSON.parse=original;rmSync(place,{recursive:true,force:true});}
 });
});

test("the worktree root is the kernel's final path on Windows (realpathSync.native, the GetFinalPathNameByHandleW name the Go launcher hashes) and the resolved links elsewhere; a missing root stays as given",()=>{
 const native=spyOn(realpathSync,"native"),directory=mkdtempSync(join(tmpdir(),"canon-"));
 try{
  const actual=canonicalRootV1(directory,"win32");expect(native.mock.calls).toEqual([[directory]]);expect(actual).toBe(realpathSync.native(directory));
  const calls=native.mock.calls.length;expect(canonicalRootV1(directory,"darwin")).toBe(realpathSync(directory));expect(native.mock.calls.length).toBe(calls);
  for(const platform of["win32","linux"]as const)expect(canonicalRootV1(join(directory,"missing"),platform)).toBe(join(directory,"missing"));
 }finally{native.mockRestore();rmSync(directory,{recursive:true,force:true});}
});

test.skipIf(process.platform!=="win32")("on Windows every drive-letter and component case of one checkout gives one worktree id",()=>{
 const directory=mkdtempSync(join(tmpdir(),"Canon-Case-")),variant=directory.slice(0,1).toLowerCase()+directory.slice(1).toUpperCase();
 try{expect(worktreeIdV1(canonicalRootV1(variant))).toBe(worktreeIdV1(canonicalRootV1(directory)));}finally{rmSync(directory,{recursive:true,force:true});}
});

test("without XDG_RUNTIME_DIR the endpoint directory follows TMPDIR only (TMP and TEMP are no inputs), equal to the independent Python reading of the Go launcher's os.TempDir rule",()=>{
 const python=testPythonV1();
 if(python===null)throw Error("python is required for the endpoint oracle");
 const environments:readonly Record<string,string>[]=[{},{TMP:"/x/tmp"},{TEMP:"/y/temp"},{TMPDIR:"/var/t/"},{TMPDIR:"/var/t",TMP:"/x/tmp",TEMP:"/y/temp"},{XDG_RUNTIME_DIR:"/run/user/1000",TMPDIR:"/var/t"}];
 const reading=Bun.spawnSync([python,"-c","import hashlib,json,os,sys\nroot,owner,rows=sys.argv[1],sys.argv[2],json.loads(sys.argv[3])\nident=hashlib.sha256(('semio-repo-service-v1\\0'+root).encode()).hexdigest()[:12]\nprint(json.dumps([os.path.join(row.get('XDG_RUNTIME_DIR') or (row.get('TMPDIR') or '/tmp').rstrip('/'),'semio-repo-'+owner,ident) for row in rows]))",realRoot,"1000",JSON.stringify(environments)],{stdout:"pipe",stderr:"pipe"});
 expect(reading.exitCode,reading.stderr.toString()).toBe(0);
 const expected=JSON.parse(reading.stdout.toString()) as string[];
 environments.forEach((environment,index)=>expect(endpointsV1(realRoot,environment,"linux","1000").directory,JSON.stringify(environment)).toBe(expected[index]));
});

afterAll(()=>{for(const path of[base,shortBase])try{rmSync(path,{recursive:true,force:true});}catch{}});
