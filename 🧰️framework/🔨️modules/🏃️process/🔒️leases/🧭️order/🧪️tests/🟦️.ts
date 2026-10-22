import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import toolchain from "./🧫️fixtures/🦀️toolchain/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import authored from "../🔣️policy.json";
import {checkHoldAndWaitV1,checkLeaseOrderV1,checkSelfDeadlockV1,classifyLeaseResourceV1,findLeaseCycleV1,formatLeaseViolationV1,leaseBlockersV1,leaseOrderPolicyV1,leaseProcessDependenciesV1,readLeaseOrderPolicyV1,type LeaseNodeV1,type LeaseViolationV1} from "../🟦️.ts";

const policy=readLeaseOrderPolicyV1(fixture.policy);
const ajv=new Ajv({strict:true,allowUnionTypes:true});
const validatePolicy=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Policy"} as never);
const validateViolation=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/Violation"} as never);

test("the authored policy and the test policy satisfy the closed schema in two validators",()=>{
 expect(validatePolicy(authored)).toBe(true);expect(validatePolicy(fixture.policy)).toBe(true);
 expect(readLeaseOrderPolicyV1(authored).classes.length).toBeGreaterThan(5);
 expect(validatePolicy({...authored,extra:true})).toBe(false);expect(()=>readLeaseOrderPolicyV1({...authored,extra:true})).toThrow("Closed lease order Policy");
 expect(validatePolicy({...authored,rules:{...authored.rules,holdAndWait:"sometimes"}})).toBe(false);expect(()=>readLeaseOrderPolicyV1({...authored,rules:{...authored.rules,holdAndWait:"sometimes"}})).toThrow();
});

test("a policy with duplicate classes, duplicate matches or edges between unknown classes is refused",()=>{
 const cls=authored.classes[0]!;
 expect(()=>readLeaseOrderPolicyV1({...authored,classes:[cls,cls]})).toThrow("declared twice");
 expect(()=>readLeaseOrderPolicyV1({...authored,classes:[cls,{...cls,id:"other"}]})).toThrow("match declared twice");
 expect(()=>readLeaseOrderPolicyV1({...authored,allowed:[{from:"nope",to:cls.id,reason:{en:"x",de:"y"}}]})).toThrow("unknown class");
});

test("every census family of the authored policy lands in the class its owner declared",()=>{
 const live=readLeaseOrderPolicyV1(authored),expected:Record<string,string>={"native-fixed-image:/w/.cache/b/d":"image","native-storage:abc":"image","repository-graph-facts:/w":"graph","nx:graph-pool:abc":"graph","nx:daemon-startup:abc":"tool-singleton","nx:daemon-slot:abc":"tool-singleton","build-graph-store:/r":"graph","bazel-workspace-x":"tool-singleton","bazel-snapshot-restore":"tool-singleton","cargo:registry:/h":"cargo-coordination","cargo:configuration-view:41":"toolchain","rustup:home":"toolchain","rustup:generation-shard:24":"toolchain","cargo-preparation:/w":"cargo-coordination",'["semio.cargo.build-lease/v1","/b"]':"cargo-coordination","rust-code:/c":"namespace","javascript-dependency-bindings:/w":"namespace","artifact:/a/b":"artifact","semio.compiler.cpu-picker/v1":"compiler-capacity","semio.compiler.cpu/v1/3":"compiler-capacity","nx-tooling-activation":"tool-singleton","some-future-resource":"unclassified"};
 for(const [resource,id] of Object.entries(expected))expect(`${resource} → ${classifyLeaseResourceV1(live,resource).id}`).toBe(`${resource} → ${id}`);
});

test("the lease-store image never orders: held shared it is the medium, any other image or mode still orders",()=>{
 const live=readLeaseOrderPolicyV1(authored),at=(resource:string,mode:"shared"|"exclusive")=>[{directory:"/w/.cache/b/l/m/data",resource,mode,atMs:1,task:1}];
 const request={directory:"/w/.cache/b/l/m/data",resource:"repository-graph-facts:/w",mode:"exclusive" as const,nonblocking:false};
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/l","shared"),request)).toBeUndefined();
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/l","exclusive"),request)?.kind).toBe("out-of-order");
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/d","shared"),request)?.kind).toBe("out-of-order");
 expect(live.allowed.some(edge=>edge.from==="image"&&edge.to==="graph")).toBe(false);
});

for(const row of fixture.classification)test(`classification ${row.resource}`,()=>expect(classifyLeaseResourceV1(policy,row.resource).id).toBe(row.class));

for(const row of fixture.order)test(`lock order ${row.id}`,()=>{
 const found=checkLeaseOrderV1(policy,row.held as never,row.request as never);
 if(row.expect===null)return void expect(found).toBeUndefined();
 expect(found).toMatchObject(row.expect);expect(validateViolation(found)).toBe(true);
});

for(const row of fixture.selfDeadlock)test(`self-deadlock ${row.id}`,()=>{
 const found=checkSelfDeadlockV1(policy,row.held as never,row.request as never,row.waitedMilliseconds);
 if(row.expect===null)return void expect(found).toBeUndefined();
 expect(found).toMatchObject(row.expect);expect(validateViolation(found)).toBe(true);
});

