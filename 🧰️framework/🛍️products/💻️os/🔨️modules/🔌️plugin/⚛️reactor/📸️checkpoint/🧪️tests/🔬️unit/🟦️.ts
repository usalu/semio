import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv/dist/2020";

/** 📸️ Validates the language-neutral checkpoint actor contract with an independent schema oracle. */
export function checkpointActorOracle(root: string): number {
  const base=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint");
  const schema=JSON.parse(readFileSync(join(base,"🧬️schema/🔣️.json"),"utf8"));
  const fixture=JSON.parse(readFileSync(join(base,"🧫️fixtures/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
  for(const sample of fixture.cases)if(Boolean(validate(sample.pack))!==sample.expected)throw Error(`${sample.id}: ${JSON.stringify(validate.errors)}`);
  return fixture.cases.length;
}
