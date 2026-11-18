import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";

/** 🧰️ A short scratch directory (views refuse long roots), removed by the caller. */
export const scratchV1=(tag:string):string=>mkdtempSync(join(tmpdir(),tag+"-"));
export const removeV1=(path:string):void=>{rmSync(path,{recursive:true,force:true});};

/** 🌳️ Writes a tree of `count` files of deterministic bytes below `root` and answers the relative paths. */
export function growTreeV1(root:string,count:number,seed:number=1):string[]{
 const paths:string[]=[];
 for(let index=0;index<count;index++){
  const path=`d${index%11}/e${index%3}/f${index}.rs`;
  mkdirSync(dirname(join(root,path)),{recursive:true});
  writeFileSync(join(root,path),`// ${seed} ${index}\n`.repeat(1+index%17));
  paths.push(path);
 }
 return paths;
}

const python=Bun.which("python3");
export const oracleV1=(request:unknown):unknown=>{
 const run=Bun.spawnSync([python!,"-I",join(import.meta.dir,"..","🐍️oracle","🐍️.py")],{stdin:new TextEncoder().encode(JSON.stringify(request)),stdout:"pipe",stderr:"pipe"});
 if(run.exitCode!==0)throw Error("oracle failed: "+run.stderr.toString());
 return JSON.parse(run.stdout.toString());
};
export const hasOracleV1=python!==null;

/** 🔬️ Every file below `root` with its bytes, relative paths sorted. */
export function snapshotV1(root:string):Record<string,string>{
 const rows:Record<string,string>={};
 const walk=(directory:string):void=>{for(const name of readdirSync(directory).sort()){const full=join(directory,name),info=statSync(full);if(info.isDirectory())walk(full);else rows[full.slice(root.length+1)]=readFileSync(full,"utf8");}};
 walk(root);return rows;
}

export const sleepV1=(milliseconds:number):Promise<void>=>new Promise(done=>setTimeout(done,milliseconds));
