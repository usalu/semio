/** 🧹️ Contract: the project inference of a graph read keeps its parent inside the declared memory: the governor sees the budget and the same ports as the collector, the collection follows the managed bytes and never the resident set, every finished project is offered to the collector, and the collector, the peak and the farm reach the wave meter. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import ts from"typescript";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("project-nodes.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
const calls=()=>{const found:{text:string;callee:string}[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node))found.push({text:node.getText(tree),callee:node.expression.getText(tree)});ts.forEachChild(node,visit);};visit(tree);return found;};

test("the governor of a call gets the managed budget of the parent and the ports its collector reads",()=>{
 const all=calls(),governors=all.filter(call=>call.callee==="createGraphParentMemoryGovernorV1"),collectors=all.filter(call=>call.callee==="createParentCollectorV1"),budgets=all.filter(call=>call.callee==="graphParentManagedBytesV1");
 expect(governors).toHaveLength(1);expect(collectors).toHaveLength(1);expect(budgets).toHaveLength(1);
 expect(governors[0]!.text).toContain("budgetBytes:parentBytes");expect(governors[0]!.text).toContain("ports:memoryPorts");
 expect(collectors[0]!.text).toContain("ports:memoryPorts");expect(collectors[0]!.text).toContain("budgetBytes:parentBytes");
});

test("the collection never follows the resident set and every finished project is offered to the collector",()=>{
 expect(source).not.toContain("memoryUsage().rss");expect(source).not.toContain("collectAbove");
 const offers=calls().filter(call=>call.callee==="parentCollector.afterProject");
 expect(offers.length).toBeGreaterThanOrEqual(1);
 expect(source).toMatch(/const loop=async[^\n]*=>\{[^\n]*collectResident\(\)/u);
});

test("the end of the inference reports the collector and the running projects report their peak to the wave meter before the phase ends",()=>{
 const all=calls(),finish=all.find(call=>call.callee==="graphWaveMeterV1.parent");
 expect(finish?.text).toBe("graphWaveMeterV1.parent(parentCollector.finish())");
 expect(source.indexOf("graphWaveMeterV1.parent(")).toBeLessThan(source.indexOf('phases?.end("inference"'));
 const observe=all.find(call=>call.callee==="graphWaveMeterV1.observe");
 expect(observe?.text).toBe("graphWaveMeterV1.observe(memoryPorts.sample(),active.keys())");
});
