/** 🔁️ The store worker import graph stays acyclic, value, type-only and dynamic edges included: our own strongly-connected-component scan agrees with dependency-cruiser on a portable corpus and finds no cycle in the real tree. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve,sep} from "node:path";
import ts from "typescript";
import {cruise} from "dependency-cruiser";
import corpus from "./🧫️fixtures/🔣️.json";

type Case={id:string;files:Record<string,string>;cycles:string[][]};
const workerRoot=resolve(import.meta.dir,"../..");

const sourceFiles=(dir:string):string[]=>readdirSync(dir).flatMap(name=>{
  if(name==="node_modules")return [];
  const path=join(dir,name);
  return statSync(path).isDirectory()?sourceFiles(path):/\.tsx?$/u.test(name)?[path]:[];
});

const cyclesOf=(files:readonly string[]):string[][]=>{
  const known=new Set(files);
  const graph=new Map<string,string[]>(files.map(file=>[file,ts.preProcessFile(readFileSync(file,"utf8"),true,true).importedFiles.map(entry=>entry.fileName).filter(name=>name.startsWith(".")).map(name=>resolve(dirname(file),name)).filter(target=>known.has(target))]));
  const index=new Map<string,number>(),low=new Map<string,number>(),onStack=new Set<string>(),stack:string[]=[],found:string[][]=[];
  let counter=0;
  const visit=(node:string):void=>{
    index.set(node,counter);low.set(node,counter);counter+=1;stack.push(node);onStack.add(node);
    for(const next of graph.get(node)!){
      if(!index.has(next)){visit(next);low.set(node,Math.min(low.get(node)!,low.get(next)!));}
      else if(onStack.has(next))low.set(node,Math.min(low.get(node)!,index.get(next)!));
    }
    if(low.get(node)!==index.get(node))return;
    const component:string[]=[];
    let member:string;
    do{member=stack.pop()!;onStack.delete(member);component.push(member);}while(member!==node);
    if(component.length>1||graph.get(node)!.includes(node))found.push(component.sort());
  };
  for(const file of files)if(!index.has(file))visit(file);
  return found.sort((a,b)=>a[0]!.localeCompare(b[0]!));
};

const oracleCycles=async(root:string):Promise<string[][]>=>{
  const result=await cruise([root],{tsPreCompilationDeps:true,ruleSet:{forbidden:[{name:"no-circular",severity:"error",from:{},to:{circular:true}}]},validate:true});
  const output=result.output as {summary:{violations:{from:string;cycle?:{name:string}[]}[]}};
  const seen=new Map<string,string[]>();
  for(const violation of output.summary.violations){
    const members=[...new Set([violation.from,...(violation.cycle??[]).map(step=>step.name)])].map(name=>resolve(name)).sort();
    if(members.every(member=>member.startsWith(root+sep)))seen.set(members.join("|"),members);
  }
  return [...seen.values()].sort((a,b)=>a[0]!.localeCompare(b[0]!));
};

test("portable corpus: own scan, oracle and expectation agree on every case",async()=>{
  for(const row of corpus.cases as unknown as Case[]){
    const dir=realpathSync(mkdtempSync(join(tmpdir(),"semio-acyclic-")));
    try{
      for(const [name,text] of Object.entries(row.files)){mkdirSync(dirname(join(dir,name)),{recursive:true});writeFileSync(join(dir,name),text);}
      const files=Object.keys(row.files).map(name=>resolve(dir,name));
      const expected=row.cycles.map(members=>members.map(name=>resolve(dir,name)).sort());
      expect(cyclesOf(files),row.id+" own").toEqual(expected);
      expect(await oracleCycles(dir),row.id+" oracle").toEqual(expected);
    }finally{rmSync(dir,{recursive:true,force:true});}
  }
},120000);

test("store worker has no import cycle of any edge kind",async()=>{
  const files=sourceFiles(workerRoot).map(file=>resolve(file));
  expect(files.length).toBeGreaterThan(40);
  expect(cyclesOf(files)).toEqual([]);
  expect(await oracleCycles(workerRoot)).toEqual([]);
},300000);
