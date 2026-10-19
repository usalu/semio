/** 🔌️ Language-agnostic law of the UI host seam: the corpus states, per binding state and check, whether a host patch claim is accepted. */
import {expect,test} from "bun:test";
import corpus from "./🧫️fixtures/🔣️.json";

type Case={id:string;bound:boolean;approves:boolean;check:"matches"|"matchesOwner"|"matchesReceipt";expected:boolean};
const seam=async(tag:string)=>import(`../🟦️.ts?${tag}`) as Promise<typeof import("../🟦️.ts")>;

test("the UI patch seam accepts a claim only through the bound host brand checks",async()=>{
  for(const row of corpus.cases as Case[]){
    const fresh=await seam(row.id);
    if(row.bound)fresh.bindUiPatchAuthority({matches:(_source,_activation,_lifetime):_source is never=>row.approves,matchesOwner:(_source,_owner):_source is never=>row.approves,matchesReceipt:()=>row.approves});
    const authority=fresh.uiPatchAuthority(),activation={actorId:"a",activationGeneration:1n,assertActive(){}},lifetime={activationGeneration:1n,instanceId:1,guestLifetime:1n};
    const observed=row.check==="matches"?authority.matches({},activation,lifetime):row.check==="matchesOwner"?authority.matchesOwner({},{}):authority.matchesReceipt({},{},{});
    expect(observed,row.id).toBe(row.expected);
  }
});

test("the UI patch authority binds exactly once",async()=>{
  const fresh=await seam("double-bind");
  const authority={matches:(_source:unknown):_source is never=>true,matchesOwner:(_source:unknown):_source is never=>true,matchesReceipt:()=>true};
  expect(()=>fresh.bindUiPatchAuthority(authority)).not.toThrow();
  expect(()=>fresh.bindUiPatchAuthority(authority)).toThrow(corpus.doubleBind.secondRefusedWith);
});

type Seam={id:string;bind:string;use:string;unboundRefusal:string;doubleBind:string};
const seamModule=(row:Seam)=>row.id==="input-host"?import(`../📥️input/🟦️.ts?${row.id}`):import(`../🏦️composition/🟦️.ts?${row.id}`);

test("the kernel input and shard composition seams refuse use while unbound and bind exactly once",async()=>{
  for(const row of corpus.seams as Seam[]){
    const fresh=await seamModule(row) as Record<string,(value?: unknown)=>unknown>;
    expect(()=>fresh[row.use]!(),row.id).toThrow(row.unboundRefusal);
    expect(()=>fresh[row.bind]!({}),row.id).not.toThrow();
    expect(fresh[row.use]!(),row.id).toEqual({});
    expect(()=>fresh[row.bind]!({}),row.id).toThrow(row.doubleBind);
  }
});

test("the return content seam accepts a content parser only through the bound kernel brand check",async()=>{
  for(const row of corpus.returnContent as {id:string;bound:boolean;approves:boolean;expected:boolean}[]){
    const fresh=await import(`../📦️return-content/🟦️.ts?${row.id}`) as typeof import("../📦️return-content/🟦️.ts");
    if(row.bound)fresh.bindReturnContentMatcher({matches:()=>row.approves});
    expect(fresh.returnContentMatches({},{},{},{actorId:"a",activationGeneration:1n,assertActive(){}},{activationGeneration:1n,instanceId:1,guestLifetime:1n}),row.id).toBe(row.expected);
  }
  const once=await import(`../📦️return-content/🟦️.ts?${"once"}`) as typeof import("../📦️return-content/🟦️.ts");
  once.bindReturnContentMatcher({matches:()=>true});
  expect(()=>once.bindReturnContentMatcher({matches:()=>true})).toThrow(corpus.returnContentDoubleBind);
});
