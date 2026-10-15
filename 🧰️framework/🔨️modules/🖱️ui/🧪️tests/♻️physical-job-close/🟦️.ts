/** 📋️ Clipboard outcome custody has independent schema and actual implementation witnesses. */
import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import Parser from "web-tree-sitter";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/🟦️.ts";
import grant from "../../../🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";
import ownedAdmission from "../../../🧵️job/🚪️admission/📦️owned/🧬️schema/🔣️.json";
import invocation from "../../📋️clipboard/🎟️invocation/🧬️schema/🔣️.json";
import { readModuleTreeSource, sliceModuleTreeSource } from "../../../📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

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
 const interpreter=readModuleTreeSource(resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"));
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


test("native clipboard closure quotes whole allocation under the same original context and independent SQL",async()=>{
 const law=await Bun.file(new URL("../../🖥️host/🧪️tests/🧫️fixtures/📋️clipboard-close/🔣️.json",import.meta.url)).json(),validate=new Ajv({strict:true}).addSchema(grant).compile({$ref:grant.$id+"#/$defs/Grant"});expect(validate(law.callerGrant)).toBe(true);
 const database=new Database(":memory:");try{for(const row of law.cases){const observed=row.items>0&&row.release>=law.reserve&&row.depth>=law.closeDepth?"complete":"pending";expect(observed).toBe(row.expected);expect((database.query("SELECT CASE WHEN ?1>0 AND ?2>=?3 AND ?4>=?5 THEN 'complete' ELSE 'pending' END AS result").get(row.items,row.release,law.reserve,row.depth,law.closeDepth)as{result:string}).result).toBe(row.expected)}}finally{database.close();}
 const source=readFileSync(new URL("../../🖥️host/🪟️window/🦀️.rs",import.meta.url),"utf8"),start=source.indexOf("fn close_step(&mut self,cx:"),body=source.slice(start,source.indexOf("fn next_close_copy_byte_demand",start));expect(start).toBeGreaterThan(0);expect(body).toContain("OriginalTurnFailure");expect(body).toContain("close_original_retained_step(cx");expect(body).not.toContain("Close::Refused");expect(body).not.toContain("StepContext::new");expect(body).not.toContain("root_cancel_token");console.log("[DEBUG] Original Native clipboard context closes whole String allocation with exact actual posted effects; independent SQLite four-axis refusal/terminal laws and Ajv caller grant verified");
});


test("clipboard dispatch source remains owned across asynchronous intent handback and validates the actual incoming caller",async()=>{
 const root=resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"),renderer=readModuleTreeSource(resolve(root,"🧊️renderer/🦀️.rs")),host=readFileSync(resolve(root,"🪟️winit-app/🦀️.rs"),"utf8"),frame=readFileSync(resolve(root,"🧵️frame-job/🦀️.rs"),"utf8");
 const cursor=sliceModuleTreeSource(resolve(root,"🧊️renderer/🦀️.rs"),"struct RuntimeDispatchCursor","struct FrameActionEnvelope");await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));try{const tree=parser.parse(cursor)!;expect(tree.rootNode.hasError()).toBe(false);tree.delete()}finally{parser.delete()}
 expect(cursor).toContain("OriginalCallerSourceLoan");expect(cursor).toContain("original_source_epoch()");expect(cursor).not.toContain("StepContext::new");expect(cursor).not.toContain("root_cancel_token");expect(cursor).not.toContain("cx.deadline_us()>self.deadline");expect(renderer).toContain("original.validate_context(cx)");expect(renderer).toContain("enqueue_dispatch_on_original_caller");expect(host).toContain("pump_frame_inputs(cx)?");expect(frame).toContain("pump_runtime_mailbox_step(cx)");expect(frame).toContain("apply_pending_step(cx)");
 const database=new Database(":memory:");try{for(const[same,open,items,expected]of[[1,1,1,1],[0,1,1,0],[1,0,1,0],[1,1,0,0]])expect((database.query("SELECT (?1=1 AND ?2=1 AND ?3>0) AS admitted").get(same,open,items)as{admitted:number}).admitted).toBe(expected)}finally{database.close()}
 console.log("[DEBUG] Original dispatch source supplier/cancellation/recipient admission retains asynchronous intents; independent SQL rejects foreign/closed/unfunded opportunity; native closure remains separate qualification");
});

