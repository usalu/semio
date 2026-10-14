import {readFileSync} from "node:fs";
const schema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8"));
const nativeSchema=JSON.parse(readFileSync(new URL("../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",import.meta.url),"utf8"));
const key="SEMIO_OWNER_ARGUMENTS";

/** 🧬️ Admits the closed native caller and command contracts in the Node executor. */
export function conformsOwnerContractV1(contract,value){
 if(contract.const!==undefined&&value!==contract.const)return false;
 if(contract.type==="object"){if(value===null||typeof value!=="object"||Array.isArray(value))return false;const keys=Object.keys(value);if(contract.required?.some(key=>!Object.hasOwn(value,key)))return false;if(contract.additionalProperties===false&&keys.some(key=>!Object.hasOwn(contract.properties??{},key)))return false;return Object.entries(contract.properties??{}).every(([key,rule])=>!Object.hasOwn(value,key)||conformsOwnerContractV1(rule,value[key]));}
 if(contract.type==="array")return Array.isArray(value)&&(contract.minItems===undefined||value.length>=contract.minItems)&&(contract.maxItems===undefined||value.length<=contract.maxItems)&&value.every(item=>conformsOwnerContractV1(contract.items,item));
 if(contract.type==="integer")return Number.isSafeInteger(value)&&(contract.minimum===undefined||value>=contract.minimum)&&(contract.maximum===undefined||value<=contract.maximum);
 if(contract.type==="boolean")return typeof value==="boolean";
 if(contract.type==="string")return typeof value==="string"&&(contract.minLength===undefined||value.length>=contract.minLength)&&(contract.maxLength===undefined||value.length<=contract.maxLength)&&(!contract.pattern||new RegExp(contract.pattern,"u").test(value));
 return Object.hasOwn(contract,"const");
}

/** 🔤️ Reads a literal process command without evaluating shell operators or substitutions. */
export function literalCommandTokensV1(command){
 const tokens=[];let cursor=0;
 while(cursor<command.length){while(/\s/u.test(command[cursor]??"")&&cursor<command.length)cursor++;if(cursor===command.length)break;const start=cursor;
  if(command[cursor]==='"'){cursor++;while(cursor<command.length){if(command[cursor]==="\\"){cursor+=2;continue;}if(command[cursor++]==='"')break;}tokens.push(JSON.parse(command.slice(start,cursor)));}
  else if(command[cursor]==="'"){cursor++;const end=command.indexOf("'",cursor);if(end<0)throw Error("Unclosed original native command");tokens.push(command.slice(cursor,end));cursor=end+1;}
  else{while(cursor<command.length&&!/\s/u.test(command[cursor]))cursor++;const token=command.slice(start,cursor);if(/[;&|<>`$"']/u.test(token))throw Error("Native command requires literal discovered arguments");tokens.push(token);}
  if(cursor<command.length&&!/\s/u.test(command[cursor]))throw Error("Native command requires distinct literal arguments");
 }
 return tokens;
}

/** 📜️ Binds one discovered literal native child command without shell evaluation. */
export function declaredNativeOwnerCommandV1(manifest,workingDirectory,command){
 const tokens=literalCommandTokensV1(command),result={kind:"native-owner-command",manifest,workingDirectory,program:tokens[0],arguments:tokens.slice(1)};
 if(!conformsOwnerContractV1(nativeSchema.properties.command,result))throw Error("Original discovered native command refused");return result;
}

/** 🛡️ Admits literal process arguments against their owned envelope authority. */
function admit(value){
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).length!==schema.required.length||value.version!==schema.properties.version.const||!Array.isArray(value.arguments)||value.arguments.length>schema.properties.arguments.maxItems||value.arguments.some(argument=>typeof argument!=="string"||argument.length>schema.properties.arguments.items.maxLength||argument.includes("\u0000")))throw Error("Invalid owner argument envelope");
 return [...value.arguments];
}

/** 📤️ Encodes raw argv as environment data independently of platform shell syntax. */
export function encodeOwnerArgumentsV1(arguments_){
 const value={version:1,arguments:arguments_};admit(value);return JSON.stringify(value);
}

/** 📥️ Removes the private envelope before handing unchanged arguments to the actual child owner. */
export function consumeOwnerArgumentsV1(environment){
 const next={...environment},encoded=next[key];delete next[key];
 if(encoded===undefined)return {arguments:[],environment:next};
 return {arguments:admit(JSON.parse(encoded)),environment:next};
}

