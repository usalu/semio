import {test,expect} from "bun:test";import Ajv from "ajv";import {Database} from "bun:sqlite";import fixture from "../🧫️fixtures/🔣️.json";import schema from "./schema.json";
import sourceSchema from "../../🧬️schema/🔣️.json";
test("original snapshot capture reports transfer and issuer clone",()=>{const law=fixture.captureWork;expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ?+? success, 0 denied").get(law.transferRead,law.cloneFactory)).toEqual({success:law.success,denied:law.denied})}finally{db.close()}});

test("captured snapshot retires only its original custody", () => {
    expect(new Ajv({strict:true}).validate(sourceSchema, fixture)).toBe(true);
    const db = new Database(":memory:");
    try {
        for (const row of fixture.cases) {
            const result = db.query(`
                with original_registry as (select 1 + ? as aliases),
                     custody(read, registry, active, factory) as (
                         select 1, 1, 0, 1
                         union all select 0, 1, 0, 1
                         union all select 0, 1, 1, 1 from original_registry where aliases = 1
                         union all select 0, 0, 0, 1
                         union all select 0, 0, 0, 0
                     )
                select max(active) arcRootAdmission, max(active) extraDepth,
                       (select aliases > 1 from original_registry) returnedReadSurvives,
                       sum(read + registry + active + factory = 0) complete
                from custody
            `).get(Number(row.registryHeld)) as Record<string, number>;
            const expected = fixture.retirement[row.registryHeld ? "shared" : "last"];
            expect(result).toEqual({
                arcRootAdmission: Number(expected.arcRootAdmission),
                extraDepth: expected.extraDepth,
                returnedReadSurvives: Number(expected.returnedReadSurvives),
                complete: Number(expected.complete),
            });
        }
    } finally {
        db.close();
    }
});
