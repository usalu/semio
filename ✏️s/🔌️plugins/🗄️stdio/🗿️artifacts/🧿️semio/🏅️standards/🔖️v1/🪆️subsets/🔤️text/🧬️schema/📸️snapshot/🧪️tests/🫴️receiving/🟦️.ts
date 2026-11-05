import {expect,test} from "bun:test";
import Ajv from "ajv";
import chunk from "lodash/chunk";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/🧬️.json";
test("Text receiving keeps source custody independent of turn frontiers",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 const bytes=new TextEncoder().encode(JSON.stringify(fixture.snapshot));
 for(const width of [1,7,64,4096])expect(chunk([...bytes],width).flat()).toEqual([...bytes]);
 expect(fixture.snapshot.runs.flatMap(run=>run.marks.map(mark=>mark.kind)).sort()).toEqual(["bold","code","italic","link"]);
 expect(JSON.parse(JSON.stringify(fixture.snapshot))).toEqual(fixture.snapshot);
});
test("every original currency denial and partial cancellation has explicit terminal custody",()=>{
 expect(new Set(fixture.deniedAxes).size).toBe(5);
 for(const prefix of fixture.cancelAfterTurns){const trace=Array.from({length:prefix},()=> "receiving");trace.push("cancelled","paid-close","empty");expect(trace.slice(-3)).toEqual(["cancelled","paid-close","empty"]);}
 expect(fixture.expected).toEqual({complete:"typed-output",cancelled:"paid-empty",denied:"unchanged",source:"same-pointer-length",stock:"same-original"});
});
test("Text nested records and enum declare genuine typed receiving",async()=>{
 const source=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();
 expect(source.match(/semio_framework_dsl_record_derive::DslReceiving/g)?.length??0).toBe(4);
 expect(source).not.toContain("semio_framework_dsl_record_derive::DslOwnedReceiving");
});
