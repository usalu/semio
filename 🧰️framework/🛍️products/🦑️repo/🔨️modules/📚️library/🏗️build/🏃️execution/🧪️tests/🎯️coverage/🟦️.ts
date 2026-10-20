/** 🎯️ Contract: every task has exactly one tool family and one disposition, agreed by the neutral classifier, an independent Python rule evaluator and a SQLite census. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import coverageSchema from "../../🧬️schema/🎯️coverage.json";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema.json";
import {taskCoverageReportV1,taskCoverageV1,taskDispositionV1,taskFamilyV1,type TaskCoverageInputV1} from "../../🟦️.ts";

const python=testPythonV1(),definitions=coverageSchema.definitions,rows=fixture.rows as readonly {name:string;input:TaskCoverageInputV1;expected:{family:string;disposition:string;reason?:string}}[];
const ajv=()=>new Ajv({strict:true}).addSchema(coverageSchema,"coverage");

test("coverage fixture is closed and every row, input and report is admitted by the neutral schema",()=>{
 const validate=ajv().compile(fixtureSchema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...fixture,extra:1})).toBe(false);
 expect(validate({...fixture,rows:[{...fixture.rows[0],input:{...fixture.rows[0]!.input,cache:{cache:true,reason:"declared"}}}]})).toBe(false);
 expect(validate({...fixture,rows:[{...fixture.rows[0],expected:{family:"rust",disposition:"action"}}]})).toBe(false);
 expect(new Set(rows.map(row=>row.name)).size).toBe(rows.length);
});

test("family rules are ordered, unique, total and every family and disposition is reachable",()=>{
 const rules=definitions.Rules.const,validate=ajv().compile({$ref:"coverage#/definitions/Rule"});
 expect(rules.length).toBeGreaterThan(10);expect(new Set(rules.map(rule=>rule.id)).size).toBe(rules.length);
 for(const rule of rules)expect(validate(rule),rule.id).toBe(true);
 expect(rules.at(-1)).toEqual({id:"bun-script",family:"bun-script"});expect(rules.filter(rule=>Object.keys(rule).length===2).map(rule=>rule.id)).toEqual(["bun-script"]);
 expect(new Set(rules.map(rule=>rule.family))).toEqual(new Set(definitions.Family.enum));
 expect(new Set(rows.map(row=>row.expected.family))).toEqual(new Set(definitions.Family.enum));
 expect(new Set(rows.map(row=>row.expected.disposition))).toEqual(new Set(definitions.Disposition.enum));
 expect(new Set(rows.flatMap(row=>row.expected.reason??[]))).toEqual(new Set(definitions.Reason.enum.filter(reason=>reason!=="aggregate")));
 expect(taskFamilyV1({target:"anything",root:"",kind:"command",cache:{cache:true}})).toBe("bun-script");
});

test("the classifier reproduces every authored fixture row",()=>{
 for(const row of rows){
  const actual=taskCoverageV1(row.input);
  expect(actual,row.name).toEqual(row.expected);expect(taskFamilyV1(row.input),row.name).toBe(row.expected.family);expect(taskDispositionV1(row.input).disposition,row.name).toBe(row.expected.disposition);
 }
});

test.skipIf(!python)("an independent Python rule evaluator agrees on every family and disposition",()=>{
 const program=[
  "import json,re,sys",
  "rules,policy,rows=json.loads(sys.stdin.read())",
  "def fam(i):",
  " for r in rules:",
  "  if 'noop' in r and i['kind']!='noop': continue",
  "  if 'target' in r and not re.search(r['target'],i['target']): continue",
  "  if 'root' in r and not re.search(r['root'],i['root']): continue",
  "  if 'command' in r and not ('command' in i and re.search(r['command'],i['command'])): continue",
  "  if 'script' in r and not ('script' in i and re.search(r['script'],i['script'])): continue",
  "  return r['family']",
  "def disp(i):",
  " if i['kind']=='noop': return ['action',None]",
  " c=i['cache']",
  " if c['cache']: return ['test' if re.search(policy['test']['target'],i['target']) and i.get('outputs')==[] else 'action',None]",
  " r=c['reason']",
  " return ['interactive' if r in policy['interactive'] else 'refused' if r in policy['refused'] else 'uncached',r]",
  "out=[]",
  "for i in rows:",
  " d,r=disp(i)",
  " x={'family':fam(i),'disposition':d}",
  " if r: x['reason']=r",
  " out.append(x)",
  "print(json.dumps(out,ensure_ascii=False))",
 ].join("\n");
 const result=Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify([definitions.Rules.const,definitions.Policy.const,rows.map(row=>row.input)])),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 expect(JSON.parse(result.stdout.toString())).toEqual(rows.map(row=>row.expected));
});

test("coverage reports agree with a SQLite census and the authored counts",()=>{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE tasks(family TEXT,disposition TEXT)");
  for(const report of fixture.reports){
   database.exec("DELETE FROM tasks");for(const row of report.rows)database.query("INSERT INTO tasks VALUES(?,?)").run(row.family,row.disposition);
   const actual=taskCoverageReportV1(report.rows as never);expect(actual,report.name).toEqual(report.expected as never);
   const byDisposition=database.query("SELECT disposition,COUNT(*) AS n FROM tasks GROUP BY disposition ORDER BY disposition").all() as {disposition:string;n:number}[],byFamily=database.query("SELECT family,disposition,COUNT(*) AS n FROM tasks GROUP BY family,disposition ORDER BY family,disposition").all() as {family:string;disposition:string;n:number}[];
   expect(actual.dispositions).toEqual(Object.fromEntries(byDisposition.map(row=>[row.disposition,row.n])));
   const families:Record<string,Record<string,number>>={};for(const row of byFamily)(families[row.family]??={})[row.disposition]=row.n;expect(actual.families).toEqual(families);
   expect(ajv().compile({$ref:"coverage#/definitions/Report"})(actual)).toBe(true);
  }
 }finally{database.close();}
 expect(taskCoverageReportV1([])).toEqual({version:1,tasks:0,dispositions:{},families:{}});
});
