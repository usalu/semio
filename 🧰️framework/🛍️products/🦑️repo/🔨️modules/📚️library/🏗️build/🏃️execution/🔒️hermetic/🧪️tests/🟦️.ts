/** 🔒️ Contract: the hermetic class table is closed, selects targets exactly as an independent Python implementation does, admits only declared host inputs, declares a spawn strategy per host and selects the installed dependency closure of a generation layout like an independent Python walk. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔒️hermetic-classes.json";
import {HERMETIC_TABLE_V1,hermeticGlobV1,hermeticInputsV1,hermeticSelectV1,hermeticStrategiesV1} from "../🟦️.ts";
import {hermeticDependenciesV1} from "../📦️dependencies/🟦️.ts";

const python=testPythonV1(),run=(program:string,input:unknown)=>Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});

test("the class table is closed, ordered by evidence of its rows and names its runtime inputs",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(HERMETIC_TABLE_V1),JSON.stringify(validate.errors)).toBe(true);
 expect(new Set(HERMETIC_TABLE_V1.classes.map(row=>row.id)).size).toBe(HERMETIC_TABLE_V1.classes.length);
 for(const row of HERMETIC_TABLE_V1.classes)for(const port of Object.keys(row.providers))expect(port).toMatch(/^[a-z][a-z-]*$/u);
 expect(HERMETIC_TABLE_V1.runtime.inputs).toContain(HERMETIC_TABLE_V1.runtime.entry);
 expect(validate({...HERMETIC_TABLE_V1,classes:[]})).toBe(false);
 expect(validate({...HERMETIC_TABLE_V1,extra:true})).toBe(false);
 expect(validate({...HERMETIC_TABLE_V1,exceptions:[{project:"p",target:"t",reason:"because"}]})).toBe(false);
 expect(validate({...HERMETIC_TABLE_V1,strategies:{...HERMETIC_TABLE_V1.strategies,darwin:["local"]}})).toBe(false);
});

test("class selection follows the table rows and agrees with an independent Python implementation",()=>{
 for(const row of fixture.selection){
  const actual=hermeticSelectV1(row.facts),shown=actual.kind==="class"?{kind:"class",class:actual.class.id}:actual;
  expect(shown as unknown,row.name).toEqual(row.expected);
 }
 const excepted={...HERMETIC_TABLE_V1,exceptions:[{project:"@teaching/architecture-course",target:"test",reason:"undeclared-assets"}]};
 if(python){
  const rows=[...fixture.selection.map(row=>row.facts),...HERMETIC_TABLE_V1.classes.flatMap(row=>row.select.projects.flatMap(project=>["test","test-x","build-y","typecheck","check","verify-z","lint-a","generate","typegen-b","publish","dev","testing"].map(target=>({project,target,root:"r"}))))];
  const result=run("import json,re,sys\ntable,rows=json.loads(sys.stdin.read())\ndef rx(p):\n    out=''\n    i=0\n    while i<len(p):\n        c=p[i]\n        if c=='*':\n            if p[i+1:i+2]=='*':\n                out+='.*';i+=1\n            else:\n                out+='[^/]*'\n        else:\n            out+=re.escape(c)\n        i+=1\n    return '^'+out+'$'\ndef m(ps,t):\n    return any(re.fullmatch(rx(p),t,re.S) for p in ps)\nres=[]\nfor f in rows:\n    hit=None\n    for c in table['classes']:\n        s=c['select']\n        if m(s['targets'],f['target']) and m(s['projects'],f['project']) and not m(s['excludeRoots'],f['root']):\n            hit=c['id'];break\n    ex=[e for e in table['exceptions'] if e['project']==f['project'] and e['target']==f['target']]\n    res.append(None if hit is None else ('excepted:'+ex[0]['reason'] if ex else hit))\nprint(json.dumps(res))",[HERMETIC_TABLE_V1,rows]);
  expect(result.exitCode,result.stderr.toString()).toBe(0);
  const expected=JSON.parse(result.stdout.toString()) as (string|null)[];
  expect(rows.map(facts=>{const actual=hermeticSelectV1(facts);return actual.kind==="class"?actual.class.id:actual.kind==="unselected"?null:"excepted:"+actual.reason;})).toEqual(expected);
 }
 expect(hermeticSelectV1({project:"@teaching/architecture-course",target:"test",root:"r"},excepted as never)).toEqual({kind:"excepted",reason:"undeclared-assets"});
 expect(hermeticSelectV1({project:"@teaching/architecture-course",target:"build",root:"r"},excepted as never).kind).toBe("class");
 expect(hermeticGlobV1("a/*/c").test("a/b/c")).toBe(true);expect(hermeticGlobV1("a/*/c").test("a/b/x/c")).toBe(false);expect(hermeticGlobV1("a/**/c").test("a/b/x/c")).toBe(true);expect(hermeticGlobV1("a.b").test("axb")).toBe(false);
});

