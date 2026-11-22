import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../🧫️fixtures/publication-stall.json";

test("publication stalls count consecutive unchanged attempts per instance and operation", () => {
    const db = new Database(":memory:");
    try {
        db.exec("CREATE TABLE event(instance INTEGER, operation INTEGER, turn INTEGER, reset INTEGER)");
        const add = db.query("INSERT INTO event VALUES (?, ?, ?, ?)");
        db.transaction(() => {
            for (let turn = 1; turn <= fixture.rounds; turn++) {
                for (const stream of fixture.streams) {
                    if (turn <= stream.skipBefore) continue;
                    const checkpoint = stream.progressEvery > 0 && turn % stream.progressEvery === 0;
                    const receiptEvery = stream.receiptEvery ?? 0;
                    const receipt = receiptEvery > 0 && turn % receiptEvery === 0;
                    const changed = checkpoint || receipt;
                    add.run(stream.instance, stream.operation, turn, Number(changed || stream.genesis));
                }
            }
        })();
        const outcome = db.query<{ streak: number; firstFault: number | null }, [number, number, number]>(`
            WITH groups AS (
                SELECT turn, reset, sum(reset) OVER (ORDER BY turn) AS group_id
                FROM event WHERE instance = ?1 AND operation = ?2
            ), streaks AS (
                SELECT turn, sum(1 - reset) OVER (PARTITION BY group_id ORDER BY turn) AS streak
                FROM groups
            )
            SELECT coalesce((SELECT streak FROM streaks ORDER BY turn DESC LIMIT 1), 0) AS streak,
                   min(CASE WHEN streak >= ?3 THEN turn END) AS firstFault
            FROM streaks
        `);
        for (const stream of fixture.streams) {
            expect(outcome.get(stream.instance, stream.operation, fixture.faultCeiling)).toEqual({
                streak: stream.streak, firstFault: stream.firstFault,
            });
        }
    } finally {
        db.close();
    }
});

test("stall accounting follows the selected mounted owner across the publication unit", async () => {
    const source = await Bun.file(new URL(
        "../../🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs",
        import.meta.url,
    )).text();
    const positions = [
        "if cx.fuel_exhausted()",
        "return self.advance_latest_wins_command_one",
        "let Some((index, operation_id))",
        "self.tools.tool_operations.remove(operation_id)",
        "let before = publication_witness(&mounted)",
        "self.publish_original_emission_run(&mut mounted",
        "let after = publication_witness(&mounted)",
        "&mut mounted.publication_stall_streak, attempted, before != after",
        "self.tools.fault_stalled_typed_operation_publication(operation_id, streak)",
    ].map((statement) => source.indexOf(statement));
    expect(positions.every((position) => position >= 0)).toBe(true);
    expect(positions).toEqual([...positions].sort((a, b) => a - b));
    expect(source).not.toContain("AtomicU64");
    expect(source).toContain("Some(ArtifactToolCompletionValue::Genesis(_, Some(_)))");
    expect(source).toMatch(/driver_result\?;\s*if streak >= TYPED_OPERATION_STALL_FAULT_CEILING/);
    const pump = await Bun.file(new URL(
        "../../🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs",
        import.meta.url,
    )).text();
    expect(pump).toContain("let mut attempted = false;");
    expect(pump).toMatch(/if cx\.fuel_exhausted\(\).*return Ok\(attempted\)\}\s*attempted = true;/);
    const operation = await Bun.file(new URL("../../🧵️retained-command/🔄️full-operation/🦀️.rs", import.meta.url)).text();
    expect(operation).toContain("self.publication_checkpoint != Some(checkpoint) || ownership != RetainedCloneProgress::default()");
    const unit = await Bun.file(new URL("../../🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs", import.meta.url)).text();
    expect(unit).toMatch(/PreparationProgress\(checkpoint, ownership\) => \{\s*if !ownership.fits.*\n\s*mounted.note_publication_progress\(checkpoint, ownership\);/);
    const returnedReads = unit.slice(unit.indexOf("returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE"), unit.indexOf("let mut turn_ceiling"));
    expect(returnedReads).toMatch(/plugin_effect_turn\(cx,[\s\S]*?advance_document_snapshot_read_return\(&mut \*self\.stores, effect\)/);
    expect(returnedReads).toMatch(/if step\.progress\(\) != RetainedCloneProgress::default\(\)\s*\{\s*mounted\.note_publication_progress\(checkpoint, step\.progress\(\)\);/);
});
