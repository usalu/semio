import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

const base = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(resolve(base, "🔣️.json"), "utf8"));

test("completion checkout preserves its bounded owner on pending and refusal", () => {
  const schema = JSON.parse(readFileSync(resolve(base, "🧬️schema/🔣️.json"), "utf8"));
  expect(new Ajv().compile(schema)(fixture)).toBe(true);
  const ready = fixture.ready.map((row: any) => ({ ...row }));
  let held: { revision: number; restoresInteraction: boolean } | undefined;
  let index = 0;
  const db = new Database(":memory:");
  db.exec("CREATE TABLE owner(revision INTEGER PRIMARY KEY, ordinal INTEGER, restoring INTEGER, held INTEGER)");
  for (let at = 0; at < ready.length; at++) db.query("INSERT INTO owner VALUES(?,?,?,0)").run(ready[at].revision, at, Number(ready[at].restoresInteraction));
  for (const step of fixture.steps) {
    if (step.op === "checkout") {
      index = step.index;
      held = ready.splice(index, 1)[0];
      db.query("UPDATE owner SET held=1 WHERE revision=?").run(held!.revision);
    } else if (step.op === "enqueue") {
      const admitted = ready.length + Number(!!held) < fixture.capacity - 1;
      const oracle = (db.query("SELECT COUNT(*) AS n FROM owner").get() as { n: number }).n < fixture.capacity - 1;
      expect(admitted).toBe(oracle);
      expect(admitted).toBe(step.admitted);
      if (admitted) {
        ready.push({ revision: step.revision, restoresInteraction: false });
        db.query("INSERT INTO owner VALUES(?,?,0,0)").run(step.revision, 100 + step.revision);
      }
    } else if (step.op === "pending" || step.op === "refuse") {
      ready.splice(Math.min(index, ready.length), 0, held!);
      db.query("UPDATE owner SET held=0 WHERE revision=?").run(held!.revision);
      held = undefined;
    } else {
      db.query("DELETE FROM owner WHERE revision=?").run(held!.revision);
      held = undefined;
    }
    expect(ready.map((owner: any) => owner.revision)).toEqual(step.ready);
    expect(held?.revision ?? null).toBe(step.held);
    expect(ready.length + Number(!!held)).toBe(step.length);
    expect((db.query("SELECT COUNT(*) AS n FROM owner").get() as { n: number }).n).toBe(step.length);
    expect(ready.some((owner: any) => owner.restoresInteraction) || !!held?.restoresInteraction).toBe(step.ownerOutstanding);
    expect((db.query("SELECT COUNT(*) AS n FROM owner WHERE restoring=1").get() as { n: number }).n > 0).toBe(step.ownerOutstanding);
  }
  db.exec("CREATE TABLE application(phase TEXT PRIMARY KEY, fuel INTEGER, items INTEGER)");
  for (const phase of fixture.application.phases) db.query("INSERT INTO application VALUES(?,1,1)").run(phase);
  expect(db.query("SELECT SUM(items) AS n FROM application").get()).toEqual({ n: fixture.application.metadataItems });
  expect(db.query("SELECT MAX(fuel) AS n FROM application").get()).toEqual({ n: fixture.application.fuelPerTurn });
  db.exec("CREATE TABLE recovery(stage TEXT PRIMARY KEY, metadata INTEGER, replacements INTEGER)");
  for (const stage of fixture.recovery.stages) db.query("INSERT INTO recovery VALUES(?,1,0)").run(stage);
  expect(db.query("SELECT SUM(metadata) AS n FROM recovery").get()).toEqual({ n: fixture.recovery.metadataItems });
  expect(db.query("SELECT SUM(replacements) AS n FROM recovery").get()).toEqual({ n: fixture.recovery.replacementAllocations });
  db.close();
  const core = readFileSync(resolve(base, "../🦀️.rs"), "utf8");
  const mailbox = readFileSync(resolve(base, "../../🧊️renderer/📮️mailbox-impl/🦀️.rs"), "utf8");
  expect(core).toContain("fn checkout_at");
  expect(core).toContain("fn return_checkout");
  expect(mailbox).toContain("fn apply_pending_step");
  expect(mailbox).toContain("completion.apply = RuntimeApply::Applied");
  expect(mailbox).toContain("let child_controls_work");
  expect(mailbox).toContain("fn restore_presenter_interaction_step");
  expect(mailbox).toContain("fn recover_original_frame_step");
  expect(mailbox).toContain("FrameOriginalRecovery::release_empty");
  const actions = readFileSync(resolve(base, "../../🧊️renderer/🛎️frame-actions/🦀️.rs"), "utf8") + readFileSync(resolve(base, "../../🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs"), "utf8");
  expect(actions).toContain("Arc::try_unwrap(original.owner)");
  expect(actions).toContain("owner.try_borrow_mut");
  expect(actions).toContain("retain_cancelled_action(action)");
  expect(actions).toContain("ControlledRetirement::retain((controller_id, action, args))");
  expect(actions).toContain("owner.return_alias_step(cx.original_cancel_token(), grant)");
  expect(mailbox).not.toContain("applied_revisions: Mutex::new(std::collections::HashMap::new())");
});
