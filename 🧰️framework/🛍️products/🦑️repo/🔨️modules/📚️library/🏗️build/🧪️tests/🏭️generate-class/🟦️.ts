import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";

/** 🏭️ A generate-class target (a task that writes generated files into the tree) is either authored uncacheable as a side effect, or it declares its inputs (an own `inputs` list or an input-precision rule that replaces `default`); a cacheable one never declares a tracked file as an output, because a cache hit would write an old copy over a newer tracked file and the replica would copy it to every machine. The closure is the one of the readiness probe run (`norm-din4108-rs:build`) plus the generate targets of the same projects. */
const root=(()=>{let at=process.env.SEMIO_WORKSPACE_ROOT||import.meta.dirname;while(!existsSync(join(at,"nx.json"))&&dirname(at)!==at)at=dirname(at);return at;})();
const CLOSURE=[
 ["@semio-tech/dsl-derive-rs","🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📋️project.json"],
 ["@semio-tech/framework-schema","🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json"],
 ["@semio-tech/ui-styling-tokens","🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json"],
 ["@semio-tech/framework-rs","🧰️framework/📦️packages/🦀️rust/📋️project.json"],
 ["@semio-tech/ui-contract-rs","🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📋️project.json"],
] as const;
const RULES="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json";
type Target={cache?:boolean;inputs?:unknown[];outputs?:string[];metadata?:{semio?:{cache?:{class?:string;reason?:string}}}};
type Rule={select?:{project?:string;targets?:string[]};replaces?:string[]};
const read=<T>(path:string)=>JSON.parse(readFileSync(join(root,path),"utf8")) as T;
const rules=read<{rules:Rule[]}>(RULES).rules;

export type Row={project:string;target:string;state:"uncacheable"|"declared"|"undeclared"|"tracked-output";why:string};
export function classify():Row[]{
 const rows:Row[]=[];
 for(const [project,file] of CLOSURE){
  const targets=read<{targets:Record<string,Target>}>(file).targets;
  for(const [name,target] of Object.entries(targets).filter(([name])=>name.startsWith("generate")).sort(([left],[right])=>left<right?-1:1)){
   if(target.cache===false){const row=target.metadata?.semio?.cache;rows.push({project,target:name,state:row?.class==="side-effecting"&&row.reason?"uncacheable":"undeclared",why:`${row?.class}/${row?.reason}`});continue;}
   const own=(target.inputs?.length??0)>0,covered=rules.some(rule=>rule.select?.project===project&&rule.select.targets?.includes(name)&&rule.replaces?.includes("default"));
   const outputs=(target.outputs??[]).filter(path=>!path.startsWith("{projectRoot}")&&!path.startsWith("{workspaceRoot}/dist")&&!/\.cache|generated|target/u.test(path));
   rows.push({project,target:name,state:outputs.length>0?"tracked-output":own||covered?"declared":"undeclared",why:own?"own inputs":covered?"precision rule":outputs.length>0?outputs.join(","):"default inputs only"});
  }
 }
 return rows;
}

test("every generate-class target of the closure is uncacheable by declaration or declares its inputs, and none declares a tracked output",()=>{
 const rows=classify();
 expect(rows.length).toBeGreaterThanOrEqual(6);
 const bad=rows.filter(row=>row.state==="undeclared"||row.state==="tracked-output");
 expect(bad.map(row=>`${row.project}:${row.target} ${row.state} (${row.why})`)).toEqual([]);
 expect(rows.filter(row=>row.state==="uncacheable").map(row=>`${row.project}:${row.target}`).sort()).toEqual(["@semio-tech/framework-schema:generate","@semio-tech/ui-styling-tokens:generate"]);
});

const python=Bun.which("python3");
test("an independent Python classification agrees with the rows",()=>{
 if(!python)return;
 const code=`import json,sys
root,rulesPath,closure=sys.argv[1],sys.argv[2],json.loads(sys.argv[3])
rules=json.load(open(root+'/'+rulesPath,encoding='utf8'))['rules']
out=[]
for project,file in closure:
    targets=json.load(open(root+'/'+file,encoding='utf8'))['targets']
    for name in sorted(targets):
        if not name.startswith('generate'): continue
        t=targets[name]
        if t.get('cache') is False: out.append([project,name,'uncacheable']); continue
        own=len(t.get('inputs',[]))>0
        cov=any(r.get('select',{}).get('project')==project and name in r.get('select',{}).get('targets',[]) and 'default' in r.get('replaces',[]) for r in rules)
        out.append([project,name,'declared' if own or cov else 'undeclared'])
print(json.dumps(out,ensure_ascii=False))`;
 const run=Bun.spawnSync([python,"-I","-c",code,root,RULES,JSON.stringify(CLOSURE)],{stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 expect(JSON.parse(run.stdout.toString())).toEqual(classify().map(row=>[row.project,row.target,row.state==="tracked-output"?"declared":row.state]));
});
