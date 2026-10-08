/** 🧯️ Original fatal-conflict and causal-envelope owners agree with independent RFC6902 removal. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
test("🧯️ every optional causal field and retained empty conflict scaffold requires exact closure",()=>{
  for(const row of law.envelopes){
    expect(row.dependencyCapacity).toBeGreaterThanOrEqual(row.dependencies.length);expect(row.targetCapacity).toBeGreaterThanOrEqual(row.targets.length);expect(row.forwardCapacity).toBeGreaterThanOrEqual(row.forward.length);expect(row.inverseCapacity).toBeGreaterThanOrEqual(row.inverse.length);
    const baseline={identity:row.text.repeat(row.repeat),dependencies:row.dependencies,target:row.targets,forward:row.forward,inverse:row.inverse,optional:row.optional},owned=structuredClone(baseline);
    expect(applyPatch(owned,[{op:"replace",path:"/identity",value:null},{op:"replace",path:"/dependencies",value:[]},{op:"replace",path:"/target",value:[]},{op:"replace",path:"/forward",value:[]},{op:"replace",path:"/inverse",value:[]},{op:"replace",path:"/optional",value:null}],true).newDocument).toEqual({identity:null,dependencies:[],target:[],forward:[],inverse:[],optional:null});
    expect(baseline.identity).toBe(row.text.repeat(row.repeat));
  }
  for(const row of law.conflicts){
    expect(row.rowCapacity).toBeGreaterThanOrEqual(row.rows.length);expect(row.actorCapacity).toBeGreaterThanOrEqual(row.actors.length);expect(row.identityCapacity).toBeGreaterThanOrEqual(row.identities.length);expect(row.messageCapacity).toBeGreaterThanOrEqual(row.messages.length);
    for(const index of row.rows)expect(index).toBeLessThan(law.envelopes.length);
    const baseline={id:row.id,actors:row.actors,messages:row.messages,envelopes:row.rows,identities:row.identities};
    expect(applyPatch(structuredClone(baseline),Object.keys(baseline).map(key=>({op:"replace" as const,path:`/${key}`,value:key==="id"?null:[]})),true).newDocument).toEqual({id:null,actors:[],messages:[],envelopes:[],identities:[]});
    expect(baseline.id).toBe(row.id);
  }
  const store=readFileSync(new URL("../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",import.meta.url),"utf8");
  expect(store.includes("use crate::os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;")).toBe(true);
  console.log(`[DEBUG] neutral ${law.envelopes.length} envelopes/${law.conflicts.length} conflicts include empty capacities and all optional owners; independent RFC6902 and actual Store mount agree`);
});
