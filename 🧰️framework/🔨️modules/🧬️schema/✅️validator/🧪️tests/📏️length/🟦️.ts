import assert from"node:assert/strict";
import Ajv from"ajv";
import{compileJsonSchemaSubset,validateJsonSchemaSubset,jsonSchemaSubsetValueEquals}from"../../🟦️.ts";
import corpus from"./🔣️.json";
import corpusSchema from"./🧬️schema/🔣️.json";
import original from"../🧫️fixtures/🔣️.json";
/** 🧪️ Describes one neutral synchronous Source proof without a test-runner dependency or Native authority. */
export interface SchemaStringLawV1{readonly name:string;readonly prove:()=>number}
function recorder(){let assertions=0;return{equal(actual:unknown,expected:unknown,id?:string){assertions++;assert.deepStrictEqual(actual,expected,id);},count(){return assertions;}};}
function portable():number{
 const proof=recorder();proof.equal(new Ajv({strict:true}).validate(corpusSchema,corpus),true);
 for(const row of corpus.cases){const value=String.fromCharCode(...row.units).repeat(row.repeat),schemas=[{type:"string",minLength:row.characters,maxLength:row.characters},{type:"string",minLength:row.characters+1},...(row.characters?[{type:"string",maxLength:row.characters-1}]:[])];for(const[index,schema]of schemas.entries()){const prepared=compileJsonSchemaSubset(schema),oracle=new Ajv({strict:true,allErrors:true}).compile(schema),accepted=prepared.validate(value).length===0;proof.equal(accepted,index===0,row.id);proof.equal(accepted,oracle(value),row.id);proof.equal(prepared.validate(value),validateJsonSchemaSubset(schema,value),row.id);}}
 const value=String.fromCharCode(...corpus.diagnostic.units);proof.equal(compileJsonSchemaSubset(corpus.diagnostic.schema).validate(value),corpus.diagnostic.errors);proof.equal(new Ajv({strict:true,allErrors:true}).compile(corpus.diagnostic.schema)(value),false);proof.equal(compileJsonSchemaSubset({type:"array",items:corpus.diagnostic.schema}).validate([value]),corpus.diagnostic.errors.map(error=>"/0"+error));
 for(const row of original.cases){const documents="documents"in row.input?row.input.documents:[],prepared=compileJsonSchemaSubset(row.input.schema,row.input.schema,documents),oracle=new Ajv({strict:false,schemas:documents as object[]}).compile(row.input.schema);proof.equal(prepared.validate(row.value).length===0,row.valid,row.id);proof.equal(oracle(row.value),row.valid,row.id);proof.equal(prepared.validate(row.value),validateJsonSchemaSubset(row.input.schema,row.value,row.input.schema,documents),row.id);}
 for(const row of original.comparisons){proof.equal(jsonSchemaSubsetValueEquals(row.left,row.right),row.equal,row.id);proof.equal(new Ajv({strict:false}).compile({const:row.left})(row.right),row.equal,row.id);}
 return proof.count();
}
function generated():number{
 const proof=recorder(),validators=Array.from({length:5},(_,length)=>{const schema={type:"string",minLength:length,maxLength:length};return{prepared:compileJsonSchemaSubset(schema),oracle:new Ajv({strict:true}).compile(schema)};});let state=0x51e1a,validations=0;
 for(let index=0;index<corpus.generatedPairs;index++){state=(Math.imul(state,1664525)+1013904223)>>>0;const first=state&65535;state=(Math.imul(state,1664525)+1013904223)>>>0;const second=state&65535,value=String.fromCharCode(first,second);for(const item of validators){proof.equal(item.prepared.validate(value).length===0,item.oracle(value));validations++;}}
 const schema={type:"object",required:["value"],properties:{value:{type:"string",minLength:1,maxLength:1}},additionalProperties:false},compiled=compileJsonSchemaSubset(schema),source={value:"😀"};proof.equal(compiled.validate(source),[]);source.value="😀x";proof.equal(compiled.validate(source).length,1);source.value="é";proof.equal(compiled.validate(source),[]);return proof.count();
}
function work():number{
 const proof=recorder(),prepared=corpus.performance.schemas.map(schema=>compileJsonSchemaSubset(schema)),iterator=String.prototype[Symbol.iterator],character=String.prototype.charCodeAt,counts:{iterators:number;characterReads:number}[]=[];
 for(const validator of prepared){let iterators=0,characterReads=0;String.prototype[Symbol.iterator]=function(this:string){if(String(this)===corpus.performance.value)iterators++;return Reflect.apply(iterator,this,[]);};String.prototype.charCodeAt=function(this:string,index:number){if(String(this)===corpus.performance.value)characterReads++;return Reflect.apply(character,this,[index]);};try{for(let index=0;index<corpus.iterations;index++)proof.equal(validator.validate(corpus.performance.value),[]);}finally{String.prototype[Symbol.iterator]=iterator;String.prototype.charCodeAt=character;}counts.push({iterators,characterReads});}
 proof.equal(counts.map(row=>row.iterators),prepared.map(()=>0));proof.equal(counts.slice(0,2).map(row=>row.characterReads),[0,0]);return proof.count();
}
/** 🔬️ Supplies immutable first-party Source laws for any runner, retaining all portable vectors. */
export function schemaStringLawsV1():readonly SchemaStringLawV1[]{return Object.freeze([Object.freeze({name:"neutral schema string validation preserves complete portable Unicode and diagnostic cases",prove:portable}),Object.freeze({name:"neutral schema string validation observes all generated UTF16 pairs without value caching",prove:generated}),Object.freeze({name:"neutral schema string validation avoids iterator work in actual prepared consumers",prove:work})]);}
