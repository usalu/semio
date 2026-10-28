import {expect,test} from "bun:test";
import cases from "./step-emission.json" with {type:"json"};
import schema from "./step-emission.schema.json" with {type:"json"};
import Ajv from "ajv/dist/2020.js";
import {parseIfc4x3Step,writeIfc4x3Step} from "../step.ts";

const envelope=(data:string)=>`ISO-10303-21;HEADER;FILE_DESCRIPTION(('test'),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;${data}ENDSEC;END-ISO-10303-21;`;

test("shared native emitter input cases roundtrip in the independent language implementation",async()=>{
 expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);
 for(const row of cases.cases){const graph=await parseIfc4x3Step(envelope(row.data));expect(await parseIfc4x3Step(await writeIfc4x3Step(graph))).toEqual(graph);if(row.name==="Exact Integer and Negative Zero"){expect(graph.entities[0]!.id).toBe(9007199254740993n);expect(graph.entities[0]!.args[1]).toEqual({kind:"real",value:{bits:9223372036854775808n}});}}
});
