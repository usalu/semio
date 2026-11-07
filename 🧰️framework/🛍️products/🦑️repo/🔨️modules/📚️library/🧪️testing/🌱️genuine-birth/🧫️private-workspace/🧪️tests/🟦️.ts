import {Database} from "bun:sqlite";
import {readPrivateNativeFundingV1} from "../📏️limits/🟦️.ts";
import {buildOrchestrationContextV1,projectBuildOrchestrationCapabilitiesV1} from "../../../../🏗️build/📥️caller/📤️orchestration/🟨️.mjs";
import {resolve} from "node:path";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../🏗️build/📥️caller/🟦️.ts";
import {createNativeNxCallerCapabilitiesV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import nxSchema from "../../../../⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json" with {type:"json"};
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "../../../../🏗️build/🧬️schema/📥️caller.json" with {type:"json"};
import privateSchema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🧪️ The portable caller boundary requires an explicit bounded immutable code root. */
test("native code identity is mandatory and independently schema checked",()=>{
 const native={artifactDirectory:"/workspace/.r/i/m",transport:{maximumBytes:1048576,maximumLines:1024},child:{maximumElapsedMilliseconds:60000},network:{offline:true}},base={tool:"bazel",workspaceRoot:"/workspace",codeRoot:"/code",arguments:["repo","show","projects"],selection:["repo","show","projects"],launches:[["repo","show","projects"]],buildMode:"prototype",native},validate=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.cases){const value:Record<string,unknown>={...base,...row.patch};if(row.remove)delete value.codeRoot;expect(validateJsonSchemaSubset(schema,value).length===0,row.name).toBe(row.accepted);expect(validate(value),row.name).toBe(row.accepted);}
});

/** 🎯️ Only an actual fixed run selection authorizes a private Native test purpose. */
test("private Native purpose rejects incidental target tokens",()=>{const validate=new Ajv({strict:true}).compile(privateSchema.definitions.ParentSelectionHeader);for(const row of fixture.selections){const header=row.header.slice(0,3);expect(validateJsonSchemaSubset(privateSchema.definitions.ParentSelectionHeader,header).length===0,row.name).toBe(row.accepted);expect(validate(header),row.name).toBe(row.accepted);}});

/** 🏭️ Actual configured factories publish the mandatory code owner before any birth. */
test("fresh canonical build and Nx factory schema matches the actual checkout",()=>{const root=resolve(process.cwd()),build=createNativeBuildCallerCapabilitiesV1(root,["show","projects"],"bazel",{}),nx=createNativeNxCallerCapabilitiesV1(root,["nx","run","fixture:test"],["run","fixture:test"],{});for(const [value,contract]of [[build,schema],[nx,nxSchema]]as const){const issues=validateJsonSchemaSubset(contract,value);expect(issues).toEqual([]);expect(new Ajv({strict:true}).validate(contract,value)).toBe(true);const actual=readNativeCallerAuthorityV1(value);expect(actual.codeRoot).toBe(root);expect(actual.repositoryRoot).toBe(root);}});

/** 🧭️ Neutral task projection preserves distinct code identity without issuing an Original. */
test("orchestration wire carries the caller code identity unchanged",()=>{const root=resolve(process.cwd()),base=createNativeBuildCallerCapabilitiesV1(root,["run","parent:build"],"bazel",{}),caller={...base,codeRoot:resolve(root,"immutable-code")},projects={parent:{targets:{build:{metadata:{semio:{execution:{version:1,kind:"orchestration",children:[{project:"child",target:"build"}]}}}}}},child:{targets:{build:{}}}},context=buildOrchestrationContextV1(projects,{project:"parent",target:"build"}),now=Date.now(),wire={version:1,policy:{version:1,owner:"fixture:code-root-projection",maximumElapsedMilliseconds:1000},capabilities:caller,deadlineEpochMilliseconds:now+1000},projected=projectBuildOrchestrationCapabilitiesV1(wire,root,"bazel",context,now);expect(projected.codeRoot).toBe(caller.codeRoot);expect(projected.workspaceRoot).toBe(root);expect(projected.native).toBe(caller.native);expect(validateJsonSchemaSubset(schema,projected)).toEqual([]);expect(new Ajv({strict:true}).validate(schema,projected)).toBe(true);});

/** 🧮️ A finite native grant refines an unlimited parent without extending residual time or capture custody. */
test("private native funding follows portable cases and independent SQLite minima",()=>{const db=new Database(":memory:");try{for(const row of fixture.funding){if(row.expected===null){expect(()=>readPrivateNativeFundingV1(row.value),row.name).toThrow();continue;}const value=readPrivateNativeFundingV1(row.value),oracle=db.query("SELECT CAST(min(coalesce(?1,?2),?2) AS INTEGER) AS maximumMilliseconds,min(?3,16777216) AS maximumBytes,min(?4,65536) AS maximumLines").get(row.value.remainingMilliseconds,row.value.maximumChildMilliseconds,row.value.maximumBytes,row.value.maximumLines);expect(value,row.name).toEqual(row.expected);expect(oracle,row.name).toEqual(value);}}finally{db.close();}});

/** 📄️ Closed receiver records reject unbounded or aliased metadata before body effects. */
test("private Native receiver records preserve bounded actual file custody",async()=>{const {mkdtemp,writeFile,symlink,link,rm,realpath}=await import("node:fs/promises"),{tmpdir}=await import("node:os"),{join}=await import("node:path"),{readPrivateNativeRecordsV1}=await import("../📄️records/🟦️.ts"),validate=new Ajv({strict:true}).compile(privateSchema.definitions.RecordCase);for(const row of fixture.records){expect(validate(row),row.name).toBe(true);expect(validateJsonSchemaSubset(privateSchema.definitions.RecordCase,row),row.name).toEqual([]);const root=await realpath(await mkdtemp(join(tmpdir(),"pnr-")));try{const metadata=join(root,".native-test-birth.json"),owner=join(root,".owner"),source=join(root,"source");await writeFile(owner,"1".repeat(row.ownerBytes));if(row.kind==="regular")await writeFile(metadata,"x".repeat(row.metadataBytes));else{await writeFile(source,"x".repeat(row.metadataBytes));try{if(row.kind==="symlink")await symlink(source,metadata);else await link(source,metadata);}catch(error){if(process.platform==="win32"&&["EPERM","ENOTSUP","EACCES"].includes((error as NodeJS.ErrnoException).code??""))continue;throw error;}}if(row.accepted){const actual=readPrivateNativeRecordsV1(root);expect(new Uint8Array(actual.metadata)).toEqual(new Uint8Array(await Bun.file(metadata).arrayBuffer()));expect(new Uint8Array(actual.owner)).toEqual(new Uint8Array(await Bun.file(owner).arrayBuffer()));}else expect(()=>readPrivateNativeRecordsV1(root),row.name).toThrow();}finally{await rm(root,{recursive:true,force:true});}}});
