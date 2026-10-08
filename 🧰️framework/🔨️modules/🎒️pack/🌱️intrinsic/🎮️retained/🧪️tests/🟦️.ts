import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {parse,ParseError} from "jsonc-parser";
import contract from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import {readIntrinsicBody} from "../../../🌱️value/🧪️tests/🫳️preflight/🔮️pack/🟦️.ts";

test("retained intrinsic ingress has a variable production contract and finite work grants",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(contract);const request=ajv.compile({$ref:contract.$id+"#/$defs/Request"});const document=ajv.compile({$ref:contract.$id+"#/$defs/DocumentRequest"});const grant=ajv.compile({$ref:contract.$id+"#/$defs/AdvanceGrant"});
 for(const maximumOwnedBytes of [1,1024,1048576])expect(request({inputBytes:7,maximumOwnedBytes,maximumDepth:64,maximumItems:100})).toBe(true);
 expect(request({inputBytes:7,maximumOwnedBytes:0,maximumDepth:64,maximumItems:100})).toBe(false);expect(request({inputBytes:7,maximumOwnedBytes:1024,maximumDepth:0,maximumItems:100})).toBe(false);
 for(const maximumUnits of corpus.units)expect(grant({maximumUnits,cancelled:false})).toBe(true);expect(grant({maximumUnits:-1,cancelled:false})).toBe(false);
 expect(document({inputBytes:256,maximumOwnedBytes:1048576,maximumDepth:64,maximumItems:100})).toBe(true);expect(document({inputBytes:256,maximumOwnedBytes:0,maximumDepth:64,maximumItems:100})).toBe(false);expect(document({inputBytes:256,maximumOwnedBytes:1048576,maximumDepth:64,maximumItems:100,guessBody:true})).toBe(false);
});

function exactWords(value:unknown):unknown{
 if(typeof value==="number")return String(value);
 if(Array.isArray(value))return value.map(exactWords);
 if(value!==null&&typeof value==="object")return Object.fromEntries(Object.entries(value).map(([key,child])=>[key,exactWords(child)]));
 return value;
}

test("hand-authored neutral Body vectors agree with the adjacent independent wire reader and JSON parser",()=>{
 for(const row of corpus.cases){
  const bytes=Uint8Array.from(row.hex.match(/../g)!.map(pair=>parseInt(pair,16)));const actual=readIntrinsicBody(bytes);expect(actual).toEqual(exactWords(row.expected));
  const errors:ParseError[]=[];expect(parse(JSON.stringify(actual),errors,{disallowComments:true,allowTrailingComma:false})).toEqual(exactWords(row.expected));expect(errors).toEqual([]);
 }
});