for(const row of fixture.holdAndWait)test(`hold-and-wait ${row.id}`,()=>{
 const found=checkHoldAndWaitV1(policy,row.held as never,row.request as never,row.waitedMilliseconds,(row as {blocker?:{pid:number;heldSeconds:number}}).blocker);
 if(row.expect===null)return void expect(found).toBeUndefined();
 expect(found).toMatchObject(row.expect);expect(validateViolation(found)).toBe(true);
});

test("rules set to observe or off still classify but the pure checks honour off",()=>{
 const off=Object.freeze({...policy,rules:Object.freeze({outOfOrder:"off" as const,holdAndWait:"off" as const,selfDeadlock:"off" as const,cycle:"off" as const})});
 const row=fixture.order.find(item=>item.id==="outward-refused")!,wait=fixture.holdAndWait.find(item=>item.id==="after-grace-refused")!;
 expect(checkLeaseOrderV1(off,row.held as never,row.request as never)).toBeUndefined();
 expect(checkHoldAndWaitV1(off,wait.held as never,wait.request as never,99999)).toBeUndefined();
});

const oracle=(nodes:readonly LeaseNodeV1[],from:string):boolean=>{
 const database=new Database(":memory:");database.exec("CREATE TABLE edge(a TEXT,b TEXT)");
 const insert=database.prepare("INSERT INTO edge VALUES(?,?)");
 for(const node of nodes)for(const to of [...leaseBlockersV1(nodes,node),...leaseProcessDependenciesV1(nodes,node)])insert.run(node.id,to);
 const row=database.query("WITH RECURSIVE walk(node,depth) AS (SELECT b,1 FROM edge WHERE a=? UNION SELECT edge.b,walk.depth+1 FROM edge JOIN walk ON edge.a=walk.node WHERE walk.depth<64) SELECT COUNT(*) AS found FROM walk WHERE node=?").get(from,from) as {found:number};
 database.close();return row.found>0;
};

for(const row of fixture.cycles)test(`deadlock cycle ${row.id}`,()=>{
 const nodes=row.nodes as unknown as LeaseNodeV1[],found=findLeaseCycleV1(nodes,row.from);
 if(row.expect===null)expect(found).toBeUndefined();else{expect(found?.members).toEqual(row.expect.members);expect(found?.victim).toBe(row.expect.victim);}
 expect(oracle(nodes,row.from)).toBe(found!==undefined);
});

test("exactly one member of every cycle is the victim, whichever member asks",()=>{
 for(const row of fixture.cycles.filter(item=>item.expect!==null)){
  const nodes=row.nodes as unknown as LeaseNodeV1[],victims=new Set(row.expect!.members.map(member=>findLeaseCycleV1(nodes,member)!.victim));
  expect([...victims]).toEqual([row.expect!.victim]);
 }
});

test("the SQLite recursive-walk oracle agrees with the cycle finder on 500 seeded random graphs, and every cycle has exactly one victim",()=>{
 let state=0x9e3779b9;const next=(limit:number)=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return (state>>>8)%limit;};
 let cycles=0;
 for(let round=0;round<500;round++){
  const size=2+next(6),nodes:LeaseNodeV1[]=[];
  for(let index=0;index<size;index++){
   const holds=[...new Set([next(5),next(5)])].filter(()=>next(3)>0).map(key=>({key:`k${key}`,mode:next(3)===0?"shared" as const:"exclusive" as const}));
   nodes.push({id:`${index+1}/1`,holds,...next(4)>0?{wait:{key:`k${next(5)}`,mode:next(3)===0?"shared" as const:"exclusive" as const,sinceMs:next(50),queued:next(2)===0}}:{}});
  }
  const involved=nodes.filter(node=>findLeaseCycleV1(nodes,node.id)!==undefined);
  for(const node of nodes)expect(`${node.id}:${findLeaseCycleV1(nodes,node.id)!==undefined}`).toBe(`${node.id}:${oracle(nodes,node.id)}`);
  for(const node of involved){const found=findLeaseCycleV1(nodes,node.id)!;expect(found.members.includes(found.victim)).toBe(true);}
  if(involved.length>0){cycles++;const victims=new Set(involved.map(node=>findLeaseCycleV1(nodes,node.id)!.victim));expect(victims.size).toBeGreaterThanOrEqual(1);}
 }
 expect(cycles).toBeGreaterThan(20);
});

for(const row of fixture.messages)test(`diagnostic ${row.id}`,()=>{
 const text=formatLeaseViolationV1(row.language as "en"|"de",row.violation as LeaseViolationV1);
 for(const part of row.contains)expect(text).toContain(part);
 expect(validateViolation(row.violation)).toBe(true);
});

test("the environment switch overrides every rule and rejects unknown values",()=>{
 expect(leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"off"}).rules).toEqual({outOfOrder:"off",holdAndWait:"off",selfDeadlock:"off",cycle:"off"});
 expect(leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"enforce"}).rules.outOfOrder).toBe("enforce");
 expect(String(leaseOrderPolicyV1({}).rules.outOfOrder)).toBe(String(authored.rules.outOfOrder));
 expect(()=>leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"sometimes"})).toThrow("must be off, observe or enforce");
});

