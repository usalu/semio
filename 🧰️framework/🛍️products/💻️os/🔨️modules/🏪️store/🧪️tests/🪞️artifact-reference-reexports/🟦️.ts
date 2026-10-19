/** 🪞️ Contract test: the Store re-exports the neutral link and blob owners and keeps no second copy of their codec. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
const store=join(import.meta.dir,"../.."),read=(path:string)=>readFileSync(join(store,path),"utf8");
test("the Store re-exports the neutral artifact link owner without a second codec",()=>{
 const source=read("🦀️.rs"),child=read("🪆️child/🦀️.rs");
 expect(source).toContain("pub use semio_framework_os_store_link_schema::{ArtifactLink,LinkPin};");expect(source).not.toContain("mod artifact_link_schema");expect(source).not.toContain("DslField for ArtifactLink");expect(child).not.toContain("fn link_pin_");
});
test("the Store re-exports the neutral blob identity owner without a second copy",()=>{
 const source=read("🦀️.rs"),retirement=read("♻️retirement/🦀️.rs"),child=read("🪆️child/🦀️.rs");
 expect(source).toContain("pub use semio_framework_os_store_blob_identity::BlobRef;");expect(source).not.toContain("mod blob_reference_schema");expect(retirement).not.toContain("super::BlobRef");expect(child).not.toContain("blob_reference_schema");
});
