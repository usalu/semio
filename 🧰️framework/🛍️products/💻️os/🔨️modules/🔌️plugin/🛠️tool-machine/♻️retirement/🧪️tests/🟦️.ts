/** 🧩️ Tool integration follows the canonical neutral original alias authority. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import Ajv from "ajv";
const vectors=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
test("tool original custody vectors satisfy their neutral schema",()=>{const validate=new Ajv2020({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8")));expect(validate(vectors)).toBe(true);});
/** 🎟️ Independent transaction ownership survives zero and one-below retirement grants. */
test("tool member emission forwards original mutation and cause retirement grants",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE custody(id INTEGER PRIMARY KEY, owned INTEGER, released INTEGER)");db.query("INSERT INTO custody VALUES(7,1,0)").run();
 for(const admitted of vectors.admission){if(admitted)db.query("UPDATE custody SET owned=0,released=1 WHERE id=7").run();expect(db.query("SELECT owned+released AS total FROM custody WHERE id=7").get()).toEqual({total:1});}db.close();
 const native=readFileSync(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url),"utf8");
 expect(native.includes("store.retire_owned_mutation(&mut typed.current,self.grant)")).toBe(true);expect(native.includes("fn close_step(&mut self,grant:RetainedCloneGrant)")).toBe(true);expect(native.includes("::protocol::close_protocol_error_one(&mut self.refusal,grant)")).toBe(true);
 console.log("[DEBUG] SQLite member mutation custody conserves original through zero/below/exact ownership transfer");
});

/** 🪞️ SQLite's held original batch blocks a second refold until exact maintenance admission. */
test("tool overlay originals wait for granted maintenance instead of cold draining",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE refolds(generation INTEGER PRIMARY KEY, held INTEGER)");db.query("INSERT INTO refolds VALUES(1,1)").run();
 expect(db.query("SELECT COUNT(*) AS blockers FROM refolds WHERE held=1").get()).toEqual({blockers:1});
 db.query("UPDATE refolds SET held=0 WHERE generation=1").run();db.query("INSERT INTO refolds SELECT 2,1 WHERE NOT EXISTS(SELECT 1 FROM refolds WHERE held=1)").run();expect(db.query("SELECT MAX(generation) AS current FROM refolds").get()).toEqual({current:2});db.close();
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("pending_overlay_aliases")).toBe(true);expect(source.includes("tool_overlay_retirement_step")).toBe(true);expect(source.includes("retire_overlay_alias(self.store.retire_snapshot_alias(alias))")).toBe(false);
 console.log("[DEBUG] SQLite original overlay custody pauses later refolds until granted alias maintenance");
});

/** 🎛️ SQLite admits only one original run against each driver's turn budget. */
test("tool run driver consumes one original grant across multiple runs",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE admitted(turn INTEGER PRIMARY KEY, run INTEGER, copied INTEGER)");
 for(let turn=0;turn<vectors.driver.turns;turn++){const run=vectors.driver.runs[turn%vectors.driver.runs.length];db.query("INSERT INTO admitted VALUES(?,?,?)").run(turn,run,vectors.driver.copyBytes);expect(db.query("SELECT SUM(copied) AS total FROM admitted WHERE turn=?").get(turn)).toEqual({total:vectors.driver.copyBytes});}
 expect(db.query("SELECT run,COUNT(*) AS count FROM admitted GROUP BY run ORDER BY run").all()).toEqual(vectors.driver.expectedCounts);db.close();
 const native=readFileSync(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url),"utf8");const driver=native.slice(native.indexOf("pub(crate) async fn drive_tool_run_turn"),native.indexOf("fn retire_tool_runs_until"));
 expect(driver.includes("driver_cursor")).toBe(true);expect(driver.includes("for run in runs")).toBe(false);expect(driver.includes("let runs: Vec")).toBe(false);
 console.log("[DEBUG] SQLite three original runs each receive three distinct 4096 turns; no same-turn grant reuse or run inventory allocation");
});

/** 🧾️ Original failure text enters the same first-party typed frontier as its native oracle. */
test("tool member ValueError retirement keeps canonical typed original custody",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE cause(id INTEGER PRIMARY KEY, phase TEXT, backing INTEGER)");db.query("INSERT INTO cause VALUES(1,'original',8192)").run();
 for(const phase of vectors.cause.phases){db.query("UPDATE cause SET phase=? WHERE id=1").run(phase);expect(db.query("SELECT backing FROM cause WHERE phase<> 'terminal'").all()).toEqual(phase==="terminal"?[]:[{backing:8192}]);}db.close();
 const native=readFileSync(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url),"utf8");expect(native.includes("ControlledRetirement<ValueError>")).toBe(true);expect(native.includes("retirement_refusal_close")).toBe(true);expect(native.includes("match &error.message")).toBe(false);
 console.log("[DEBUG] SQLite borrowed/unused owned cause custody enters canonical typed frontier before exact backing release");
});

