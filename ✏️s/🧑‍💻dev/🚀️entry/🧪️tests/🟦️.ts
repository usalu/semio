import {test,expect} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {join,dirname} from "node:path";
import {sDevCompositionLawsV1} from "../../🧩️service-composition/🧪️tests/🔄️lifecycle/🟦️.ts";

test("canonical S entry references its actual serialized injected catalog authority",async()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"🧰️framework")))root=dirname(root);
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")),catalog=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json"),"utf8"));
 expect(schema.properties.catalog.$ref).toBe(catalog.$id);
 await sDevCompositionLawsV1();
 console.log("[DEBUG] actual S entry current type/serialized payload/runtime mount and locked Ajv agree");
});