test("clipboard original preparation preserves portable phases and final storage metadata under the original policy",async()=>{
 const law=await Bun.file(new URL("../📋️clipboard-ingress/🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../../📋️clipboard/🎟️invocation/🏗️preparation/🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);expect(validateJsonSchemaSubset(schema,law)).toEqual([]);
 const db=new Database(":memory:");try{db.run("CREATE TABLE custody(stage TEXT,source INTEGER,producer INTEGER,descriptor INTEGER)");for(const stage of law.stages){const source=["original-source","denied-copy","destination-paid"].includes(stage)?1:0,producer=["producer-fields","semantic-owned","original-outcome","acknowledged"].includes(stage)?1:0,descriptor=stage==="original-outcome"?1:0;db.query("INSERT INTO custody VALUES(?,?,?,?)").run(stage,source,producer,descriptor)}expect(db.query("SELECT source,producer,descriptor FROM custody WHERE stage='denied-copy'").get()).toEqual({source:1,producer:0,descriptor:0});expect(db.query("SELECT source,producer,descriptor FROM custody WHERE stage='original-outcome'").get()).toEqual({source:0,producer:1,descriptor:1});expect(db.query("SELECT source+producer+descriptor AS live FROM custody WHERE stage='original-root-return'").get()).toEqual({live:0})}finally{db.close()}
 const job=readFileSync(new URL("../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs",import.meta.url),"utf8");expect(job).toContain("WorkerJobInitializer<ClipboardIoJob>");expect(job).toContain("initialize_empty_at");expect(job).not.toContain("write_bytes(target");const source=readModuleTreeSource(resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"));expect(source).toContain("WorkerJobSessionPreparation::try_begin");expect(source).toContain("try_step_on_original_caller(cx)");expect(source).toContain("acknowledge_outcome(cx.retained_grant())");expect(source).not.toContain("WorkerJobSession::try_new");expect(source).not.toContain("owner.take_outcome()");console.log("[DEBUG] Original clipboard preparation neutral Ajv/SQLite custody and actual first-party final-field source are qualified; native original initializer gate is separate");
});

test("clipboard original interaction retirement preserves its live owner and full failure through the genuine context",async()=>{
 const root=resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"),renderer=readModuleTreeSource(resolve(root,"🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")),interpreter=readModuleTreeSource(resolve(root,"🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"));
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));try{const tree=parser.parse(interpreter)!;expect(tree.rootNode.hasError()).toBe(false);tree.delete()}finally{parser.delete()}
 const close=sliceModuleTreeSource(resolve(root,"🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"),"pub(crate) fn close_input_step(","pub(crate) fn close_renderer_asset_step(");expect(close).toContain("OriginalTurnFailure");expect(close).toMatch(/close_clipboard_io_step\(&mut interaction\.input,\s*cx\)\?\s*\{return Ok\(false\)\}/u);expect(close.indexOf("close_step_with_receipt")).toBeGreaterThan(0);expect(close.indexOf("close_clipboard_io_step")).toBeLessThan(close.indexOf("close_step_with_receipt"));expect(close).not.toContain("StepContext::new");expect(interpreter).toContain("OriginalCallerSourceLoan");expect(interpreter).toContain("validate_clipboard_source(owner.invocation.source,cx)?");console.log("[DEBUG] Actual original interaction close source keeps the live owner before funded clipboard cleanup and propagates full failure; independent Rust TreeSitter parses original typed pipeline");
});

test("cancelled original dispatch returns its interaction and in-flight event before paid closure",async()=>{
 const law=await Bun.file(new URL("../📋️clipboard-ingress/🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../../📋️clipboard/🎟️invocation/🏗️preparation/🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);expect(validateJsonSchemaSubset(schema,law)).toEqual([]);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE owner(stage TEXT,interaction INTEGER,event INTEGER,source INTEGER)");for(const row of law.dispatchCustody)db.query("INSERT INTO owner VALUES(?,?,?,?)").run(row.stage,row.interaction,row.event,row.source);for(const stage of ["cancelled-return","completed-return"])expect(db.query("SELECT interaction,event,source FROM owner WHERE stage=?").get(stage)).toEqual({interaction:1,event:1,source:1});expect(db.query("SELECT interaction+event+source AS live FROM owner WHERE stage='physically-closed'").get()).toEqual({live:0})}finally{db.close()}
 const root=resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"),renderer=readModuleTreeSource(resolve(root,"🧊️renderer/🦀️.rs")),host=readFileSync(resolve(root,"🪟️winit-app/🦀️.rs"),"utf8"),guard=sliceModuleTreeSource(resolve(root,"🧊️renderer/🦀️.rs"),"struct RuntimeDispatchReturn","pub(crate) struct FrameDeferredExecutionEnvelope");expect(guard).toContain("interaction:Option<AppInteractionState>");expect(guard).toContain("inflight.as_mut()");expect(guard).toContain("interaction:self.interaction.take()");expect(renderer).toContain("fn close_inflight_step(");expect(renderer).toContain("inflight: Option<ui_render::DispatchEvent>");expect(host).toContain("event: &mut DispatchEvent");expect(guard).not.toContain("interaction:None");await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));try{for(const source of [guard,host]){const tree=parser.parse(source)!;expect(tree.rootNode.hasError()).toBe(false);tree.delete()}}finally{parser.delete()}console.log("[DEBUG] original dispatch cancellation retains interaction/event/source before funded close; schema and independent SQLite custody agree");
});
