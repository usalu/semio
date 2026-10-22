import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {resolve} from "node:path";
import catalogue from "../../../⚙️operations/🔣️catalog.json" with {type:"json"};

const script=resolve(import.meta.dir,"../../../📜️script.ts"),cleanEnvironment=Object.fromEntries(Object.entries(process.env).filter(([key])=>!key.startsWith("SEMIO_SCRIPT")&&!key.startsWith("SEMIO_BUILD_")&&key!=="SEMIO_LANGUAGE"));
const help=(...options:string[])=>spawnSync(process.execPath,[script,"repo","run","--help",...options],{encoding:"utf8",timeout:90_000,env:{...cleanEnvironment,SEMIO_LANGUAGE:"en"}});

test("every global option of the catalogue, in its equals spelling, reaches the selected operation of the full build script without refusing the declared selection",()=>{
 const globals=catalogue.flags.filter(flag=>(flag as {operations?:unknown}).operations===undefined&&flag.value==="required").map(flag=>`${flag.name}=${/^\{[a-z]+\}$/u.test((flag as {valueName?:string}).valueName??"")?"nx":((flag as {valueName?:string}).valueName??"x").split("|")[0]}`);
 expect(globals.length).toBeGreaterThanOrEqual(3);
 for(const option of [...globals,"--graph-policy=current"]){
  const result=help(option);
  expect(result.stderr,option).not.toContain("Undeclared original Repo build selection");
  expect(result.status,option).toBe(0);
  expect(result.stdout,option).toContain("repo run - Run one task of one project.");
 }
 const german=help("--lang=de");
 expect(german.status).toBe(0);
 expect(german.stdout).toContain("Verwendung");
},180_000);
