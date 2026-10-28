import "../../🖼️resources/🧪️tests/🟦️.ts";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{printCompilationIntentV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
test("Print compilation intent is finite immutable DATA with no output or authority field",()=>{const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");try{db.exec("create table intents(kind text,json text)");for(const row of corpus.valid){db.exec("delete from intents");expect(validate(row)).toBe(true);const actual=printCompilationIntentV1(row);db.query("insert into intents values(?,?)").run(row.kind,JSON.stringify(row));const stored=db.query<{json:string},[string]>("select json from intents where kind=?").get(row.kind);if(!stored)throw Error("Independent intent row required");expect(JSON.stringify(actual)).toBe(stored.json);expect(Object.isFrozen(actual)).toBe(true);}for(const row of corpus.invalid)expect(()=>printCompilationIntentV1(row)).toThrow();}finally{db.close();}});

import ts from"typescript";
import{readFileSync}from"node:fs";
test("actual Print compiler has no free-output probe or global hidden font preparation",()=>{const path=new URL("../../🟦️.ts",import.meta.url),source=ts.createSourceFile(path.pathname,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),names:string[]=[];function visit(node:ts.Node):void{if(ts.isFunctionDeclaration(node)&&node.name)names.push(node.name.text);ts.forEachChild(node,visit);}visit(source);expect(names.includes("compilePrintTexOnce")).toBe(false);expect(source.text.includes("let stagedPrintFonts")).toBe(false);});

import "../../🟦️.ts";

test("Print intent refuses accessors without executing them before immutable capture",()=>{let reads=0;const request={version:1,kind:"probe",name:"probe.tex",get source(){reads++;return"TeX";}};expect(()=>printCompilationIntentV1(request)).toThrow();expect(reads).toBe(0);});

test("Print publication maps original names to the actual staged compiler names",()=>{const url=new URL("../../🟦️.ts",import.meta.url),source=ts.createSourceFile(url.pathname,readFileSync(url,"utf8"),ts.ScriptTarget.Latest,true);const calls:ts.CallExpression[]=[];function visit(node:ts.Node):void{if(ts.isCallExpression(node)&&node.expression.getText(source)==="printTemplatePdfNames"&&node.arguments[0]?.getText(source)==="staged.texPath")calls.push(node);ts.forEachChild(node,visit);}visit(source);expect(calls).toHaveLength(1);});

test("actual compiler classifies genuine terminal failure before preserving exact primary",()=>{const url=new URL("../../🟦️.ts",import.meta.url),file=ts.createSourceFile(url.pathname,readFileSync(url,"utf8"),ts.ScriptTarget.Latest,true),body=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="compileV1")?.getText(file)??"";expect(body).toContain("captureOwnedCommandResultV1");expect(body).toContain("new OwnedCommandFailure");expect(body).toContain("capturePrintCompilerRefusalV1(primary");expect(body.indexOf("capturePrintCompilerRefusalV1(primary")).toBeLessThan(body.indexOf("throw primary"));});

test("Print probe Source recheck follows actual requested consumer drainage",()=>{const url=new URL("../../🟦️.ts",import.meta.url),file=ts.createSourceFile(url.pathname,readFileSync(url,"utf8"),ts.ScriptTarget.Latest,true),body=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="withCompiledPrintProbeV1")?.getText(file)??"";expect(body).toContain("withRequestedIssuedResourceControlV1(context.control,0");expect(body.lastIndexOf("await recheckCompilationV1(context,staged)")).toBeGreaterThan(body.indexOf("value:await withRequestedIssuedResourceControlV1"));});
