import {expect,test} from "bun:test";
import {parse,Kind,type TypeNode} from "graphql";
import protobuf from "protobufjs";
import fixture from "./🔣️.json";
import {fileURLToPath} from "node:url";
const source=(path:string)=>Bun.file(new URL(path,import.meta.url)).text();
const named=(node:TypeNode):string=>node.kind===Kind.NAMED_TYPE?node.name.value:named(node.type);
test("GraphQL independently resolves typed TIFF owned mutation fields",async()=>{
 for(const[path,name,fields,payload,type]of [["../../🧬️mutations/📥️insert-ifd/🔗️.graphql","InsertIfdMutation",fixture.insertIfd,"ifd","TiffIfdInput"],["../../🧬️mutations/🏷️replace-tag/🔗️.graphql","ReplaceTagMutation",fixture.replaceTag,"values","TiffValuesInput"]] as const){const ast=parse(await source(path));const definition=ast.definitions.find(item=>item.kind===Kind.INPUT_OBJECT_TYPE_DEFINITION&&item.name.value===name);expect(definition?.kind).toBe(Kind.INPUT_OBJECT_TYPE_DEFINITION);if(definition?.kind===Kind.INPUT_OBJECT_TYPE_DEFINITION){expect(definition.fields?.map(field=>field.name.value)).toEqual(fields);const field=definition.fields?.find(field=>field.name.value===payload);expect(field&&named(field.type)).toBe(type);}}
 console.info("[DEBUG] independent GraphQL parser witnesses typed TIFF mutation payloads");
});
test("Protocol Buffers independently resolves owned TIFF page and scalar payload messages",()=>{
 const root=protobuf.loadSync(fileURLToPath(new URL("../../🧬️mutations/🛰️.proto",import.meta.url)));root.resolveAll();const insert=root.lookupType("stdio.tiff.mutation.InsertIfdMutation");const replace=root.lookupType("stdio.tiff.mutation.ReplaceTagMutation");expect(Object.keys(insert.fields)).toEqual(fixture.insertIfd);expect(Object.keys(replace.fields)).toEqual(fixture.replaceTag);expect(insert.fields.ifd.resolvedType?.fullName).toBe(".semio.s_stdio_tiff.snapshot.TiffIfd");expect(replace.fields.values.resolvedType?.fullName).toBe(".semio.s_stdio_tiff.snapshot.TiffValues");expect(Object.keys(root.lookupType("semio.s_stdio_tiff.snapshot.TiffIfd").fields)).toEqual(fixture.ifd);expect(Object.keys(root.lookupType("semio.s_stdio_tiff.snapshot.TiffValues").fields)).toEqual(fixture.values);
 console.info("[DEBUG] independent Protocol Buffers resolver witnesses TIFF exact owned payloads");
});
