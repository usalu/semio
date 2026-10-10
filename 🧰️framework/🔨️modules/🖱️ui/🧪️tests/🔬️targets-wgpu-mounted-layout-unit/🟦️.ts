import {existsSync as testingSchemaExists} from "node:fs";
/** 🤝️ Original worker receiving has schema and independent syntax witnesses before native acceptance. */
import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import Parser from "web-tree-sitter";
import Ajv from "ajv";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/🟦️.ts";
import law from "./🧫️fixtures/🫴️receiving.json";
import grant from "../../../🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";

test("mounted receiving declares the original policy and exact ownership obligations",()=>{
expect(testingSchemaExists(new URL("./🧬️schema/🫴️receiving.json",import.meta.url))).toBe(false); const oracle=new Ajv({strict:true}).addSchema(grant).compile({$ref:grant.$id+"#/$defs/Grant"});
 for(const [value,expected]of [[law.grant,true],[{...law.grant,maximumCopyBytes:-1},false],[{...law.grant,maximumCopyBytes:law.grant.maximumCopyBytes+1},true]]as const){expect(validateJsonSchemaSubset(grant.$defs.Grant,value).length===0).toBe(expected);expect(Boolean(oracle(value))).toBe(expected);}
 console.log("[DEBUG] Mounted receiving genuineGrantAjv=true wholeCorpusSchema=false fixedCallerPolicyOriginalNativeLaw=unqualified runtime=false");
});

test("mounted receiving uses original loans and explicit paid acknowledgement",async()=>{
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));
 const root=resolve(import.meta.dir,"../.."),source=readFileSync(resolve(root,law.source),"utf8"),engine=readFileSync(resolve(root,law.engine),"utf8");
 try{const ast=parser.parse(source)!;const syntax:Parser.SyntaxNode[]=[];const errors=(node:Parser.SyntaxNode):void=>{if(node.type==="ERROR"||node.isMissing())syntax.push(node);for(const child of node.namedChildren)errors(child);};errors(ast.rootNode);expect(syntax.filter(node=>!(node.type==="ERROR"&&/^[13]$/.test(node.text)&&node.parent?.type==="tuple_pattern"&&node.parent.text.startsWith("(UiNode::")&&node.parent.text.endsWith(", "+node.text+"..)")))).toHaveLength(0);const steps:Parser.SyntaxNode[]=[];const visit=(node:Parser.SyntaxNode):void=>{if(node.type==="function_item"&&node.childForFieldName("name")?.text==="step")steps.push(node);for(const child of node.namedChildren)visit(child);};visit(ast.rootNode);expect(steps).toHaveLength(1);expect(steps[0]!.text).toContain("JobOutcomeBorrow");expect(source).not.toContain("StepOutcome");expect(source).not.toContain("RetainedJobPayload::empty");expect(source).toContain("RetainedPayloadBuilder");expect(source).toContain("fn borrow_outcome");expect(engine).toContain("acknowledge_checked_out_outcome");expect(engine).not.toContain("take_checked_out_outcome");ast.delete();}finally{parser.delete();}
 console.log("[DEBUG] Mounted receiving actual source loans=true independentTreeSitter=true runtime=false");
});


test("mounted session birth borrows original authority and retains pending admission custody", () => {
expect(testingSchemaExists(new URL("./🧬️schema/🚪️admission.json",import.meta.url))).toBe(false); const root=resolve(import.meta.dir,"../.."),engine=readFileSync(resolve(root,law.engine),"utf8");
 expect(engine).toContain("MountedWorkerJobSession::try_admit_owned");
 expect(engine).not.toContain("MountedWorkerJobSession::try_new");
 expect(engine).toContain("generation: cx.generation()");
 expect(engine).toContain("layout_session_generation");
 expect(engine).toContain("LayoutWorkerAdmission");
 expect(engine).toContain("original_cancel_token()");expect(engine).toContain("window.layout_admission.failure=Some(error)");expect(engine).toContain("failure_close");expect(engine).not.toContain("std::mem::size_of::<semio_framework_job::BatchJobParams>() + std::mem::size_of::<semio_framework_async::CancelTokenRetirement>()");
 const quote=engine.match(/fn parameter_retirement_demands\([^)]*\)[^{]*\{([\s\S]*?)\n    \}/)![1]!;expect(quote).toContain("copy_bytes: 0");expect(quote).not.toContain("size_of");const body=engine.match(/struct LayoutWorkerAdmission \{([^}]+)\}/)![1]!;expect([...body.matchAll(/^\s*(\w+):/gm)].map(field=>field[1])).toEqual(law.admission.pendingFields);const close=engine.slice(engine.indexOf("impl LayoutWorkerAdmission {"),engine.indexOf("impl Drop for LayoutWorkerAdmission"));expect(close).not.toContain("std::mem::size_of::<Option<semio_framework_async::CancelTokenRetirement>>");expect(close).not.toContain("std::mem::size_of::<Option<semio_framework_value::ValueError>>");
 for(const marker of ["pub fn close_surface_one","pub fn step_layouts"]){const at=engine.indexOf(marker),body=engine.slice(at,engine.indexOf("let next = match phase",at)>at?engine.indexOf("let next = match phase",at):engine.indexOf("let Some(root)",at));expect(body).toContain("layout_admission.close_step");expect(body).toContain("!window.layout_admission.terminal_is_empty()");expect(body.indexOf("layout_admission.close_step")).toBeLessThan(body.indexOf("layout_retirement_refusal = None"));}
 console.log("[DEBUG] mounted birth original caller generation and semantic layout generation stay distinct; actual pending job/params custody required; sourceCustody=true nativeRuntime=false");
});
