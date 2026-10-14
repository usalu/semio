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
   if(row.startsWith(" ")){before++;after++;}else if(row.startsWith("-"))before++;else if(row.startsWith("+"))after++;else if(row.startsWith("\\ No newline at end of file"))continue;else if(row===""&&before===oldCount&&after===newCount)continue;else throw Error("Tooling patch unprefixed hunk line");
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
  const files:Record<string,string>={};
  for(const [path,additions] of toolPatchAdditions(patch)){
    const bytes=readFileSync(join(directory,path)),lines=new Set(bytes.toString("utf8").split("\n"));
    for(const line of additions)if(!lines.has(line))throw Error(`Requested tool patch was not applied: ${path}`);
    files[path]=createHash("sha256").update(bytes).digest("hex");
  }
  return{version:1,files};
}
