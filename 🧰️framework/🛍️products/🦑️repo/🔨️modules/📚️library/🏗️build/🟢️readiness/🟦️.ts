import {request} from "node:http";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import componentSchema from "../🧬️schema/🔣️.json";

const policy=componentSchema.$defs.ReadinessPolicy.const,schemaDocument=componentSchema as object,HOSTS="(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0)";

/** 🟢️ A declared readiness: the port or its environment variable, the path after the printed address, and whether the printed address is taken whole. */
export interface ReadinessDeclarationV1{port?:number;portEnv?:string;path?:string;printed?:boolean}
/** 🎯 The readiness of one run once its port is known. */
export interface ReadinessV1{port:number;path:string;printed?:true}
/** 👀 Receives the visible output of a task and decides, as a promise, when its address answers. */
export interface ReadinessWatchV1{observe(output:string):void;end():void;stop():void;readonly ready:Promise<string>}
/** 🔌 Asks one announced local address whether its HTTP handler answers a 2xx or 3xx status. */
export type ReadinessProbeV1=(address:string,signal:AbortSignal)=>Promise<boolean>;

/** 📣 The line a task of any build tool prints once its announced address answers; the build tool awaits it. */
export const readinessMarkerV1:string=policy.marker;

/** 🛂 Admits one declared readiness. */
export function admitReadinessDeclarationV1(declared:unknown,identity:string):ReadinessDeclarationV1{
 const errors=validateJsonSchemaSubset(componentSchema.$defs.ReadyDeclaration,declared,schemaDocument);
 if(errors.length)throw Error(`Invalid readiness declaration for ${identity}: ${errors.slice(0,3).join("; ")}`);
 return declared as ReadinessDeclarationV1;
}

/** 📖️ Reads and validates the readiness a target declares at `metadata.semio.dashboard.ready`. */
export function readReadinessDeclarationV1(metadata:unknown,identity:string):ReadinessDeclarationV1|undefined{
 const declared=(metadata as {semio?:{dashboard?:{ready?:unknown}}}|undefined)?.semio?.dashboard?.ready;
 return declared===undefined?undefined:admitReadinessDeclarationV1(declared,identity);
}

/** 🧭 Resolves the port of one run: the value of `portEnv` when the environment carries one, else the declared `port`. */
export function resolveReadinessV1(declaration:ReadinessDeclarationV1,environment:Readonly<Record<string,string|undefined>>,identity:string):ReadinessV1{
 const stated=declaration.portEnv===undefined?undefined:environment[declaration.portEnv];
 let port:number;
 if(stated!==undefined&&stated!==""){
  if(!/^[0-9]{1,5}$/u.test(stated)||Number(stated)<1||Number(stated)>65535)throw Error(`${identity}: ready port ${JSON.stringify(stated)} is not a port`);
  port=Number(stated);
 }else if(declaration.port!==undefined)port=declaration.port;
 else throw Error(`${identity}: ready.portEnv ${JSON.stringify(declaration.portEnv)} has no value; set it or declare \`port\``);
 const ready:ReadinessV1={port,path:declaration.path??"",...(declaration.printed===true?{printed:true}:{})};
 if(validateJsonSchemaSubset(componentSchema.$defs.Ready,ready,schemaDocument).length)throw Error(`${identity}: resolved readiness refused`);
 return ready;
}

