import {ValueError} from "../⚠️refusal/🟦️.ts";

/** 🔤️ Measures one owned Unicode scalar without materializing encoded bytes. */
export function textScalarUtf8Width(text:string,index:number):readonly[number,number]{
  const word=text.charCodeAt(index);
  if(!Number.isInteger(index)||index<0||index>=text.length)throw new ValueError("invalidValue","Invalid Unicode scalar position");
  if(word<0x80)return[1,1];
  if(word<0x800)return[2,1];
  if(word>=0xd800&&word<=0xdbff){const next=text.charCodeAt(index+1);if(next>=0xdc00&&next<=0xdfff)return[4,2];}
  else if(word<0xdc00||word>0xdfff)return[3,1];
  throw new ValueError("invalidValue","Unicode text has an unpaired surrogate");
}

/** 📏️ Counts validated Unicode text bytes without allocating an encoded copy. */
export function textUtf8ByteLength(text:string):number{
  let bytes=0;
  for(let index=0;index<text.length;index++){
    const word=text.charCodeAt(index);
    if(word>=0xd800&&word<=0xdbff){const next=text.charCodeAt(++index);if(!(next>=0xdc00&&next<=0xdfff))throw new ValueError("invalidValue","Unicode text has an unpaired surrogate");bytes+=4;}
    else{if(word>=0xdc00&&word<=0xdfff)throw new ValueError("invalidValue","Unicode text has an unpaired surrogate");bytes+=word<0x80?1:word<0x800?2:3;}
  }
  return bytes;
}
