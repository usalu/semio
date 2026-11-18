/** 🚪️ Contract: every child of the pool that ends leaves one record with its reason and a line in English and German; none ends silently. Oracle: the table of causes below, written out by hand. */
import{expect,test}from"bun:test";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{ownerPoolExitRecordV1,ownerPoolExitsAddV1}from"../../🚪️exits/🟦️.ts";

const base={pid:77,code:0 as number|null,drained:false,killed:false,startedMs:1000,now:61000,owners:3};
const reason=(over:Record<string,unknown>)=>ownerPoolExitRecordV1({...base,...over}).reason;

test("the reason of an exit follows what the watchdog saw, what the pool did, what the child said and the code, in that order",()=>{
 expect(reason({watchdog:"lease-stale",killed:true,drained:true,reason:"recycled"})).toBe("watchdog");
 expect(reason({killed:true,drained:true})).toBe("stopped");
 expect(reason({reason:"revision",code:76})).toBe("revision");
 expect(reason({reason:"open",code:77})).toBe("open-failed");
 expect(reason({reason:"recycled",code:75,drained:true})).toBe("recycled");
 expect(reason({reason:"drained",code:78,drained:true})).toBe("drained");
 expect(reason({drained:true,code:78})).toBe("drained");
 expect(reason({reason:"finished"})).toBe("finished");
 expect(reason({code:0})).toBe("finished");
 expect(reason({code:null,signal:"SIGKILL"})).toBe("unexpected");
 expect(reason({code:78})).toBe("unexpected");
});

test("the record carries age in seconds, owners done and one line per language that names the process and the cause",()=>{
 const exit=ownerPoolExitRecordV1({...base,drained:true,code:78,target:0,why:"pressure"});
 expect(exit).toMatchObject({pid:77,reason:"drained",code:78,ageSeconds:60,owners:3});
 expect(exit.text.en).toContain("77");expect(exit.text.en).toContain("60 s");expect(exit.text.en).toContain("pressure");
 expect(exit.text.de).toContain("77");expect(exit.text.de).toContain("60 s");expect(exit.text.de).toContain("pressure");
 expect(exit.text.en).not.toBe(exit.text.de);
 expect(ownerPoolExitRecordV1({...base,watchdog:"lease-stale"}).text.de).toContain("lease-stale");
});

test("every exit text exists in both languages and no placeholder is left unfilled",()=>{
 const T=schema.definitions.Texts.const as{en:Record<string,string>;de:Record<string,string>};
 for(const key of Object.keys(T.en).filter(key=>key.startsWith("exit")))expect(T.de[key],key).toBeTruthy();
 for(const input of[{},{watchdog:"parent-gone"},{killed:true},{reason:"revision"},{reason:"open"},{reason:"recycled"},{drained:true},{code:null,signal:"SIGSEGV"}]){
  const exit=ownerPoolExitRecordV1({...base,...input});
  expect(exit.text.en).not.toMatch(/[{}]/u);expect(exit.text.de).not.toMatch(/[{}]/u);
 }
});

test("the counts by reason grow and only the newest rows are kept",()=>{
 let state:{exits:Record<string,number>;lastExits:ReturnType<typeof ownerPoolExitRecordV1>[]}={exits:{},lastExits:[]};
 for(let at=0;at<12;at++)state=ownerPoolExitsAddV1(state,ownerPoolExitRecordV1({...base,pid:at,drained:at%3===0}),8);
 expect(state.exits).toEqual({drained:4,finished:8});
 expect(state.lastExits.map(row=>row.pid)).toEqual([4,5,6,7,8,9,10,11]);
});
