/** ♨️ Contract: the warm verdicts attribute every action of a warm Bazel run exactly as the authored cases in `./🔣️.json` and an independent Python derivation say, parse the pretty-printed execution log of the pinned Bazel, and a real two-build experiment against the pinned Bazel names a restored action `hit`, a `no-cache` action `uncached` and an edited input `key-changed`. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1,makeTestTreeWritableV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {bazelWarmVerdictsV1,parseBazelWarmSpawnsV1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const logOf=(rows:readonly unknown[]):string=>rows.map(row=>JSON.stringify(row,null,2)).join("\n");
const ask=(cases:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({cases}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],expected:[{...corpus.cases[0]!.expected[0],verdict:"maybe"}]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],warm:[{...corpus.cases[0]!.warm[0],targetLabel:"t_ab12"}]}]})).toBe(false);
});

test("every case equals the authored verdicts and the Python derivation",()=>{
 const answers=ask(corpus.cases);
 for(const[index,row]of corpus.cases.entries()){
  const actual=bazelWarmVerdictsV1(parseBazelWarmSpawnsV1(logOf(row.cold)),parseBazelWarmSpawnsV1(logOf(row.warm)));
  expect(actual,row.name).toEqual(row.expected as never);
  expect(answers[index],row.name).toEqual(row.expected);
 }
});

test("the execution log parser reads pretty-printed and one-per-line logs and ignores other lines",()=>{
 const row=corpus.cases[0]!.cold[0]!,compact=JSON.stringify(row);
 expect(parseBazelWarmSpawnsV1("INFO: noise\n"+compact+"\nmore noise\n"+compact+"\n")).toHaveLength(2);
 expect(parseBazelWarmSpawnsV1(logOf([row]))[0]).toEqual({label:"//:t_ab12",mnemonic:"RepoGenerate",runner:"local",cacheHit:false,cacheable:true,args:["bun","runner","work-action","@p"],env:{PATH:"/bin"},inputs:{"a/1/source.ts":"h1","implementation.json":"i1","t_ab12.input":"f1","t_ab12.json":"m1"},outputs:{"t_ab12.snapshot/snapshot.json":"o1"}});
 expect(parseBazelWarmSpawnsV1("")).toEqual([]);
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,real=binary?test:test.skip;
let directory="";
beforeAll(()=>{directory=realpathSync(mkdtempSync(join(tmpdir(),"bzl-warm-")));});
afterAll(()=>{rmSync(directory,{recursive:true,force:true});});

real("the pinned Bazel agrees: a restored action is a hit, a `no-cache` action is uncached and an edited input moves the key",()=>{
 const workspace=join(directory,"w"),cache=join(directory,"disk");
 mkdirSync(workspace);mkdirSync(cache);
 const build=(round:string)=>{
  const output=join(directory,"ob-"+round),log=join(directory,round+".execlog.jsonl"),startup=[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=10","--host_jvm_args=-Xmx512m"];
  const run=spawnSync(binary!,[...startup,"build","--lockfile_mode=off","--incompatible_autoload_externally=","--spawn_strategy=local",`--disk_cache=${cache}`,`--execution_log_json_file=${log}`,"//:t_cached","//:t_uncached"],{cwd:workspace,encoding:"utf8"});
  spawnSync(binary!,[...startup,"shutdown"],{cwd:workspace});
  expect(run.status,run.stderr).toBe(0);
  return parseBazelWarmSpawnsV1(readFileSync(log,"utf8"));
 };
 try{
  writeFileSync(join(workspace,"MODULE.bazel"),'module(name = "warm", version = "0.0.0")\n');
  writeFileSync(join(workspace,"rules.bzl"),`def _task(ctx):
    out = ctx.actions.declare_file(ctx.label.name + ".out")
    ctx.actions.run_shell(inputs = ctx.files.srcs, outputs = [out], command = "cat $@ > " + out.path, arguments = [f.path for f in ctx.files.srcs], execution_requirements = {"no-cache": "1"} if ctx.attr.uncached else {})
    return [DefaultInfo(files = depset([out]))]
repo_task = rule(implementation = _task, attrs = {"srcs": attr.label_list(allow_files = True), "uncached": attr.bool(default = False)})
`);
  writeFileSync(join(workspace,"a.txt"),"hello\n");
  writeFileSync(join(workspace,"BUILD.bazel"),'load(":rules.bzl", "repo_task")\nexports_files(["a.txt"])\nrepo_task(name = "t_cached", srcs = ["a.txt"])\nrepo_task(name = "t_uncached", srcs = ["a.txt"], uncached = True)\n');
  const cold=build("cold"),same=build("same");
  expect(cold.map(row=>row.cacheHit)).toEqual([false,false]);
  const unchanged=Object.fromEntries(bazelWarmVerdictsV1(cold,same).map(row=>[row.label,row.verdict]));
  expect(unchanged).toEqual({"//:t_cached":"hit","//:t_uncached":"uncached"});
  writeFileSync(join(workspace,"a.txt"),"edited\n");
  const edited=Object.fromEntries(bazelWarmVerdictsV1(same,build("edited")).map(row=>[row.label,row]));
  expect(edited["//:t_cached"]!.verdict).toBe("key-changed");expect(edited["//:t_cached"]!.changedInputs).toEqual(["a.txt"]);expect(edited["//:t_cached"]!.outputsIdentical).toBe(false);expect(edited["//:t_uncached"]!.verdict).toBe("uncached");
 }finally{makeTestTreeWritableV1(directory);}
},600000);
