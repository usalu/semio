/** 🔤️ Controlled Unicode scalar key ordering matches native UTF-8 BTree ordering. */
import {artifactSqliteCheckpoint,type ArtifactSqliteOptions} from "../🟦️.ts";
type Phase="projectSnapshot"|"reconstructSnapshot";
/** 🧭️ Compare borrowed literal keys with known UTF-16 scanning progress. */
export async function artifactSqliteCompareKeys(a:string,b:string,options:ArtifactSqliteOptions={},phase:Phase="projectSnapshot"):Promise<number>{
 let i=0,j=0,work=0;await artifactSqliteCheckpoint(options,phase,0,a.length+b.length,false);
 while(i<a.length&&j<b.length){const left=a.codePointAt(i)!,right=b.codePointAt(j)!;if(left!==right)return left<right?-1:1;i+=left>0xffff?2:1;j+=right>0xffff?2:1;if(++work%256===0)await artifactSqliteCheckpoint(options,phase,i+j,a.length+b.length);}
 return i===a.length?(j===b.length?0:-1):1;
}
/** 🗂️ Order supplied borrowed entries in place without constructing an additional sort buffer. */
export async function artifactSqliteOrderKeyEntries<V>(values:[string,V][],options:ArtifactSqliteOptions={}):Promise<void>{
 const compare=(a:string,b:string)=>artifactSqliteCompareKeys(a,b,options);
 const sift=async(root:number,end:number)=>{while(root<Math.floor(end/2)){let child=root*2+1;if(child+1<end&&await compare(values[child]![0],values[child+1]![0])<0)child++;if(await compare(values[root]![0],values[child]![0])>=0)return;[values[root],values[child]]=[values[child]!,values[root]!];root=child;}};
 for(let root=Math.floor(values.length/2)-1;root>=0;root--)await sift(root,values.length);
 for(let end=values.length-1;end>0;end--){[values[0],values[end]]=[values[end]!,values[0]!];await sift(0,end);if(end%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",values.length-end,values.length);}
}
