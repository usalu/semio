import {afterEach,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {dependencyClosureV1} from "../../🌳️inputs/🟦️.ts";

const scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const oracle=join(import.meta.dir,"🐍️.py");

function checkout():string{const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"closure-"));scratch.push(root);return root;}
function artifact(directory:string,name:string,hash:string,body:string,dependencies?:readonly (readonly [string,string])[],svh?:string):void{
 mkdirSync(directory,{recursive:true});
 for(const kind of ["rlib","rmeta"])writeFileSync(join(directory,`lib${name}-${hash}.${kind}`),kind==="rmeta"?body:"rlib "+body);
 if(dependencies!==undefined)writeFileSync(join(directory,`lib${name}-${hash}.rmeta.semio-deps`),JSON.stringify({version:1,svh,dependencies}));
}
const hex=(character:string):string=>character.repeat(16),svhOf=(character:string):string=>character.repeat(32);
function oracleOf(directories:readonly string[],direct:readonly string[],rlibs:boolean):Record<string,string[]>{
 const run=spawnSync(testPythonV1()??"python3",["-I",oracle],{input:JSON.stringify({directories,direct,rlibs}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 return JSON.parse(run.stdout);
}
function ours(directories:readonly string[],direct:readonly string[],rlibs:boolean):Record<string,string[]>{
 return Object.fromEntries([...dependencyClosureV1(directories,direct,undefined,rlibs)!].map(([directory,files])=>[directory,[...files]]));
}

test("A -> B -> C where the command of A names only B: the shipped set holds C too, by the dependency edges recorded beside B (no --extern of C, found through -L dependency= only)",()=>{
 const base=checkout(),a=join(base,"a"),b=join(base,"b"),c=join(base,"c");
 artifact(c,"c",hex("c"),"c body",[["core-"+hex("0"),svhOf("0")]],svhOf("c"));
 artifact(b,"b",hex("b"),"b body",[["c-"+hex("c"),svhOf("c")]],svhOf("b"));
 artifact(a,"a",hex("a"),"a body",[["b-"+hex("b"),svhOf("b")],["c-"+hex("c"),svhOf("c")]],svhOf("a"));
 const direct=[join(b,`libb-${hex("b")}.rlib`)],directories=[a,b,c],found=ours(directories,direct,true);
 expect(found[c]).toEqual([`libc-${hex("c")}.rlib`,`libc-${hex("c")}.rmeta`]);expect(found[b]).toEqual([`libb-${hex("b")}.rlib`,`libb-${hex("b")}.rmeta`]);expect(found[a]).toBeUndefined();
 expect(found).toEqual(oracleOf(directories,direct,true));
});

test("the dependency was rebuilt under another extra-filename with the same svh (a restored unit keeps the name of the build it came from): it is found by name and svh, and a different svh of the same name is left behind",()=>{
 const base=checkout(),b=join(base,"b"),c=join(base,"c");
 artifact(b,"b",hex("b"),"b body",[["guillotiere-"+hex("9"),svhOf("7")]],svhOf("b"));
 artifact(c,"guillotiere",hex("6"),"same crate, other extra-filename",[],svhOf("7"));
 artifact(c,"guillotiere",hex("5"),"other version",[],svhOf("8"));
 const direct=[join(b,`libb-${hex("b")}.rlib`)],directories=[b,c],found=ours(directories,direct,true);
 expect(found[c]).toEqual([`libguillotiere-${hex("6")}.rlib`,`libguillotiere-${hex("6")}.rmeta`]);
 expect(found).toEqual(oracleOf(directories,direct,true));
 const library=ours(directories,[join(b,`libb-${hex("b")}.rmeta`)],false);
 expect(library[c]).toEqual([`libguillotiere-${hex("6")}.rmeta`]);expect(library).toEqual(oracleOf(directories,[join(b,`libb-${hex("b")}.rmeta`)],false));
});

test("an artifact without a record keeps the extra-filenames of its metadata as the edges, and a dependency of a recorded artifact without a record of its own is taken by name",()=>{
 const base=checkout(),a=join(base,"a"),b=join(base,"b"),c=join(base,"c");
 artifact(a,"a",hex("a"),`metadata names ${hex("b")}`);
 artifact(b,"b",hex("b"),"b body");
 artifact(c,"d",hex("d"),"d body without a record");
 artifact(a,"e",hex("e"),"e body",[["d-"+hex("1"),svhOf("1")]],svhOf("e"));
 const directories=[a,b,c],direct=[join(a,`liba-${hex("a")}.rlib`),join(a,`libe-${hex("e")}.rlib`)],found=ours(directories,direct,true);
 expect(found[b]).toEqual([`libb-${hex("b")}.rlib`,`libb-${hex("b")}.rmeta`]);expect(found[c]).toEqual([`libd-${hex("d")}.rlib`,`libd-${hex("d")}.rmeta`]);
 expect(found).toEqual(oracleOf(directories,direct,true));
});

const lane="/Users/ueli/.sinc/play/.cache/b/r/m/u/l/p9a4254/wasm32-unknown-unknown/release/build";
test.skipIf(!existsSync(lane)||!existsSync(join(lane,"semio-hub-puzzle")))("the real semio_hub_puzzle of the play lane: guillotiere sits there under another extra-filename than the one semio_hub_puzzle recorded, and the closure of the unit that links it ships it",()=>{
 const directories=readdirSync(lane).flatMap(crate=>readdirSync(join(lane,crate)).map(unit=>join(lane,crate,unit,"out"))).filter(directory=>existsSync(directory));
 const puzzle=directories.find(directory=>basename(dirname(dirname(directory)))==="semio-hub-puzzle")!,rlib=readdirSync(puzzle).find(name=>name.endsWith(".rlib"))!;
 const found=ours(directories,[join(puzzle,rlib)],true),all=Object.values(found).flat();
 expect(all.some(name=>/^libguillotiere-[0-9a-f]{16}\.rlib$/u.test(name))).toBe(true);
 expect(all.some(name=>name.startsWith("libsemio_hub_puzzle-")&&name.endsWith(".rlib"))).toBe(true);
});
