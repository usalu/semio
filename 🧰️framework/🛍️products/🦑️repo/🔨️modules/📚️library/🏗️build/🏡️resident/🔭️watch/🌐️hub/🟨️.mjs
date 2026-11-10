/** 🌐 One native watcher process for every root of a host. The hub speaks the `serve` protocol of `semio-repo-watch` and hands each supervisor a process-shaped handle (`stdout` with that root's frames, `stdin.end()` closes the root, `exit` when the root or the hub ends), so `createWatchSupervisorV1({spawnProcess: hub.spawnProcess})` needs no other change. */
import {spawn} from "node:child_process";
import {EventEmitter} from "node:events";
import {PassThrough} from "node:stream";

const MAXIMUM_LINE_BYTES=64<<20,SEPARATOR="\u001f";

/** 🔧 The `open` command of the `watch` arguments one supervisor would have given a process of its own. */
export function openCommandV1(tag,args){
 const value=name=>{const at=args.indexOf(name);return at<0?"":String(args[at+1]);};
 const all=name=>args.flatMap((item,at)=>item===name?[String(args[at+1])]:[]);
 return["open",tag,value("--root"),value("--latency-ms")||"5",value("--exclusion-version")||"1",all("--exclude").join(SEPARATOR),all("--exclude-name").join(SEPARATOR),value("--since"),value("--uuid")].join("\t")+"\n";
}

/** 🌐 Creates the hub; the process starts with the first root and again after it ended. */
export function createWatchHubV1({binary,spawnHub=spawn,maximumRoots=64}){
 let hub,counter=0,buffer=Buffer.alloc(0);
 const routes=new Map(),stats={roots:0,frames:0,restarts:0};
 const finish=(tag,code)=>{const route=routes.get(tag);if(!route)return;routes.delete(tag);stats.roots=routes.size;route.stdout.end();setImmediate(()=>route.emit("exit",code,null));};
 const dispatch=line=>{
  const match=/^\{"v":1,"r":"([^"]*)",/u.exec(line);
  if(!match)return;
  stats.frames++;
  const tag=match[1],route=routes.get(tag);
  if(!route)return;
  route.stdout.write(line+"\n");
  if(line.includes('"t":"closed"'))finish(tag,/"requested":true/u.test(line)?0:1);
 };
 const start=()=>{
  if(hub)return hub;
  const child=spawnHub(binary,["serve"],{stdio:["pipe","pipe","ignore"]});
  hub=child;buffer=Buffer.alloc(0);
  child.stdout.on("data",chunk=>{
   buffer=Buffer.concat([buffer,chunk]);
   if(buffer.length>MAXIMUM_LINE_BYTES&&buffer.indexOf(10)<0){child.kill("SIGKILL");return;}
   for(let cut=buffer.indexOf(10);cut>=0;cut=buffer.indexOf(10)){dispatch(buffer.subarray(0,cut).toString("utf8"));buffer=buffer.subarray(cut+1);}
  });
  child.stdin.on("error",()=>undefined);
  const ended=code=>{if(hub!==child)return;hub=undefined;stats.restarts++;for(const tag of[...routes.keys()])finish(tag,code??1);};
  child.once("exit",ended);child.once("error",()=>ended(1));
  return child;
 };
 return{
  stats,
  /** 🚀 Drop-in `spawnProcess` of the watch supervisor for the `watch` command. */
  spawnProcess(_binary,args){
   if(args[0]!=="watch")throw new Error("The watch hub serves the watch command only");
   if(routes.size>=maximumRoots)throw new Error("The watch hub serves at most "+maximumRoots+" roots");
   const tag="r"+(++counter),route=new EventEmitter();
   route.stdout=new PassThrough();route.pid=undefined;
   route.stdin={end(){if(!routes.has(tag))return;try{hub?.stdin.write("close\t"+tag+"\n");}catch{}finish(tag,0);},on(){return route.stdin;}};
   route.kill=()=>route.stdin.end();
   routes.set(tag,route);stats.roots=routes.size;
   const child=start();
   route.pid=child.pid;
   try{child.stdin.write(openCommandV1(tag,args));}catch(error){finish(tag,1);}
   return route;
  },
  async stop(){
   const child=hub;if(!child)return;
   const exited=new Promise(resolve=>child.once("exit",resolve));
   try{child.stdin.end();}catch{}
   const killer=setTimeout(()=>child.kill("SIGKILL"),1000);killer.unref?.();
   await exited;clearTimeout(killer);
  },
 };
}
