/** 📊️ Contract: the walk farm of a process reports its counters for the slice report: nothing while no farm was asked for, one row per workspace once a recipe asked, and no farm where the environment switched it off or fewer than two children are granted. @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{offloadClosureRecipeV1,recipeFarmStatsV1}from"../../🟨️.mjs";

test("the counters are empty before a recipe asked, name the workspace after, and show no farm where none is granted",async()=>{
 expect(recipeFarmStatsV1()).toEqual([]);
 const saved=process.env.SEMIO_GRAPH_WALK_FARM;process.env.SEMIO_GRAPH_WALK_FARM="off";
 try{expect(await offloadClosureRecipeV1("/nowhere",{fn:"fixture"})).toBeUndefined();expect(recipeFarmStatsV1()).toEqual([]);}
 finally{if(saved===undefined)delete process.env.SEMIO_GRAPH_WALK_FARM;else process.env.SEMIO_GRAPH_WALK_FARM=saved;}
 expect(await offloadClosureRecipeV1("/nowhere",{fn:"fixture"},{size:1,after:0})).toBeUndefined();
 const rows=recipeFarmStatsV1();
 expect(rows.length).toBe(1);expect(rows[0]!.key.startsWith("/nowhere")).toBe(true);expect(rows[0]!.farm).toBe(false);expect(rows[0]!.calls).toBe(0);
});
