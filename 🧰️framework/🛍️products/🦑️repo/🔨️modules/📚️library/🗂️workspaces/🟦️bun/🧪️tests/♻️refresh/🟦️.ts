import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";
import * as ownership from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

const checked = (file: string): string => { if (file.length > 256) throw Error("Refresh fixture path capacity exceeded"); return file; };
const put = (root: string, file: string, value: unknown): void => { const target=checked(join(root,file)); mkdirSync(join(target,".."),{recursive:true}); writeFileSync(target,JSON.stringify(value)); };
const source = (name: string, members: string[], owners: string[] = []): object => ({name,private:true,semio:{workspace:{schemaVersion:1,members,owners}},workspaces:["members/a"]});

test("semantic frontier refresh outcomes match the independent SQLite contract", () => {
 const validate=new Ajv({strict:true}).compile(schema.$defs.FrontierRefreshCase),database=new Database(":memory:");
 try { for(const row of fixture.frontierRefresh) {
  expect(validate(row),JSON.stringify(validate.errors)).toBe(true);
  const oracle=database.query("SELECT CASE WHEN ?2='cancel' THEN 'canceled' WHEN ?2='undefined' THEN 'failed' WHEN ?1 IN ('empty-directory','nonmember-decision','source') AND ?2='none' THEN 'accepted' ELSE 'rejected' END first, CASE WHEN ?1 IN ('empty-directory','nonmember-decision','source') AND ?2='none' THEN ?3='none' ELSE NULL END later, ?1 IN ('empty-directory','nonmember-decision') AND ?2='none' adopted").get(row.action,row.finalMutation,row.laterMutation) as {first:string;later:number|null;adopted:number};
  expect(oracle.first,row.name).toBe(row.first);expect(oracle.later===null?null:Boolean(oracle.later),row.name).toBe(row.later);expect(Boolean(oracle.adopted),row.name).toBe(row.adopted);
 }} finally {database.close();}
});

for(const row of fixture.frontierRefresh) test("actual semantic frontier refresh: "+row.name, async () => {
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Owned refresh artifacts required");checked(join(output,"rf000000/r/members/empty/new/value.ts"));mkdirSync(output,{recursive:true});
 const container=mkdtempSync(join(output,"rf")),root=checked(join(container,"r")),stop=new AbortController(),reason=Object.freeze({refreshOriginal:row.name}),events:string[]=[];
 let body=false,sawDiscovery=false,changed=false,afterChanged=0,lastPath="",first:string|undefined,later:boolean|null=null,rejected=false,failure:unknown;
 try {
  put(container,"package.json",{name:"empty-parent",private:true,workspaces:[]});mkdirSync(root);
  put(root,"package.json",source("refresh",["members/**"],["*/package.json"]));put(root,"members/a/package.json",{name:"refresh-a",version:"1.0.0"});mkdirSync(checked(join(root,"members/empty")));mkdirSync(checked(join(root,"outside")));
  const operation=ownership.withPublishedBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:stop.signal,advance:async progress=>{
   if(!body)return;events.push(progress.operation);if(progress.operation==="state"||progress.operation==="list")sawDiscovery=true;
   if(changed&&progress.operation==="frontier")afterChanged++;
   if(!changed&&sawDiscovery&&row.finalMutation!=="none"&&progress.operation==="frontier"&&progress.path===lastPath){
    changed=true;
    if(row.finalMutation==="directory")put(root,"members/late/package.json",{name:"late"});
    if(row.finalMutation==="manifest")writeFileSync(checked(join(root,"outside/package.json")),JSON.stringify({name:"different-decision"}));
    if(row.finalMutation==="cancel")stop.abort(reason);
    if(row.finalMutation==="undefined")throw undefined;
    await Promise.resolve();
   }
  }},async custody=>{
   body=true;lastPath=custody.observed().paths.at(-1)!.path;
   if(row.action==="empty-directory")mkdirSync(checked(join(root,"members/empty/new")));
   if(row.action==="nonmember-decision")put(root,"outside/package.json",{name:"nonmember-decision"});
   if(row.action==="source")writeFileSync(checked(join(root,"members/a/value.ts")),"export const value=1;");
   if(row.action==="member")put(root,"members/b/package.json",{name:"new-member"});
   if(row.action==="remove-required")rmSync(checked(join(root,"members/a/package.json")));
   if(row.action==="manifest-bytes")writeFileSync(checked(join(root,"members/a/package.json")),JSON.stringify({name:"refresh-a",version:"1.0.0",description:"changed"}));
   if(row.action==="patterns")put(root,"package.json",source("refresh",["members/a"],["*/package.json"]));
   if(row.action==="owner")put(root,"outside/package.json",source("new-scope",["children/**"]));
   try{await custody.recheck();first="accepted";}catch(error){first=stop.signal.aborted?"canceled":row.finalMutation==="undefined"&&error===undefined?"failed":"rejected";throw error;}
   expect(first,row.name).toBe(row.first);
   if(row.adopted){expect(events.some(event=>event==="state"||event==="list"),row.name).toBe(true);const observed=custody.observed(),path=join(root,row.action==="empty-directory"?"members/empty/new":"outside/package.json");expect(observed.paths.find(record=>record.path===path)?.state[0]).toBe(row.action==="empty-directory"?"directory":"file");if(row.action==="nonmember-decision")expect(observed.texts.find(record=>record.path==="outside/package.json")?.sha256).toBe(createHash("sha256").update(readFileSync(path)).digest("hex"));}
   const start=events.length;await custody.recheck();expect(events.slice(start).filter(event=>event==="state"||event==="list"),row.name).toEqual([]);
   if(row.laterMutation==="decision-bytes")writeFileSync(checked(join(root,"outside/package.json")),readFileSync(join(root,"outside/package.json"),"utf8")+" ");
   if(row.laterMutation==="decision-removal")rmSync(checked(join(root,"outside/package.json")));
   try{await custody.recheck();later=true;}catch(error){later=false;throw error;}
  });
  try{await operation;}catch(error){rejected=true;failure=error;}
  expect(body,row.name).toBe(true);expect(first,row.name).toBe(row.first);expect<unknown>(later,row.name).toBe(row.later);expect(rejected,row.name).toBe(row.first!=="accepted"||row.later===false);
  if(row.finalMutation!=="none"){expect(changed,row.name).toBe(true);expect(afterChanged,row.name).toBe(0);}
  if(row.first==="canceled")expect(failure).toBe(reason);
  else if(row.first==="failed")expect(failure).toBeUndefined();
  else if(rejected)expect(String(failure),row.name).toMatch(/changed|drift|regular manifest/u);
 } finally {stop.abort();rmSync(container,{recursive:true,force:true});}
});
