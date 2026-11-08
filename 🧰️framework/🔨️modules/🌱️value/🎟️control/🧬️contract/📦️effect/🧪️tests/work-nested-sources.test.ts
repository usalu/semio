import {expect,test} from "bun:test";
import {existsSync,readdirSync,readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
const skip=new Set(["node_modules","target",".cache","🗑️generated",".git"]);
const root=(()=>{let at=import.meta.dir;while(!existsSync(join(at,"AGENTS.md"))&&dirname(at)!==at)at=dirname(at);return at})();
const walk=(dir:string,out:string[]):string[]=>{for(const entry of readdirSync(dir,{withFileTypes:true})){if(skip.has(entry.name))continue;const path=join(dir,entry.name);if(entry.isDirectory())walk(path,out);else if(entry.name.endsWith(".rs"))out.push(path)}return out};
const wholeCeiling=/\.reborrow\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*?,\s*\w+\.maximum_work_units\(\)\)/;
/** 🪆️ No producer may reborrow a parent effect with the parent's whole authored ceiling: it refuses after any spent work, `nested` lends exactly what is left. */
test("no rust source reborrows an effect with its own whole ceiling",()=>{
  const roots=["🧰️framework","✏️s"].map(name=>join(root,name)).filter(existsSync);
  const hits=roots.flatMap(dir=>walk(dir,[])).filter(path=>wholeCeiling.test(readFileSync(path,"utf8")));
  expect(hits.map(path=>path.slice(root.length))).toEqual([]);
  const oracle=spawnSync("/usr/bin/grep",["-rlE","--include=*.rs","--exclude-dir=node_modules","--exclude-dir=target","--exclude-dir=.cache","reborrow\\([^;]*,[^;]*maximum_work_units\\(\\)\\)",...roots]);
  const oracleHits=oracle.stdout.toString().split("\n").filter(Boolean).filter(path=>!path.endsWith("📦️effect/🦀️.rs"));
  expect(oracleHits).toEqual([]);
},300000);
