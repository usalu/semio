import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{createHash}from"node:crypto";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json";
import law from"../../../../../🧪️tests/🧫️fixtures/🧊️wgpu/🎬️media-slots/🎟️collect/🔣️.json";
import accepted from"../../../../../🧪️tests/🧫️fixtures/🎬️presented-media-slots/🔣️.json";
import { readModuleTreeSource } from "../../../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
test("original roster source owns its typed grammar, paid coverage and original collection until close",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 expect(createHash("sha256").update(JSON.stringify(accepted.tokenIdentity)).digest("hex")).toBe(law.token);
 const db=new Database(":memory:");try{expect(db.query("SELECT json_array_length(?) AS fields").get(JSON.stringify(accepted.tokenIdentity))).toEqual({fields:8})}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const required of["UiPresentedTreeLoan","NativeDecodeControl::resume","OriginalPresentedSlot::admit","self.coverage.step(source,engine,&self.loans","ArtifactCanonicalJsonTreeCursor::admit","RetainedCloneSource::admit_owned(original","source.take_authority","semio_framework_job::OriginalClosureFailure{initial:","close_failed_step","engine.return_presented_tree_source(loan,cx)","*output=std::mem::take(&mut owners.slots)"]){expect(source.includes(required)).toBe(true)}
 expect(source.includes("serde_json::from_str")).toBe(false);expect(source.includes("serde_json::to_vec")).toBe(false);
 const publication=readFileSync(new URL("../../../📸️render-snapshot/🦀️.rs",import.meta.url),"utf8");expect(publication.includes("source:&mut Option<RenderSnapshot>")).toBe(true);expect(publication.includes("incoming.as_ref().unwrap().revision==0")).toBe(true);expect(publication.includes("close_shared(owner,cx)")).toBe(true);
 const presenter=readFileSync(new URL("../../../🪟️winit-app/🦀️.rs",import.meta.url),"utf8");expect(presenter.includes("self.original_snapshot=Some(")).toBe(true);expect(presenter.includes("retain_original_publication(&mut self.original_snapshot,cx)")).toBe(true);
});

test("original native resource owner follows the full tree and same caller through publication",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const custody=law.nativeResourceCustody,db=new Database(":memory:");
 try{db.exec("CREATE TABLE custody(stage INTEGER PRIMARY KEY,arena INTEGER,generation TEXT,loans INTEGER)");custody.events.forEach((_,index)=>db.query("INSERT INTO custody VALUES(?,?,?,?)").run(index,custody.arenaSlot,custody.arenaGeneration,custody.loanCounts[index]!));expect(db.query("SELECT count(DISTINCT arena || generation) AS owners,max(loans) AS peak FROM custody").get()).toEqual({owners:1,peak:2});expect(db.query("SELECT loans FROM custody ORDER BY stage DESC LIMIT 1").get()).toEqual({loans:0});}finally{db.close()}
 const root=process.cwd(),read=(path:string)=>readFileSync(`${root}/${path}`,"utf8"),ui="🧰️framework/🔨️modules/🖱️ui/🧠️runtime",plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor";
 const producer=read(`${ui}/🎭️present/🦀️.rs`);expect(producer.includes("try_new(tree: ComponentTree")).toBe(true);expect(producer.includes("self.source.take()")).toBe(true);
 const reconcile=read(`${ui}/⏭️cursor/🦀️.rs`);expect(reconcile.includes("native_resources: tree.native_resources")).toBe(true);expect(reconcile.includes("native_resources: self.native_resources.take()")).toBe(true);expect(reconcile.includes("close_native_resources_on_original")).toBe(true);
 const patches=read(`${plugin}/🩹️patches/🦀️.rs`);expect(patches.includes("commit_source(&mut self, source: &mut Option<ComponentTree>")).toBe(true);expect(patches.includes("pub fn drive_one(&self, cx:")).toBe(true);expect(patches.includes("StepContext::new(")).toBe(false);
 const turn=read(`${plugin}/🔄️turn/🦀️.rs`);expect(turn.includes("grant.commit_source(source,cx)")).toBe(true);expect(turn.includes("patches.drive_one(cx)")).toBe(true);
});

