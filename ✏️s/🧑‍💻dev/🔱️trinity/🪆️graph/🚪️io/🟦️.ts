/** 📦️ Specific Jack foreign carriers preserve the complete original native archive. */
import {decodeJsonSyntax} from "@semio-tech/pack-json-decode-ts";
import {JsonMemberPolicy} from "@semio-tech/framework-pack-json-ts/members";
export type JackArchiveFormat="json"|"txt";
export interface JackArchiveControl {admit(bytes:number):void;checkpoint(completed:number,total:number):void;yield():Promise<void>;}
export const JACK_ARCHIVE_SCHEMA="s.dev.trinity.jack.archive.v1";
export const JACK_ARCHIVE_MAXIMUM_BYTES=4*1024*1024;
const hex="0123456789abcdef";
function archiveExtent(bytes:Uint8Array):void{if(!bytes.length||bytes.length>JACK_ARCHIVE_MAXIMUM_BYTES||bytes[0]!==1)throw Error("Jack requires the original versioned native document archive");}
export function encodeJackArchive(bytes:Uint8Array,format:JackArchiveFormat,control:JackArchiveControl):string{
 archiveExtent(bytes);const prefix=format==="json"?'{"schema":"'+JACK_ARCHIVE_SCHEMA+'","archive":"':JACK_ARCHIVE_SCHEMA+"\n",suffix=format==="json"?'"}':"\n";
 control.admit((bytes.length*2+prefix.length+suffix.length)*2);control.checkpoint(0,bytes.length);let output=prefix;
 for(let start=0;start<bytes.length;start+=256){const end=Math.min(start+256,bytes.length);let part="";for(let index=start;index<end;index++)part+=hex[bytes[index]!>>>4]+hex[bytes[index]!&15];output+=part;control.checkpoint(end,bytes.length);}return output+suffix;
}
export async function decodeJackArchive(text:string,format:JackArchiveFormat,control:JackArchiveControl):Promise<Uint8Array>{
 if(text.length>JACK_ARCHIVE_MAXIMUM_BYTES*2+128)throw Error("Jack archive text extent exceeds the authored ceiling");control.admit(text.length*2);control.checkpoint(0,text.length);
 let encoded:string;
 if(format==="json"){let completed=0;const value=await decodeJsonSyntax(text,JsonMemberPolicy.Reject,{maximumBytes:JACK_ARCHIVE_MAXIMUM_BYTES*2+128,maximumNodes:3,maximumDepth:2,chunk:256,cancelled:()=>{control.checkpoint(completed,text.length);return false;},progress:event=>{completed=event.bytes;control.checkpoint(completed,text.length);},yield:()=>control.yield()});if(value.kind!=="object"||value.members.length!==2)throw Error("Jack archive closed object required");const schema=value.members.find(field=>field.name==="schema")?.value,archive=value.members.find(field=>field.name==="archive")?.value;if(schema?.kind!=="string"||schema.value!==JACK_ARCHIVE_SCHEMA||archive?.kind!=="string")throw Error("Jack archive closed schema refused");encoded=archive.value;}
 else{const prefix=JACK_ARCHIVE_SCHEMA+"\n";if(!text.startsWith(prefix)||!text.endsWith("\n"))throw Error("Jack archive text framing refused");encoded=text.slice(prefix.length,-1);}
 if(!encoded.length||encoded.length%2||encoded.length>JACK_ARCHIVE_MAXIMUM_BYTES*2)throw Error("Jack archive hex extent refused");
 control.admit(encoded.length/2);control.checkpoint(0,encoded.length);const bytes=new Uint8Array(encoded.length/2);
 for(let start=0;start<encoded.length;start+=512){const end=Math.min(start+512,encoded.length);for(let index=start;index<end;index+=2){const high=hex.indexOf(encoded[index]!),low=hex.indexOf(encoded[index+1]!);if(high<0||low<0)throw Error("Jack archive lowercase octets required");bytes[index/2]=(high<<4)|low;}control.checkpoint(end,encoded.length);}archiveExtent(bytes);return bytes;
}
