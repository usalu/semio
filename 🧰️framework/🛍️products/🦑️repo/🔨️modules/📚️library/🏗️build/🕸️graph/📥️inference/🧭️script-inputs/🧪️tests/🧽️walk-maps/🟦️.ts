/** 🧽️ Contract: the digest-keyed walk facts and walk members of the closure entries are bounded, named caches (they were unbounded maps that only a process restart emptied).
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{boundedCacheStatsV1}from"../../../../../🧊️bounded/🟨️.mjs";
import{resetClosureEntriesV1}from"../../🟨️.mjs";

test("the two walk caches are registered with finite bounds and a reset empties them",()=>{
 resetClosureEntriesV1();
 const rows=boundedCacheStatsV1().filter(row=>row.name.startsWith("inference:closure-walk-"));
 expect(rows.map(row=>row.name).sort()).toEqual(["inference:closure-walk-facts","inference:closure-walk-members"]);
 for(const row of rows){expect(Number.isSafeInteger(row.maxEntries)).toBe(true);expect(row.entries).toBe(0);}
});
