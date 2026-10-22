import patchSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {isAbsolute,join} from "node:path";

/** 🧷️ Requires one original SDK file section with exact unified hunk counts and cumulative coordinates. */
export function assertToolingPatchV1(patch:string):void{
 const lines=patch.split("\n"),files=new Set<string>();let file="",delta=0,oldEnd=0,newEnd=0,hunks=0;
 for(let index=0;index<lines.length;index++){
  const line=lines[index]!;
  if(line.startsWith("diff --git ")){
   const match=line.match(/^diff --git a\/(\S+) b\/(\S+)$/u);if(!match||match[1]!==match[2])throw Error("Original tooling patch file required");
   file=match[1]!;if(isAbsolute(file)||/[:\\\0]/u.test(file)||file.split("/").some(part=>!part||part==="."||part===".."))throw Error("Relative tooling patch file required");if(files.has(file))throw Error("Tooling patch file repeated: "+file);files.add(file);delta=0;oldEnd=0;newEnd=0;continue;
  }
  if(!line.startsWith("@@"))continue;
  const header=line.match(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@(?:.*)$/u);
  if(!file||!header)throw Error("Original tooling patch hunk required");
  const oldStart=Number(header[1]),oldCount=Number(header[2]??1),newStart=Number(header[3]),newCount=Number(header[4]??1);
  if([oldStart,oldCount,newStart,newCount].some(value=>!Number.isSafeInteger(value)||value<0))throw Error("Bounded tooling patch coordinates required");
  if(oldStart<oldEnd||newStart<newEnd||newStart+(newCount===0?1:0)!==oldStart+(oldCount===0?1:0)+delta)throw Error("Tooling patch cumulative coordinates changed");
  let before=0,after=0;
  while(index+1<lines.length&&!lines[index+1]!.startsWith("diff --git ")&&!lines[index+1]!.startsWith("@@")){
   const row=lines[++index]!;
   if(row.startsWith(" ")){before++;after++;}else if(row.startsWith("-"))before++;else if(row.startsWith("+"))after++;else if(row.startsWith("\\ No newline at end of file"))continue;else if(row===""&&(index+1===lines.length||before===oldCount&&after===newCount))continue;else throw Error("Tooling patch unprefixed hunk line");
  }
  if(before!==oldCount||after!==newCount)throw Error("Tooling patch hunk counts changed");
  oldEnd=oldStart+oldCount;newEnd=newStart+newCount;delta+=newCount-oldCount;hunks++;
 }
 if(!files.size||!hunks)throw Error("Original tooling patch sections required");
}

/** 🩹️ Records requested additions that survive all ordered patch sections for each owned file. */
export function toolPatchAdditions(patch:string):ReadonlyMap<string,ReadonlySet<string>> {
  const files=new Map<string,Set<string>>();let file:Set<string>|undefined;
  for(const line of patch.split("\n")){
    if(line.startsWith("+++ b/")){
      const path=line.slice(6);if(!path||isAbsolute(path)||path.includes("\\")||path.includes("\0")||path.split("/").some(part=>!part||part==="."||part===".."))throw Error("Relative tool patch file required");
      file=files.get(path)??new Set();files.set(path,file);
    }else if(line.startsWith("diff --git")||line.startsWith("--- "))file=undefined;
    else if(file&&line.startsWith("+")&&line.slice(1).trim())file.add(line.slice(1));
    else if(file&&line.startsWith("-"))file.delete(line.slice(1));
  }
  if(!files.size)throw Error("Owned tool patch file required");return files;
}

