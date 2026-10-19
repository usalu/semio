/** 🧪️ Contract test: the blob identity schema owner agrees with an independent Ajv and TOML reading of its manifest and fixture. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import fixture from "./🔣️.json" with {type:"json"};
import fixtureSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import { blobRefSchema as schema } from "@semio-tech/schema-ts/artifact-reference";
test("blob identity has one small independent schema owner and exact neutral JSON",()=>{
 expect(new Ajv({strict:true}).compile(fixtureSchema)(fixture)).toBe(true);
 const root=join(import.meta.dir,".."),manifest=TOML.parse(readFileSync(join(root,"📦️packages/🦀️rust/Cargo.toml"),"utf8")) as any;
 expect(manifest.package.name).toBe(fixture.owner);expect(resolve(root,"📦️packages/🦀️rust",manifest.package.workspace)).toBe(resolve(import.meta.dir,"../../../../../.."));expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual(fixture.sourceBoundary);
 expect(Object.keys(manifest.dependencies).sort()).toEqual(["semio-framework-pack-json","semio-framework-value","serde"]);
 const accepts=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.cases){if(row.json.includes("1844674407370955161"))continue;expect(accepts(JSON.parse(row.json)),row.json).toBe(row.accepted);}
 const source=readFileSync(join(root,"🦀️.rs"),"utf8");expect(source).toContain("artifact_retire_struct!(BlobRef");expect(source).not.toContain("semio_framework_os_kernel");expect(source).not.toContain("crate::{FromValue");
});
