import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json";
import law from"./🧫️fixtures/🔣️.json";
test("media command pending owns the original decoder and full failure through paid source close",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const original=Buffer.from(law.command.parent);expect(original.toString("utf8")).toBe(law.command.parent);const integer=Buffer.alloc(8);integer.writeBigUInt64LE(BigInt(law.command.revision));expect(integer.readBigUInt64LE()).toBe(18446744073709551615n);
 const db=new Database(":memory:");try{expect(db.query("SELECT value AS stage FROM json_each(?) ORDER BY key").all(JSON.stringify(law.close)).map(row=>(row as{stage:string}).stage)).toEqual(law.close)}finally{db.close()}
 const runtime=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(runtime.includes("match owner.original_command()")).toBe(true);expect(runtime.includes("PluginCommandIngress::Decoded(owner)")).toBe(true);expect(runtime.includes("FailedDecoded{owner,failure:Some(failure)")).toBe(true);expect(runtime.includes("owner.close_original_step(cx)")).toBe(true);const manifest=readFileSync(new URL("../🛂️manifest/🦀️.rs",import.meta.url),"utf8");expect(manifest.includes("owner.close_step(cx)")).toBe(true);expect(manifest.includes("self.wire.close_original_step(cx)")).toBe(true);expect(runtime.includes("admit_original_domain_failure(&mut compound,cx)")).toBe(true);expect(runtime.includes("TerminalOriginalFailure(failure)=>return Err(failure.into())")).toBe(true);
 console.log("[DEBUG] media original decoder Source declarations + independent neutral Ajv/SQLite/Buffer wire; actual native custody law still requires native gate");
});