/** 🔏️ Verifies requested final patch lines and receipts the actual immutable package files. */
export function toolPatchReceipt(directory:string,patch:string):{version:1;files:Record<string,string>}{
  assertToolingPatchV1(patch);const files:Record<string,string>={},images=new Map<string,readonly string[]>();
  for(const [path,additions] of toolPatchAdditions(patch)){
    const bytes=readFileSync(join(directory,path)),image=bytes.toString("utf8").split("\n"),lines=new Set(image);images.set(path,image);
    for(const line of additions)if(!lines.has(line))throw Error(`Requested tool patch was not applied: ${path}`);
    files[path]=createHash("sha256").update(bytes).digest("hex");
  }
  let path="";const rows=patch.split("\n");
  for(let index=0;index<rows.length;index++){
   const row=rows[index]!;if(row.startsWith("+++ b/"))path=row.slice(6);const header=row.match(/^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@/u);if(!header)continue;
   const expected:string[]=[];while(index+1<rows.length&&!rows[index+1]!.startsWith("@@")&&!rows[index+1]!.startsWith("diff --git ")){const line=rows[++index]!;if(line.startsWith(" ")||line.startsWith("+"))expected.push(line.slice(1));}
   const image=images.get(path),start=Number(header[1])-1;if(!image||expected.some((line,offset)=>image[start+offset]!==line))throw Error("Exact SDK post-hunk context changed: "+path);
  }
  return{version:1,files};
}

/** 🩹️ Emits independently owned exact post-images for the admitted package patch. */
export function applyToolingPatchImagesV1(source:ReadonlyMap<string,Uint8Array>,patch:string):ReadonlyMap<string,Uint8Array>{
 const policy=patchSchema.definitions.Protocol.const;if(Buffer.byteLength(patch)>policy.maximumBytes)throw Error("SDK patch input byte ceiling exceeded");assertToolingPatchV1(patch);const rows=patch.split("\n"),output=new Map<string,Uint8Array>(),decoder=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true});let path="",original:{text:string;newline:boolean}[]=[],result:{text:string;newline:boolean}[]=[],cursor=0,total=0;
 const finish=()=>{if(!path)return;result.push(...original.slice(cursor));const bytes=Buffer.from(result.map(row=>row.text+(row.newline?"\n":"")).join(""));total+=bytes.byteLength;if(total>policy.maximumBytes||output.size>=policy.maximumFiles)throw Error("SDK patch output byte or file ceiling exceeded");output.set(path,bytes);};
 for(let index=0;index<rows.length;index++){
  const line=rows[index]!;if(line.startsWith("diff --git ")){finish();path=line.match(/^diff --git a\/(\S+) b\//u)![1]!;if(Buffer.byteLength(path)>policy.maximumPathBytes)throw Error("SDK patch path ceiling exceeded");const bytes=source.get(path);if(!bytes||bytes.byteLength>policy.maximumBytes)throw Error("Original bounded SDK patch image required");const text=decoder.decode(bytes),lines=text.split("\n");if(text.endsWith("\n"))lines.pop();original=text===""?[]:lines.map((line,offset)=>({text:line,newline:offset<lines.length-1||text.endsWith("\n")}));result=[];cursor=0;continue;}
  const header=line.match(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/u);if(!header)continue;const before=Number(header[2]??1),start=Number(header[1])-(before===0?0:1);if(start<cursor||start>original.length)throw Error("Exact SDK patch original position required");result.push(...original.slice(cursor,start));cursor=start;const operations:{kind:string;text:string;newline:boolean}[]=[];
  while(index+1<rows.length&&!rows[index+1]!.startsWith("diff --git ")&&!rows[index+1]!.startsWith("@@")){const row=rows[++index]!;if(row.startsWith("\\ No newline")){if(!operations.length)throw Error("SDK patch newline witness required");operations[operations.length-1]!.newline=false;}else if(/[ +\-]/u.test(row[0]??""))operations.push({kind:row[0]!,text:row.slice(1),newline:true});}
  for(const operation of operations){if(operation.kind!=="+"){const actual=original[cursor++];if(!actual||actual.text!==operation.text||actual.newline!==operation.newline)throw Error("Exact SDK patch original bytes changed: "+path);}if(operation.kind!=="-")result.push({text:operation.text,newline:operation.newline});}
 }finish();return output;
}