/** 🪟️ SQLite's unchanged original preview remains retained through missing owner and pressure refusal. */
test("window preview aliases retain in place until genuine granted maintenance",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE previews(id INTEGER PRIMARY KEY, phase TEXT, backing INTEGER)");db.query("INSERT INTO previews VALUES(7,'original',8192)").run();
 for(const admitted of vectors.admission){if(admitted)db.query("UPDATE previews SET phase='pending' WHERE id=7").run();expect(db.query("SELECT id,backing FROM previews").all()).toEqual([{id:7,backing:8192}]);}db.query("UPDATE previews SET phase='terminal',backing=0 WHERE id=7").run();expect(db.query("SELECT backing FROM previews").get()).toEqual({backing:0});db.close();
 const native=readFileSync(new URL("../../../🪟️window/🎚️config/🦀️.rs",import.meta.url),"utf8");expect(native.includes("pending_preview")).toBe(true);expect(native.includes("preview_retirement_step")).toBe(true);expect(native.includes("preview:&mut Option<WindowConfigSnapshot>")).toBe(true);expect(native.includes("retire_overlay_alias(partition.store.retire_snapshot_alias")).toBe(false);
 console.log("[DEBUG] SQLite original window preview survives missing/type/pressure refusal and enters exact native maintenance without cold drains");
});

/** 🧰️ Logical emptiness retains original backing and unopened run metadata until admission. */
test("tool ledger close preserves unopened entries and releases empty backing explicitly",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE ledger(id INTEGER PRIMARY KEY, entries INTEGER, capacity INTEGER, released INTEGER)");for(const row of vectors.ledger)db.query("INSERT INTO ledger VALUES(?,?,?,?)").run(row.id,row.entries,row.capacity,row.released);
 db.query("UPDATE ledger SET released=capacity,capacity=0 WHERE entries=0").run();expect(db.query("SELECT id,entries,capacity,released FROM ledger ORDER BY id").all()).toEqual(vectors.closedLedger);db.close();
 const native=readFileSync(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url),"utf8");const begin=native.slice(native.indexOf("    pub fn begin_close(&mut self)"),native.indexOf("    /// 🧹️ One bounded close unit"));expect(begin.includes("while !self.entries.is_empty()")).toBe(false);expect(native.includes("close_ledger_backing")).toBe(true);expect(native.includes("ledger_backing_is_empty")).toBe(true);
 console.log("[DEBUG] SQLite logical empty8192 owner closes only its exact backing; unopened original entry remains intact before granted cancellation");
});

/** 📄️ An original job outcome remains staged while a distinct physical release grant arrives. */
test("tool job outcomes retain originals between independently granted closure turns",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE outcome(id INTEGER PRIMARY KEY, retained INTEGER, released INTEGER)");db.query("INSERT INTO outcome VALUES(1,?,0)").run(vectors.jobOutcome.capacity);
 for(const turn of vectors.jobOutcome.turns){db.query("UPDATE outcome SET retained=retained-?,released=released+? WHERE id=1").run(turn.released,turn.released);expect(db.query("SELECT retained,released FROM outcome").get()).toEqual({retained:turn.retained,released:vectors.jobOutcome.capacity-turn.retained});}db.close();
 console.log("[DEBUG] SQLite original job outcome remains intact on zero/below admission and releases only its independent physical backing grant");
 const native=readFileSync(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url),"utf8");expect(native).toContain("pending_job_outcome");expect(native).toContain("pending_job_outcome: semio_framework_job::JobOutcomeSlot");expect(native).toContain("semio_framework_job::step_outcome_slot_retirement_demands(original)");expect(native).toContain("semio_framework_job::close_step_outcome_slot(original,grant)");expect(native).not.toContain("JobPayloadCloseStep");expect(native).not.toContain("fn close_job_payload");expect(native).not.toContain("fn close_step_outcome");
});

/** 📭️ Native ToolRun integration preserves the shared neutral in-place outcome header law. */
test("tool outcome slot retains original metadata until separately granted presence removal",()=>{
 const root=new URL('../../../../../../../🔨️modules/🧵️job/♻️retirement/📄️payload/',import.meta.url);const law=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",root),"utf8"));expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("🧬️schema/🔣️.json",root),"utf8")))(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE outcomes(variant TEXT,pages INTEGER,grant INTEGER,present INTEGER)");for(const variant of law.variants)for(const pages of law.pages)for(const grant of [0,law.slotPresenceBytes])db.query("INSERT INTO outcomes VALUES(?,?,?,?)").run(variant,pages,grant,Number(grant<law.slotPresenceBytes));expect(db.query("SELECT COUNT(*) AS n FROM outcomes WHERE grant=0 AND present=1").get()).toEqual({n:18});expect(db.query("SELECT COUNT(*) AS n FROM outcomes WHERE grant=1 AND present=0").get()).toEqual({n:18});}finally{db.close();}
 console.log("[DEBUG] SQLite18 ToolRun original outcome headers survive zero-grant and retire in place at shared neutral presence1, without an8256-byte Option move");const native=readFileSync(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url),"utf8");expect(native).toContain("pending_job_outcome: semio_framework_job::JobOutcomeSlot");expect(native).not.toContain("pending_job_outcome: Option<semio_framework_job::StepOutcome>");
});
