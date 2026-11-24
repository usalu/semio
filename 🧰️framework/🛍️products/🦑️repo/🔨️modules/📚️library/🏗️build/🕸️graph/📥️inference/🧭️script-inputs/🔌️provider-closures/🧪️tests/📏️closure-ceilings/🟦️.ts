/** 📏️ Contract: the closure a project record is validated through can only shrink on purpose. Every shared closure group of the providers project (declared tools and provider ports, each through its view) has a size ceiling and appears in the table; a group of the scheduling view holds no file of a scheduling domain, a group of the native view holds none outside the native-keyed modules; and the unrelated subsystems that are still reachable (content store, unit cache) have a ceiling of their own, with the subsystems the scheduling policy already leaves out at zero.
 * @see ../../🔣️.json */
import {expect,setDefaultTimeout,test} from "bun:test";
import {relative,resolve} from "node:path";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

setDefaultTimeout(300000);
const checkout=process.env.SEMIO_TEST_CHECKOUT??resolve(import.meta.dir,"../".repeat(12));
const library=resolve(checkout,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../../👁️observation/🟨️.mjs") as any,scripts=await import("../../../🟨️.mjs") as any;
const scheduling=await import("../../../../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts") as any;
const groups=await observation.observeInferenceSnapshotV1(async()=>{
 const api=scripts.closureApiV1(inference.createScriptInputCache({workspaceRoot:checkout}),checkout);
 return new Map<string,unknown[]>(await api.sharedClosureGroups(relative(checkout,resolve(library,"🏃️process/🔌️providers")),"🔣️.json"));
}) as Map<string,unknown[]>;
const filesOf=(name:string)=>groups.get(name)!.filter((item):item is string=>typeof item==="string"&&item.startsWith("{workspaceRoot}/")).map(item=>item.slice(16));

test("the ceiling table is closed under its schema and names exactly the groups of the providers project",()=>{
 expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);
 expect([...groups.keys()].sort()).toEqual(Object.keys(fixture.groups).sort());
});

test("no group is larger than its ceiling",()=>{
 for(const name of groups.keys())expect(filesOf(name).length,name).toBeLessThanOrEqual((fixture.groups as Record<string,number>)[name]!);
});

test("a group holds no file of a scheduling domain its view leaves out",()=>{
 const everything=scheduling.buildSchedulingPrefixesV1(),native=scheduling.buildSchedulingNativePrefixesV1();
 for(const name of groups.keys()){
  const prefixes=name.endsWith(":native")?native:everything;
  expect(filesOf(name).filter(file=>prefixes.some((prefix:string)=>file.startsWith(prefix))),name).toEqual([]);
 }
});

test("an unrelated subsystem reaches no group beyond its ceiling",()=>{
 for(const[subsystem,{prefix,maximumFilesPerGroup}]of Object.entries(fixture.subsystems))
  for(const name of groups.keys())expect(filesOf(name).filter(file=>file.startsWith(relative(checkout,library)+"/")&&file.includes("/"+prefix)).length,subsystem+" in "+name).toBeLessThanOrEqual(maximumFilesPerGroup);
});
