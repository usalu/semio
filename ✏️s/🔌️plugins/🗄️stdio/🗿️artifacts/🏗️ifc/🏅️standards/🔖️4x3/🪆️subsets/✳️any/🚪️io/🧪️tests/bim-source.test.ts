/** 🫳️ BIM export owns original external data before interpreting any authored field. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './bim-source-cases.json';
import schema from './bim-source.schema.json';
import {ifcBimAxisFixture} from './bim-axis-fixtures.ts';
import {bimSpatialToIfc4x3} from '../bim.ts';
import type {IfcIoControl} from '../control.ts';
const comparable=(value:any)=>{const result=structuredClone(value);if(result.artifact)result.artifact.header.fileName[1]={kind:'string',value:'generated timestamp'};return result;};
test('external BIM source cases have a closed schema',()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
for(const phase of cases.semanticPhases)test('BIM source remains private at '+phase,async()=>{
 const source=ifcBimAxisFixture(.3),expected=comparable(await bimSpatialToIfc4x3(source));expect(expected.artifact).toBeDefined();let changed=false;
 const result=await bimSpatialToIfc4x3(source,{progress:progress=>{if(progress.phase===phase&&!changed){changed=true;source.project.name='changed';source.sites.site.latitude=0;source.buildings.building.origin.x=99;}}});
 expect(changed).toBe(true);expect(comparable(result)).toEqual(expected);
});
for(const variant of cases.accessors)test('BIM capture never invokes '+variant+' accessors',async()=>{
 const source=ifcBimAxisFixture(.3);let owner:object=source,key='project',reads=0;
 if(variant==='name'){owner=source.project;key='name';}
 if(variant==='latitude'){owner=source.sites.site;key='latitude';}
 if(variant==='origin'){owner=source.buildings.building.origin;key='x';}
 if(variant==='constructor'){owner=source.project;key='constructor';}
 if(variant==='array-constructor'){owner=source.project.phase_names;key='constructor';}
 const value=Object.getOwnPropertyDescriptor(owner,key)?.value;Object.defineProperty(owner,key,{get:()=>{reads++;return value;},enumerable:true,configurable:true});
 let result;try{result=await bimSpatialToIfc4x3(source);}catch{}
 expect(reads).toBe(0);expect(result?.artifact).toBeUndefined();expect(result?.diagnostics[0]?.code).toBe(cases.sourceRefusal);
});
test('BIM external limits refuse before semantic work',async()=>{
 for(const limit of cases.limits){let semantic=false;const result=await bimSpatialToIfc4x3(ifcBimAxisFixture(.3),{...limit,progress:progress=>{if(cases.semanticPhases.includes(progress.phase))semantic=true;}});expect(result.artifact).toBeUndefined();expect(result.diagnostics[0]?.code).toBe(cases.sourceRefusal);expect(semantic).toBe(false);}
});
for(const phase of cases.capturePhases)test('BIM '+phase+' retains original cancellation',async()=>{
 const source=ifcBimAxisFixture(.3),before=structuredClone(source),original=new AbortController(),replacement=new AbortController();let reached=false,semantic=false,replacements=0;
 const control:IfcIoControl={signal:original.signal,batchSize:1,progress:progress=>{if(cases.semanticPhases.includes(progress.phase))semantic=true;if(progress.phase===phase&&progress.completed>=2){reached=true;control.signal=replacement.signal;control.progress=()=>replacements++;original.abort();}}};
 await expect(bimSpatialToIfc4x3(source,control)).rejects.toHaveProperty('code',cases.cancellation);expect(reached).toBe(true);expect(semantic).toBe(false);expect(replacements).toBe(0);expect(source).toEqual(before);
});

