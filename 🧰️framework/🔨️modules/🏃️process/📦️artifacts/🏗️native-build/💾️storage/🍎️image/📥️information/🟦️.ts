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
 if(!take().startsWith("<plist "))throw Error("System image plist required");const result=value();if(take()!=="</plist>"||peek()||!result||typeof result!=="object"||Array.isArray(result))throw Error("Invalid system image plist");return result as Record<string,unknown>;
}

