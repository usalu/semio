import {test,expect} from 'bun:test';
import {Database} from 'bun:sqlite';
import Ajv from 'ajv/dist/2020';
import ts from 'typescript';
import {readFileSync,existsSync} from 'node:fs';
import {resolve,dirname} from 'node:path';
import {bindBuildCommandV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🟨️.mjs";
import {applyBuildGeneratedTaskDependenciesV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import schema from '../🧬️schema/🔣️.json';
import vectors from './🔣️.json';
const root=resolve(import.meta.dir,"../../../../../../../..");
const manifest=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📋️project.json"),expectedProducer=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🏗️build/📜️script.ts"),consumer=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts"),generated=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts");
const options={target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,allowJs:true,noLib:true,skipLibCheck:true,resolveJsonModule:true};
/** 🧭️ Resolves real source imports without materializing or reading declared generated output. */
function consumesGenerated(entry:string):boolean{
 const program=ts.createProgram([entry],options),cache=ts.createModuleResolutionCache(root,path=>path,options);
 for(const source of program.getSourceFiles()){
  let found=false;const visit=(node:ts.Node)=>{
   const request=ts.isImportDeclaration(node)||ts.isExportDeclaration(node)?node.moduleSpecifier:ts.isCallExpression(node)&&node.expression.kind===ts.SyntaxKind.ImportKeyword?node.arguments[0]:ts.isImportTypeNode(node)&&ts.isLiteralTypeNode(node.argument)?node.argument.literal:undefined;
   if(request&&ts.isStringLiteral(request)){
    if(request.text.startsWith(".")&&resolve(dirname(source.fileName),request.text)===generated)found=true;
    const resolved=ts.resolveModuleName(request.text,source.fileName,options,ts.sys,cache).resolvedModule?.resolvedFileName;if(resolved&&resolve(resolved)===generated)found=true;
   }
   ts.forEachChild(node,visit);
  };ts.forEachChild(source,visit);if(found)return true;
 }return false;
}
test('producer command and neutral generated boundaries retain actual ownership',()=>{
 const validate=new Ajv({allErrors:true}).compile(schema);expect(validate(vectors),JSON.stringify(validate.errors)).toBe(true);
 const project=JSON.parse(readFileSync(manifest,"utf8")),target=project.targets.wasm,bound=bindBuildCommandV1(target.options.command,root,target.options.cwd,project.root,[]);
 expect(bound.script).toBe(expectedProducer);expect(existsSync(expectedProducer)).toBe(true);expect(expectedProducer.length).toBeLessThanOrEqual(256);
 const source=readFileSync(expectedProducer,"utf8");expect(source).toContain("buildAdmittedWasmWebV1");expect(source).toContain("capture => publishFlowBrowserPackage(capture, this.invocation)");
 const old=readFileSync(resolve(root,project.root,"📜️script.ts"),"utf8");expect(old).not.toMatch(/class WasmScript|register\("wasm"|defaultCommand: "wasm"/u);
 const outputs=new Map([["producer",consumesGenerated(expectedProducer)],["consumer",consumesGenerated(consumer)]]),database=new Database(":memory:");
 try{for(const row of vectors.cases){const actual=outputs.get(row.entry)!;expect(actual,row.id).toBe(row.generated);const producer=project.name+":wasm",consumerName=project.name+":"+row.target,oracle=database.query("SELECT CASE WHEN ? = 1 AND ? = ? THEN 1 ELSE 0 END AS refused").get(Number(actual),producer,consumerName) as {refused:number};let refused=false;try{const result=applyBuildGeneratedTaskDependenciesV1({name:project.name,targets:{[row.target]:{inputs:["default"],dependsOn:[] as string[]}}},{[row.target]:actual?[producer]:[]});if(actual)expect(result.targets[row.target].dependsOn).toContain(producer);}catch(error){if(!String(error).includes("Generated task cannot consume its own output"))throw error;refused=true;}expect(refused,row.id).toBe(Boolean(oracle.refused));expect(refused,row.id).toBe(row.refused);}}finally{database.close();}
});
