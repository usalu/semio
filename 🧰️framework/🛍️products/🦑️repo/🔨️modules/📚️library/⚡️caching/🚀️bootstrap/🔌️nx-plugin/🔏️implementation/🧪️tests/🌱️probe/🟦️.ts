/** 🌱️ Contract: a fresh process evaluates the current projection sources and reports their identity and revisions; a process that fails, times out, answers garbage or answers too much is an error, and the report of a real child process is read back unchanged. */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{tmpdir}from"node:os";
import{join,resolve}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{readFileSync}from"node:fs";
import ts from"typescript";

const module_=resolve(import.meta.dirname,"../../🟨️.mjs"),source=readFileSync(module_,"utf8"),tree=ts.createSourceFile(module_,source,ts.ScriptTarget.Latest,true);
const pick=(name:string)=>tree.statements.find(node=>(ts.isFunctionDeclaration(node)&&node.name?.text===name)||(ts.isVariableStatement(node)&&node.declarationList.declarations.some(declaration=>declaration.name.getText(tree)===name)))!.getText(tree).replace(/^export /u,"");
const code=ts.transpileModule(pick("probeCode")+"\n"+pick("nxBuildDependencySourceProbeV1"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText;
const probe=new Function("spawnSync","fileURLToPath","library",code+";return nxBuildDependencySourceProbeV1;")(spawnSync,fileURLToPath,pathToFileURL(resolve(import.meta.dirname,"../../../../../../")+"/"));
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"projection-probe-"));
const identity="c".repeat(64),revision={path:"/x/a.mjs",sha256:"d".repeat(64)};

test("a real child process loads the named module and its report is read back unchanged",()=>{
 const dir=scratch();
 try{
  const root=join(dir,"root.mjs");writeFileSync(root,`export const nxBuildDependencyImplementationV1=()=>${JSON.stringify(identity)};export const nxBuildDependencyImplementationRevisionsV1=()=>[${JSON.stringify(revision)}];\n`);
  expect(probe({root})).toEqual({identity,revisions:[revision]});
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a child that throws while it evaluates the sources is an error naming the first line",()=>{
 const dir=scratch();
 try{
  const root=join(dir,"root.mjs");writeFileSync(root,`export const nxBuildDependencyImplementationV1=()=>{throw new Error("Loaded private Nx dependency Source changed\\nsecond line");};export const nxBuildDependencyImplementationRevisionsV1=()=>[];\n`);
  expect(()=>probe({root})).toThrow(/status 1: Loaded private Nx dependency Source changed$/u);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a child that does not answer in the declared time, answers garbage or answers more than the bound is an error",()=>{
 const dir=scratch();
 try{
  const slow=join(dir,"slow.mjs");writeFileSync(slow,"await new Promise(resolve=>setTimeout(resolve,20000));export const nxBuildDependencyImplementationV1=()=>'x';export const nxBuildDependencyImplementationRevisionsV1=()=>[];\n");
  expect(()=>probe({root:slow,timeoutMilliseconds:300})).toThrow();
  const bad=join(dir,"bad.mjs");writeFileSync(bad,"export const nxBuildDependencyImplementationV1=()=>'not-a-digest';export const nxBuildDependencyImplementationRevisionsV1=()=>[];\n");
  expect(()=>probe({root:bad})).toThrow(/answered no identity/u);
  const big=join(dir,"big.mjs");writeFileSync(big,`export const nxBuildDependencyImplementationV1=()=>${JSON.stringify(identity)};export const nxBuildDependencyImplementationRevisionsV1=()=>Array.from({length:5000},(_,at)=>({path:"/x/"+at,sha256:${JSON.stringify("d".repeat(64))}}));\n`);
  expect(()=>probe({root:big,maximumBytes:4096})).toThrow();
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the command, the environment and the bounds handed to the child process are the declared ones",()=>{
 let seen:{command?:string;args?:string[];options?:Record<string,unknown>}={};
 const result=probe({execPath:"/bin/bun",root:"/r/🟨️.mjs",timeoutMilliseconds:7,maximumBytes:9,run:(command:string,args:string[],options:Record<string,unknown>)=>{seen={command,args,options};return{status:0,stdout:JSON.stringify({identity,revisions:[revision]})};}});
 expect(result.identity).toBe(identity);expect(seen.command).toBe("/bin/bun");expect(seen.args![0]).toBe("--eval");
 expect(seen.options).toMatchObject({timeout:7,maxBuffer:9,windowsHide:true,env:expect.objectContaining({SEMIO_NX_PROJECTION_PROBE:"/r/🟨️.mjs"})});
});
