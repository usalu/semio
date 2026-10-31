import {test,expect} from "bun:test";
import Ajv2020 from "ajv";
import {toByteArray} from "base64-js";
import fixture from "../🧫️fixtures/🎟️funding.json";
import { retainedCloneContractSchema as schema } from "@semio-tech/value-ts/retained-clone/contract/schema";
import {BinarySourceJob} from "../../🟦️.ts";
const validate=new Ajv2020({strict:true}).compile(schema);
const funded={maximumItems:1,maximumCopyBytes:32,maximumCapacityBytes:4,maximumReleaseBytes:0,maximumDepth:1};
test("encoded source original funding denies independent currencies without mutation",()=>{
 expect(Array.from(toByteArray(fixture.input.data.split(",")[1]!))).toEqual(fixture.expected);
 for(const row of fixture.cases){expect(validate(row.grant)).toBe(true);const job=new BinarySourceJob(fixture.input);for(let n=0;n<256;n++){if(row.stage==="header"||row.stage==="allocation"&&job.nextCapacityByteDemand()>0||row.stage==="handoff"&&job.progress().done)break;job.advance(fixture.input.data,funded);}const before=job.progress();if(row.stage==="handoff"){const result=job.takeResult(row.grant);expect(result!==undefined).toBe(row.accepted);if(result)expect(Array.from(result.value)).toEqual(fixture.expected);}else{const step=job.advance(fixture.input.data,row.grant);expect(step.receipt.copiedItems>0).toBe(row.accepted);if(!row.accepted)expect(job.progress()).toEqual(before);}job.cancel();expect(()=>job.advance(fixture.input.data,funded)).toThrow(/cancel/i);expect(fixture.input.data).toBe("data:application/octet-stream;base64,AQIDBA==");}
});
