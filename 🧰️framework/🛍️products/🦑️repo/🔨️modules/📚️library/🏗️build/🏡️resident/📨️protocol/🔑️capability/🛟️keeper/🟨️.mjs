import{statSync,utimesSync,writeFileSync}from"node:fs";

/** 🛟 Keeps the root key file of a running service alive: a file that nobody writes again is removed by the operating system's temporary-file cleanup (macOS `dirhelper` deletes files of the per-user temporary directory that are older than three days, daily at 03:35), so `tick` touches the key and the named files before they age out and writes the key back, byte for byte and private, when the file is missing; a different key that a newer generation wrote is never overwritten. Returns `"intact"`, `"refreshed"`, `"restored"` or `"failed"`.
 * @see https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html */
export function keepRootKeyV1({path,key,touch=[],refreshMilliseconds,now=Date.now}){
 const refresh=target=>{
  try{
   const at=now(),status=statSync(target);
   if(at-Math.min(status.mtimeMs,status.atimeMs)<refreshMilliseconds)return false;
   utimesSync(target,new Date(at),new Date(at));return true;
  }catch{return false;}
 };
 return{
  tick(){
   let outcome="intact";
   try{statSync(path);if(refresh(path))outcome="refreshed";}
   catch(error){
    if(error?.code!=="ENOENT")return"failed";
    try{writeFileSync(path,key,{flag:"wx",mode:0o600});outcome="restored";}
    catch(failure){if(failure?.code!=="EEXIST")return"failed";}
   }
   for(const target of touch)refresh(target);
   return outcome;
  },
 };
}
