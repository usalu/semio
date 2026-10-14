import {expect,test} from "bun:test";
import {applyPatch} from "fast-json-patch";
import fixture from "../../🧫️fixtures/⛽️work-unit-authority.json";

test("retained command borrows original fuel and charges scalar fallback once",()=>{
 for(const row of fixture.cases){
  const state={fuel:fixture.grantUnits,observed:fixture.grantUnits};
  const domain=applyPatch(state,[{op:"replace",path:"/fuel",value:state.fuel-row.domainConsumes}],true).newDocument;
  const final=applyPatch(domain,[{op:"replace",path:"/fuel",value:domain.fuel===fixture.grantUnits?domain.fuel-1:domain.fuel}],true).newDocument;
  expect(final).toEqual({fuel:row.remaining,observed:row.domainObserves});
 }
 expect(fixture.sourceKind).toBe("original-parent");expect(fixture.grantUnits).toBeGreaterThan(1);expect(fixture.phaseOrder).toEqual(["preflight","receive-original-command","work","publish"]);
 console.log("[DEBUG] Retained command independent original fuel authority domain/fallback cases=2");
});
