import {describe,expect,test} from "bun:test";
import {existsSync,readdirSync,readFileSync,statSync} from "node:fs";
import {dirname,join,resolve} from "node:path";

/** 🔒️ No module of the build library or the process module spells a machine state path (`<checkout>/.cache`, `<checkout>/.nx`) outside the resolver: a machine that declares `SEMIO_MACHINE_STATE_DIRECTORY` must never write below the checkout by a site that forgot it. @see ../../🟦️.ts */
const here=import.meta.dir,resolver=resolve(here,"..","..","🟦️.ts"),framework=resolve(here,"..","..","..","..","..","..");
const scopes=[join(framework,"🛍️products","🦑️repo","🔨️modules","📚️library"),join(framework,"🔨️modules","🏃️process")];
const skip=new Set(["node_modules","target",".cache","🧪️tests","🧫️fixtures","🗑️generated"]);
const FIRST_ARGUMENTS_OF_OTHER_ROOTS=new Set(["home","homedir()","fixture","match[1]!","at","config.root"]);
const SPELLING=/(?<![\w$.])(?:\w+\.)?(?:join|resolve)\(\s*([^,()]*(?:\([^()]*\))?[^,()]*),\s*["']\.(?:cache|nx)(?:["'/])/gu;
function* sources(directory:string):Generator<string>{
 for(const name of readdirSync(directory)){
  if(skip.has(name))continue;
  const path=join(directory,name),stat=statSync(path);
  if(stat.isDirectory())yield*sources(path);
  else if(/\.(ts|mjs)$/u.test(name)&&!/\.d\.m?ts$/u.test(name))yield path;
 }
}
describe("machine state spelling",()=>{
 test("no source joins the checkout root with .cache or .nx outside the resolver",()=>{
  const offenders:string[]=[];
  for(const scope of scopes)for(const file of sources(scope))for(const found of readFileSync(file,"utf8").matchAll(SPELLING))if(!FIRST_ARGUMENTS_OF_OTHER_ROOTS.has(found[1]!.trim()))offenders.push(`${file.slice(framework.length)}: ${found[0].slice(0,80)}`);
  expect(offenders).toEqual([]);
 });
 test("every module that calls the resolver imports it from the resolver file",()=>{
  const wrong:string[]=[];
  for(const scope of scopes)for(const file of sources(scope)){
   const text=readFileSync(file,"utf8");
   if(file===resolver||!text.includes("machinePathV1("))continue;
   const imported=/import\s*\{[^}]*\bmachinePathV1\b[^}]*\}\s*from\s*"([^"]+)"/u.exec(text);
   if(imported===null||!existsSync(resolve(dirname(file),imported[1]!))||resolve(dirname(file),imported[1]!)!==resolver)wrong.push(file.slice(framework.length));
  }
  expect(wrong).toEqual([]);
 });
});
