/** 🔗️ Contract: a selected read of the load module goes through the fleet exchange, an unselected read does not, the attempt itself is untouched, and the admission facts the exchange reads and seeds are the ones the process remembers.
 * @see ../../🔌️attempt/🟦️.ts */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

const text=readFileSync(process.env.R171_WIRING_SOURCE??new URL("../../../🟦️.ts",import.meta.url),"utf8");
const unpinned=text.slice(text.indexOf("async function loadBuildGraphUnpinnedV1"));

test("the attempt of a selected scope runs through the fleet exchange inside the graph custody, the unselected read does not",()=>{
 expect(text).toContain('import {loadSelectedThroughFleetV1} from "./🤝️selected-exchange/🔌️attempt/🟦️.ts";');
 const call=unpinned.indexOf("selectedThroughFleetV1({root,invocation,scope,staging,attempt})");expect(call).toBeGreaterThan(0);
 expect(unpinned.slice(0,call)).toContain("withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl,critical)=>{");
 expect(unpinned.slice(call-40,call)).toContain("scope===undefined?attempt():");
 expect(unpinned).toContain("const attempt=()=>loadBuildGraphAttemptV1(root,invocation,actualControl,options,registry,scope,critical);");
 const helper=text.slice(text.indexOf("function selectedThroughFleetV1"),text.indexOf("async function readSealedBuildGraphV1"));
 expect(helper).toContain('path:join(input.staging,input.scope+".json"),authorityPath:join(input.staging,input.scope+".authority.json")');
 expect(helper).toContain("holds:rows=>buildGraphSourceFactsHoldV1(rows,input.invocation)");
 expect(helper).toContain("module:loadedGraphModuleAuthorityV1");
 expect(helper).toContain("signal:input.invocation?.control.signal");
 expect(helper).toContain("loadSelectedThroughFleetV1({");
});

test("the admission facts of the exchange are the memo of the process, keyed by scope, admission format and question",()=>{
 const block=text.slice(text.indexOf("export const selectedAdmissionFactsV1"),text.indexOf("export async function observeOwnerFactsMemoizedV1"));
 expect(block).toContain('scope+"\\0"+loadedGraphAdmissionFormatV1+"\\0"');
 expect(block).not.toContain("loadedGraphModuleAuthorityV1");
 expect(block).toContain("rememberOwnerFactsV1(");
 expect(text).toContain('const suffix=owner.name+"\\0"+JSON.stringify([targets,inputs]),key=scope+"\\0"+loadedGraphAdmissionFormatV1+"\\0"+suffix');
});

test("the attempt of the load module is not touched by the exchange",()=>{
 const attempt=text.slice(text.indexOf("async function loadBuildGraphAttemptV1"),text.indexOf("/** \u{1F9F2}\uFE0F The attempt of a selected scope offered through the fleet"));
 expect(attempt.length).toBeGreaterThan(1000);expect(attempt).not.toContain("selected-exchange");expect(attempt).not.toContain("loadSelectedThroughFleetV1");
});
