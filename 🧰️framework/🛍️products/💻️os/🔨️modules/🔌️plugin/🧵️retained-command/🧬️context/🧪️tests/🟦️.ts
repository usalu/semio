import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import Ajv2020 from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
test("original context header custody never certifies unsupported payload leaves",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);expect(validate(corpus),JSON.stringify(validate.errors)).toBe(true);
 for(const row of corpus.cases){const original={pending:row.original};const projected=applyPatch(original,row.headerAdmitted?[{op:"move",from:"/pending",path:"/issued"}]:[],true,false).newDocument as unknown as {pending?:typeof row.original;issued?:typeof row.original};expect(projected.pending??projected.issued).toEqual(row.original);expect({retained:true,terminal:false}).toEqual(row.expected);}
 const source=readFileSync(join(import.meta.dir,"../../../📜️declaration/🧩️app/🛬️lifted/🧬️context/🦀️.rs"),"utf8");expect(source.includes("SealedShared::admit(original,issuer,grant)")).toBe(true);expect(source.includes("Source::Refused{original,fault}")).toBe(true);expect(source.includes("let issuer=SharedIssuer::owned()")).toBe(true); const fields=readFileSync(join(import.meta.dir,"../../../📜️declaration/🧩️app/🛬️lifted/📸️fields/🦀️.rs"),"utf8"); expect(fields.includes("ControlledRetirement::new(parts.metadata)")).toBe(true); expect(fields.includes("residual:ManuallyDrop::new(Some(parts.residual))")).toBe(true); expect(fields.includes("self.metadata.terminal_is_empty()&&self.residual.is_none()")).toBe(true);expect(source.includes("Arc::")).toBe(false);
 const host=readModuleTreeSource(join(import.meta.dir,"../../🦀️.rs"));expect(host.includes("context: Option<ArtifactOwnedContextHandle<A>>,")).toBe(true);expect(host.includes("else if self.context.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.context,&mut self.input_retirement,grant))}")).toBe(true);expect(host.includes("&& self.context.is_none()")).toBe(true);expect(host.includes("&& self.input_retirement.is_none()")).toBe(true);expect(host.includes("context_retirement")).toBe(false);
});
   