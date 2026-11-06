import Ajv from "ajv";
import {inflateSync} from "node:zlib";
import corpus from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
if(!new Ajv().validate(schema,corpus))throw Error("invalid PNG text corpus");
function decode(kind:string,hex:string):[string,string]{const bytes=Buffer.from(hex,"hex"),zero=(start:number)=>{const end=bytes.indexOf(0,start);if(end<0)throw Error("separator");return end;},first=zero(0),key=bytes.subarray(0,first).toString("latin1"),utf8=(data:Uint8Array)=>new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(data);if(!key||key.length>79)throw Error("keyword");if(kind==="tEXt")return[key,bytes.subarray(first+1).toString("latin1")];if(kind==="zTXt"){if(bytes[first+1]!==0)throw Error("method");return[key,inflateSync(bytes.subarray(first+2)).toString("latin1")]};const flag=bytes[first+1],method=bytes[first+2];if((flag!==0&&flag!==1)||method!==0)throw Error("compression");const languageEnd=zero(first+3),translatedEnd=zero(languageEnd+1);if(bytes.subarray(first+3,languageEnd).some(byte=>byte>127))throw Error("language");utf8(bytes.subarray(languageEnd+1,translatedEnd));const text=bytes.subarray(translatedEnd+1);return[key,utf8(flag?inflateSync(text):text)];}
for(const row of corpus){let actual:unknown;try{actual=row.chunks.map(chunk=>decode(chunk.kind,chunk.hex))}catch{actual=null}if(JSON.stringify(actual)!==JSON.stringify(row.expected))throw Error(row.name)}
console.log(`PNG text oracle: ${corpus.length} cases`);