/** 🔎 Finds the first announced address in visible output however the output is cut into chunks. */
export class ReadinessMatcherV1{
 readonly #ready:ReadinessV1;readonly #pattern:RegExp;#line="";#found:string|undefined;#done=false;#state:""|"escape"|"intermediate"|"control"|"string"|"stringEscape"="";
 constructor(ready:ReadinessV1){this.#ready=ready;this.#pattern=new RegExp(`http://${HOSTS}:${ready.port}(?![0-9])${ready.printed?"\\S*":""}`,"u");}

 /** 📜 Reads raw terminal text: escape sequences vanish, a carriage return, line feed or tab ends the line. */
 push(output:string):void{
  if(this.#done)return;
  for(const character of output){
   if(this.#done)return;
   const code=character.codePointAt(0)!;
   switch(this.#state){
    case "escape":
     if(character==="[")this.#state="control";
     else if(character==="]"||character==="P"||character==="X"||character==="^"||character==="_")this.#state="string";
     else if(code>=0x20&&code<=0x2f)this.#state="intermediate";
     else this.#state="";
     continue;
    case "intermediate":if(code>=0x30&&code<=0x7e)this.#state="";continue;
    case "control":if(code>=0x40&&code<=0x7e)this.#state="";else if(code===0x1b)this.#state="escape";continue;
    case "string":if(code===0x07||code===0x9c)this.#state="";else if(code===0x1b)this.#state="stringEscape";continue;
    case "stringEscape":this.#state=character==="\\"?"":character==="]"?"string":"escape";continue;
    default:
   }
   if(code===0x1b){this.#state="escape";continue;}
   if(character==="\r"||character==="\n"||character==="\t"){this.endLine();continue;}
   if(code<0x20||code>=0x7f&&code<=0x9f)continue;
   this.#line+=character;
  }
  this.#search(false);
 }

 /** ↩️ The visible text read so far ends here: an address that reaches this end is decided. */
 endLine():void{this.#search(true);this.#line="";}

 /** 🏁 The output ended: nothing further can change a pending address. */
 end():void{this.#search(true);this.#line="";this.#state="";}

 /** 🔭 Hands out the announced address once; a matcher finds one address in its life. */
 take():string|undefined{const found=this.#found;this.#found=undefined;return found;}

 #search(final:boolean):void{
  if(this.#done||!this.#line)return;
  const match=this.#pattern.exec(this.#line);
  if(!match){
   if(this.#line.length>policy.maximumLineCharacters)this.#line=this.#line.slice(-policy.keptLineCharacters);
   return;
  }
  const end=match.index+match[0].length;
  if(!final&&end===this.#line.length&&(!this.#ready.printed||this.#line.length-match.index<=policy.maximumAddressCharacters))return;
  this.#found=this.#ready.printed?match[0]:match[0]+this.#ready.path;this.#done=true;
 }
}

/** 🌐️ Probes the exact announced local address with one bounded GET and accepts any 2xx or 3xx status. */
export const probeReadinessV1:ReadinessProbeV1=async(address,signal)=>{
 const parsed=new RegExp(`^http://(${HOSTS}):([0-9]{1,5})([^]*)$`,"u").exec(address);
 if(!parsed)return false;
 const host=parsed[1]!,port=Number(parsed[2]),tail=parsed[3]!.split("#")[0]!,target=tail.startsWith("/")?tail:"/"+tail;
 if(port<1||port>65535||/[\u0000- \u007f]/u.test(target))return false;
 const addresses=host==="localhost"?["::1","127.0.0.1"]:["127.0.0.1"],deadline=Date.now()+policy.probeBudgetMilliseconds;
 for(const candidate of addresses){
  if(signal.aborted)return false;
  const remaining=deadline-Date.now();
  if(remaining<=0)return false;
  const status=await new Promise<number|undefined>(accept=>{
   const controller=new AbortController(),stop=()=>controller.abort(),timer=setTimeout(stop,remaining);
   signal.addEventListener("abort",stop,{once:true});
   const finish=(value:number|undefined)=>{clearTimeout(timer);signal.removeEventListener("abort",stop);accept(value);};
   const outgoing=request({host:candidate,port,path:target,method:"GET",agent:false,signal:controller.signal,headers:{Host:`${host}:${port}`,Connection:"close"},maxHeaderSize:policy.maximumHeaderBytes},response=>{const code=response.statusCode;response.destroy();finish(code);});
   outgoing.once("error",()=>finish(undefined));
   outgoing.end();
  });
  if(status!==undefined&&status>=200&&status<400)return true;
 }
 return false;
};

/** 👁️ Watches the output of one task and probes its announced address with a growing delay until it answers or the watch stops. */
export function watchReadinessV1(ready:ReadinessV1,parent:AbortSignal,probe:ReadinessProbeV1=probeReadinessV1,sleep:(milliseconds:number,signal:AbortSignal)=>Promise<void>=pause):ReadinessWatchV1{
 const matcher=new ReadinessMatcherV1(ready),controller=new AbortController(),abort=()=>controller.abort(parent.reason);
 parent.addEventListener("abort",abort,{once:true});if(parent.aborted)abort();
 let accept!:(address:string)=>void,probing=false;
 const answered=new Promise<string>(resolve=>{accept=resolve;});
 const probeUntilAnswered=async(address:string):Promise<void>=>{
  let delay=policy.firstProbeMilliseconds;
  while(!controller.signal.aborted){
   if(await probe(address,controller.signal)){accept(address);stop();return;}
   await sleep(delay,controller.signal);delay=Math.min(delay*2,policy.lastProbeMilliseconds);
  }
 };
 const announce=()=>{const address=matcher.take();if(address!==undefined&&!probing){probing=true;void probeUntilAnswered(address);}};
 const stop=()=>{parent.removeEventListener("abort",abort);controller.abort(Error("Readiness watch stopped"));};
 return{
  observe(output){matcher.push(output);announce();},
  end(){matcher.end();announce();},
  stop,
  ready:answered
 };
}

/** ⏱️ Waits a bounded time and ends early when the watch stops. */
function pause(milliseconds:number,signal:AbortSignal):Promise<void>{
 return new Promise(accept=>{
  let timer:ReturnType<typeof setTimeout>|undefined;
  const done=()=>{clearTimeout(timer);signal.removeEventListener("abort",done);accept();};
  timer=setTimeout(done,milliseconds);
  signal.addEventListener("abort",done,{once:true});if(signal.aborted)done();
 });
}
