import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{join,resolve,dirname}from"node:path";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import TOML from"@iarna/toml";
import fixture from"./🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
test("pack byte source and sink have one bounded independent owner and portable SQLite oracle",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const root=join(import.meta.dir,".."),manifest=TOML.parse(readFileSync(join(root,"📦️packages/🦀️rust/Cargo.toml"),"utf8"))as any;
 expect(manifest.package.name).toBe(fixture.owner);expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual(fixture.sourceBoundary);expect(Object.keys(manifest.dependencies)).toEqual(["semio-framework-pack-error"]);
 const db=new Database(":memory:");try{for(const row of [...fixture.reads,...fixture.exact]){const input=Buffer.from(row.input,"hex"),offset=BigInt(row.offset),expected=offset>BigInt(input.length)?row.offset:offset+BigInt(row.length)>BigInt(input.length)&&!("name"in row)?String(input.length):null;expect(row.truncatedAt).toBe(expected);const output=offset>BigInt(input.length)?"":(db.query("select lower(hex(substr(?1,?2,?3))) as output").get(input,Number(offset)+1,row.length)as{output:string}).output;expect(output).toBe(row.output);}for(const row of fixture.writes)expect(row.chunks.join("")).toBe(row.output);}finally{db.close();}
 const source=readFileSync(join(root,"🦀️.rs"),"utf8");expect(Buffer.byteLength(source)).toBeLessThan(fixture.sourceBoundary.maxSourceBytes);expect(source).not.toContain("protocol::");expect(source).not.toContain("semio_framework_replication");console.log("[DEBUG] Independent SQLite BLOB reads and closed Ajv/TOML corpus qualify the actual pack byte owner");
});
test("actual callers use the nominal byte owner without source-mounted or compatibility barrels",()=>{
 const repo=resolve(import.meta.dir,"../../../../.."),owner=resolve(import.meta.dir,"../📦️packages/🦀️rust");
 for(const relative of fixture.consumers){const path=resolve(repo,relative),manifest=TOML.parse(readFileSync(path,"utf8"))as any;expect(resolve(dirname(path),manifest.dependencies[fixture.owner].path)).toBe(owner);}
 for(const relative of ["🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs"]){const body=readFileSync(resolve(repo,relative),"utf8");expect(body).not.toContain("pub use protocol::source");expect(body).not.toContain("pub use crate::source");expect(body).not.toContain('"../../🚰️source/🦀️.rs"');}
 console.log("[DEBUG] Seven concrete Cargo callers depend directly on the one pack byte owner");
});
