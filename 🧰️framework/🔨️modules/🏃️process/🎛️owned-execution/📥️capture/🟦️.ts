import {spawn} from "node:child_process";
import {StringDecoder} from "node:string_decoder";
import {terminateOwnedChildTree} from "../../🪓️termination/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import outputSchema from "../🧬️schema/📬️output.json" with {type:"json"};

/** 📪️ Preserves both complete UTF8 streams and the actual native terminal values. */
export type OwnedCommandCaptureV1=Readonly<{version:1;stdout:string;stderr:string;status:number|null;signal:string|null}>;
/** 🎛️ Supplies finite output custody and the actual existing operation control. */
export type OwnedCommandCaptureOptionsV1=Readonly<{control:ResourceLeaseControl;output:Readonly<{maximumBytes:number;maximumLines:number}>;env?:Readonly<Record<string,string|undefined>>}>;
type Options=Readonly<{env?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;control?:ResourceLeaseControl;output?:Readonly<{maximumBytes:number;maximumLines:number}>}>;

/** 🔐️ Admits separated terminal evidence without replacing missing native status or signal. */
export function readOwnedCommandCaptureV1(value:unknown):OwnedCommandCaptureV1{if(validateJsonSchemaSubset(schema,value).length)throw Error("Complete owned separated capture required");return Object.freeze({...value as OwnedCommandCaptureV1});}

/** 📤️ Accepts one already bounded captured payload without exposing a platform stream type. */
export interface OwnedCommandOutputSinkV1{write(text:string,complete:(error?:Error|null)=>void):unknown}
/** 🪢️ Awaits actual payload delivery under the same nonrenewable Original deadline and cancellation. */
export async function writeOwnedCommandOutputV1(sink:OwnedCommandOutputSinkV1,text:string,control:ResourceLeaseControl):Promise<void>{
 checkResourceLeaseControl(control);if(typeof sink.write!=="function"||typeof text!=="string")throw Error("Actual owned output sink and payload required");const remaining=control.remainingMilliseconds();
 await new Promise<void>((accept,reject)=>{let settled=false,timer:ReturnType<typeof setTimeout>|undefined;const finish=(error?:unknown)=>{if(settled)return;settled=true;if(timer)clearTimeout(timer);control.signal.removeEventListener("abort",abort);error?reject(error):accept();},abort=()=>finish(control.signal.reason??Error("Original output cancelled"));control.signal.addEventListener("abort",abort,{once:true});if(remaining!==null)timer=setTimeout(()=>finish(Error("Original output deadline exhausted")),remaining);if(control.signal.aborted){abort();return;}try{sink.write(text,error=>finish(error));}catch(error){finish(error);}});
 checkResourceLeaseControl(control);
}

