import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import schema from './🔣️.json';
import law from './commit-cases.json';
import {stagedFormStatus} from './🟦️.ts';
test('ready source status carries the typed commit payload admitted by the neutral contract',()=>{
 const admit=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:'#/$defs/Status'});
 for(const row of law.cases){
  const status={token:law.token,ready:row.ready,commitArgs:row.commitArgs,diagnostics:[],diagnosticOffset:'0',diagnosticTotal:'0'};
  expect(admit(status)).toBe(row.allowed);
 }
 const response={token:law.token,ready:true,fields:[],diagnostics:[]};
 expect(()=>stagedFormStatus(response)).toThrow('staging.commit-args');
 expect(stagedFormStatus(response,0,law.commitArgs).commitArgs).toBe(law.commitArgs);
 expect(()=>stagedFormStatus({...response,ready:false},0,law.commitArgs)).toThrow('staging.commit-args');
});

test('neutral diagnostic counters preserve exact u64 wire text and schema bounds',async()=>{
 const law=(await import('./status-counter-cases.json')).default;
 const admit=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:'#/$defs/Status'});
 for(const row of law.cases){
  const diagnostics=Array.from({length:row.count},()=>law.diagnostic);
  const status={token:law.token,ready:false,commitArgs:null,diagnostics,diagnosticOffset:row.offset,diagnosticTotal:row.total};
  expect(admit(status),row.id).toBe('schemaAllowed'in row?row.schemaAllowed:row.allowed);
  expect(JSON.parse(JSON.stringify(status))).toEqual(status);
 }
});
