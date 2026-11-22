/** 🏚️ Tests for the fail-fast of a dependency store that is not attached. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import fixture from "./🔣️.json";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {DanglingDependencyStoreErrorV1,danglingDependencyStoreOfV1,exitIfStaleImplementationV1,formatStampProtocolV1,formatStampTextV1,staleImplementationOfV1} from "../../🟨️.mjs";
import {runWithStaleReexecV1} from "../../🔄️reexec/🟨️.mjs";

const protocol=formatStampProtocolV1(),scratches:string[]=[];
const scratch=()=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"ds-")));scratches.push(path);return path;};
afterAll(()=>{for(const path of scratches)rmSync(path,{recursive:true,force:true});});
const resolveFailure=(message:string)=>Object.assign(new Error(message),{name:"ResolveMessage"});
const link=(target:string,path:string)=>symlinkSync(target,path,"junction");
const dangling=(root:string)=>{
 const store=join(root,".cache","b","d","m"),file=join(root,"framework","a","file.ts");
 mkdirSync(join(root,"framework","a"),{recursive:true});link(store,join(root,"node_modules"));
 return {store,file,message:"Cannot find package '@semio-tech/schema-validator-ts' from '"+file+"'"};
};

test("the package a resolution failure asks for equals the independent oracle on a tree whose node_modules dangles everywhere",()=>{
 const python=testPythonV1();
 const expected=JSON.parse(spawnSync(python!,["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"}).stdout) as (string|null)[];
 expect(expected).toEqual(fixture.messages.map(row=>row.package));
 const everywhereDangling={lstat:()=>({isSymbolicLink:()=>true}),exists:()=>false,readlink:()=>"../store"};
 expect(fixture.messages.map(row=>danglingDependencyStoreOfV1(resolveFailure(row.message),everywhereDangling)?.package??null)).toEqual(expected);
});

test("a dangling node_modules link, a dangling package link and the link of an aggregated cause are found and a healthy tree is not",()=>{
 const root=scratch(),{store,file,message}=dangling(root);
 expect(danglingDependencyStoreOfV1(resolveFailure(message))).toEqual({package:"@semio-tech/schema-validator-ts",link:join(root,"node_modules"),target:store});
 expect(danglingDependencyStoreOfV1(new AggregateError([new Error("x"),resolveFailure(message)],"Several"))?.link).toBe(join(root,"node_modules"));
 const healthy=scratch();mkdirSync(join(healthy,"node_modules","@semio-tech"),{recursive:true});mkdirSync(join(healthy,"f"));
 const healthyFile=join(healthy,"f","file.ts"),text="Cannot find package '@semio-tech/schema-validator-ts' from '"+healthyFile+"'";
 expect(danglingDependencyStoreOfV1(resolveFailure(text))).toBeUndefined();
 link(join(healthy,"gone"),join(healthy,"node_modules","@semio-tech","schema-validator-ts"));
 expect(danglingDependencyStoreOfV1(resolveFailure(text))).toEqual({package:"@semio-tech/schema-validator-ts",link:join(healthy,"node_modules","@semio-tech","schema-validator-ts"),target:join(healthy,"gone")});
 expect(file.startsWith(root)).toBe(true);
});

test("a dangling store is no half-edited module: it is not classified as a restart cause while a missing package behind a healthy tree still is",()=>{
 const root=scratch(),{message}=dangling(root),healthy=scratch();mkdirSync(join(healthy,"node_modules"),{recursive:true});
 expect(staleImplementationOfV1(resolveFailure(message))).toBeUndefined();
 expect(staleImplementationOfV1(resolveFailure("Cannot find package 'x' from '"+join(healthy,"file.ts")+"'"))?.origin).toBe("load");
});

test("the process ends at once with its own exit code and a message that names the dangling store in both languages",()=>{
 const root=scratch(),{store,message}=dangling(root);
 for(const language of["en","de"]){
  const written:string[]=[],exits:number[]=[],slept:number[]=[];
  exitIfStaleImplementationV1(resolveFailure(message),{write:text=>written.push(text),exit:code=>{exits.push(code);},sleepSync:milliseconds=>slept.push(milliseconds),environment:{SEMIO_LANGUAGE:language}});
  expect(exits).toEqual([protocol.danglingStore.exitCode]);expect(exits[0]).not.toBe(protocol.exitCode);expect(slept).toEqual([]);
  expect(written.join("")).toContain(join(root,"node_modules"));expect(written.join("")).toContain(store);expect(written.join("")).toContain("@semio-tech/schema-validator-ts");
  expect(written.join("")).toContain(formatStampTextV1(language,"danglingstore",{package:"@semio-tech/schema-validator-ts",link:join(root,"node_modules"),target:store}));
  expect(written.join("")).not.toContain("edited by another process");
 }
});

test("the outermost command does not restart for a dangling store and raises one localized error that keeps the loader failure as its cause",async()=>{
 const root=scratch(),{store,message}=dangling(root),launches:unknown[]=[],sleeps:number[]=[],failure=resolveFailure(message);
 const error=await runWithStaleReexecV1(async()=>{throw failure;},{environment:{SEMIO_LANGUAGE:"en"},argv:["bun","/repo/script.ts","repo","dependencies","ensure"],execPath:"bun",execArgv:[],outermost:true,sleep:async milliseconds=>{sleeps.push(milliseconds);},launch:async options=>{launches.push(options);return 0;}}).catch(value=>value);
 expect(error).toBeInstanceOf(DanglingDependencyStoreErrorV1);expect(error.cause).toBe(failure);expect(error.target).toBe(store);
 expect(launches).toEqual([]);expect(sleeps).toEqual([]);
 expect(staleImplementationOfV1(error)).toBeUndefined();
 const written:string[]=[],exits:number[]=[];exitIfStaleImplementationV1(error,{write:text=>written.push(text),exit:code=>{exits.push(code);},environment:{SEMIO_LANGUAGE:"en"}});
 expect(exits).toEqual([protocol.danglingStore.exitCode]);
});

test("the message exists in both languages with the same placeholders",()=>{
 const placeholders=(language:string)=>[...String((protocol.texts as Record<string,Record<string,string>>)[language]!.danglingstore).matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 expect(placeholders("en")).toEqual(["link","package","target"]);expect(placeholders("de")).toEqual(placeholders("en"));
});
