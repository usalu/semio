/** 🎨️ All original curve flags and transforms preserve independent SVG pixel coverage. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {PathFlattenJob,type PathFlattenInput} from "../../../../🟦️.ts";
import inputSchema from "../../../../🧬️schema/🔣️.json";
import {Bank,complete,projection,retire} from "../funded.test.ts";
import {actualCoverage,originalCoverage} from "./oracle.mjs";
import contract from "./📐️.json";
import schema from "./🔣️.json";
test("funded original quadratic cubic and every ellipse flag preserve the exact independent Sharp coverage",async()=>{expect(new Ajv({strict:true}).compile(schema)(contract)).toBe(true);const validate=new Ajv({strict:true}).compile<PathFlattenInput>(inputSchema);for(const row of contract.cases){if(!validate(row.input))throw Error("Original curve oracle input invalid");const before=structuredClone(row.input),bank=new Bank(),job=new PathFlattenJob(row.input),errors:unknown[]=[];try{complete(job,bank,4096);const actual=await actualCoverage(projection(job.result()),row.input.transform,contract.size,contract.sampleScale),expected=await originalCoverage(row.input.segments,row.input.transform,contract.size,contract.sampleScale);expect(actual.length).toBe(expected.length);for(let at=0;at<actual.length;at++)expect(Math.abs(actual[at]!-expected[at]!)).toBeLessThanOrEqual(contract.maximumChannelDelta);expect(row.input).toEqual(before);}catch(error){errors.push(error);job.cancel();}try{retire(job,bank);}catch(error){errors.push(error);}if(errors.length)throw new AggregateError(errors,"Actual funded geometry, independent original SVG and same-bank cleanup failures");}},120000);
