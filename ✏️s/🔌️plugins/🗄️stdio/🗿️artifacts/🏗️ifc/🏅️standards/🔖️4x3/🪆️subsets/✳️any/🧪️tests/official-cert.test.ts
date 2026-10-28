/** 📚️ Original certification inputs retain exact source custody and independent EXPRESS observations. */
import {test,expect} from 'bun:test';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import Ajv from 'ajv/dist/2020';
import {compare,applyPatch} from 'fast-json-patch';
import manifest from '../🧪️fixtures/official-cert/cases.json';
import schema from '../🧪️fixtures/official-cert/schema.json';
import originals from '../🧪️fixtures/official/cases.json';
import {parseIfc4x3Step,writeIfc4x3Step} from '../🚪️io/step.ts';
import {applyIfcDiffAsync,inverseIfcDiffAsync} from '../🧬️schema/🔺️diff/🟦️.ts';
import {ifcMutationDiffAsync,applyIfcMutationAsync,inverseIfcMutationAsync} from '../🧬️schema/🧬️mutations/🟦️.ts';
import {ifc4x3Catalog} from '../🧬️schema/📚️catalog/🟦️.ts';
import {ifcAttributeQuerySteps} from '../../../../../📚️catalog/🔎️query/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🧬️schema/📸️snapshot/🟦️.ts';
import {compileIfc4x3Express} from '../🧬️schema/⚖️validation/compiler.ts';
import {validateIfc4x3Express} from '../🧬️schema/⚖️validation/population.ts';
const directory=new URL('../🧪️fixtures/official-cert/',import.meta.url),compilation=compileIfc4x3Express(),options={yieldEvery:4096};
const wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,item)=>typeof item==='bigint'?String(item):item));
function applicationVersion(source:IfcSnapshot){const ordinal=source.entities.findIndex(row=>row.name==='IFCAPPLICATION');expect(ordinal).toBeGreaterThanOrEqual(0);const entity=source.entities[ordinal]!,work=ifcAttributeQuerySteps<IfcValue>(ifc4x3Catalog,{entity,ordinal},'VERSION','IFCAPPLICATION');for(;;){const step=work.next();if(!step.done)continue;expect(step.value.status).toBe('unique');expect(step.value.matches).toHaveLength(1);const field=step.value.matches[0]!;expect(field.part).toBe(0);expect(field.value?.kind).toBe('string');return{entity,ordinal,field};}}
test('certification corpus has exact separate source identities and a closed observation contract',()=>{const admit=new Ajv({strict:true,allErrors:true}).compile(schema);expect(admit(manifest)).toBe(true);expect(admit({...manifest,fullReferenceView:true})).toBe(false);expect(manifest.cases).toHaveLength(10);expect(new Set(manifest.cases.map(row=>row.file)).size).toBe(10);expect(manifest.cases.reduce((sum,row)=>sum+row.bytes,0)).toBe(6497046);expect(manifest.cases.reduce((sum,row)=>sum+row.entities,0)).toBe(5366);expect(originals.cases).toHaveLength(46);expect(new Set([...originals.cases,...manifest.cases].map(row=>row.sha256)).size).toBe(56);for(const row of manifest.cases){expect(row.declaredMvd).toEqual(['ViewDefinition [ReferenceView]']);expect(row.diagnostics).toEqual([]);}});
test('actual original license text has exact bytes and explicit attribution',async()=>{const bytes=await readFile(new URL(manifest.license.file,directory));expect(createHash('sha256').update(bytes).digest('hex')).toBe(manifest.license.sha256);expect(bytes.toString('utf8')).toContain('buildingSMART International Ltd.');expect(bytes.toString('utf8')).toContain('Creative Commons Attribution 4.0 International License');});
for(const row of manifest.cases)test('unchanged certification ADD2 '+row.file,async()=>{
 const bytes=await readFile(new URL(row.file,directory));expect(bytes.length).toBe(row.bytes);expect(createHash('sha256').update(bytes).digest('hex')).toBe(row.sha256);expect(createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex')).toBe(row.blob);
 const text=bytes.toString('utf8');for(const header of Object.values(row.headers))expect(text).toContain(header);const source=await parseIfc4x3Step(text,{batchSize:4096});expect(source.entities).toHaveLength(row.entities);expect(source.header.fileSchema).toEqual([{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]);expect(await applyIfcDiffAsync(source,{},options)).toEqual(source);expect(await parseIfc4x3Step(await writeIfc4x3Step(source,{batchSize:4096}),{batchSize:4096})).toEqual(source);
 const selected=applicationVersion(source),mutation={mutation:'setEntityArg'as const,id:selected.entity.id,index:selected.field.index,value:{kind:'string'as const,value:'semio authored reversible fixture edit'}},diff=await ifcMutationDiffAsync(source,mutation,options),edited=await applyIfcDiffAsync(source,diff,options);expect(compare(wire(edited),applyPatch(wire(source),[{op:'replace',path:selected.field.pointer,value:wire(mutation.value)}]).newDocument)).toEqual([]);expect(await applyIfcDiffAsync(edited,await inverseIfcDiffAsync(source,diff,options),options)).toEqual(source);expect(await applyIfcMutationAsync(edited,await inverseIfcMutationAsync(source,mutation,options),options)).toEqual(source);expect(await parseIfc4x3Step(await writeIfc4x3Step(edited,{batchSize:4096}),{batchSize:4096})).toEqual(edited);
 const compiled=await compilation;expect(compiled.complete).toBe(true);const semantic=await validateIfc4x3Express(source,{maximumWork:200_000_000,batchSize:4096},compiled.program);expect(semantic.complete).toBe(true);expect(semantic.valid).toBe(true);expect(semantic.diagnostics).toEqual(row.diagnostics);
},120_000);
