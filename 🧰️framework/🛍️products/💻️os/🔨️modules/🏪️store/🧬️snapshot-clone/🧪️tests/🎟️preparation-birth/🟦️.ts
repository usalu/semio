import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";

test("funded preparation birth preserves original request on independent currency refusal",()=>{
  const law=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️preparation-birth/🔣️.json",import.meta.url),"utf8"));
  for(const currency of law.refusals){expect(["items","capacity","depth"]).toContain(currency);expect(applyPatch(law.request,[],true,false).newDocument).toEqual(law.request);}
  expect(law.afterRefusal).toEqual({requestReturned:true,allocatedBytes:0,releasedBytes:0});
  expect(law.deferred).toEqual(["snapshot source","mutation source","clone cursor","edit cursor"]);
  const admitted=applyPatch({pending:law.request,retained:null},[{op:"replace",path:"/retained",value:law.request},{op:"replace",path:"/pending",value:null}],true,false).newDocument;
  expect(admitted.retained).toEqual(law.request);expect(admitted.pending).toBeNull();expect(law.preflight.afterRefusalCalls).toBe(0);expect(applyPatch({calls:law.preflight.afterRefusalCalls},[{op:"replace",path:"/calls",value:1}],true,false).newDocument.calls).toBe(law.preflight.afterBirthCalls);
  expect([law.grant.maximumCopyBytes,law.grant.maximumCapacityBytes,law.grant.maximumReleaseBytes,law.grant.maximumDepth]).toEqual([3,512,97,7]);
  console.log("[DEBUG] funded preparation birth RFC6902 oracle preserves original request and independent constructor funding");
});
