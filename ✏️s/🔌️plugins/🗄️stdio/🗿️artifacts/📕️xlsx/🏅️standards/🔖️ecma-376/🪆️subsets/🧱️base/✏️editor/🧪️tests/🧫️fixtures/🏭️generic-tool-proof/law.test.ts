import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../📊️sheet-grid/🔣️.json";

test("generic tool proof names the outer factory", () => {
    using db = new Database(":memory:");
    const oracle = db.query(`
        with recursive components(name) as (
            select substr(?1, 1, instr(?1, '<') - 1)
            union all
            select substr(name, instr(name, '::') + 2) from components where instr(name, '::') > 0
        )
        select name from components where instr(name, '::') = 0
    `);
    for (const row of fixture.genericToolProofs) {
        expect(oracle.get(row.qualifiedFactory)).toEqual({ name: row.factory });
    }
});
