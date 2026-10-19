/** 🪪️ Language-agnostic law of the instance-to-resident seam: the corpus states, per binding state, whether a scope is accepted. */
import {expect,test} from "bun:test";
import corpus from "./🧫️fixtures/🔣️.json";

type Case={id:string;bound:boolean;approves:boolean;expected:boolean};
const seam=async(tag:string)=>import(`../🟦️.ts?${tag}`) as Promise<typeof import("../🟦️.ts")>;

test("the resident scope seam accepts a scope only through the bound brand check",async()=>{
  for(const row of corpus.cases as Case[]){
    const fresh=await seam(row.id);
    if(row.bound)fresh.bindResidentScopeMatcher(()=>row.approves);
    expect(fresh.residentScopeMatches({},{},{},{}),row.id).toBe(row.expected);
  }
});

test("the resident scope matcher binds exactly once",async()=>{
  const fresh=await seam("double-bind");
  expect(()=>fresh.bindResidentScopeMatcher(()=>true)).not.toThrow();
  expect(()=>fresh.bindResidentScopeMatcher(()=>true)).toThrow(corpus.doubleBind.secondRefusedWith);
});

test("an instance refuses a forged resident scope once the resident module has bound its brand check",async()=>{
  await import("../../../💾️resident/🟦️.ts");
  const {OwnedUiInstance}=await import("../../🟦️.ts");
  const activation={actorId:"seam-actor",activationGeneration:1n,assertActive(){}} as never;
  const instance=new OwnedUiInstance(activation,{activationGeneration:1n,instanceId:1,guestLifetime:1n},{maxNodes:8,maxDepth:4,maxChildren:4,maxTextBytes:64,maxPatchOps:4,maxPatchBytes:256} as never,{usizeBits:64} as never);
  const forged={beginClose(){},closeStep(){throw Error("forged scope must never be closed");},terminalIsEmpty(){return true;}};
  expect(instance.attachResidentScope(forged)).toBe(false);
});
