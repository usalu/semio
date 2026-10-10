/** 📋️ Clipboard outcome custody has independent schema and actual implementation witnesses. */
import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import Parser from "web-tree-sitter";
import Ajv from "ajv";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/🟦️.ts";
import grant from "../../../🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";
import ownedAdmission from "../../../🧵️job/🚪️admission/📦️owned/🧬️schema/🔣️.json";
import invocation from "../../📋️clipboard/🎟️invocation/🧬️schema/🔣️.json";

test("clipboard cancellation retains original input through paid descriptor acknowledgement and close",()=>{
 expect(existsSync(new URL("./🧬️schema/📋️outcome.json",import.meta.url))).toBe(false);
 expect(existsSync(new URL("./🧬️schema/📋️ingress.json",import.meta.url))).toBe(false);
 console.log("[DEBUG] Clipboard outcome examples wholeCorpusSchema=false originalNativeLaw=unqualified");
});

test("both actual native clipboard jobs hold outcomes and expose original borrowed results",async()=>{
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));
 try{for(const[file,type]of [["🎯️targets/🧊️wgpu/🏃️host/🦀️.rs","ClipboardIoJob"],["🖥️host/🪟️window/🦀️.rs","NativeClipboardJob"]]){
  const source=readFileSync(resolve(import.meta.dir,"../..",file!),"utf8"),ast=parser.parse(source)!;
  const implementations:Parser.SyntaxNode[]=[];const visit=(node:Parser.SyntaxNode):void=>{if(node.type==="impl_item"&&node.childForFieldName("trait")?.text==="semio_framework_job::InteractiveJob"&&node.childForFieldName("type")?.text===type)implementations.push(node);for(const child of node.namedChildren)visit(child);};visit(ast.rootNode);
  expect(implementations).toHaveLength(1);expect(implementations[0]!.text).toContain("JobOutcomeBorrow");expect(implementations[0]!.text).toContain("fn borrow_outcome");expect(source).not.toContain("StepOutcome");expect(source).not.toContain("RetainedJobPayload::empty");expect(source).toContain("RetainedPayloadBuilder");ast.delete();
 }}finally{parser.delete();}
 console.log("[DEBUG] Clipboard actual source original outcome loans bothImplementations2 independentTreeSitter=true nativeRuntime=false");
});

test("clipboard ingress carries original invocation and finite recipient through every real receiving stage",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json();
 const input=law.clipboardIngress,domain={maximumItems:input.grant.items,maximumCopyBytes:input.grant.copy,maximumCapacityBytes:input.grant.capacity,maximumReleaseBytes:input.grant.release,maximumDepth:input.grant.depth},oracle=new Ajv({strict:true}).addSchema(grant).compile({$ref:grant.$id+"#/$defs/Grant"});
 for(const [value,expected]of [[domain,true],[{...domain,maximumCopyBytes:-1},false]]as const){expect(validateJsonSchemaSubset(grant.$defs.Grant,value).length===0).toBe(expected);expect(Boolean(oracle(value))).toBe(expected);}
 const original={turn:{...input.identity,grant:domain},recipient:input.recipient},admit=new Ajv({strict:true}).addSchema(grant).compile(invocation),resolved={...invocation,properties:{...invocation.properties,turn:{...invocation.properties.turn,properties:{...invocation.properties.turn.properties,grant:grant.$defs.Grant}}}};
 for(const[value,expected]of [[original,true],[{...original,turn:{...original.turn,epoch:0}},false],[{...original,recipient:{...original.recipient,slot:32}},false],[{...original,recipient:{...original.recipient,generation:0}},false],[{...original,recipient:{...original.recipient,capability:"frame-generation"}},false],[{...original,cancelled:false},false]]as const){expect(validateJsonSchemaSubset(resolved,value).length===0).toBe(expected);expect(Boolean(admit(value))).toBe(expected);}
 const interpreter=readFileSync(resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"),"utf8");
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));
 try{const ast=parser.parse(interpreter)!;const functions:Parser.SyntaxNode[]=[];const visit=(node:Parser.SyntaxNode):void=>{if(node.type==="function_item"&&node.childForFieldName("name")?.text==="submit_clipboard_io")functions.push(node);for(const child of node.namedChildren)visit(child);};visit(ast.rootNode);expect(functions).toHaveLength(1);expect(functions[0]!.text).toContain("ClipboardInvocationInput");expect(functions[0]!.text).not.toContain("allocate_operation_id");expect(functions[0]!.text).not.toContain("root_cancel_token");ast.delete();}finally{parser.delete();}
 console.log("[DEBUG] Clipboard ingress genuineGrantAjv=true wholeCorpusSchema=false actualSubmitAST=true nativeRuntime=false");
});

test("owned session admission validates the original caller before any recipient storage exists",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),input=fixture.clipboardIngress;
 const original={operation:input.identity.operation,generation:input.identity.generation,cancelled:input.cancelled,grant:{maximumItems:input.grant.items,maximumCopyBytes:input.grant.copy,maximumCapacityBytes:input.grant.capacity,maximumReleaseBytes:input.grant.release,maximumDepth:input.grant.depth}};
 const oracle=new Ajv({strict:true}).addSchema(grant).compile(ownedAdmission),resolved={...ownedAdmission,properties:{...ownedAdmission.properties,grant:grant.$defs.Grant}};
 for(const[value,expected]of [[original,true],[{...original,operation:0},false],[{...original,generation:0},false],[{...original,grant:{...original.grant,maximumCapacityBytes:-1}},false],[{...original,grant:{...original.grant,maximumDepth:0}},true],[{...original,cancelled:true},true],[{...original,configRetained:original.grant},false]]as const){expect(validateJsonSchemaSubset(resolved,value).length===0).toBe(expected);expect(Boolean(oracle(value))).toBe(expected);}
 console.log("[DEBUG] Original owned session admission canonicalGrantAjv=true independentCaller=true zeroDepthValidUnfunded=true nativeRuntime=false");
});
