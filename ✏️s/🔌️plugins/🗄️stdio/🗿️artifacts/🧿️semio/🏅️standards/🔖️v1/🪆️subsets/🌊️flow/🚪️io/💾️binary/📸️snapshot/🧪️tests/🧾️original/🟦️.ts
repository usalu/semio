/** 🌊️ Original close admission agrees with independent SQLite and the actual native owner source. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import f from "../../🧫️fixtures/🧾️original/🔣️.json";
import schema from "../../🧬️schema/🧾️original/🔣️.json";
const fields={items:"maximumItems",copy:"maximumCopyBytes",capacity:"maximumCapacityBytes",release:"maximumReleaseBytes",depth:"maximumDepth"}as const;
test("Flow original close neutral denials match SQLite independent currency predicate",()=>{
 expect(new Ajv().compile(schema)(f)).toBe(true);
 const db=new Database(":memory:");try{
  for(const row of f.denials){const demand={maximumItems:1,maximumCopyBytes:7,maximumCapacityBytes:37,maximumReleaseBytes:113,maximumDepth:4},key=fields[row.axis as keyof typeof fields],grant={...f.policy,[key]:demand[key]-1};const admitted=db.query("SELECT (? >= ? AND ? >= ? AND ? >= ? AND ? >= ? AND ? >= ?) AS admitted").get(grant.maximumItems,demand.maximumItems,grant.maximumCopyBytes,demand.maximumCopyBytes,grant.maximumCapacityBytes,demand.maximumCapacityBytes,grant.maximumReleaseBytes,demand.maximumReleaseBytes,grant.maximumDepth,demand.maximumDepth)as{admitted:number};expect(admitted.admitted).toBe(0);expect(row.refusal).toBe(row.axis==="depth"?"error":"progress");}
 }finally{db.close();}
 console.log("[DEBUG] Flow shared five denied currencies independently match SQLite; authored fixed caller policy retained");
});
test("Flow actual decoder original slots use funded birth, physical prefix release and four-axis delegation",async()=>{
 const source=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();
 expect(source.includes("SnapshotRetirementStep")).toBe(false);expect(source.includes("retirement::owned_retirement(")).toBe(false);
 for(const marker of["artifact_retirement_owned_birth_demands","artifact_retirement_admit_owned","artifact_retirement_box_close_step","artifact_retirement_owner_close","fn next_copy_byte_demand","fn next_capacity_byte_demand","fn next_release_byte_demand","fn next_depth_demand","fn next_demand"]){expect(source.includes(marker)).toBe(true);}
 expect(source.includes("ManuallyDrop<Option<MemberOpenRequest>>")).toBe(true);expect(source.includes("ManuallyDrop<Option<SemioFlowSnapshot>>")).toBe(true);expect(source.includes("ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>" )).toBe(true);
});
test.skipIf(!process.env.SEMIO_NATIVE_FLOW_DIRECTORY)("Flow actual native denied and terminal receipts agree with SQLite after the owner replay",async()=>{
 const directory=process.env.SEMIO_NATIVE_FLOW_DIRECTORY;if(!directory)throw Error("Native original Flow receipt directory is required");
 const receipt=await Bun.file(`${directory}/original-close.json`).json()as{rows:{items:number;copy:number;capacity:number;release:number;depth:number;demandItems:number;demandCopy:number;demandCapacity:number;demandRelease:number;demandDepth:number;admitted:boolean;requested:number;released:number}[];terminal:{requested:number;released:number}};
 const db=new Database(":memory:");try{for(const row of receipt.rows){const actual=db.query("SELECT (? >= ? AND ? >= ? AND ? >= ? AND ? >= ? AND ? >= ?) AS admitted").get(row.items,row.demandItems,row.copy,row.demandCopy,row.capacity,row.demandCapacity,row.release,row.demandRelease,row.depth,row.demandDepth)as{admitted:number};expect(Boolean(actual.admitted)).toBe(row.admitted);expect(row.requested).toBe(0);expect(row.released).toBe(0);}}finally{db.close();}
 expect(receipt.rows.length).toBeGreaterThan(5);expect(receipt.terminal).toEqual({requested:0,released:0});console.log(`[DEBUG] Flow actual native original close ${receipt.rows.length} denied receipts independently match SQLite; terminal drop zero`);
});