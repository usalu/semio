import {expect,test} from "bun:test";
import Ajv from "ajv";
import {unresolvedActionArgs} from "../../🟦️.ts";
import type {ActionArgDef} from "../../../🛂️manifest/🎬️action-types-and-semantics/🟦️.ts";
import fixture from "../🧫️fixtures/🔽️choices/🔣️.json";
import schema from "./🧬️schema/🔣️.json";

type ChoiceCase=Readonly<{id:string;format:"text"|"artifactKind"|"surfaceApp";options:readonly string[];required:boolean;value:unknown;unresolved:boolean}>;

/** 🔽️ Checks staged choice resolution against the shared neutral corpus and an independent JSON Schema evaluator. */
test("action choice resolution agrees with closed enum validation",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile<{cases:ChoiceCase[]}>(schema),input:unknown=fixture;
 if(!validate(input))throw Error(JSON.stringify(validate.errors));
 const label={native:{en:"Choice",de:"Auswahl"},reuse:{en:"Choice",de:"Auswahl"}};
 for(const row of input.cases){
  const format=row.format==="artifactKind"?{kind:"artifactKind" as const,roles:[]}:row.format==="surfaceApp"?{kind:"surfaceApp" as const,roles:[],dialectArg:"dialect"}:undefined;
  const definition:ActionArgDef={id:row.id,label,schema:{kind:"string",options:row.options.map(value=>({value,label})),...(format?{format}:{})},required:row.required,nullable:false};
  const closed=row.options.length>0||row.format!=="text",valueSchema=closed?(row.options.length?{type:"string",enum:row.options}:false):{type:"string",minLength:1};
  const oracle=ajv.compile({type:"object",additionalProperties:false,properties:{value:valueSchema},required:row.required?["value"]:[]});
  const value=row.value===null||row.value===""?{}:{value:row.value},unresolved=!oracle(value);
  expect(unresolved).toBe(row.unresolved);
  expect(unresolvedActionArgs([definition],{[row.id]:row.value})).toEqual(unresolved?[row.id]:[]);
 }
});