test("host inputs are declared values, covered commands or package names and nothing else",()=>{
 for(const row of fixture.inputs)expect(hermeticInputsV1(row.inputs,row.environment),row.name).toEqual(row.expected as never);
});

test("every host lists spawn strategies that never include unsandboxed local execution by name",()=>{
 for(const row of fixture.strategies)expect([...hermeticStrategiesV1(row.platform)],row.platform).toEqual(row.expected);
 expect(()=>hermeticStrategiesV1("freebsd")).toThrow("Hermetic strategies required for platform: freebsd");
 for(const platform of ["darwin","linux","win32"])expect(HERMETIC_TABLE_V1.strategies[platform as "darwin"]).toContain("isolated-root");
});

test.skipIf(!python)("the dependency selection reads a bun isolated generation and agrees with an independent Python closure",()=>{
 const directory=mkdtempSync(join(tmpdir(),"hermetic-deps-")),root=join(directory,"checkout"),generation=join(directory,"cache/gen"),store=join(generation,"node_modules/.bun");
 const write=(path:string,text:string)=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,text);},link=(target:string,path:string)=>{mkdirSync(join(path,".."),{recursive:true});symlinkSync(target,path);};
 try{
  write(join(store,"a@1/node_modules/a/index.js"),"a");write(join(store,"a@1/node_modules/a/bin/run"),"#!/bin/sh");link("../../b@1/node_modules/b",join(store,"a@1/node_modules/b"));
  write(join(store,"b@1/node_modules/b/index.js"),"b");link("../../c@1/node_modules/c",join(store,"b@1/node_modules/c"));write(join(store,"c@1/node_modules/c/index.js"),"c");
  write(join(store,"@s+d@1/node_modules/@s/d/index.js"),"d");write(join(store,"unused@1/node_modules/unused/index.js"),"u");
  link(".bun/a@1/node_modules/a",join(generation,"node_modules/a"));link("../.bun/@s+d@1/node_modules/@s/d",join(generation,"node_modules/@s/d"));link(".bun/unused@1/node_modules/unused",join(generation,"node_modules/unused"));
  write(join(root,"lib/package.json"),"{}");write(join(root,"lib/src/x.ts"),"x");write(join(root,"other/package.json"),"{}");
  link(join(root,"lib"),join(generation,"node_modules/@semio-tech/lib"));link(join(root,"other"),join(generation,"node_modules/@semio-tech/other"));
  link(join(generation,"node_modules"),join(root,"node_modules"));
  write(join(root,"app/package.json"),JSON.stringify({dependencies:{a:"1","@semio-tech/lib":"workspace:*"},devDependencies:{"@s/d":"1"}}));
  link(join(generation,"w/0/node_modules"),join(root,"app/node_modules"));
  link("../../../node_modules/.bun/a@1/node_modules/a",join(generation,"w/0/node_modules/a"));link("../a/bin/run",join(generation,"w/0/node_modules/.bin/run"));link(join(root,"lib"),join(generation,"w/0/node_modules/@semio-tech/lib"));
  const declared=["app/package.json","lib/package.json","lib/src/x.ts"],actual=hermeticDependenciesV1({root,directories:["app"],names:["unused"],packages:true,declared})!;
  expect(actual).toBeDefined();
  expect(actual.rows.map(row=>row.path)).toEqual(["node_modules","app/node_modules"]);
  const top=actual.rows[0]!,app=actual.rows[1]!;
  expect(app.workspace).toEqual({"@semio-tech/lib":"lib"});expect(app.tree).toEqual({a:"0/a@1/node_modules/a"});expect(app.bin).toEqual({run:"../a/bin/run"});
  expect(top.workspace).toEqual({"@semio-tech/lib":"lib"});expect(top.tree).toEqual({a:"0/a@1/node_modules/a","@s/d":"0/@s+d@1/node_modules/@s/d",unused:"0/unused@1/node_modules/unused"});
  expect(actual.stores.length).toBe(1);
  const result=run("import json,os,sys\nstore,tops=json.loads(sys.stdin.read())\nseen=set()\nq=list(tops)\nwhile q:\n    i=q.pop()\n    if i in seen: continue\n    seen.add(i)\n    nm=os.path.join(store,i,'node_modules')\n    for dp,dn,fn in os.walk(nm):\n        for n in dn+fn:\n            p=os.path.join(dp,n)\n            if os.path.islink(p):\n                t=os.path.normpath(os.path.join(os.path.dirname(p),os.readlink(p)))\n                q.append(os.path.relpath(t,store).split(os.sep)[0])\nprint(json.dumps(sorted(seen)))",[actual.stores[0],["a@1","@s+d@1","unused@1"]]);
  expect(result.exitCode,result.stderr.toString()).toBe(0);
  expect(actual.ids).toEqual((JSON.parse(result.stdout.toString()) as string[]).map(id=>"0/"+id).sort());
  const links=hermeticDependenciesV1({root,directories:["app"],names:["unused"],packages:false,declared})!;
  expect(links.ids).toEqual([]);expect(links.rows.every(row=>Object.keys(row.tree).length===0)).toBe(true);expect(links.rows[1]!.workspace).toEqual({"@semio-tech/lib":"lib"});
  const unlinked=hermeticDependenciesV1({root,directories:["app"],names:[],packages:true,declared:["app/package.json"]})!;
  expect(unlinked.rows.every(row=>Object.keys(row.workspace).length===0)).toBe(true);
  rmSync(join(root,"node_modules"));expect(hermeticDependenciesV1({root,directories:["app"],names:[],packages:true,declared})).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the class table points at its sibling schema through $schema and validates against the pointed schema",()=>{
 const pointed=JSON.parse(readFileSync(join(import.meta.dir,"..",HERMETIC_TABLE_V1.$schema),"utf8"));
 expect(pointed).toEqual(schema);
 const validate=new Ajv({strict:true}).compile(pointed);
 expect(validate(HERMETIC_TABLE_V1),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...HERMETIC_TABLE_V1,$schema:1})).toBe(false);
});

