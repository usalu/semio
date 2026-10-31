/** 📦️ Pinned Cargo test archives retain the authored versions and exact immutable publisher assets on all six hosts. @see ./🔣️.json */
import{expect,test}from"bun:test";
import{join}from"node:path";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";
import policy from"../../🔣️.json";
import schema from"../../🧬️schema/🔣️.json";
import{NATIVE_TOOL_IDS_V1,nativeToolSpecV1,nativeDistributionV1,nativeToolUrlV1,nativeToolBinaryDirectoryV1,type NativeToolV1,type ToolchainHostV1}from"../../🟦️.ts";
test("the neutral release corpus and closed policy agree with Ajv",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);expect(new Ajv({strict:false}).validate(schema,policy)).toBe(true);});
for(const row of fixture.tools)test(row.id+" owns every checksum-pinned host without a second version source",()=>{
 const tool=row.id as NativeToolV1;expect(NATIVE_TOOL_IDS_V1).toContain(tool);const spec=nativeToolSpecV1(tool);expect(spec.version).toBe(row.version);expect(spec.versionArguments).toEqual(row.versionArguments);expect(spec.distributions.map(row=>row.host)).toEqual(policy.hosts);
 for(const asset of row.distributions){const host=asset.host as ToolchainHostV1,actual=nativeDistributionV1(tool,host);expect({host:actual.host,archive:actual.archive,bytes:actual.bytes,sha256:actual.sha256}).toEqual(asset);expect(actual.prefix).toBe("");expect(actual.binaryDirectory).toBe(".");expect(actual.binaries).toEqual([row.id+(host.startsWith("win32")?".exe":"")]);expect(new URL(nativeToolUrlV1(tool,host)).pathname.split("/").at(-1)).toBe(asset.archive);expect(nativeToolBinaryDirectoryV1(tool,host,"/owned/compiler/tools").replaceAll("\\","/")).toBe(join("/owned/compiler/tools","native",row.id,row.version,host+"-"+asset.sha256.slice(0,16)).replaceAll("\\","/"));}
});

import{findToolV1,TOOL_REGISTRY_V1}from"../../../../🧰️tools/📖️registry/🟦️.ts";
import{readToolPinV1,toolDigestForHostV1}from"../../../../🧰️tools/📌️pin/🟦️.ts";
import{readFileSync,existsSync}from"node:fs";
test("registered Cargo tool pins resolve the same normative archive data",()=>{
 for(const row of fixture.tools){const entry=findToolV1(row.id)!;expect(entry).toBeDefined();const pin=readToolPinV1(entry,process.cwd(),{read:path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}});expect(pin.version).toBe(row.version);expect(pin.digests.length).toBe(policy.hosts.length);
 for(const asset of row.distributions){const [platform,architecture]=asset.host.split("-");const actual=toolDigestForHostV1(pin,platform!,architecture!);expect(actual?.sha256).toBe(asset.sha256);expect(actual?.bytes).toBe(asset.bytes);expect(actual?.name).toBe(asset.archive);}
 expect(entry.locate.some(row=>row.kind==="path")).toBe(false);expect(TOOL_REGISTRY_V1.tools.filter(tool=>tool.commands.includes(row.id)).length).toBe(1);}
});

import{resolvePinV1,toolRowV1}from"../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/🟦️.ts";
test("tool setup reads the same pin and uses verified native provisioning",()=>{
 for(const row of fixture.tools){const entry=toolRowV1(row.id);expect(resolvePinV1(entry,{root:process.cwd(),read:path=>readFileSync(path,"utf8"),resolvePackage:()=>{throw Error("No external package pin");},environment:{SEMIO_LANGUAGE:"en"}})).toBe(row.version);expect(entry.pin).toEqual({format:"json",file:"🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🔣️.json",pointer:"/cargo/"+row.id+"/version"});expect(entry.setup).toEqual({kind:"native",tool:row.id});expect(entry.locate).toEqual([{kind:"provisioned",family:"native",tool:row.id,executable:row.id}]);}
});

test("registered Cargo tools name actual owned adapter and installer Sources",async()=>{
 for(const row of fixture.tools){const entry=findToolV1(row.id)!;for(const source of [entry.adapter,entry.provision.installer!]){expect(existsSync(source),source).toBe(true);expect(await Bun.file(source).exists(),source).toBe(true);}}
});
