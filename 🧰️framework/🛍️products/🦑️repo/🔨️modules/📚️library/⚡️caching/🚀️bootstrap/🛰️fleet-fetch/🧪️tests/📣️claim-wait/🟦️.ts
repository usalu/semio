/** 📣️ Law: a download that waits for the fleet claim of the same blob says so in both languages at the first wait and then at most every thirty seconds, naming the holder, the blob and the seconds of the bound; the wait itself is bounded by the work claim policy, after which this machine fetches the blob itself. */
import {expect,test} from "bun:test";
import {claimWaitNoticeV1,fleetFetchContextV1,pinnedFetchV1} from "../../🟦️.ts";
import {WORK_CLAIM_POLICY_V1} from "../../🎟️work-claim/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";

const texts=schema.definitions.ClaimTexts.const,pin={url:"https://github.com/tectonic-typesetting/tectonic/releases/download/x/tectonic.zip",sha256:"ab".repeat(32)};
const contextAt=(clock:{now:number},lines:string[])=>({...fleetFetchContextV1({SEMIO_FLEET_FETCH:"off"},undefined),now:()=>clock.now,say:(line:string)=>{lines.push(line);}});

test("the first wait speaks in both languages, the next ones only after thirty seconds, other stages never",()=>{
 const clock={now:1000},lines:string[]=[],notice=claimWaitNoticeV1(contextAt(clock,lines),pin);
 notice({stage:"claim",key:"k"});notice({stage:"ready",key:"k"});expect(lines).toEqual([]);
 notice({stage:"wait",key:"k",holder:"g1-m02:4242:ab12"});
 expect(lines.length).toBe(2);expect(lines[0]).toContain("waiting for g1-m02:4242:ab12 to fetch abababababab (github.com) for everyone: 0 s of at most 900 s");expect(lines[1]).toContain("warte auf g1-m02:4242:ab12");expect(lines[1]).toContain("von höchstens 900 s");
 clock.now+=texts.everyMilliseconds-1;notice({stage:"wait",key:"k",holder:"g1-m02:4242:ab12"});expect(lines.length).toBe(2);
 clock.now+=1;notice({stage:"wait",key:"k",holder:"g1-m02:4242:ab12"});expect(lines.length).toBe(4);expect(lines[2]).toContain("30 s of at most 900 s");
});

test("the bound named in the line is the bound of the work claim",()=>{
 expect(WORK_CLAIM_POLICY_V1.waitMaximumMilliseconds).toBe(900000);
 expect(texts.wait.en).toContain("{bound}");expect(texts.wait.de).toContain("{bound}");
});

test("a sink that throws never fails the download, and a download without a fleet claim says nothing and fetches",async()=>{
 const clock={now:0},context={...contextAt(clock,[]),say:()=>{throw Error("sink broke");}};
 expect(()=>claimWaitNoticeV1(context,pin)({stage:"wait",key:"k",holder:"h"})).not.toThrow();
 const lines:string[]=[],plain={...contextAt(clock,lines),fetch:(async()=>new Response("body",{status:200}))as unknown as typeof fetch};
 expect(await (await pinnedFetchV1(plain,{url:"https://example.test/x",sha256:"cd".repeat(32)})).text()).toBe("body");expect(lines).toEqual([]);
});

test("a download that finds the blob claimed by another machine says who it waits for, then fetches it itself when the claim lapses",async()=>{
 const lines:string[]=[],asked:string[]=[],clock={now:0};
 const fetchFake=(async(url:string,init?:RequestInit)=>{
  asked.push(`${init?.method??"GET"} ${url.replace(/^https?:\/\/[^/]+/u,"")}`);
  if(url.includes("/cas/"))return new Response("",{status:404});
  if(url.includes("/await"))return new Response(JSON.stringify({state:"lapsed"}),{status:200});
  if(url.includes("/claim/nx/"))return new Response(JSON.stringify({state:"held",by:"g1-m02:4242:ab12"}),{status:409});
  return new Response("upstream body",{status:200});
 })as unknown as typeof fetch;
 const context={...contextAt(clock,lines),remote:{endpoint:"http://fleet.test",instance:"i",token:"t",authenticated:true},fetch:fetchFake};
 const response=await pinnedFetchV1(context,{url:"https://example.test/blob.zip",sha256:"ef".repeat(32)});
 expect(await response.text()).toBe("upstream body");
 expect(lines.length).toBeGreaterThanOrEqual(2);expect(lines[0]).toContain("g1-m02:4242:ab12");expect(lines[1]).toContain("g1-m02:4242:ab12");
 expect(asked.some(row=>row.startsWith("POST")&&row.includes("/claim/nx/"))).toBe(true);expect(asked.includes("GET /blob.zip")).toBe(true);
});
