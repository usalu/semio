/** 🌱️ Tests for build command plan-child. */
import{test,expect}from"bun:test";
import{readFile}from"node:fs/promises";
import ts from"typescript";

test("a graph plan child verifies no dependency generation: the invocation that started it has just done it",async()=>{
 const source=ts.createSourceFile("runtime.ts",await readFile(new URL("../../⚙️runtime/🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),guards:string[]=[];
 const visit=(node:ts.Node):void=>{if(ts.isIfStatement(node)){const condition=node.expression.getText(source);if(condition.includes('buildOperationRequiresV1(command)==="dependencies"'))guards.push(condition);}ts.forEachChild(node,visit);};
 visit(source);
 expect(guards).toHaveLength(1);
 expect(guards[0]).toContain("process.env[BUILD_GRAPH_PLAN_OUT_V1]===undefined");
});