type SequenceStepV1=Readonly<{op:"acquire"|"release";resource:string;mode?:"shared"|"exclusive"}>;
type SequenceV1=Readonly<{id:string;steps:readonly SequenceStepV1[];violations:readonly Readonly<{held:string;requested:string}>[]}>;

/** 🧪️ Replays an acquisition sequence against the authored policy and returns every refused request. */
const replay=(live:ReturnType<typeof readLeaseOrderPolicyV1>,directory:string,steps:readonly SequenceStepV1[])=>{
 const held:{directory:string;resource:string;mode:"shared"|"exclusive";atMs:number;task:number}[]=[],found:{held:string;requested:string}[]=[];
 steps.forEach((step,index)=>{
  if(step.op==="release"){const at=held.findIndex(row=>row.resource===step.resource);if(at>=0)held.splice(at,1);return;}
  const request={directory,resource:step.resource,mode:step.mode!,nonblocking:false},violation=checkLeaseOrderV1(live,held,request);
  if(violation)found.push({held:violation.held,requested:violation.requested});
  held.push({directory,resource:step.resource,mode:step.mode!,atMs:index+1,task:1});
 });
 return found;
};

/** 🗄️ Independent SQLite oracle: ranks by longest matching prefix or exact name, then a self-join over the held rows and the declared edges. */
const sequenceOracle=(directory:string,steps:readonly SequenceStepV1[])=>{
 const live=readLeaseOrderPolicyV1(authored),database=new Database(":memory:");
 database.exec("CREATE TABLE class(id TEXT,rank INTEGER);CREATE TABLE match(id TEXT,kind TEXT,value TEXT);CREATE TABLE edge(a TEXT,b TEXT);CREATE TABLE held(resource TEXT,at INTEGER)");
 for(const row of live.classes){database.prepare("INSERT INTO class VALUES(?,?)").run(row.id,row.rank);for(const match of row.match)database.prepare("INSERT INTO match VALUES(?,?,?)").run(row.id,match.kind,match.value);}
 for(const row of live.allowed)database.prepare("INSERT INTO edge VALUES(?,?)").run(row.from,row.to);
 const rank=`SELECT class.id AS id,class.rank AS rank FROM match JOIN class ON class.id=match.id WHERE (match.kind='exact' AND match.value=?1) OR (match.kind='prefix' AND substr(?1,1,length(match.value))=match.value) ORDER BY match.kind='exact' DESC,length(match.value) DESC LIMIT 1`;
 const found:{held:string;requested:string}[]=[];
 steps.forEach((step,index)=>{
  if(step.op==="release"){database.prepare("DELETE FROM held WHERE rowid=(SELECT rowid FROM held WHERE resource=? LIMIT 1)").run(step.resource);return;}
  const wanted=database.query(rank).get(step.resource) as {id:string;rank:number}|null;
  if(wanted&&wanted.rank>0)for(const row of database.query("SELECT resource FROM held ORDER BY at").all() as {resource:string}[]){
   const now=database.query(rank).get(row.resource) as {id:string;rank:number}|null;
   if(now&&now.rank>wanted.rank&&!database.query("SELECT 1 FROM edge WHERE a=? AND b=?").get(now.id,wanted.id)){found.push({held:row.resource,requested:step.resource});break;}
  }
  database.prepare("INSERT INTO held VALUES(?,?)").run(step.resource,index+1);
 });
 database.close();return found;
};

for(const row of toolchain.sequences as readonly SequenceV1[])test(`toolchain acquisition sequence ${row.id} follows the declared lock order`,()=>{
 const live=readLeaseOrderPolicyV1(authored);
 expect<unknown>(replay(live,toolchain.directory,row.steps)).toEqual(row.violations);
 expect<unknown>(sequenceOracle(toolchain.directory,row.steps)).toEqual(row.violations);
});

test("the toolchain and Cargo readers precede compiler capacity, which precedes the graph, and capacity may still enter a tool singleton only through its declared edge",()=>{
 const live=readLeaseOrderPolicyV1(authored),rank=(resource:string)=>classifyLeaseResourceV1(live,resource).rank,capacity=rank("semio.compiler.host-capacity/v1");
 expect(rank("rustup:home")).toBeGreaterThan(0);
 for(const resource of ["rustup:home","rustup:generation-shard:24","cargo:configuration-view:41","cargo:registry:/h","cargo-preparation:/w",'["semio.cargo.build-lease/v1","/b"]'])expect(rank(resource),resource).toBeLessThan(capacity);
 expect(capacity).toBeLessThan(rank("repository-graph-facts:/w"));
 expect(live.allowed.some(edge=>edge.from==="compiler-capacity"&&edge.to==="tool-singleton")).toBe(true);
 expect(live.allowed.some(edge=>edge.from==="compiler-capacity"&&(edge.to==="toolchain"||edge.to==="cargo-coordination"))).toBe(false);
});
