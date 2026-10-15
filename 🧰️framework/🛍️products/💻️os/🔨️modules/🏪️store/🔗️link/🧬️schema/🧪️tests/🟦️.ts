/** 🔬️ Contract test: the durable link schema owns its native codec, checked against independent Ajv and TOML readings. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import fixture from "./🔣️.json" with {type:"json"};
import fixtureSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import schema from "../🔣️.json" with {type:"json"};
import blobSchema from "../../../📦️blob/🧬️schema/🔣️.json" with {type:"json"};
import referenceSchema from "../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json" with {type:"json"};
test("durable link schema owns its full native codec independently of Store",()=>{
 expect(new Ajv({strict:true}).compile(fixtureSchema)(fixture)).toBe(true);
 const root=join(import.meta.dir,".."),manifest=TOML.parse(readFileSync(join(root,"📦️packages/🦀️rust/Cargo.toml"),"utf8")) as any;
 expect(manifest.package.name).toBe(fixture.owner);expect(resolve(root,"📦️packages/🦀️rust",manifest.package.workspace)).toBe(resolve(import.meta.dir,"../../../../../../../.."));expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual(fixture.sourceBoundary);
 expect(Object.keys(manifest.dependencies).sort()).toEqual(["semio-framework-artifact-reference","semio-framework-dsl-record","semio-framework-os-store-blob-identity","semio-framework-pack-json","semio-framework-value"]);
 const ajv=new Ajv({strict:true});ajv.addKeyword("x-semio-ui");ajv.addSchema(blobSchema);ajv.addSchema(referenceSchema);const accepts=ajv.compile(schema);
 for(const row of fixture.valid)expect(accepts(row)).toBe(true);for(const row of fixture.invalid)expect(accepts(row)).toBe(false);
 const source=readFileSync(join(root,"🦀️.rs"),"utf8"),codec=readFileSync(join(root,"📦️codec/🦀️.rs"),"utf8");expect(source).toContain("artifact_retire_struct!(ArtifactLink");expect(codec).toContain("DslField for LinkPin");expect(codec).toContain("DslField for ArtifactLink");expect(codec).not.toContain("super::super");expect(codec).not.toContain("crate::os_");
 const store=join(root,"../.."),storeSource=readFileSync(join(store,"🦀️.rs"),"utf8"),child=readFileSync(join(store,"🪆️child/🦀️.rs"),"utf8");expect(storeSource).toContain("pub use semio_framework_os_store_link_schema::{ArtifactLink,LinkPin};");expect(storeSource).not.toContain("mod artifact_link_schema");expect(storeSource).not.toContain("DslField for ArtifactLink");expect(child).not.toContain("fn link_pin_");
 console.log("[DEBUG] Link pins and complete native codec have one bounded inward owner; independent Ajv/TOML agree");
});
