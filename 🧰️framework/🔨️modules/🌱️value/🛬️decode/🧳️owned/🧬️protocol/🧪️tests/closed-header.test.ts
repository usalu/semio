import {test,expect} from "bun:test";
import law from "../../♻️header/🎟️work/📐️.json";
test("a finished receiver header is one fused box release: its work is the move and the free sharing one dispatch",()=>{const fused=law.moveWork+law.freeWork-1;expect(fused).toBeLessThan(law.moveWork+law.freeWork);expect(fused).toBeGreaterThan(law.moveWork)});