test("original render tree remains stationary across receiver denial and post-render failure",()=>{
 const db=new Database(":memory:");try{expect(db.query("SELECT json_extract(?,'$.source') AS source,json_extract(?,'$.source') AS denied").get(JSON.stringify({source:"original"}),JSON.stringify({source:"original"}))).toEqual({source:"original",denied:"original"});}finally{db.close()}
 const root=process.cwd(),base=`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`,read=(path:string)=>readFileSync(`${base}/${path}`,"utf8");
 const instance=readModuleTreeSource(`${base}/🧵️instance/🦀️.rs`);for(const required of["pending_render_tree:","pending_render_tree_identity:","close_pending_render_tree_on_original","fn original_render_tree_source","self.pending_render_tree=", "stamp_pending_render_interaction_ui"]){expect(instance.includes(required)).toBe(true)}
 const app=read("⚛️reactor/🔌️app/🦀️.rs");expect(app.includes("fn original_render_tree_source")).toBe(true);
 const runtime=read("⚛️reactor/🏃️runtime/🦀️.rs");expect(runtime.includes("instance.app.original_render_tree_source(cx)")).toBe(true);
 const turn=readModuleTreeSource(`${base}/⚛️reactor/🔄️turn/🦀️.rs`);expect(turn.includes("grant.commit_source(tree).is_err()")).toBe(false);
});


test("original pending render retry copies its unchanged routing source in finite UTF8 byte slices",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const retry=law.pendingRenderRetry,bytes=Buffer.from(retry.surface),db=new Database(":memory:");
 try{for(const copy of retry.copyGrants){const result:number[]=[];for(let at=0;at<bytes.length;at+=copy){const row=db.query("SELECT substr(?, ?, ?) AS body").get(bytes,at+1,copy)as{body:Uint8Array};result.push(...row.body);}expect(Buffer.from(result).toString()).toBe(retry.surface)}}finally{db.close()}
 const root=process.cwd(),text=readFileSync(`${root}/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs`,"utf8"),patch=readModuleTreeSource(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`);
 expect(text).toContain("pub struct UiTextOriginalCopy");expect(text).toContain("source.bytes[start..end]");expect(patch).toContain("pending_render_source: Option<ui_contract::UiTextOriginalCopy>");expect(patch).toContain("source.step(&slot.surface.0,grant)");
 expect(retry.headerCopyBytes).toBe(0);
});


test("original intrinsic parser donates its complete native cause once beside its actual receipt",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const cause=law.originalPackCause,db=new Database(":memory:");try{expect(db.query("SELECT ? AS variant,? AS original,? AS offset").get(cause.variant,cause.message,cause.offset)).toEqual({variant:cause.variant,original:cause.message,offset:cause.offset})}finally{db.close()}
 const source=readFileSync(`${process.cwd()}/🧰️framework/🔨️modules/🎒️pack/🌱️intrinsic/🎮️retained/🦀️.rs`,"utf8"),start=source.indexOf("pub fn advance_original_step(&mut self,grant:"),end=source.indexOf("/// ⏱️",start),original=source.slice(start,end);
 expect(original).toContain("self.fault.take()");expect(original).not.toContain("error.clone()");expect(original).toContain("original_failed");expect(cause.donationCopyBytes).toBe(0);
});


test("original tree producer admits its exact shell and validates unchanged native children without reconstruction",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const producer=law.originalTreeProducer,db=new Database(":memory:");try{db.exec("CREATE TABLE native_children(parent INTEGER,child INTEGER,address TEXT)");db.query("INSERT INTO native_children VALUES(0,0,?)").run("original-child");expect(db.query("SELECT address FROM native_children WHERE parent=0 AND child=0").get()).toEqual({address:"original-child"});}finally{db.close()}
 const root=process.cwd(),present=readFileSync(`${root}/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/🦀️.rs`,"utf8"),patches=readModuleTreeSource(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`);
 expect(present).toContain("source: Option<ComponentTree>");expect(present).toContain("ComponentTreeProducerFrame::EMPTY");expect(present).toContain("pub fn close_step_on_original");expect(present).not.toContain("parent.admitted.try_push");expect(present).not.toContain("frame.source.next()");expect(patches).toContain("ComponentTreeProducer::required_capacity_bytes()");expect(patches).toContain("maximum_capacity_bytes<capacity");expect(patches).toContain("authority.close_step_on_original(cx)");expect(patches).toContain("released_bytes:capacity");expect(producer.donationCopyBytes).toBe(0);
});
