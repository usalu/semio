//! 🪪️ Exact namespace continuity without numeric precision loss.

type Coordinate={device:string;inode:string};
type Namespace={root:Coordinate;journal:Coordinate;entry:Coordinate};
const keys=(value:unknown,names:readonly string[]):value is Record<string,unknown>=>typeof value==="object"&&value!==null&&!Array.isArray(value)&&Object.keys(value).length===names.length&&names.every(name=>Object.hasOwn(value,name));
function coordinate(value:unknown):value is Coordinate{if(!keys(value,["device","inode"]))return false;return ["device","inode"].every(key=>{const token=value[key];return typeof token==="string"&&/^(?:0|[1-9][0-9]*)$/u.test(token)&&token.length<=20&&BigInt(token)<=18446744073709551615n&&(key!=="inode"||token!=="0");});}
function namespace(value:unknown):value is Namespace{return keys(value,["root","journal","entry"])&&["root","journal","entry"].every(key=>coordinate(value[key]));}
/** 🧭️ Rebinds one device coordinate only inside an unchanged rooted inode chain. */
export function namespaceContinuity(value:unknown):boolean{
 if(!keys(value,["published","current"])||!namespace(value.published)||!namespace(value.current))return false;
 const published=value.published,current=value.current;return published.root.device===published.journal.device&&published.root.device===published.entry.device&&current.root.device===current.journal.device&&current.root.device===current.entry.device&&(["root","journal","entry"]as const).every(key=>published[key].inode===current[key].inode);
}
