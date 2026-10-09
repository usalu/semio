import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import grantSchema from "../../🧬️schema/🔣️.json";
import progressSchema from "../../🧬️schema/🧾️progress.json";
import axes from "../../🧫️fixtures/🔣️.json";
test("original retained codecs isolate nested child workloads and preserve all axes",()=>{
 const ajv=new Ajv({strict:true});expect(ajv.compile(schema)(fixture)).toBe(true);
 const grant=JSON.parse(JSON.stringify(axes.rows[1])),receipt=JSON.parse(JSON.stringify(axes.receipts[1]));expect(ajv.compile(grantSchema)(grant)).toBe(true);expect(ajv.compile(progressSchema)(receipt)).toBe(true);expect(Object.keys(grant).length).toBe(fixture.grantUnits);expect(Object.keys(receipt).length).toBe(fixture.receiptUnits);
 const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");for(const name of["RetainedCloneGrant","RetainedCloneProgress"]){for(const direction of["ToValue","FromValue"]){const from=source.indexOf(`impl crate::${direction} for ${name}`),end=source.indexOf("\n}",from);expect(source.slice(from,end)).toContain("control.scoped_stage(|control|{control.begin_stage(");}}
});
