/** 🕳️ Profile holes never silently imply relational solid opening subtraction. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import Ajv from 'ajv/dist/2020';
import cases from './geometry-opening-cases.json';
import schema from './geometry-opening.schema.json';
import {parseIfc4x3Step} from '../step.ts';
import {extractIfc4x3Triangles} from '../geometry.ts';
import {ifc4x3ToGltf} from '../gltf.ts';
test('relational opening cases have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('original relational opening loss '+row.file,async()=>{
 const bytes=await readFile(new URL('../../🧪️fixtures/official/'+row.file,import.meta.url));expect(createHash('sha256').update(bytes).digest('hex')).toBe(row.sha256);
 const source=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),before=structuredClone(source),outcome=await extractIfc4x3Triangles(source,{batchSize:4096}),loss=outcome.diagnostics.filter(d=>d.code===row.diagnostic);
 expect(String(outcome.fidelity)).toBe(row.fidelity);expect(loss.map(d=>d.entityIds)).toEqual(row.relations.map(r=>[BigInt(r.id),BigInt(r.host),BigInt(r.opening)]));
 const exported=await ifc4x3ToGltf(source,{batchSize:4096});expect(exported.diagnostics.filter(d=>d.code===row.diagnostic)).toHaveLength(row.relations.length);expect(source).toEqual(before);
});
