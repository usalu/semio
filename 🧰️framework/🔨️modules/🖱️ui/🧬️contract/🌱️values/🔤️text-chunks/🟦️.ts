import budget from "../../📏️budget/🧬️schema/🔣️.json";

/** 🧶️ Preserves exact UTF8 text in existing bounded scalar pages. */
export function splitUiTextChunks(source:string,maximumBytes:number):string[]{
 const width=budget.const.textMaxBytes,maximum=budget.const.valueMaxItems,encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true}),bytes=encoder.encode(source);
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<0||maximumBytes>width*maximum||bytes.length>maximumBytes||decoder.decode(bytes)!==source)throw Error("UI text source byte limit exceeded");
 const chunks:string[]=[];
 let start=0;
 do{
  if(chunks.length===maximum)throw Error("UI text chunk collection capacity exceeded");
  let end=Math.min(start+width,bytes.length);
  while(end<bytes.length&&(bytes[end]!&0xc0)===0x80)end--;
  chunks.push(decoder.decode(bytes.subarray(start,end)));
  start=end;
 }while(start<bytes.length);
 return chunks;
}

/** 🪡️ Validates bounded wire chunks before reconstructing an owned text source. */
export function joinUiTextChunks(chunks:unknown,maximumBytes:number):string{
 const width=budget.const.textMaxBytes,maximum=budget.const.valueMaxItems,encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true});
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<0||maximumBytes>width*maximum||!Array.isArray(chunks)||chunks.length===0||chunks.length>maximum)throw Error("invalid UI text chunks");
 let total=0;
 for(const chunk of chunks){
  if(typeof chunk!=="string")throw Error("invalid UI text chunk");
  const bytes=encoder.encode(chunk);
  total+=bytes.length;
  if(bytes.length>width||total>maximumBytes||decoder.decode(bytes)!==chunk)throw Error("UI text chunk byte limit exceeded");
 }
 return chunks.join("");
}
