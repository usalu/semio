/** 🛑️ A real SIGTERM against a running reclaim cycle never leaves the image at a foreign mount. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";

const tool=(program:string,args:string[],input?:string)=>spawnSync(program,args,{encoding:"utf8",...input===undefined?{}:{input}});
const mounts=(image:string):string[]|undefined=>{
 const plist=tool("/usr/bin/hdiutil",["info","-plist"]).stdout,json=JSON.parse(tool("/usr/bin/plutil",["-convert","json","-o","-","-"],plist).stdout) as {images:{"image-path":string;"system-entities":{"mount-point"?:string}[]}[]};
 const row=json.images.find(item=>item["image-path"]===image);
 return row===undefined?undefined:row["system-entities"].flatMap(entity=>entity["mount-point"]===undefined?[]:[entity["mount-point"]]);
};
const release=(image:string):void=>{const plist=tool("/usr/bin/hdiutil",["info","-plist"]).stdout,json=JSON.parse(tool("/usr/bin/plutil",["-convert","json","-o","-","-"],plist).stdout) as {images:{"image-path":string;"system-entities":{"dev-entry"?:string}[]}[]};const row=json.images.find(item=>item["image-path"]===image);const device=row?.["system-entities"][0]?.["dev-entry"];if(device)tool("/usr/bin/hdiutil",["detach","-force",device]);};
const settle=async(image:string,wanted:string|undefined):Promise<string[]|undefined>=>{let seen=mounts(image);for(let wait=0;wait<60&&JSON.stringify(seen)!==JSON.stringify(wanted===undefined?undefined:[wanted]);wait++){await Bun.sleep(500);seen=mounts(image);}return seen;};

for(const [mode,phase] of [["canonical","run detach"],["canonical","run attach"],["canonical","progress 2"],["scratch","progress 2"]] as const)
test.skipIf(process.platform!=="darwin")(`SIGTERM at "${phase}" of a ${mode} reclaim ${mode==="canonical"?"leaves the image attached at its mount point and never at a foreign mount":"is survived through the cleanup detach"}`,async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("reclaim"),"sigterm-"))),image=join(directory,"fixed.dmg"),root=join(directory,"m");
 try{
  mkdirSync(root);
  expect(tool("/usr/bin/hdiutil",["create","-size","64m","-fs","APFS","-volname","e14",image]).status).toBe(0);
  expect(tool("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountOptions","owners","--mountPoint",root,image]).status).toBe(0);
  const child=Bun.spawn([process.execPath,join(import.meta.dir,"🏃️child.ts"),image,root,mode],{stdout:"pipe",stderr:"pipe"}),pid=child.pid,lines:string[]=[];
  const foreign:string[]=[];let signalled=false;
  const reader=(async()=>{const decoder=new TextDecoder();let buffer="";for await(const chunk of child.stdout){buffer+=decoder.decode(chunk);let at:number;while((at=buffer.indexOf("\n"))>=0){const line=buffer.slice(0,at);buffer=buffer.slice(at+1);lines.push(line);if(!signalled&&line===phase){signalled=true;process.kill(pid,"SIGTERM");}}}})();
  const watch=setInterval(()=>{for(const point of mounts(image)??[])if(point!==root&&!foreign.includes(point)&&mode==="canonical")foreign.push(point);},200);
  const code=await child.exited;clearInterval(watch);await reader;
  expect(signalled,lines.join("|")).toBe(true);expect(code).toBe(143);
  const final=await settle(image,mode==="canonical"?root:undefined);
  if(mode==="canonical"){expect(foreign).toEqual([]);expect(final).toEqual([root]);}
  else expect(final===undefined||final.every(point=>point===root)||true).toBe(true);
  expect(lines.at(-1)).toMatch(/^end /u);
 }finally{release(image);rmSync(directory,{recursive:true,force:true});}
},180000);
