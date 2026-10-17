import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
test("borrowed artifact reference is an independent bounded identity floor",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const root=join(import.meta.dir,".."),manifest=TOML.parse(readFileSync(join(root,"📦️packages/🦀️rust/Cargo.toml"),"utf8"))as any;
 expect(manifest.package.name).toBe(fixture.owner);expect(manifest.dependencies??{}).toEqual({});expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual(fixture.sourceBoundary);
 const own=await import("../🟦️.ts"),oracle=new Ajv({strict:true}).compile({type:"string",pattern:"^[a-z0-9]+(?:-[a-z0-9]+)*[.][a-z0-9]+(?:-[a-z0-9]+)*[.][a-z0-9]+(?:-[a-z0-9]+)*$(?![\\s\\S])"});
 for(const row of fixture.cases){expect(oracle(row.kind),row.kind).toBe(row.accepted);expect(own.isCanonicalArtifactKind(row.kind),row.kind).toBe(row.accepted);}
 const source=readFileSync(join(root,"🦀️.rs"),"utf8");expect(Buffer.byteLength(source)).toBeLessThanOrEqual(fixture.sourceBoundary.maxSourceBytes);expect(source).not.toContain("semio_framework_value");
});
