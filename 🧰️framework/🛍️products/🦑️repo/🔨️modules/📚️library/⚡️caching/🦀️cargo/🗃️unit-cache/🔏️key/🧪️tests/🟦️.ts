/** 🔏️ Contract: the Rust unit cache key reads rustc command lines, decides bypasses, normalizes lanes and externs, selects the environment, derives the key, parses dependency files and build script directives, expands response files and derives the broker facts exactly like an independent Python implementation does for the 438 command lines and the other inputs of `./🔣️.json`; the corpus is closed by its schema. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {admitFactsV1,buildKeyEnvironmentV1,expandRustcArgumentsV1,keyEnvironmentV1,laneNeutralV1,laneRestoreV1,normalizeRustcArgumentsV1,parseBuildDirectivesV1,parseDepInfoV1,parseRustcUnitV1,rustcBypassV1,sha256V1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),policy=UNIT_CACHE_POLICY_V1,posix=process.platform!=="win32";
const ask=(op:string,extra:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({op,policy,laneRoot:corpus.laneRoot,cwd:corpus.cwd,externDigests:corpus.externDigests,...extra}),encoding:"utf8",maxBuffer:256*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const plain=(value:unknown)=>JSON.parse(JSON.stringify(value));
let base="";
beforeAll(()=>{base=realpathSync(mkdtempSync(join(tmpdir(),"unit-key-")));});
afterAll(()=>{rmSync(base,{recursive:true,force:true});});

test("the corpus is closed by its schema and the policy is the authored one",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,keys:[{toolchain:"x"}]})).toBe(false);
 expect(policy.namespace).toBe("semio.cargo.unit-cache/v3");expect(corpus.vectors.length).toBeGreaterThan(400);
});

test("every command line is parsed, bypassed, normalized and admitted like the Python implementation",()=>{
 const answers=ask("commands",{vectors:corpus.vectors});let bypasses=new Set<string>(),cacheable=0,externs=0;
 for(const[index,args]of corpus.vectors.entries()){
  const label=JSON.stringify(args),unit=parseRustcUnitV1(args),expected=answers[index];
  expect({...plain(unit),target:unit.target??null,incremental:unit.incremental??null},label).toEqual(expected.unit);
  const reason=rustcBypassV1(unit,args);expect(reason??null,label).toBe(expected.bypass);if(reason)bypasses.add(reason.split(":")[0]!);else cacheable++;
  expect(normalizeRustcArgumentsV1(args,new Map(Object.entries(corpus.externDigests)),corpus.laneRoot),label).toEqual(expected.normalized);
  if(posix)expect(plain(admitFactsV1(args,{CARGO_PKG_NAME:"p",CARGO_PKG_VERSION:"1",CARGO_PRIMARY_PACKAGE:"1"},corpus.cwd)??null),label).toEqual(expected.admit);
  externs+=unit.externs.length;expect(Object.isFrozen(unit)&&Object.isFrozen(unit.externs),label).toBe(true);
 }
 expect([...bypasses].sort()).toEqual(["crate-type","native-link","no-artifact-messages","no-crate-name","no-emit","no-out-dir","no-source-file","query"]);expect(cacheable).toBeGreaterThan(5);expect(externs).toBeGreaterThan(50);
});

test("lane roots are neutralised in every spelling, longest first, and restored without loss",()=>{
 const answers=ask("lanes",{rows:corpus.lanes});
 for(const[index,row]of corpus.lanes.entries()){
  const neutral=laneNeutralV1(row.text,row.root??undefined),label=JSON.stringify(row);
  expect(neutral,label).toBe(answers[index].neutral);expect(laneRestoreV1(neutral,row.root??undefined),label).toBe(answers[index].restored);
  if(row.root&&!row.text.includes(policy.key.laneToken))expect(neutral.includes(row.root),label).toBe(false);
 }
});

test("the observed environment is selected, sorted and neutralised for compilations and for build scripts",()=>{
 const answers=ask("environments",{rows:corpus.environments});
 for(const[index,environment]of corpus.environments.entries()){
  expect(keyEnvironmentV1(environment,corpus.laneRoot),JSON.stringify(environment)).toEqual(answers[index].key);
  expect(buildKeyEnvironmentV1(environment,corpus.laneRoot),JSON.stringify(environment)).toEqual(answers[index].build);
 }
 expect(buildKeyEnvironmentV1({NUM_JOBS:"8",CARGO_MAKEFLAGS:"j8",MAKEFLAGS:"j8"},undefined)).toEqual([]);
});

test("dependency files and build script directives are read like the Python regular expressions",()=>{
 const deps=ask("depinfo",{rows:corpus.depinfo}),found=ask("directives",{rows:corpus.directives});
 for(const[index,text]of corpus.depinfo.entries())expect(parseDepInfoV1(text),JSON.stringify(text)).toEqual(deps[index]);
 for(const[index,text]of corpus.directives.entries())expect(parseBuildDirectivesV1(text),JSON.stringify(text)).toEqual(found[index]);
 expect(parseDepInfoV1("a.d: x.rs\n").dependencies).toEqual(["x.rs"]);
});

test("the unit key and the content hash equal hashlib over the compact JSON of the key inputs and change with every input",()=>{
 const keys=ask("keys",{rows:corpus.keys}),hashes=ask("hashes",{rows:corpus.hashes});
 for(const[index,row]of corpus.keys.entries())expect(unitKeyV1(row as never),JSON.stringify(row)).toBe(keys[index]);
 for(const[index,text]of corpus.hashes.entries()){expect(sha256V1(text)).toBe(hashes[index]);expect(sha256V1(new TextEncoder().encode(text))).toBe(hashes[index]);}
 const first=corpus.keys[0]!,variants=[{...first,toolchain:first.toolchain+"x"},{...first,normalized:[...first.normalized,"x"]},{...first,sourceSha256:"f".repeat(64)},{...first,externs:[]},{...first,environment:[]}];
 expect(new Set([unitKeyV1(first as never),...variants.map(row=>unitKeyV1(row as never))]).size).toBe(6);
});

test("the output-determinism contract and Cargo's unit identity are key inputs of a compilation, and of nothing else",()=>{
 const unit={toolchain:"t",normalized:["--crate-name","a"],sourceSha256:"a".repeat(64),externs:[],environment:[]},run={...unit,sourceSha256:"run:"+"b".repeat(64)};
 const keyOf=(row:object)=>ask("keys",{rows:[row]})[0] as string;
 expect(unitKeyV1(unit)).toBe(keyOf(unit));
 expect(unitKeyV1({...unit,cargoIdentity:"-3c9971acb8b0a698"})).toBe(keyOf({...unit,cargoIdentity:"-3c9971acb8b0a698"}));
 expect(new Set([unitKeyV1(unit),unitKeyV1({...unit,cargoIdentity:"-3c9971acb8b0a698"}),unitKeyV1({...unit,cargoIdentity:"-4fe346c9fadf2a8a"})]).size).toBe(3);
 expect(unitKeyV1({...run,cargoIdentity:"-3c9971acb8b0a698"})).toBe(unitKeyV1(run));
 expect(unitKeyV1(run)).toBe(sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,run.toolchain,run.normalized,run.sourceSha256,[],[]])));
 expect(unitKeyV1(unit)).not.toBe(sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,unit.toolchain,unit.normalized,unit.sourceSha256,[],[]])));
});

test("the key changes exactly when the version of the output-determinism contract changes, and the version names the rules it stands for",()=>{
 const contract=UNIT_CACHE_POLICY_V1.key.outputDeterminism,unit={toolchain:"t",normalized:["--crate-name","a"],sourceSha256:"a".repeat(64),externs:[],environment:[]};
 const under=(version:string)=>sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,unit.toolchain,unit.normalized,unit.sourceSha256,[],[],["determinism",version]]));
 expect(unitKeyV1(unit)).toBe(under(contract.version));
 expect(under(String(Number(contract.version)+1))).not.toBe(unitKeyV1(unit));
 const pinned:Record<string,string>={"2":"dd945055956faeb7"};
 expect(sha256V1(JSON.stringify(contract.rules)).slice(0,16),"the rules of a contract changed without a new version: raise key.outputDeterminism.version in the schema and pin its rules here").toBe(pinned[contract.version]);
});

test("response files expand to one argument per line, only once and only when they exist",()=>{
 const rows=corpus.responses.map((row,index)=>{const directory=join(base,"r"+index);mkdirSync(directory,{recursive:true});for(const[name,text]of Object.entries(row.files))writeFileSync(join(directory,name),text);return{directory,row};}),answers=ask("responses",{rows:corpus.responses});
 for(const[index,{directory,row}]of rows.entries())expect(expandRustcArgumentsV1(row.args,directory),row.name).toEqual(answers[index]);
});

test.skipIf(!posix)("broker facts follow the authored rules for primary, extra filename, relative paths and probes",()=>{
 const answers=ask("admit",{rows:corpus.admit});
 for(const[index,row]of corpus.admit.entries())expect(plain(admitFactsV1(row.args,row.environment,corpus.cwd)??null),JSON.stringify(row)).toEqual(answers[index]);
 expect(admitFactsV1(["-vV"],{},corpus.cwd)).toBeUndefined();
});
