import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {evaluate} from "mathjs";
const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
test("retained factory language-neutral vectors match independent Ajv and arithmetic oracles",()=>{
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({allErrors:true}).validate(schema,fixture)).toBe(true);
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(source).toContain("fn quote<");
 expect(source).toContain("admit_original_tool_job_measured");
 for(const row of fixture.cases){
  const refused=row.inputBytes!==null&&row.inputBytes>row.ownerMaximum||row.checkpoint&&(!row.resume||row.inputBytes===null||(row.checkpointBytes??0)>512);
  expect(row.refusal).toBe(refused?"invalid-value":null);
  if(!refused){const raw=row.inputBytes!==null&&row.payloadAdmitted&&row.inputBytes<=row.payloadMaximum?row.inputBytes:0;expect(evaluate("frame + raw",{frame:fixture.jobBytes,raw})).toBe(row.capacityBytes);}
 }
});
