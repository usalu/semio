import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdtempSync,mkdirSync,writeFileSync,symlinkSync,lstatSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {preparePrivateDirectoryV1} from "../🟨️.mjs";
import {writeWarmPreloadV1} from "../../../../🏊️delegate/🟨️.mjs";
import corpus from "./🔣️.json";
test("runtime and warm preload share private-directory admission without repairing unsafe inputs",()=>{
 const root=mkdtempSync(join(tmpdir(),"rd-")),database=new Database(":memory:"),posix=process.platform!=="win32";
 try{for(const row of corpus){
  const path=join(root,row.kind);
  if(row.kind==="private"||row.kind==="public")mkdirSync(path,{mode:row.private?0o700:0o755});
  if(row.kind==="file")writeFileSync(path,"untouched");
  if(row.kind==="symlink"){const target=join(root,"link-target");mkdirSync(target,{mode:0o700});symlinkSync(target,path,process.platform==="win32"?"junction":"dir");}
  const oracle=database.query("select ?1 AND NOT ?2 AND (NOT ?3 OR ?4) accepted").get(row.directory,row.link,Number(posix),row.private) as {accepted:number};
  if(oracle.accepted){preparePrivateDirectoryV1(path);const preload=writeWarmPreloadV1(path);expect(lstatSync(preload).isFile()).toBe(true);if(posix)expect(lstatSync(path).mode&0o077).toBe(0);}
  else{expect(()=>preparePrivateDirectoryV1(path)).toThrow();expect(()=>writeWarmPreloadV1(path)).toThrow();if(row.kind==="public")expect(readdirSync(path)).toEqual([]);}
 }}finally{database.close();rmSync(root,{recursive:true,force:true});}
});
