import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import {Database} from 'bun:sqlite';
import Ajv from 'ajv/dist/2020.js';
import {applyPatch,type Operation} from 'fast-json-patch';
/** 🧪️ Independent RFC6902 validates every portable original-slot payload exchange. */
export function verifyOriginalSlot():void{
 const corpus=JSON.parse(readFileSync(new URL('🧫️fixtures/🔣️.json',import.meta.url),'utf8')) as {policy:{maximumItems:number,combinedBodyCapacity:number,separateRelease:boolean,originalIssuer:boolean,foreignKeyRefused:boolean,returnBeforeMutation:boolean,stepsPerPayloadByte:number,fixedSteps:number},workCases:{payloadBytes:number,expectedSteps:number}[],cases:{name:string,original:Record<string,unknown>,replacement:Record<string,unknown>,patch:Operation[],expected:Record<string,unknown>,withdrawn:Record<string,unknown>,vacant:boolean}[]};
 const schema=JSON.parse(readFileSync(new URL('../🧬️schema/🔣️.json',import.meta.url),'utf8'));const valid=new Ajv({strict:true}).compile(schema);assert(valid(corpus),JSON.stringify(valid.errors));
 for(const row of corpus.cases){assert.deepEqual(applyPatch(structuredClone(row.original),row.patch,true,false).newDocument,row.expected,row.name);assert.deepEqual(row.replacement,row.expected,row.name);}
 const database=new Database(':memory:');try{database.exec('CREATE TABLE resident(payload TEXT)');for(const row of corpus.cases){database.query('INSERT INTO resident VALUES(?)').run(JSON.stringify(row.original));const original=database.query('DELETE FROM resident RETURNING payload').get()as{payload:string};assert.deepEqual(JSON.parse(original.payload),row.withdrawn);assert.equal((database.query('SELECT COUNT(*) n FROM resident').get()as{n:number}).n===0,row.vacant);database.query('INSERT INTO resident VALUES(?)').run(JSON.stringify(row.replacement));assert.deepEqual(JSON.parse((database.query('SELECT payload FROM resident').get()as{payload:string}).payload),row.expected);database.exec('DELETE FROM resident');}for(const row of corpus.workCases){const own=(row.payloadBytes+1)*corpus.policy.stepsPerPayloadByte+corpus.policy.fixedSteps;assert(Number.isSafeInteger(own));const oracle=database.query('SELECT (?1+1)*?2+?3 steps').get(row.payloadBytes,corpus.policy.stepsPerPayloadByte,corpus.policy.fixedSteps)as{steps:number};assert.equal(own,row.expectedSteps);assert.equal(oracle.steps,row.expectedSteps);}}finally{database.close();}
 console.log('[DEBUG] Original-slot independent RFC6902/Ajv outputs '+corpus.cases.length+' and finite fixture-work SQLite cases '+corpus.workCases.length+' passed');
}
