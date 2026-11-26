import {expect,test} from "bun:test";
import {fleetTextV1} from "../../🟦️.ts";

/** 🫀️ Contract: a worker that lives but stopped beating its readiness record is shown as `stale readiness beat` (English) and `Bereitschaftsschlag veraltet` (German), in the row and in the finding, never as plain `silent`. */
test("the silent worker state and finding say stale readiness beat in both languages",()=>{
 expect(fleetTextV1("en","workers.state.silent")).toBe("stale readiness beat");
 expect(fleetTextV1("de","workers.state.silent")).toBe("Bereitschaftsschlag veraltet");
 const values={origin:"g1-m05",age:"3 h",recorded:"running",stage:"dependencies"};
 expect(fleetTextV1("en","finding.worker-silent",values)).toContain("readiness beat is stale");
 expect(fleetTextV1("de","finding.worker-silent",values)).toContain("Bereitschaftsschlag ist veraltet");
 expect(fleetTextV1("en","finding.worker-silent",values)).toContain("g1-m05");
});
