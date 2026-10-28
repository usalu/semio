/** 🪪️ Encode a complete unsigned 128-bit identity in IFC's declared alphabet. */
export function encodeIfcGlobalId(value:bigint):string{if(value<0n||value>340282366920938463463374607431768211455n)throw new Error("ifc-global-id-range");let encoded="";for(let index=0;index<22;index++){encoded="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$"[Number(value&63n)]+encoded;value>>=6n;}return encoded;}
/** 🎲️ Mint an IFC identity using the platform's native entropy source. */
export function randomIfcGlobalId():string{const bytes=globalThis.crypto.getRandomValues(new Uint8Array(16));bytes[6]=(bytes[6]&15)|64;bytes[8]=(bytes[8]&63)|128;let value=0n;for(const byte of bytes)value=value*256n+BigInt(byte);return encodeIfcGlobalId(value);}

/** 🏷️ Stable IFC mesh and inspector identity without binary64 conversion. */
export function ifcEntityIdentity(id:bigint):string{if(id<=0n||id>18446744073709551615n)throw new Error("ifc-entity-id-range");return "ifc-"+id.toString();}
/** 🔎️ Admit only canonical positive-u64 IFC mesh identities. */
export function parseIfcEntityIdentity(identity:string):bigint|undefined{if(typeof identity!=="string"||!/^ifc-[1-9][0-9]{0,19}$/.test(identity))return undefined;const id=BigInt(identity.slice(4));return id<=18446744073709551615n?id:undefined;}
