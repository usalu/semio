/** 🛑️ The current numeric Trace door cannot spend a genuinely empty explicit caller ledger. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {BitmapTraceJob} from "../../../🟦️.ts";
import {RetainedWorkLedger} from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import rows from "../../../🧪️tests/🧫️fixtures/🔣️.json";
import nativeWork from "../../📐️.json";
import law from "./📐️.json";
import schema from "./🔣️.json";
test("existing BitmapTrace zero authority cannot create public work or real edges",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const row=rows.find(value=>value.name===law.sourceCase);if(row===undefined)throw Error("Original trace source absent");const mask=new Uint8Array(row.input.mask),job=new BitmapTraceJob({...row.input,mask}),work=new RetainedWorkLedger(law.grant),db=new Database(":memory:"),errors:unknown[]=[];try{const actual=Reflect.apply(job.advance,job,[law.quantum,work]),expected=db.query("SELECT ? AS work,? AS edges").get(law.expected.work,law.expected.edges);expect({work:actual.work,edges:actual.edges}).toEqual(expected);}catch(error){errors.push(error);}finally{try{job.cancel();const moved=job.intoRetirement();expect(moved.mask).toBe(mask);let done=false;for(let at=0;!done&&at<nativeWork.maxTurns;at++)done=moved.job.advance(1).done;expect(done).toBe(true);expect(Array.from(mask)).toEqual(row.input.mask);}catch(error){errors.push(error);}db.close();}if(errors.length)throw errors.length===1?errors[0]:new AggregateError(errors,"Actual trace zero-work assertion and existing numeric cleanup failures");});
