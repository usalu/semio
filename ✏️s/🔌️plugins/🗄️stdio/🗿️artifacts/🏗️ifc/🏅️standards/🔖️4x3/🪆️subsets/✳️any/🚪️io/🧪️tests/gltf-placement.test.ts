/** 🧭️ Actual official GLB imports satisfy the released product placement proposition. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import Ajv from 'ajv/dist/2020';
import cases from './gltf-placement-cases.json';
import schema from './gltf-placement.schema.json';
import {readIfcGltfGlb,gltfToIfc4x3} from '../gltf.ts';
import {compileIfc4x3Express} from '../../🧬️schema/⚖️validation/compiler.ts';
import {validateIfc4x3Express} from '../../🧬️schema/⚖️validation/population.ts';
test('GLB placement cases have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('official GLB has valid IFC placement '+row.file,async()=>{
 const source=await readIfcGltfGlb(await readFile(new URL(row.file,import.meta.url)),{batchSize:4096}),outcome=await gltfToIfc4x3(source,{batchSize:4096});expect(outcome.artifact).toBeDefined();
 const compilation=await compileIfc4x3Express();expect(compilation.complete).toBe(true);
 const result=await validateIfc4x3Express(outcome.artifact!,{maximumWork:20_000_000,batchSize:4096},compilation.program);
 expect(result.complete).toBe(true);expect(result.valid).toBe(true);expect(result.diagnostics).toEqual([]);
 for(const product of outcome.artifact!.entities.filter(entity=>entity.name==='IFCBUILDINGELEMENTPROXY'))expect(product.args[5]?.kind).toBe('reference');
});
