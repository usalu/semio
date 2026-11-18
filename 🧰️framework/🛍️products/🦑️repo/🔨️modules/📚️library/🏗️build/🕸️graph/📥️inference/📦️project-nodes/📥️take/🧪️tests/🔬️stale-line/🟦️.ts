/** 🔬️ Contract: an owner that was taken from the fleet cache but does not hold against this tree is named in the log of the read, once per owner and in both languages, with the count so far, so the reason of a stale record can be looked up for that very record instead of guessed from a count.
 * @see ../../🟨️.mjs `createTakeLedgerV1` */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{createTakeLedgerV1}from"../../🟨️.mjs";

test("the stale line names the owner and the count, in English and German",()=>{
 const ledger=createTakeLedgerV1(10);
 ledger.mark("@semio-tech/a","stale");ledger.mark("@semio-tech/b","stale");ledger.mark("@semio-tech/c","present");
 expect(ledger.staleLine("en","@semio-tech/b")).toBe("owner @semio-tech/b was found in the fleet cache and taken, but its record does not hold against this tree (2 stale so far)");
 expect(ledger.staleLine("de","@semio-tech/b")).toBe("Projekt @semio-tech/b wurde im Flottenspeicher gefunden und geholt, sein Eintrag gilt aber für diesen Baum nicht (2 bisher veraltet)");
 expect(ledger.staleLine("fr","@semio-tech/a")).toContain("owner @semio-tech/a");
});
test("the take pass of the read says it for every stale outcome, at most 200 times",()=>{
 const source=readFileSync(new URL("../../../🟨️.mjs",import.meta.url),"utf8");
 expect(source).toContain('outcome==="stale"&&staleTold++<200');
 expect(source).toContain("ledger.staleLine(language,name)");
});
