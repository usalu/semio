import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
/** 🌊️ Writes a JSON value as the exact text of `JSON.stringify(value, null, indent)` in pieces of about `chunkCharacters` characters, never holding more than one piece plus the recursion path: the whole expanded graph is gigabytes of text and cannot be one string. `write` may return a promise (backpressure); `signal` cancels between pieces. */
export async function writeJsonStreamV1(value,write,{indent=2,chunkCharacters=65536,signal}={}){
 if(!Number.isSafeInteger(indent)||indent<0||indent>10)throw refuseBuildV1("invalid","json.indent-out-of-range");
 const unit=" ".repeat(indent);let buffer="",pieces=0;
 const emit=async text=>{buffer+=text;if(buffer.length>=chunkCharacters){signal?.throwIfAborted();const out=buffer;buffer="";pieces++;await write(out);}};
 const resolve=(owner,key,item)=>{if(item!==null&&typeof item==="object"&&typeof item.toJSON==="function")item=item.toJSON(key);return typeof item==="function"||typeof item==="symbol"?undefined:item;};
 const walk=async(item,depth)=>{
  if(item===null||typeof item!=="object"){await emit(JSON.stringify(item));return;}
  const inner=indent?"\n"+unit.repeat(depth+1):"",outer=indent?"\n"+unit.repeat(depth):"";
  if(Array.isArray(item)){
   if(!item.length){await emit("[]");return;}
   await emit("[");
   for(let at=0;at<item.length;at++){await emit((at?",":"")+inner);const child=resolve(item,String(at),item[at]);if(child===undefined)await emit("null");else await walk(child,depth+1);}
   await emit(outer+"]");return;
  }
  const keys=Object.keys(item).filter(key=>resolve(item,key,item[key])!==undefined);
  if(!keys.length){await emit("{}");return;}
  await emit("{");
  for(let at=0;at<keys.length;at++){const key=keys[at];await emit((at?",":"")+inner+JSON.stringify(key)+(indent?": ":":"));await walk(resolve(item,key,item[key]),depth+1);}
  await emit(outer+"}");
 };
 const top=resolve({"":value},"",value);
 if(top===undefined)throw refuseBuildV1("missing","json.value-required");
 await walk(top,0);
 if(buffer){signal?.throwIfAborted();pieces++;await write(buffer);}
 return pieces;
}
