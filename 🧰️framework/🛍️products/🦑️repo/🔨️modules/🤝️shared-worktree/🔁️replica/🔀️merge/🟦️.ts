/** 📏️ Largest side in bytes the apply merges (8 MiB); a bigger file is never merged. */
export const mergeBytesLimitV1=8*1024*1024;

/** ✂️ Most line edits between the base and one side that the merge aligns (4096); a side further away is not merged. */
export const mergeEditsLimitV1=4096;

/** 🧾️ The answer of the three-way merge: the merged bytes, or why there are none. */
export type MergeOutcomeV1=Readonly<{clean:true;merged:Uint8Array}>|Readonly<{clean:false;reason:"binary"|"too-large"|"too-different"|"conflict"}>;

type Hunk=Readonly<{start:number;end:number;from:number;to:number}>;

const latin=(bytes:Uint8Array):string=>Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength).toString("latin1");

function linesOf(text:string):string[]{
 const lines:string[]=[];
 let start=0;
 for(let at=text.indexOf("\n");at!==-1;at=text.indexOf("\n",start)){lines.push(text.slice(start,at+1));start=at+1;}
 if(start<text.length)lines.push(text.slice(start));
 return lines;
}

function hunksOf(a:Int32Array,b:Int32Array,limit:number):Hunk[]|undefined{
 let prefix=0,suffix=0;
 while(prefix<a.length&&prefix<b.length&&a[prefix]===b[prefix])prefix++;
 while(suffix<a.length-prefix&&suffix<b.length-prefix&&a[a.length-1-suffix]===b[b.length-1-suffix])suffix++;
 const n=a.length-prefix-suffix,m=b.length-prefix-suffix,at=(x:number)=>a[prefix+x]!,bt=(y:number)=>b[prefix+y]!;
 if(n===0&&m===0)return [];
 const max=n+m,offset=max+1,v=new Int32Array(2*max+3),trace:Int32Array[]=[];
 let found=-1;
 for(let d=0;d<=max&&d<=limit&&found<0;d++){
  for(let k=-d;k<=d;k+=2){
   let x=k===-d||(k!==d&&v[offset+k-1]!<v[offset+k+1]!)?v[offset+k+1]!:v[offset+k-1]!+1,y=x-k;
   while(x<n&&y<m&&at(x)===bt(y)){x++;y++;}
   v[offset+k]=x;
   if(x>=n&&y>=m){found=d;break;}
  }
  trace.push(v.slice(offset-d,offset+d+1));
 }
 if(found<0)return undefined;
 const match=new Int32Array(n).fill(-1);
 let x=n,y=m;
 for(let d=found;d>0;d--){
  const previous=trace[d-1]!,k=x-y,old=(key:number)=>previous[key+d-1]!,down=k===-d||(k!==d&&old(k-1)<old(k+1)),key=down?k+1:k-1,px=old(key),py=px-key,sx=down?px:px+1;
  for(let i=sx;i<x;i++)match[i]=py+(down?1:0)+(i-sx);
  x=px;y=py;
 }
 for(let i=0;i<x;i++)match[i]=i;
 const hunks:Hunk[]=[];
 let i=0,j=0;
 for(;;){
  const si=i,sj=j;
  while(i<n&&match[i]===-1)i++;
  const nj=i<n?match[i]!:m;
  if(i>si||nj>sj)hunks.push({start:prefix+si,end:prefix+i,from:prefix+sj,to:prefix+nj});
  if(i>=n)break;
  i++;j=nj+1;
 }
 return hunks;
}

/** 🔀️ Three-way merge of the lines of `ours` and `theirs` against their common `base`, byte exact (lines end at `\n`, the last one may lack it). Changes of the two sides that overlap or touch (no unchanged base line between them) are a conflict unless both sides made the same change; binary content (a NUL byte), sides over `mergeBytesLimitV1` and sides more than `mergeEditsLimitV1` line edits away from the base are never merged. Clean merges agree with `git merge-file`, which may still merge a few cases this merge calls a conflict. */
export function mergeTextV1(base:Uint8Array,ours:Uint8Array,theirs:Uint8Array):MergeOutcomeV1{
 if(Buffer.compare(ours,theirs)===0||Buffer.compare(base,theirs)===0)return {clean:true,merged:ours};
 if(Buffer.compare(base,ours)===0)return {clean:true,merged:theirs};
 if(base.byteLength>mergeBytesLimitV1||ours.byteLength>mergeBytesLimitV1||theirs.byteLength>mergeBytesLimitV1)return {clean:false,reason:"too-large"};
 if(base.includes(0)||ours.includes(0)||theirs.includes(0))return {clean:false,reason:"binary"};
 const ids=new Map<string,number>(),intern=(lines:string[])=>Int32Array.from(lines,line=>{let id=ids.get(line);if(id===undefined){id=ids.size;ids.set(line,id);}return id;});
 const [b,o,t]=[base,ours,theirs].map(bytes=>linesOf(latin(bytes))) as [string[],string[],string[]];
 const bi=intern(b),left=hunksOf(bi,intern(o),mergeEditsLimitV1),right=hunksOf(bi,intern(t),mergeEditsLimitV1);
 if(left===undefined||right===undefined)return {clean:false,reason:"too-different"};
 const all=[...left.map(hunk=>({hunk,side:0})),...right.map(hunk=>({hunk,side:1}))].sort((p,q)=>p.hunk.start-q.hunk.start||p.hunk.end-q.hunk.end||p.side-q.side),sides=[o,t],out:string[]=[];
 let cursor=0;
 for(let index=0;index<all.length;){
  const group=[all[index]!];
  let end=all[index]!.hunk.end;
  for(index++;index<all.length&&all[index]!.hunk.start<=end;index++){group.push(all[index]!);end=Math.max(end,all[index]!.hunk.end);}
  const start=group[0]!.hunk.start;
  for(let line=cursor;line<start;line++)out.push(b[line]!);
  const span=(side:number):string[]=>{const own=group.filter(member=>member.side===side).map(member=>member.hunk),first=own[0]!,last=own.at(-1)!;return sides[side]!.slice(first.from-(first.start-start),last.to+(end-last.end));};
  const touched=new Set(group.map(member=>member.side));
  if(touched.size===1)out.push(...span(group[0]!.side));
  else{
   const mine=span(0),yours=span(1);
   if(mine.length!==yours.length||mine.some((line,at)=>line!==yours[at]))return {clean:false,reason:"conflict"};
   out.push(...mine);
  }
  cursor=end;
 }
 for(let line=cursor;line<b.length;line++)out.push(b[line]!);
 return {clean:true,merged:Buffer.from(out.join(""),"latin1")};
}
