import {lstatSync,readdirSync,unlinkSync,statSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const;
/** ⏱️ The stall bound in milliseconds, from the schema. */
export const installerStallMillisecondsV1:number=protocol.stallMilliseconds;
/** 🐕️ The decision core of the stall rule: fed the samples of the probes in time order it answers the clock value at which the installer counts as stalled, or null while it progresses. See https://bun.sh/docs/cli/install */
export function installerStallDecisionV1(events:readonly Readonly<{at:number;changed:boolean}>[],stallMilliseconds:number=protocol.stallMilliseconds):number|null{
 let last=events.length?events[0]!.at:0;
 for(const event of events){
  if(event.changed)last=event.at;
  else if(event.at-last>=stallMilliseconds)return event.at;
 }
 return null;
}
/** 🧾️ The last HTTP failure an installer reported (`error: GET <url> - 502`), without credentials, query and fragment: the real cause behind an installer that retried until it stalled, `undefined` when its output names none. */
export function installerHttpFailureV1(output:string):string|undefined{
 let found:RegExpExecArray|undefined;
 for(const match of output.matchAll(new RegExp(protocol.httpFailure,"gu")))found=match;
 if(found===undefined)return undefined;
 return `${found[1]} ${found[2]!.replace(/^(https?:\/\/)[^/@]*@/u,"$1")} - ${found[3]}`.slice(0,protocol.maximumFailureCharacters);
}
/** 👁️ The progress signal of an installer: the newest modification time of every probed path (directory, its node_modules and .bin, the package cache); a missing path counts as 0. */
export function installerProgressStampV1(paths:readonly string[]):number{
 let newest=0;
 for(const path of paths)try{newest=Math.max(newest,statSync(path).mtimeMs);}catch{}
 return newest;
}
/** 🚨️ Watches one running installer child and terminates it when its probes stand still for the stall bound; `stalled()` tells afterwards why it ended. */
export function watchInstallerStallV1(options:Readonly<{probes:readonly string[];kill:()=>void;stallMilliseconds?:number;pollMilliseconds?:number;now?:()=>number;stamp?:(paths:readonly string[])=>number}>):Readonly<{stop:()=>void;stalled:()=>boolean}>{
 const now=options.now??Date.now,stamp=options.stamp??installerProgressStampV1,bound=options.stallMilliseconds??protocol.stallMilliseconds;
 let previous=stamp(options.probes),since=now(),fired=false;
 const timer=setInterval(()=>{
  const current=stamp(options.probes),at=now();
  if(current!==previous){previous=current;since=at;return;}
  if(!fired&&at-since>=bound){fired=true;clearInterval(timer);try{options.kill();}catch{}}
 },options.pollMilliseconds??protocol.pollMilliseconds);
 timer.unref?.();
 return Object.freeze({stop:()=>clearInterval(timer),stalled:()=>fired});
}
/** 🔗️ Removes the symbolic links the repository itself bound into a generation (down to `bindingDepth` below `node_modules`), keeping every installed package: Bun reconciles its own workspace links on a rerun, but livelocks in `mkdirat` when a link points elsewhere, so a rerun resumes from the extracted packages instead of from zero. Returns the number of links removed. */
export function releaseInstallerBindingsV1(directory:string):number{
 let removed=0;
 const walk=(path:string,depth:number)=>{
  let names:string[];try{names=readdirSync(path);}catch{return;}
  for(const name of names){
   const child=join(path,name);let stat;try{stat=lstatSync(child);}catch{continue;}
   if(stat.isSymbolicLink()){try{unlinkSync(child);removed++;}catch{}}
   else if(stat.isDirectory()&&depth<protocol.bindingDepth&&name.startsWith("@"))walk(child,depth+1);
  }
 };
 walk(join(directory,"node_modules"),1);
 try{for(const name of readdirSync(join(directory,"w")))walk(join(directory,"w",name,"node_modules"),1);}catch{}
 return removed;
}
