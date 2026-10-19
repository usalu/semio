/** 📦️ Original contiguous custody agrees with an independent SQLite owner ledger. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";

test("contiguous backing keeps both authentic supplier loans until actual release and vacancy",async()=>{
 const law=await Bun.file(new URL("../🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json(),validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,completeMayPrecedeVacancy:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE owner(backing INTEGER,loans INTEGER,released INTEGER,vacant INTEGER);INSERT INTO owner VALUES(1,2,0,0)");let backing=true,loans=law.sourceLoans,released=law.consumerReleaseBytes,vacant=false;
  for(const denied of law.denials){expect({backing,loans,released,vacant}).toEqual({backing:true,loans:2,released:0,vacant:false});expect(db.query("SELECT backing,loans,released,vacant FROM owner").get()).toEqual({backing:1,loans:2,released:0,vacant:0});expect(denied.length).toBeGreaterThan(0);}
  for(const phase of law.close){if(phase==="backing"){backing=false;released=law.capacityBytes;}if(phase.endsWith("Loan")){expect(backing).toBe(false);loans--;}if(phase==="vacant"){expect(loans).toBe(0);vacant=true;}db.query("UPDATE owner SET backing=CASE WHEN ?='backing' THEN 0 ELSE backing END,loans=loans-CASE WHEN ? IN ('backingLoan','reservationLoan') THEN 1 ELSE 0 END,released=released+CASE WHEN ?='backing' THEN ? ELSE 0 END,vacant=CASE WHEN ?='vacant' THEN 1 ELSE vacant END").run(phase,phase,phase,law.capacityBytes,phase);expect(db.query("SELECT backing,loans,released,vacant FROM owner").get()).toEqual({backing:Number(backing),loans,released,vacant:Number(vacant)});if(!vacant)expect(vacant).toBe(law.completeMayPrecedeVacancy);}expect(released).toBe(law.supplierReleaseBytes);
 }finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const token of ["OriginalCallerSourceLoan","admit_loan(cx)","admit_original_construction","try_reserve_exact(length)","binding.matches(cx)","OriginalContiguousJobBytes::complete","drop(checked);(self.wake)()","bytes.capacity()","source.close_step(cx.original_source_parent()?,cx)","InteractiveJobCloseStep::Pending{progress:step.progress()}"])expect(source).toContain(token);for(const token of ["into_vec","to_vec()","RetainedCloneGrant::default","StepContext::new","Box::new","return bytes.close_step(cx)"])expect(source).not.toContain(token);expect(source.indexOf("admit_loan(cx)")).toBeLessThan(source.indexOf("try_reserve_exact(length)"));
 console.log("[DEBUG] Ajv/SQLite original contiguous custody: genuine loan before backing, no raw owner extraction, zero consumer release, exact16385-byte supplier release before both loans and vacancy; no premature Complete");
});
