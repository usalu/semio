import{plugin}from"bun";
import*as real from"fs";
import*as realPromises from"fs/promises";
const env=process.env,state={count:0,target:Number(env.SEMIO_CRASH_AT??0),tear:env.SEMIO_CRASH_TEAR==="1",journal:env.SEMIO_CRASH_JOURNAL??""};
const half=(data:unknown)=>typeof data==="string"?data.slice(0,Math.floor(data.length/2)):data instanceof Uint8Array?data.subarray(0,Math.floor(data.length/2)):data;

/** 💥️ Counts one state-changing call; the call numbered `SEMIO_CRASH_AT` is killed before it takes effect, or after half of its bytes when tearing. */
function strike(kind:string,detail:string,partial?:()=>void):void{
 state.count++;
 if(state.journal)real.appendFileSync(state.journal,`${state.count}\t${kind}\t${detail}\n`);
 if(state.target!==0&&state.count===state.target){if(state.tear&&partial){try{partial();}catch{}}process.kill(process.pid,"SIGKILL");}
}
const creating=(flags:unknown)=>typeof flags==="number"?(flags&(real.constants.O_CREAT|real.constants.O_TRUNC|real.constants.O_APPEND))!==0:typeof flags==="string"&&/[wa]/u.test(flags);

const sync:Record<string,unknown>={...real};
for(const name of["renameSync","unlinkSync","mkdirSync","rmdirSync","rmSync","linkSync","symlinkSync","truncateSync","ftruncateSync","copyFileSync","chmodSync","fchmodSync","mkdtempSync","utimesSync"]){
 const call=(real as unknown as Record<string,(...a:unknown[])=>unknown>)[name]!;sync[name]=(...a:unknown[])=>{strike(name,String(a[0]));return call(...a);};
}
const writeFile=real.writeFileSync,appendFile=real.appendFileSync,writeSync=real.writeSync,openSync=real.openSync;
sync.writeFileSync=(path:unknown,data:unknown,...rest:unknown[])=>{strike("writeFileSync",String(path),()=>writeFile(path as string,half(data) as string,...(rest as [])));return writeFile(path as string,data as string,...(rest as []));};
sync.appendFileSync=(path:unknown,data:unknown,...rest:unknown[])=>{strike("appendFileSync",String(path),()=>appendFile(path as string,half(data) as string,...(rest as [])));return appendFile(path as string,data as string,...(rest as []));};
sync.writeSync=(fd:number,data:unknown,...rest:unknown[])=>{strike("writeSync",String(fd),()=>{if(typeof data==="string"||rest.length===0)(writeSync as (...a:unknown[])=>unknown)(fd,half(data));else(writeSync as (...a:unknown[])=>unknown)(fd,data,rest[0],Math.floor(Number(rest[1]??0)/2),...rest.slice(2));});return(writeSync as (...a:unknown[])=>unknown)(fd,data,...rest);};
sync.openSync=(path:unknown,flags:unknown,...rest:unknown[])=>{if(creating(flags))strike("openSync",String(path));return(openSync as (...a:unknown[])=>unknown)(path,flags,...rest);};
sync.default=sync;

const asyncOps:Record<string,unknown>={...realPromises};
for(const name of["rename","unlink","mkdir","rmdir","rm","link","symlink","truncate","copyFile","chmod","mkdtemp","utimes","writeFile","appendFile"]){
 const call=(realPromises as unknown as Record<string,(...a:unknown[])=>Promise<unknown>>)[name]!;asyncOps[name]=(...a:unknown[])=>{strike("p."+name,String(a[0]));return call(...a);};
}
asyncOps.open=async(path:unknown,flags?:unknown,...rest:unknown[])=>{
 if(creating(flags))strike("p.open",String(path));
 const handle=await(realPromises.open as (...a:unknown[])=>Promise<unknown>)(path,flags,...rest);
 return new Proxy(handle as object,{get(target,key){const value=Reflect.get(target,key);if(typeof value!=="function")return value;return["write","writeFile","appendFile","truncate","chmod","utimes"].includes(String(key))?(...a:unknown[])=>{strike("h."+String(key),String(path));return value.apply(target,a);}:value.bind(target);}});
};
asyncOps.default=asyncOps;
(sync as Record<string,unknown>).promises=asyncOps;

plugin({name:"semio-crash",setup(build){
 build.module("semio:fs",()=>({exports:sync,loader:"object"}));
 build.module("semio:fs/promises",()=>({exports:asyncOps,loader:"object"}));
 build.onLoad({filter:/^(?!.*\/node_modules\/).*\.(ts|mts)$/u},async args=>{
  const text=await Bun.file(args.path).text();
  return{contents:text.replace(/\bfrom\s*(["'])node:fs\/promises\1/gu,'from"semio:fs/promises"').replace(/\bfrom\s*(["'])node:fs\1/gu,'from"semio:fs"'),loader:"ts"};
 });
}});
