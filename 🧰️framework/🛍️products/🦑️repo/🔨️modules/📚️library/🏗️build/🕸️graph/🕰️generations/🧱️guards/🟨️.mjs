import{createHash}from"node:crypto";
import{brotliDecompressSync}from"node:zlib";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};

/** 📏️ The declared names and limits of the graph generation store; the schema file is their single source. */
export const GENERATION_POLICY_V1=Object.freeze({...schema.definitions.Policy.const,policies:Object.freeze([...schema.definitions.Policy.const.policies])});
const P=GENERATION_POLICY_V1,HEX=/^[a-f0-9]{64}$/u,sha256=bytes=>createHash("sha256").update(bytes).digest("hex"),isObject=value=>value!==null&&typeof value==="object"&&!Array.isArray(value);

const unpack=(bytes,limit)=>JSON.parse(brotliDecompressSync(bytes,{maxOutputLength:limit}).toString("utf8"));
const aborted=signal=>signal?.throwIfAborted();
const fail=(message,code)=>Object.assign(Error(message),code?{code}:{});

function requireKeys(value,required,optional,what){
 if(!isObject(value))throw fail("Closed "+what+" required");
 for(const key of required)if(!Object.hasOwn(value,key))throw fail("Closed "+what+" required: missing "+key);
 for(const key of Object.keys(value))if(!required.includes(key)&&!optional.includes(key))throw fail("Closed "+what+" required: unknown "+key);
}
const requireHex=(value,what)=>{if(typeof value!=="string"||!HEX.test(value))throw fail("Digest required: "+what);};
const requireCount=(value,what,maximum=Number.MAX_SAFE_INTEGER)=>{if(!Number.isSafeInteger(value)||value<0||value>maximum)throw fail("Bounded count required: "+what);};
const requireText=(value,what,maximum=P.textMaximumCharacters)=>{if(typeof value!=="string"||!value||value.length>maximum)throw fail("Bounded text required: "+what);};
/** ⏱️ A yielder for one long loop: `await yielder()` is free until `yieldMilliseconds` of the loop have passed, then gives the event loop one macrotask turn. */
const createYielderV1=()=>{let since=performance.now();return async()=>{if(performance.now()-since<P.yieldMilliseconds)return;await new Promise(resolve=>setImmediate(resolve));since=performance.now();};};
export{createYielderV1,P,HEX,sha256,isObject,unpack,aborted,fail,requireKeys,requireHex,requireCount,requireText};
