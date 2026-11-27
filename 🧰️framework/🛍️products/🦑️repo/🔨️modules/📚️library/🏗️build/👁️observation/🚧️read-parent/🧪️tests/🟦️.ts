/** 🚧️ Contract: the guard of the parent of a read. Marked while its farm exists, the process refuses every computation that belongs to a farm child with one refusal that names the kind and says it in both languages; not marked, or marked and the farm gone, nothing is refused.
 * @see ../🟨️.mjs */
import{expect,test}from"bun:test";
import{assertNotReadParentV1,markReadParentV1,queuedJobTextV1,readParentV1,withReadParentScopeV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";

test("an unmarked process computes, a marked one is refused with kind, code and both languages, and the mark ends with its owner",()=>{
 expect(()=>assertNotReadParentV1("infer","p")).not.toThrow();
 let farm=true;const unmark=markReadParentV1(()=>farm);
 expect(readParentV1()).toBe(true);
 for(const kind of schema.$defs.kinds.const){
  let caught:any;try{assertNotReadParentV1(kind,"@semio-tech/x");}catch(error){caught=error;}
  expect(caught?.code,kind).toBe("graph.read-parent-computes");expect(caught.kind).toBe(kind);
  expect(caught.message).toContain(kind);expect(caught.message).toContain("Elternprozess");expect(caught.message).toContain("parent of a read");
 }
 farm=false;
 expect(()=>assertNotReadParentV1("infer","p"),"the farm is gone: the parent may compute, loudly said elsewhere").not.toThrow();
 farm=true;unmark();
 expect(readParentV1()).toBe(false);expect(()=>assertNotReadParentV1("derive")).not.toThrow();
});

test("a failing liveness answer never refuses and the queue line names the project and the reason in both languages",()=>{
 const unmark=markReadParentV1(()=>{throw Error("broken");});
 expect(()=>assertNotReadParentV1("walk")).not.toThrow();unmark();
 const line=queuedJobTextV1("@semio-tech/forms-js","the first child is starting");
 expect(line).toContain("@semio-tech/forms-js waits in the queue");expect(line).toContain("wartet in der Warteschlange");expect(line).toContain("the first child is starting");
});

test("a read arms its own scope: the flows it started are refused while the farm exists, a flow it did not start never is",async()=>{
 let farm=true;
 const outside=new Promise<void>(done=>setTimeout(done,20));
 await withReadParentScopeV1(async scope=>{
  expect(()=>assertNotReadParentV1("derive"),"not armed yet: the setup of the read computes").not.toThrow();
  const disarm=scope.arm(()=>farm);
  expect(readParentV1()).toBe(true);
  await Bun.sleep(5);
  expect(()=>assertNotReadParentV1("refs")).toThrow();
  await new Promise<void>(done=>setTimeout(()=>{expect(()=>assertNotReadParentV1("walk"),"late callbacks of the read too").toThrow();done();},5));
  farm=false;expect(()=>assertNotReadParentV1("infer"),"the farm is gone").not.toThrow();
  farm=true;disarm();expect(()=>assertNotReadParentV1("infer"),"disarmed").not.toThrow();
 });
 await outside;
 expect(()=>assertNotReadParentV1("infer"),"outside the read").not.toThrow();
 let started=false;const other=new Promise<void>(done=>setTimeout(()=>{started=true;expect(()=>assertNotReadParentV1("walk"),"a flow of the process that the read did not start").not.toThrow();done();},30));
 await withReadParentScopeV1(async scope=>{scope.arm(()=>true);await Bun.sleep(60);});
 await other;expect(started).toBe(true);
});

test("the scope of a read does not keep the body or its inputs alive once the read is over",async()=>{
 let freed=0,kept=0;
 for(let attempt=0;attempt<5&&freed===0;attempt++){
  const state:{ref?:WeakRef<object>;kept?:unknown}={};
  const start=()=>{const inputs={rows:Array.from({length:1000},(_,at)=>"row-"+at+Math.random())};state.ref=new WeakRef(inputs);return withReadParentScopeV1(async scope=>{state.kept=scope;return inputs.rows.length;});};
  expect(await start()).toBe(1000);
  for(let round=0;round<8&&state.ref!.deref()!==undefined;round++){await new Promise(resolve=>setTimeout(resolve,5));globalThis.Bun.gc(true);}
  if(state.kept!==undefined)kept++;
  if(state.ref!.deref()===undefined)freed++;
 }
 expect(kept).toBeGreaterThan(0);
 expect(freed).toBeGreaterThan(0);
});
