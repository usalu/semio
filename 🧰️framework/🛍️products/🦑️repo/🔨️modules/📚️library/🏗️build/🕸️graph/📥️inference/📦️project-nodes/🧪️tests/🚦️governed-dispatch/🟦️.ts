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
 const creations=all.filter(call=>call.callee==="createMemoryGovernorV1");
 expect(creations.length).toBe(1);expect(creations[0]!.text).toContain("workers:limit");expect(creations[0]!.text).toContain("graphMemoryBudgetBytesV1(process.env)");
 expect(creations[0]!.text).toContain("signal:context.phaseControl?.signal");
 const ends=all.filter(call=>call.callee==="phases?.end"&&call.text.includes('"inference"'));
 expect(ends.length).toBe(1);expect(ends[0]!.text).toContain("governor.stats()");
});

test("the running projects are named in the inference phase while they run, removed when they end and the tick stops with the loops",()=>{
 const all=calls(),named=all.filter(call=>call.callee==="phases?.running"&&call.text.includes('"inference"'));
 expect(named.length).toBe(1);expect(named[0]!.text).toContain("active");
 const inferences=all.filter(call=>call.callee==="inferProject");
 for(const inference of inferences)expect(inference.ancestors.some(node=>ts.isTryStatement(node)&&node.finallyBlock!==undefined&&node.finallyBlock.getText(tree).includes("active.delete(job.name)")),inference.text).toBe(true);
 expect(all.some(call=>call.callee==="clearInterval"&&call.text==="clearInterval(ticker)"&&call.ancestors.some(node=>ts.isTryStatement(node)&&node.finallyBlock?.getText(tree).includes("clearInterval(ticker)")===true))).toBe(true);
});

test("the memory governor of a call is disposed in the finally of the same call, so its abort listener never keeps the environment of a finished chunk alive",()=>{
 const found=calls().filter(call=>call.callee==="governor?.dispose");
 expect(found.length).toBe(1);
 expect(found[0]!.ancestors.some(node=>ts.isTryStatement(node)&&node.finallyBlock!==undefined&&node.finallyBlock.getText(tree).includes("governor?.dispose()"))).toBe(true);
});