/** 🧺️ Shares one actual process owner between bounded separated results and bounded diagnostic tails. */
export async function captureOwnedCommandBodyV1(command:string,args:readonly string[],cwd:string,options:Options,separated:boolean):Promise<Readonly<{result:OwnedCommandCaptureV1;tail:string}>>{
 if(options.output&&validateJsonSchemaSubset(outputSchema,options.output).length)throw Error("Owned data capture requires complete output limits");
 if(separated&&(!options.control||!options.output||options.output.maximumBytes>16777216||options.output.maximumLines>65536))throw Error("Separated capture requires finite Original control and bounded output");
 const signal=options.control?.signal??options.signal;
 if(options.control){const remaining=checkResourceLeaseControl(options.control);if(options.signal&&options.signal!==signal)throw Error("Owned capture requires its original signal");if(remaining!==null&&Math.floor(remaining)<1)throw Error("Original compiler admission deadline exhausted");}
 signal?.throwIfAborted();
 const child=spawn(command,[...args],{cwd,env:(options.env??process.env) as NodeJS.ProcessEnv,shell:false,detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"],windowsHide:true}),started=performance.now(),streams={stdout:"",stderr:""},open={stdout:false,stderr:false},decoders={stdout:new StringDecoder("utf8"),stderr:new StringDecoder("utf8")},strict={stdout:new TextDecoder("utf-8",{fatal:true}),stderr:new TextDecoder("utf-8",{fatal:true})};
 let tail="",bytes=0,lines=0,stopped:unknown,closed=false,timer:ReturnType<typeof setTimeout>|undefined,publication=Promise.resolve();
 const stop=(cause:unknown)=>{stopped??=cause;if(!closed)terminateOwnedChildTree(child);},abort=()=>stop(signal?.reason??Error("Original capture cancelled"));signal?.addEventListener("abort",abort,{once:true});if(signal?.aborted)abort();
 const awaitOriginal=async(body:Promise<void>)=>{const control=options.control!;const remaining=checkResourceLeaseControl(control);let expiry:ReturnType<typeof setTimeout>|undefined,remove=()=>{};const interrupted=new Promise<never>((_,reject)=>{const abort=()=>reject(control.signal.reason);control.signal.addEventListener("abort",abort,{once:true});remove=()=>control.signal.removeEventListener("abort",abort);if(control.signal.aborted)abort();if(remaining!==null)expiry=setTimeout(()=>reject(Error("Original capture deadline exhausted")),remaining);});try{await Promise.race([body,interrupted]);checkResourceLeaseControl(control);}finally{clearTimeout(expiry);remove();}};
 const progress=async()=>{if(closed||!options.control)return;try{const remaining=checkResourceLeaseControl(options.control);if(separated){await awaitOriginal(Promise.resolve(options.control.publish({resource:command,mode:"shared",elapsedMs:performance.now()-started})));await awaitOriginal(options.control.yieldContinuation());}if(!closed)timer=setTimeout(()=>{publication=progress();},Math.min(remaining??1000,1000));}catch(error){stop(error);}};
 if(options.control){const remaining=checkResourceLeaseControl(options.control);timer=setTimeout(()=>{publication=progress();},Math.min(remaining??1000,1000));}
 const append=(text:string,stream:"stdout"|"stderr")=>{streams[stream]=separated?streams[stream]+text:(streams[stream]+text).slice(-65536);tail=separated?"":options.output?tail+text:(tail+text).slice(-65536);};
 const capture=(chunk:Buffer,stream:"stdout"|"stderr")=>{if(stopped!==undefined)return;try{if(options.output){bytes+=chunk.length;for(const byte of chunk){if(!open[stream])lines++;open[stream]=byte!==10;}if(bytes>options.output.maximumBytes||lines>options.output.maximumLines)throw Error("Owned data capture output limit exceeded");}append(separated?strict[stream].decode(chunk,{stream:true}):decoders[stream].write(chunk),stream);}catch(error){stop(error);}};
 for(const stream of ["stdout","stderr"] as const){child[stream]!.on("data",chunk=>capture(chunk,stream));child[stream]!.once("end",()=>{if(stopped!==undefined)return;try{append(separated?strict[stream].decode():decoders[stream].end(),stream);}catch(error){stop(error);}});}
 const result=await new Promise<{status:number|null;signal:string|null}>(accept=>{child.once("error",error=>stop(error));child.once("close",(status,signal)=>{closed=true;accept({status,signal});});});
 clearTimeout(timer);signal?.removeEventListener("abort",abort);await publication;
 if(stopped!==undefined)throw stopped;if(options.control)checkResourceLeaseControl(options.control);signal?.throwIfAborted();
 return{result:readOwnedCommandCaptureV1({version:1,...streams,...result}),tail};
}

/** 📥️ Captures finite separate streams while joining the actual child before every result or refusal. */
export async function captureOwnedCommandResultV1(command:string,args:readonly string[],cwd:string,options:OwnedCommandCaptureOptionsV1):Promise<OwnedCommandCaptureV1>{return(await captureOwnedCommandBodyV1(command,args,cwd,options,true)).result;}
