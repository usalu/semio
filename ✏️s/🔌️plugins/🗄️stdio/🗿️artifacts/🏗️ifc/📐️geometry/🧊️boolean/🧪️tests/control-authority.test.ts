import Ajv from "ajv";
import {test} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {booleanSolid,subtractHalfSpace,type SolidBooleanOptions,type SolidMesh,type SolidPoint} from "../🟦️.ts";

interface Row{operation:"difference"|"union"|"intersection"|"halfspace";observer:"tick"|"progress";phase:"initial"|"interior"|"completion";outcome:"error"|"undefined"|"false";}
const cases=JSON.parse(readFileSync(new URL("./control-authority.json",import.meta.url),"utf8")) as {version:1;cases:Row[]};
const box=(low:SolidPoint,high:SolidPoint):SolidMesh=>{const[a,b,c]=low,[x,y,z]=high;return {vertices:[[a,b,c],[x,b,c],[x,y,c],[a,y,c],[a,b,z],[x,b,z],[x,y,z],[a,y,z]],triangles:[[0,2,1],[0,3,2],[4,5,6],[4,6,7],[0,1,5],[0,5,4],[1,2,6],[1,6,5],[2,3,7],[2,7,6],[3,0,4],[3,4,7]]};};
const run=(operation:Row["operation"],options:SolidBooleanOptions,left=box([0,0,0],[4,4,4]),right=box([2,0,0],[6,4,4])):SolidMesh=>operation==="halfspace"?subtractHalfSpace(left,{origin:[0,0,2],normal:[0,0,1]},options):booleanSolid(operation,left,right,options);
test("Boolean authority contracts are closed language-neutral original-observer obligations",()=>{const schema=JSON.parse(readFileSync(new URL("./control-authority.schema.json",import.meta.url),"utf8")),validate=new Ajv({strict:true,allErrors:true}).compile(schema);assert.equal(validate(cases),true,JSON.stringify(validate.errors));assert.equal(new Set(cases.cases.map(row=>JSON.stringify(row))).size,60);});
for(const row of cases.cases)test("captured Boolean authority "+Object.values(row).join(" "),()=>{
 let total=0;run(row.operation,row.observer==="tick"?{tick:()=>{total++;}}:{progress:()=>{total++;return true;}});
 assert.ok(total>4);const target=row.phase==="initial"?1:row.phase==="completion"?total:Math.floor(total/2),sentinel=Error("original observer identity"),left=box([0,0,0],[4,4,4]),right=box([2,0,0],[6,4,4]),before=JSON.stringify({left,right});let calls=0,replacement=0,returned=false,caught=false,fault:unknown;
 const options:SolidBooleanOptions={maxWork:2_000_000,maxPolygons:200_000,maxDepth:256};
 const observe=():boolean=>{calls++;if(calls===1){options.tick=()=>{replacement++;};options.progress=()=>{replacement++;return true;};options.maxWork=1;options.maxPolygons=1;options.maxDepth=1;}if(calls===target){if(row.outcome==="false")return false;throw row.outcome==="undefined"?undefined:sentinel;}return true;};
 if(row.observer==="tick")options.tick=observe;else options.progress=observe;
 try{run(row.operation,options,left,right);returned=true;}catch(error){caught=true;fault=error;}
 assert.equal(returned,false,"original observer still owns the boundary");assert.equal(caught,true);assert.equal(calls,target);assert.equal(replacement,0);if(row.outcome==="false")assert.match(String(fault),/boolean-cancelled/);else assert.equal(fault,row.outcome==="undefined"?undefined:sentinel);assert.equal(JSON.stringify({left,right}),before);
});
for(const operation of ["difference","union","intersection","halfspace"] as const)test("Boolean options are captured exactly once "+operation,()=>{const reads={maxWork:0,maxPolygons:0,maxDepth:0,tick:0,progress:0},options:SolidBooleanOptions={};Object.defineProperties(options,{maxWork:{get:()=>{reads.maxWork++;return 2_000_000;}},maxPolygons:{get:()=>{reads.maxPolygons++;return 200_000;}},maxDepth:{get:()=>{reads.maxDepth++;return 256;}},tick:{get:()=>{reads.tick++;return ()=>{};}},progress:{get:()=>{reads.progress++;return ()=>true;}}});run(operation,options);assert.deepEqual(reads,{maxWork:1,maxPolygons:1,maxDepth:1,tick:1,progress:1});});

