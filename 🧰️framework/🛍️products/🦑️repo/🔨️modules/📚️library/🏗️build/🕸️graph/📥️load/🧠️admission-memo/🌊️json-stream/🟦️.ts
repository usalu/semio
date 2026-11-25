import type{FileHandle}from"node:fs/promises";

/** 🌊️ One event of a streamed JSON object: a whole field, one element of a field named as streamed, or the end of such a field. */
export type JsonStreamEventV1={key:string;value:unknown}|{key:string;element:unknown}|{key:string;end:true};

const QUOTE=34,BACKSLASH=92,OPEN_OBJECT=123,CLOSE_OBJECT=125,OPEN_ARRAY=91,CLOSE_ARRAY=93,COMMA=44,COLON=58;
const blank=(byte:number)=>byte===32||byte===10||byte===13||byte===9;

/** 🌊️ Reads one JSON object from a file in chunks and yields its fields in file order; the elements of the arrays named in `streamed` are yielded one by one, so no array of them and no text of the whole file exists at once. The text of one field or element is held while it is parsed. A malformed text throws. The bytes are scanned structurally (strings and escapes are tracked, a multi-byte character is decoded only after its whole field is gathered).
 * @see ../🟦️.ts
 * @see ./🧪️tests/🟦️.ts */
export async function* streamJsonObjectV1(file:FileHandle,streamed:ReadonlySet<string>,chunkBytes=1<<20):AsyncGenerator<JsonStreamEventV1>{
  const buffer=Buffer.allocUnsafe(chunkBytes);
  let position=0,state:"start"|"key"|"key-text"|"colon"|"value"|"capture"|"after"|"element"|"capture-element"|"after-element"|"done"="start",key="";
  let parts:Buffer[]=[],depth=0,inString=false,escaped=false,keyBytes:number[]=[];
  const fail=():never=>{throw Error("Malformed streamed JSON object");};
  for(;;){
    const {bytesRead}=await file.read(buffer,0,chunkBytes,position);
    if(!bytesRead)break;
    position+=bytesRead;
    let from=0;
    for(let at=0;at<bytesRead;at++){
      const byte=buffer[at]!;
      if(state==="capture"||state==="capture-element"){
        let end=-1,consumed=true;
        for(;at<bytesRead;at++){
          const each=buffer[at]!;
          if(inString){
            if(escaped)escaped=false;else if(each===BACKSLASH)escaped=true;else if(each===QUOTE){inString=false;if(depth===0){end=at+1;break;}}
          }else if(each===QUOTE)inString=true;
          else if(each===OPEN_OBJECT||each===OPEN_ARRAY)depth++;
          else if(each===CLOSE_OBJECT||each===CLOSE_ARRAY){if(depth===0){end=at;consumed=false;break;}if(--depth===0){end=at+1;break;}}
          else if(depth===0&&(each===COMMA||blank(each))){end=at;consumed=false;break;}
        }
        if(end<0)continue;
        parts.push(Buffer.from(buffer.subarray(from,end)));
        const value=JSON.parse(Buffer.concat(parts).toString("utf8"));parts=[];depth=0;inString=false;escaped=false;
        if(state==="capture"){yield{key,value};state="after";}else{yield{key,element:value};state="after-element";}
        if(!consumed)at--;
        continue;
      }
      if(blank(byte))continue;
      if(state==="start"){if(byte!==OPEN_OBJECT)fail();state="key";}
      else if(state==="after"){
        if(byte===CLOSE_OBJECT)state="done";else if(byte===COMMA)state="key";else fail();
      }else if(state==="key"){
        if(byte===CLOSE_OBJECT){state="done";continue;}
        if(byte!==QUOTE)fail();
        keyBytes=[byte];escaped=false;state="key-text";
      }else if(state==="key-text"){
        keyBytes.push(byte);
        if(escaped)escaped=false;else if(byte===BACKSLASH)escaped=true;else if(byte===QUOTE){key=JSON.parse(Buffer.from(keyBytes).toString("utf8"));state="colon";}
      }else if(state==="colon"){if(byte!==COLON)fail();state="value";}
      else if(state==="value"){
        if(byte===OPEN_ARRAY&&streamed.has(key)){state="element";continue;}
        state="capture";from=at;at--;
      }else if(state==="element"||state==="after-element"){
        if(byte===CLOSE_ARRAY){yield{key,end:true};state="after";continue;}
        if(state==="after-element"){if(byte!==COMMA)fail();state="element";continue;}
        state="capture-element";from=at;at--;
      }else if(state==="done")fail();
    }
    if(state==="capture"||state==="capture-element"){parts.push(Buffer.from(buffer.subarray(from,bytesRead)));}
  }
  if(state!=="done")fail();
}
