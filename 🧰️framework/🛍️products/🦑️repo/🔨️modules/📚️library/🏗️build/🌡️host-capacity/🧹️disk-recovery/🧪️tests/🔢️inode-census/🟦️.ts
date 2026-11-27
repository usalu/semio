import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {diskTextV1,inodeCensusV1,sweepDiskV1} from "../../🟦️.ts";

const tree=():string=>{
 const root=mkdtempSync(join(tmpdir(),"semio-census-"));
 for(const [folder,files] of [["a/big",30],["a/small",3],["b/mid",12]] as const){mkdirSync(join(root,folder,"sub"),{recursive:true});for(let index=0;index<files;index++)writeFileSync(join(root,folder,index%2?"sub":".",`f${index}`),"x");}
 symlinkSync(root,join(root,"a","loop"));
 return root;
};
const find=(path:string):number=>Number(spawnSync("sh",["-c",`find "${path}" -mindepth 1 | wc -l`],{encoding:"utf8"}).stdout.trim());

test("census counts every entry two levels down like find does, biggest first, never following links",()=>{
 const root=tree();
 try{
  const rows=inodeCensusV1(root,1_000_000);
  expect(rows.map(row=>row.root)).toEqual(["a/big","b/mid","a/small"]);
  for(const row of rows){expect(row.partial).toBe(false);expect(row.entries).toBe(find(join(root,row.root)));}
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("census shares one budget and marks the folder it cut short",()=>{
 const root=tree();
 try{
  const rows=inodeCensusV1(root,10);
  expect(rows.some(row=>row.partial)).toBe(true);
  expect(rows.reduce((sum,row)=>sum+row.entries,0)).toBeLessThanOrEqual(10);
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("an inode sweep that cannot free enough names the holders in both languages",async()=>{
 const root=tree(),lines:string[]=[];
 try{
  await sweepDiskV1({facts:()=>({partition:root,totalBytes:100,freeBytes:50}),needBytes:0,scarce:"inodes",census:{root},writers:[],language:"both",report:line=>lines.push(line)});
  const expected=diskTextV1("recovery.inode-holder","both",{root:"a/big",entries:find(join(root,"a/big")),partial:""});
  expect(lines).toContain(expected);
  expect(expected).toContain("Inode-Halter");
 }finally{rmSync(root,{recursive:true,force:true});}
});
