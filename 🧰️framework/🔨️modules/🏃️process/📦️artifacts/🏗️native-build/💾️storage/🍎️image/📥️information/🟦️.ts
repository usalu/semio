import{setTimeout as pause}from"node:timers/promises";

/** 🏷️ Decodes the system tool's plist primitives without a runtime XML library. */
export function nativeImageInformationV1(xml:string):Record<string,unknown>{
 const tokens=xml.replace(/<\?[\s\S]*?\?>|<!DOCTYPE[^>]*>/gu,"").match(/<[^>]+>|[^<]+/gu)??[];let index=0;
 const peek=():string=>{while(index<tokens.length&&!tokens[index]!.trim())index++;return tokens[index]??"";},take=():string=>{const token=peek();index++;return token;};
 const decode=(value:string)=>value.replace(/&(amp|lt|gt|quot|apos);/gu,(_,key)=>({amp:"&",lt:"<",gt:">",quot:'"',apos:"'"} as Record<string,string>)[key]!);
 const value=():unknown=>{const token=take();if(token==="<true/>")return true;if(token==="<false/>")return false;if(token==="<array/>")return[];if(token==="<dict/>")return{};
  if(token==="<dict>"){const result:Record<string,unknown>=Object.create(null);while(peek()!=="</dict>"){if(take()!=="<key>")throw Error("Invalid image plist key");const key=decode(take());if(take()!=="</key>"||Object.hasOwn(result,key))throw Error("Invalid image plist dictionary");result[key]=value();}take();return result;}
  if(token==="<array>"){const result:unknown[]=[];while(peek()!=="</array>"){if(!peek())throw Error("Incomplete image plist array");result.push(value());}take();return result;}
  const scalar=/^<(string|integer|real|data|date)>$/u.exec(token);if(scalar){const text=peek()===`</${scalar[1]}>`?"":take();if(take()!==`</${scalar[1]}>`)throw Error("Invalid image plist scalar");return scalar[1]==="integer"||scalar[1]==="real"?Number(text):decode(text);}
  throw Error("Unsupported system image plist token");};
 if(!take().startsWith("<plist "))throw Error("System image plist required: "+JSON.stringify(xml.trimStart().slice(0,120)));const result=value();if(take()!=="</plist>"||peek()||!result||typeof result!=="object"||Array.isArray(result))throw Error("Invalid system image plist");return result as Record<string,unknown>;
}


export type NativeImageInventoryPolicyV1=Readonly<{attempts:number;backoffMilliseconds:number;signal?:AbortSignal}>;

/** 📸️ Accepts only a noise-free system image document: concurrent attach and detach by any process make the tool print `cannot find IO media` beside a valid plist. */
export function nativeImageInventoryV1(text:string):Record<string,unknown>{
 const start=text.indexOf("<?xml"),end=text.lastIndexOf("</plist>");
 if(start<0||end<0)return nativeImageInformationV1(text);
 const noise=(text.slice(0,start)+text.slice(end+8)).trim();
 if(noise)throw Error("Torn system image inventory: "+JSON.stringify(noise.slice(0,160)));
 return nativeImageInformationV1(text.slice(start,end+8));
}

/** 🔁️ Re-reads the kernel image inventory until a snapshot is whole and satisfies `accept`: a snapshot taken during concurrent attach or detach may drop images or list one mount point twice, while a lasting violation survives every attempt. */
export async function observeNativeImageInventoryV1<T>(capture:()=>Promise<string>,accept:(information:Record<string,unknown>,text:string)=>T,policy:NativeImageInventoryPolicyV1):Promise<T>{
 let failure:unknown;
 for(let attempt=0;attempt<policy.attempts;attempt++){
  if(attempt)await pause(policy.backoffMilliseconds*attempt+process.pid%7*10,undefined,policy.signal?{signal:policy.signal}:undefined);
  let text:string;
  try{text=await capture();}catch(error){if(!(error instanceof Error&&/ exited with status [0-9]+ signal null/u.test(error.message)))throw error;failure=error;continue;}
  try{return accept(nativeImageInventoryV1(text),text);}catch(error){failure=error;}
 }
 throw failure instanceof Error?new Error(failure.message+" ("+policy.attempts+" consistent inventory attempts exhausted)",{cause:failure}):failure;
}
