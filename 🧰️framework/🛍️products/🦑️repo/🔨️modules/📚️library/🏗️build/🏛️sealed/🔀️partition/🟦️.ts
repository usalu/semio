import type {SealedTreeV1} from "../🟦️.ts";

/** 🤝️ Requires both authenticated captures to agree wherever their source and filesystem observations overlap. */
export function sealedTreesCoherentV1(left:SealedTreeV1,right:SealedTreeV1):boolean{
 const matches=(a:readonly (readonly string[])[],b:readonly (readonly string[])[])=>{const rows=new Map(a.map(([path,...rest])=>[path,JSON.stringify(rest)]));return b.every(([path,...rest])=>!rows.has(path)||rows.get(path)===JSON.stringify(rest));};
 if(!matches(left.files.map(([path,hash])=>[path,hash]),right.files.map(([path,hash])=>[path,hash]))||!matches(left.directories,right.directories)||!matches(left.listings,right.listings))return false;
 const present=(tree:SealedTreeV1)=>new Set([...tree.files.map(row=>row[0]),...tree.directories.map(row=>row[0]),...tree.listings.map(row=>row[0])]);
 const a=present(left),b=present(right);return left.absent.every(path=>!b.has(path))&&right.absent.every(path=>!a.has(path));
}
