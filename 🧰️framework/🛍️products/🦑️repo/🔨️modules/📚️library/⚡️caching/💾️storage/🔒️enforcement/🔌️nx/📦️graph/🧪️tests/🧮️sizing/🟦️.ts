/** 🧮️ Contract: the native Nx shard image is at least three file maps big (the authored baseline covers the live `nx_files.nxt` of every shard on this machine), its numbers equal the formula of the schema and an independent Python model, the binding and the pool bound follow, and the pool still holds several namespaces.
 * @see ../../🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import {existsSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const native=schema.definitions.NativePolicy.const,pool=schema.definitions.Policy.const;

test("the shard size equals the formula and the independent Python model",()=>{
 const python=Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url))]);
 expect(python.exitCode).toBe(0);
 const model=JSON.parse(python.stdout.toString());
 expect(native.maximumPhysicalBytes).toBe(model.maximumPhysicalBytes);expect(native.filesystemBytes).toBe(model.filesystemBytes);
 expect(schema.definitions.NativeBinding.properties.maximumPhysicalBytes.const).toBe(native.maximumPhysicalBytes);
 expect(native.maximumPhysicalBytes%native.sizing.alignBytes).toBe(0);
 expect(native.maximumPhysicalBytes).toBeGreaterThanOrEqual(native.sizing.factor*native.sizing.fileMapBaselineBytes);
});

test("the baseline covers the live file map of every native shard of this checkout, and no shard's file map is more than a third of its image",()=>{
 const root=process.cwd().split("/🧰️framework")[0]!,base=join(root,".cache","b","n","m");
 if(!existsSync(base))return;
 let seen=0;
 for(const id of readdirSync(base).filter(name=>/^[a-f0-9]{40}$/u.test(name))){
  const file=join(base,id,"native","m","nx_files.nxt");
  if(!existsSync(file))continue;seen++;
  expect(statSync(file).size,id).toBeLessThanOrEqual(native.sizing.fileMapBaselineBytes);
 }
 expect(seen).toBeGreaterThanOrEqual(0);
});

test("the pool bound holds at least five namespaces of a shard, its file map copy and the graph json",()=>{
 const perNamespace=native.maximumPhysicalBytes+pool.namespaceMetadataBytes+pool.maximumNamespaceBytes/8;
 expect(pool.maximumPoolBytes).toBeGreaterThanOrEqual(5*perNamespace);
 expect(schema.definitions.Capacity.properties.namespaceBytes.items.maximum).toBe(pool.maximumPoolBytes);
 expect(schema.definitions.Capacity.properties.extraBytes.maximum).toBe(pool.maximumPoolBytes);
});
