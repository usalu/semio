import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {cargoCompilerIntermediatePathsV1} from "../../🟦️.ts";
import {cargoBudgetV1,canonicalRootsV1,generatedPathOffendersV1,generatedTreeLongestV1,laneDirectoryV1,readBudgetPackagesV1,sourcePathOffendersV1,trackedFilesV1} from "../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
const budget=schema.definitions.Budget.const,policy=schema.definitions.Policy.const;
const workspace=(()=>{let directory=dirname(fileURLToPath(import.meta.url));while(directory!==dirname(directory)&&!existsSync(join(directory,".cargo","config.toml")))directory=dirname(directory);return directory;})();
const files=trackedFilesV1(workspace),packages=readBudgetPackagesV1(workspace,files);
const describe=(rows:readonly {length:number;root:string;target:string;profile:string;subject:string}[])=>rows.slice(0,64).map(row=>[row.length,row.root,row.target,row.profile,row.subject].join(" "));
test("the canonical roots carry the longest legal account name of each platform",()=>{
 expect(canonicalRootsV1().map(row=>[row.id,row.root.length])).toEqual([["macos",39],["linux",28],["windows",8]]);
 expect(laneDirectoryV1(canonicalRootsV1()[0]!)).toBe("/Users/"+"a".repeat(16)+"/Documents/semio/.cache/b/r/m/u/l/q000000");
});
test("every Cargo package fits the 256 character bound for every output below every canonical root, target and profile",()=>{
 expect(packages.length).toBeGreaterThan(5000);
 expect(describe(cargoBudgetV1(packages).offenders)).toEqual([]);
});
test("every tracked or trackable file fits the 256 character bound below the longest canonical root",()=>{
 expect(files.length).toBeGreaterThan(1000);
 expect(describe(sourcePathOffendersV1(files))).toEqual([]);
});
test("the trees the bootstrap generates fit the bound below the longest canonical root on this machine",()=>{
 const longest=generatedTreeLongestV1(workspace);
 if(longest)expect(describe(generatedPathOffendersV1(longest))).toEqual([]);
});
test("the worst path per root, target and profile equals an independent pathlib computation",()=>{
 const result=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify({budget,policy,packages:packages.map(row=>({...row,zones:[...row.zones]}))}),encoding:"utf8",maxBuffer:1<<28});
 expect(result.status,result.stderr).toBe(0);
 const oracle=JSON.parse(result.stdout) as {root:string;target:string;profile:string;length:number}[];
 expect(oracle.map(row=>[row.root,row.target,row.profile,row.length])).toEqual(cargoBudgetV1(packages).table.map(row=>[row.root,row.target,row.profile,row.length]));
});
test("codegen temporaries leave the unit directory and no longer grow with the package name",()=>{
 const directory=laneDirectoryV1(canonicalRootsV1()[0]!),unit=(name:string)=>({packageName:name,name:name.replaceAll("-","_"),platform:null,profileName:"prototype",kind:"library",operation:"build"}),temporary=(name:string)=>cargoCompilerIntermediatePathsV1(directory,[unit(name)],"posix").filter(path=>path.startsWith(join(directory,policy.temporaryDirectory)+"/")),short=temporary("a".repeat(80)),long=temporary("a".repeat(85));
 expect(short.length).toBe(2);expect(Math.max(...short.map(path=>path.length))).toBe(directory.length+1+policy.temporaryDirectory.length+1+policy.temporaryFileCharacters);
 expect(long.map(path=>path.length)).toEqual(short.map(path=>path.length));
});
