import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),body=(name:string)=>{const start=source.indexOf("async function "+name+"(");if(start<0)throw Error("Actual function required: "+name);const end=source.indexOf("\n/**",start);return source.slice(start,end<0?undefined:end);};

test("a second synchronization with nothing changed reuses the recorded ownership index in the preservation pass as in every other ownership walk",()=>{
 for(const name of["retireJavascriptDependencyPreservationsInStoreV1"]){
  const text=body(name),walk=text.indexOf("withCurrentBunWorkspaceOwnershipV1(workspace,{kind:\"all\"}"),seed=text.indexOf("seedBunWorkspaceOwnershipIndexV1(workspace,index)");
  expect(walk,name).toBeGreaterThan(0);expect(seed,name).toBeGreaterThan(0);expect(seed,name).toBeLessThan(walk);
 }
});

test("a synchronization whose recorded proof is current ends before the exclusive bindings gate, the preservation pass and the collection",()=>{
 const text=body("runJavascriptDependencyStoreGenerationsV1"),done=text.indexOf('if(built.state==="current")return;'),gate=text.indexOf('acquireConcurrentLeaseV1("bindings"');
 expect(done).toBeGreaterThan(0);expect(gate).toBeGreaterThan(done);
});