test("the non-TypeScript classes name a provisioned or a pending toolchain and every pending member keeps host state keyed by its pin",()=>{
 const toolchains=new Map(HERMETIC_TABLE_V1.toolchains.map(row=>[row.id,row]));
 expect(toolchains.size).toBe(HERMETIC_TABLE_V1.toolchains.length);
 for(const row of HERMETIC_TABLE_V1.classes)expect(toolchains.has(row.toolchain),row.id).toBe(true);
 for(const row of HERMETIC_TABLE_V1.toolchains){
  expect(row.provisioned||"pending" in row,row.id).toBe(true);
  expect(row.provisioned&&row.covered.length>0).toBe(false);
  for(const source of row.covered)expect(()=>new RegExp(source,"u")).not.toThrow();
 }
 expect(HERMETIC_TABLE_V1.classes.filter(row=>row.toolchain==="bun").every(row=>row.dependencies!=="toolchain")).toBe(true);
 expect(HERMETIC_TABLE_V1.classes.filter(row=>row.toolchain!=="bun").every(row=>row.dependencies==="toolchain"&&row.providers&&Object.keys(row.providers).length===0)).toBe(true);
 const command=(kind:string,tail="")=>`repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts" fingerprint ${kind}${tail}`;
 expect(hermeticInputsV1([{runtime:command("python",' "a/b"')}],{},"python")).toEqual({env:{},packages:[]});
 expect(hermeticInputsV1([{runtime:command("go")}],{},"go")).toEqual({env:{},packages:[]});
 expect("refused" in hermeticInputsV1([{runtime:command("go")}],{},"python")).toBe(true);
 expect("refused" in hermeticInputsV1([{runtime:command("go")}],{})).toBe(true);
 expect("refused" in hermeticInputsV1([{runtime:"go version"}],{},"go")).toBe(true);
 expect("refused" in hermeticInputsV1([{runtime:command("go","; rm -rf /")}],{},"go")).toBe(true);
 for(const [project,target,id] of [["@semio-tech/repo-tickets-go","test","go-test"],["@semio-tech/repo-cli-go","build","go-build"],["@semio-tech/ui-styling-py","test","python-test"],["@semio-tech/ui-styling-py","build","python-build"],["@semio-tech/ui-styling-dotnet","build","dotnet-build"],["workspace","cpp-custody","cmake-check"],["workspace","cpp-test-toolchain","cmake-check"]] as const){
  const selected=hermeticSelectV1({project,target,root:"x"});
  expect({project,target,kind:selected.kind,class:selected.kind==="class"?selected.class.id:undefined}).toEqual({project,target,kind:"class",class:id});
 }
 expect(hermeticSelectV1({project:"@semio-tech/repo-tickets-go",target:"dev",root:"x"}).kind).toBe("unselected");
 expect(hermeticSelectV1({project:"workspace",target:"cpp-build",root:"x"}).kind).toBe("unselected");
});
