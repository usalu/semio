import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🔭️ The declared constants of the verified stamp; the schema file is the single source and spells the bitmap layout. */
export const COVERAGE_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
const P=COVERAGE_POLICY_V1,BASE64=/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/u,fail=message=>Error(message),isObject=value=>value!==null&&typeof value==="object"&&!Array.isArray(value);

/** 🗺️ Encodes the covered owners as a bitmap over the owner names of one manifest (code point order, least significant bit first). */
export function encodeVerifiedScopeV1(owners,covered){
 if(!Array.isArray(owners)||owners.length>P.maximumOwners)throw fail("Bounded owner list required for a verified scope");
 const bytes=Buffer.alloc(Math.ceil(owners.length/8));
 owners.forEach((name,at)=>{if(covered.has(name))bytes[at>>3]|=1<<(at&7);});
 return{count:owners.length,owners:bytes.toString("base64")};
}

/** 🔍️ Admits a closed verified stamp: an exact time, the owner count and the bitmap of exactly that many bits. */
export function readVerifiedStampV1(value){
 if(!isObject(value))throw fail("Closed verified stamp required");
 for(const key of Object.keys(value))if(!["atMs","count","owners"].includes(key))throw fail("Closed verified stamp required: unknown "+key);
 for(const key of["atMs","count","owners"])if(!Object.hasOwn(value,key))throw fail("Closed verified stamp required: missing "+key);
 if(!Number.isSafeInteger(value.atMs)||value.atMs<0)throw fail("Epoch milliseconds required in a verified stamp");
 if(!Number.isSafeInteger(value.count)||value.count<0||value.count>P.maximumOwners)throw fail("Bounded owner count required in a verified stamp");
 if(typeof value.owners!=="string"||!BASE64.test(value.owners))throw fail("Base64 bitmap required in a verified stamp");
 const bytes=Buffer.from(value.owners,"base64");
 if(bytes.length!==Math.ceil(value.count/8)||bytes.toString("base64")!==value.owners)throw fail("Bitmap of exactly the owner count required in a verified stamp");
 if(value.count%8!==0&&bytes[bytes.length-1]>>(value.count%8)!==0)throw fail("Zero padding bits required in a verified stamp");
 return Object.freeze({atMs:value.atMs,count:value.count,owners:value.owners});
}

/** 📖️ Reads the covered owner names back from a stamp and the owner names of the manifest it was published with. */
export function decodeVerifiedScopeV1(owners,stamp){
 readVerifiedStampV1(stamp);
 if(owners.length!==stamp.count)throw fail("Verified stamp differs from the owners of its manifest");
 const bytes=Buffer.from(stamp.owners,"base64"),names=new Set();
 owners.forEach((name,at)=>{if(bytes[at>>3]>>(at&7)&1)names.add(name);});
 return names;
}

/** ➕️ The stamp of a build: the owners it verified (`scope`, every owner when undefined) at the time its observation began, united with the part of its parent's stamp it reused (`carried`) at the older of the two times; owners that left the manifest drop out. */
export function mergeVerifiedStampV1({owners,scope,observedAtMs,carried}){
 if(!Number.isSafeInteger(observedAtMs)||observedAtMs<0)throw fail("Epoch milliseconds required for a verified stamp");
 const names=scope===undefined?new Set(owners):new Set(scope);
 if(scope!==undefined&&carried)for(const name of carried.names)names.add(name);
 const kept=new Set(owners.filter(name=>names.has(name))),atMs=scope!==undefined&&carried?Math.min(carried.atMs,observedAtMs):observedAtMs;
 return readVerifiedStampV1({atMs,...encodeVerifiedScopeV1(owners,kept)});
}

/** 🔭️ Answers what an already sealed generation covers for a request that began at `requestedAtMs` under `implementation`: nothing without a stamp, a younger request than the stamp, or another implementation; otherwise the owners of the stamp. */
export function coverageOfV1({stamp,owners,implementation,manifestImplementation,requestedAtMs}){
 if(stamp===undefined||manifestImplementation!==implementation||!Number.isFinite(requestedAtMs)||stamp.atMs<requestedAtMs||owners.length!==stamp.count)return undefined;
 return Object.freeze({atMs:stamp.atMs,names:decodeVerifiedScopeV1(owners,stamp)});
}

/** ➖️ The part of `needed` a coverage leaves open (every owner of the manifest when `needed` is undefined); an empty set means the request is answered. */
export function uncoveredOfV1({needed,owners,coverage}){
 const wanted=needed===undefined?owners:[...needed];
 return new Set(wanted.filter(name=>!coverage.names.has(name)));
}
