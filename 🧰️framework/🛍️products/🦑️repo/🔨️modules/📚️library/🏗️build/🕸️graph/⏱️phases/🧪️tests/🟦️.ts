/** ⏱️ Contract: the phase clock publishes and times exactly what an independent Python throttle model predicts and refuses malformed phase use. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {createGraphPhaseClockV1} from "../🟨️.mjs";

const replay=async(steps:typeof fixture.cases[number]["steps"],minimumMilliseconds:number)=>{
 let at=0;const published:string[]=[],written:unknown[]=[],clock=createGraphPhaseClockV1({now:()=>at,minimumMilliseconds,publish:resource=>{published.push(resource);},write:row=>{written.push(row);}}),rows:unknown[]=[];
 for(const step of steps){at=step.at;if(step.op==="begin")await clock.begin(step.phase,step.total!);else if(step.op==="advance")await clock.advance(step.phase,step.done!);else rows.push(await clock.end(step.phase));}
 return {published,rows,written,clock};
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

for(const row of fixture.cases)test("clock case: "+row.name,async()=>{
 const result=await replay(row.steps,row.minimumMilliseconds);
 expect(result.published).toEqual(row.published);expect(result.rows).toEqual(row.rows);expect(result.written).toEqual(row.rows);expect(result.clock.rows()).toEqual(row.rows);
});

for(const row of fixture.refusals)test("clock refusal: "+row.name,async()=>{await expect(replay(row.steps,250)).rejects.toThrow();});

test("details of a phase are kept next to its timing and a clock without ports never throws",async()=>{
 let at=0;const clock=createGraphPhaseClockV1({now:()=>at});await clock.begin("inference",2);at=40;await clock.advance("inference",2);at=75;
 expect(await clock.end("inference",{projects:2})).toEqual({phase:"inference",milliseconds:75,done:2,total:2,projects:2});
});
