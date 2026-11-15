/** 🫥️ Tests for dropping the local entries of one crate. */
import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {forgetCrateEntriesV1} from "../../🟦️.ts";

const oracle=`import json,os,sys
root,crate=sys.argv[1:3];n=0
for kind in ("e","r"):
    base=os.path.join(root,kind)
    if not os.path.isdir(base): continue
    for d,_,fs in os.walk(base):
        for f in fs:
            try: c=json.load(open(os.path.join(d,f))).get("crate")
            except Exception: continue
            if c==crate: n+=1
print(n)`;
const write=(root:string,kind:string,key:string,name:string,crate:string)=>{const directory=join(root,kind,key.slice(0,2),key);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,name),JSON.stringify({version:1,key,crate}));};
test("forgetting a crate drops exactly its local entries and build script runs, as an independent Python count agrees",()=>{
 const root=mkdtempSync(join(tmpdir(),"forget-"));
 try{
  write(root,"e","aa11","1.json","serde");write(root,"e","aa11","2.json","serde");write(root,"e","bb22","1.json","syn");write(root,"r","cc33","1.json","serde");write(root,"r","dd44","1.json","quote");
  const count=(crate:string)=>Number(spawnSync("python3",["-I","-c",oracle,root,crate],{encoding:"utf8"}).stdout);
  expect(count("serde")).toBe(3);
  expect(forgetCrateEntriesV1(root,"serde")).toEqual({entries:3});
  expect(count("serde")).toBe(0);expect(count("syn")).toBe(1);expect(count("quote")).toBe(1);
  expect(existsSync(join(root,"e","aa","aa11"))).toBe(false);expect(existsSync(join(root,"e","bb","bb22","1.json"))).toBe(true);
  expect(forgetCrateEntriesV1(root,"serde")).toEqual({entries:0});
  expect(()=>forgetCrateEntriesV1(root,"")).toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});
