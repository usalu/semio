import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import ts from"typescript";
import{sDevCompositionLawsV1}from"../🔄️lifecycle/🟦️.ts";
import{sDevCompositionSourceOwnershipV1}from"../🏷️ownership/🟦️.ts";
test("neutral actual s-dev inventory lifecycle retains every portable oracle vector",async()=>{await sDevCompositionLawsV1();});
test("neutral actual s-dev defining owners preserve independent AST bundle and TOML oracles",async()=>{await sDevCompositionSourceOwnershipV1(process.cwd());},30000);
test("neutral actual generic inventory mount never passes an unchecked indexed contribution",()=>{const text=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("composition.ts",text,ts.ScriptTarget.Latest,true),unsafe:ts.Node[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="mount"){const item=node.arguments[0];if(item&&ts.isElementAccessExpression(item))unsafe.push(item);}ts.forEachChild(node,visit);};visit(tree);expect(unsafe).toHaveLength(0);});
