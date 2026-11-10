import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import ts from"typescript";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("project-nodes.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);

/** 🔎️ Every call expression in the file with its text and its ancestors. */
function calls(){
 const found:{text:string;callee:string;ancestors:ts.Node[]}[]=[];
 const visit=(node:ts.Node,ancestors:ts.Node[])=>{
  if(ts.isCallExpression(node))found.push({text:node.getText(tree),callee:node.expression.getText(tree),ancestors});
  ts.forEachChild(node,child=>visit(child,[...ancestors,node]));
 };
 visit(tree,[]);return found;
}

test("every project inference starts through the memory governor, bounded by the worker count, and its statistics reach the phase row",()=>{
 const all=calls(),inferences=all.filter(call=>call.callee==="inferProject");
 expect(inferences.length).toBeGreaterThan(0);
 for(const inference of inferences)expect(inference.ancestors.some(node=>ts.isCallExpression(node)&&node.expression.getText(tree)==="governor.run"),inference.text).toBe(true);
 const creations=all.filter(call=>call.callee==="createGraphParentMemoryGovernorV1");
 expect(creations.length).toBe(1);expect(creations[0]!.text).toContain("workers:limit");expect(creations[0]!.text).toContain("environment:process.env");
 expect(creations[0]!.text).toContain("signal:context.phaseControl?.signal");
 const ends=all.filter(call=>call.callee==="phases?.end"&&call.text.includes('"inference"'));
 expect(ends.length).toBe(1);expect(ends[0]!.text).toContain("governor.stats()");
});

test("the running projects are named in the inference phase while they run, removed when they end and the tick stops with the loops",()=>{
 const all=calls(),named=all.filter(call=>call.callee==="phases?.running"&&call.text.includes('"inference"'));
 expect(named.length).toBe(1);expect(named[0]!.text).toContain("list");expect(source).toContain("[...active].sort(");
 const inferences=all.filter(call=>call.callee==="inferProject");
 for(const inference of inferences)expect(inference.ancestors.some(node=>ts.isTryStatement(node)&&node.finallyBlock!==undefined&&node.finallyBlock.getText(tree).includes("active.delete(job.name)")),inference.text).toBe(true);
 expect(all.some(call=>call.callee==="clearInterval"&&call.text==="clearInterval(ticker)"&&call.ancestors.some(node=>ts.isTryStatement(node)&&node.finallyBlock?.getText(tree).includes("clearInterval(ticker)")===true))).toBe(true);
});

test("the memory governor of a call is disposed in the finally of the same call, so its abort listener never keeps the environment of a finished chunk alive",()=>{
 const found=calls().filter(call=>call.callee==="governor?.dispose");
 expect(found.length).toBe(1);
 expect(found[0]!.ancestors.some(node=>ts.isTryStatement(node)&&node.finallyBlock!==undefined&&node.finallyBlock.getText(tree).includes("governor?.dispose()"))).toBe(true);
});

test("every governed owner names its kind, the learned peaks persist in the inference data directory and the shared closures are primed once before the first owner",()=>{
 const all=calls(),runs=all.filter(call=>call.callee==="governor.run");
 expect(runs.length).toBeGreaterThan(0);
 for(const run of runs)expect(run.text).toContain("kind:ownerKindV1(job.json,workspaceRoot)");
 const creation=all.find(call=>call.callee==="createGraphParentMemoryGovernorV1")!;
 expect(creation.text).toContain("peaks:createGraphOwnerPeaksFileV1(");expect(creation.text).toContain("owner-peaks.json");
 const primes=all.filter(call=>call.callee==="primeSharedClosuresV1");
 expect(primes.length).toBe(1);
 expect(primes[0]!.ancestors.some(node=>ts.isIfStatement(node)&&node.expression.getText(tree)==="late")).toBe(true);
 expect(source.indexOf("primeSharedClosuresV1(workspaceRoot")).toBeLessThan(source.indexOf("const workers=Array.from"));
});

test("the owner stage reports every running owner with kind and observed rows and the admission decision in the progress detail, and writes nothing to the stderr of the job",()=>{
 const all=calls(),running=all.filter(call=>call.callee==="phases?.running"&&call.text.includes('"inference"'));
 expect(running.length).toBe(1);expect(running[0]!.text).toContain("list,decision");for(const word of["row.kind","observedInferenceRowsV1(row.cachePath)","options?.onOwnerRunning?.(list,decision)"])expect(source).toContain(word);
 const creation=all.find(call=>call.callee==="createGraphParentMemoryGovernorV1")!;
 expect(creation.text).toContain("decide:event=>{decision=event;}");expect(creation.text).not.toContain("report");expect(creation.text).not.toContain("stderr");
 expect(all.some(call=>call.callee==="forgetObservedInferenceRowsV1")).toBe(true);
});
