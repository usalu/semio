import{test,expect}from"bun:test";
import{spawn,spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"./🔣️.json"with{type:"json"};
import{captureOwnedCommandV1}from"../../../../../../🎛️owned-execution/🟦️.ts";
import{nativeImageInformationV1,observeNativeImageInventoryV1}from"../🟦️.ts";

const escape=(value:string)=>value.replaceAll("&","&amp;").replaceAll("<","&lt;").replaceAll(">","&gt;");
const entry=(image:string,mount:string)=>"<dict><key>image-path</key><string>"+escape(image)+"</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/disk9s1</string><key>mount-point</key><string>"+escape(mount)+"</string></dict></array></dict>";
const rows:Record<string,[string,string]>={parent:[fixture.image,fixture.root],child:["/repository/cache space/child/fixed.dmg",fixture.child]};
const plist=(names:readonly string[])=>'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\n<plist version="1.0"><dict><key>images</key><array>'+names.map(name=>entry(...rows[name]!)).join("")+"</array></dict></plist>\n";
const render=(capture:unknown):string=>{
 if(Array.isArray(capture))return plist(capture);
 const row=capture as{raw?:string;noise?:boolean;noiseAfter?:boolean;images?:string[];exit?:boolean};
 if(row.exit)throw Error(fixture.exit);
 if(row.raw!==undefined)return row.raw;
 return (row.noise?fixture.noise:"")+plist(row.images!)+(row.noiseAfter?fixture.noise:"");
};
const accept=(information:Record<string,unknown>):number=>{
 const mounts:string[]=[];
 for(const image of information.images as Record<string,unknown>[])for(const entity of image["system-entities"] as Record<string,unknown>[])if(typeof entity["mount-point"]==="string"&&(entity["mount-point"]===fixture.root||(entity["mount-point"] as string).startsWith(fixture.root+"/")))mounts.push(entity["mount-point"] as string);
 if(mounts.filter(mount=>mount===fixture.root).length!==1||new Set(mounts).size!==mounts.length)throw Error("Bounded unique parent roster required");
 return mounts.length;
};
const policy=fixture.policy;

for(const row of fixture.cases)test("system image inventory: "+row.name,async()=>{
 let calls=0;
 const run=observeNativeImageInventoryV1(async()=>{const capture=(row as{repeat?:boolean}).repeat?row.captures[0]:row.captures[Math.min(calls,row.captures.length-1)];calls++;return render(capture);},accept,policy);
 if(row.outcome==="accepted")await run;else await expect(run).rejects.toThrow(row.outcome);
 expect(calls).toBe(row.calls);
});

test("system image inventory: a retried snapshot agrees with the system plist converter",()=>{
 const text=plist(["parent","child"]);
 const direct=nativeImageInformationV1(text);
 if(process.platform==="darwin"){
  const root=mkdtempSync(join(tmpdir(),"inventory-oracle-"));
  try{const path=join(root,"info.plist");writeFileSync(path,text);const oracle=spawnSync("/usr/bin/plutil",["-convert","json","-o","-",path],{encoding:"utf8"});expect(oracle.status).toBe(0);expect(JSON.parse(oracle.stdout)).toEqual(direct);}finally{rmSync(root,{recursive:true,force:true});}
 }
 expect((direct.images as unknown[]).length).toBe(2);
});

test("system image inventory: noise and structure are named in the refusal",()=>{
 expect(()=>nativeImageInformationV1("hdiutil: info: cannot find IO media\n")).toThrow("System image plist required: ");
 expect(()=>nativeImageInformationV1("")).toThrow("System image plist required");
});

const run=(program:string,args:readonly string[])=>spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});
test.skipIf(process.platform!=="darwin")("system image inventory: stays consistent for a stable image while other images attach and detach concurrently",async()=>{
 const real=fixture.real,root=realpathSync(mkdtempSync(join(tmpdir(),"inventory-real-"))),attached:string[]=[],children:ReturnType<typeof spawn>[]=[];
 try{
  const make=(name:string)=>{const dir=join(root,name),mount=join(dir,"m");mkdirSync(mount,{recursive:true});const image=join(dir,"t.dmg");expect(run("/usr/bin/hdiutil",["create","-size",real.imageMegabytes+"m","-fs","HFS+","-volname",name,"-type","UDIF",image]).status).toBe(0);return{dir,mount,image};};
  const stable=make("stable");
  expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-mountpoint",stable.mount,stable.image]).status).toBe(0);
  attached.push(stable.mount);
  const churners=Array.from({length:real.churners},(_,index)=>make("churn"+index));
  for(const churner of churners){
   const script='i=0; while [ ! -e "$1/stop" ]; do /usr/bin/hdiutil attach -nobrowse -noautoopen -mountpoint "$2" "$3" >/dev/null 2>&1 && /usr/bin/hdiutil detach "$2" >/dev/null 2>&1; i=$((i+1)); done';
   children.push(spawn("/bin/sh",["-c",script,"churn",root,churner.mount,churner.image],{stdio:"ignore"}));
  }
  const accepted=(information:Record<string,unknown>)=>{
   const listed=(information.images as Record<string,unknown>[]).filter(image=>image["image-path"]===stable.image);
   const mounts=listed.flatMap(image=>(image["system-entities"] as Record<string,unknown>[]).filter(entity=>entity["mount-point"]===stable.mount));
   if(listed.length!==1||mounts.length!==1)throw Error("Bounded unique parent roster required");
   const all=(information.images as Record<string,unknown>[]).flatMap(image=>(image["system-entities"] as Record<string,unknown>[]).map(entity=>entity["mount-point"]).filter(point=>typeof point==="string"));
   if(new Set(all).size!==all.length)throw Error("Bounded unique parent roster required");
  };
  for(let sample=0;sample<real.samples;sample++)await observeNativeImageInventoryV1(()=>captureOwnedCommandV1("/usr/bin/hdiutil",["info","-plist"],root,{output:{maximumBytes:1048576,maximumLines:32768}}),accepted,{attempts:real.attempts,backoffMilliseconds:real.backoffMilliseconds});
 }finally{
  writeFileSync(join(root,"stop"),"");
  await Promise.all(children.map(child=>new Promise<void>(done=>{if(child.exitCode!==null||child.signalCode!==null)return done();child.once("exit",()=>done());})));
  for(const dir of ["stable",...Array.from({length:fixture.real.churners},(_,index)=>"churn"+index)])run("/usr/bin/hdiutil",["detach","-force",join(root,dir,"m")]);
  rmSync(root,{recursive:true,force:true});
 }
},180000);
