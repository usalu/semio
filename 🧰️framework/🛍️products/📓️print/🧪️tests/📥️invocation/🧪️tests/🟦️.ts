import{test,expect}from"bun:test";
import ts from"typescript";
import{readFileSync}from"node:fs";
import{compileAdapterVizProbeV1}from"../🟦️.ts";
import corpus from"./📋️callers.json"with{type:"json"};
test("actual Print adapters pass the real context at every ordinary probe call",()=>{expect(typeof compileAdapterVizProbeV1).toBe("function");for(const path of corpus.callers){const source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);let count=0;function visit(node:ts.Node):void{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="compileAdapterVizProbeV1"){count++;expect(node.arguments[0]?.getText(source)).toBe("ctx");expect(node.arguments[2]?.getText(source).includes("workDir")).toBe(false);}ts.forEachChild(node,visit);}visit(source);expect(count,path).toBeGreaterThan(0);}expect(corpus.callers.length).toBeGreaterThan(50);});
