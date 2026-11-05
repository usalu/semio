/** 🌱️ Neutral field coverage and independent initial-value oracle for final-address Rust births. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import fixture from "../🧫️fixtures/🌱️initialization/🔣️.json";
import schema from "../🧫️fixtures/🌱️initialization/📋️schema.json";

const root = new URL("../../", import.meta.url);
const body = (source: string, marker: string): string => {
    const start = source.indexOf(marker);
    expect(start).toBeGreaterThanOrEqual(0);
    const open = source.indexOf("{", start);
    let depth = 1, end = open + 1;
    while (depth > 0 && end < source.length) { if (source[end] === "{") depth++; if (source[end] === "}") depth--; end++; }
    expect(depth).toBe(0);
    return source.slice(open + 1, end - 1);
};
const initialValue = (field: string, expression: string): unknown => {
    if (/^(?:ManuallyDrop::new\()?None/.test(expression)) return null;
    if (expression === "false") return false;
    if (expression === "0") return 0;
    if (/^Vec(?:Deque)?::new\(\)$/.test(expression)) return [];
    if (field === "session" && expression === "TimeTravelSession::default()") return "defaultSession";
    if (field === "turn_clock" && expression === "semio_framework_job::default_now_us") return "defaultNowUs";
    if (field === "label_of" && expression === "label_of") return "labelOf";
    if (expression === "Default::default()" && field === "command_retirements") return "emptyRetirementQueue";
    if (expression === "Default::default()" && field === "commit_request_close") return "emptyRetirement";
    throw Error(field + ": unsupported initial expression " + expression);
};

test("validates the language-neutral default corpus with independent Ajv", () => {
    const validate = new Ajv().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});
for (const owner of fixture.owners) test(owner.name + " covers every field exactly once and preserves neutral defaults", () => {
    const source = readFileSync(new URL(owner.path, root), "utf8");
    const declaration = body(source, "struct " + owner.type);
    const fields = [...declaration.matchAll(/pub(?:\([^)]*\))?\s+(\w+)\s*:/g)].map(match => match[1]);
    expect(fields.sort()).toEqual(Object.keys(owner.initial).sort());
    const defaults = body(source, owner.name === "ledger" ? "fn default() -> Self" : "pub fn new(label_of").replace(/\s+/g, "");
    for (const [field, value] of Object.entries(owner.initial)) {
        if (field === "label_of") { expect(defaults).toContain("label_of,"); continue; }
        const expression = value === null ? (field === "snapshot_issuers" || field === "snapshot_close" ? "ManuallyDrop::new(None)" : "None")
            : value === false ? "false" : value === 0 ? "0"
            : Array.isArray(value) && value.length > 0 ? (field === "snapshot_aliases" ? "ManuallyDrop::new(std::array::from_fn(|_|None))" : "std::array::from_fn(|_|None)")
            : Array.isArray(value) ? (field === "retirements" ? "VecDeque::new()" : "Vec::new()")
            : value === "defaultSession" ? "TimeTravelSession::default()"
            : value === "emptyDocument" ? "TimeTravelStoreState::new(time_travel_mutation_label::<A>)"
            : value === "defaultNowUs" ? "semio_framework_job::default_now_us" : "Default::default()";
        expect(defaults).toContain(field + ":" + expression);
    }
    const initialize = body(source, "pub unsafe fn initialize_field_at");
    const quote = body(source, "pub fn initial_field_demands");
    const written = [...initialize.matchAll(/addr_of_mut!\(\(\*target\)\.(\w+)\)/g)].map(match => match[1]);
    expect(written.sort()).toEqual(fields);
    const arms = [...initialize.matchAll(/^\s*(\d+) => (.+)$/gm)];
    const demandArms = [...quote.matchAll(/^\s*(\d+) => (.+)$/gm)];
    const expectedIndices = owner.name === "ledger"
        ? [...Array.from({length: 10}, (_, index) => index), ...Array.from({length: 16}, (_, index) => index + 51)]
        : Array.from({length: 41}, (_, index) => index);
    expect(arms.map(match => Number(match[1]))).toEqual(expectedIndices);
    expect(demandArms.map(match => Number(match[1]))).toEqual(expectedIndices);
    const values: Record<string, unknown> = {};
    for (const arm of arms) {
        const field = arm[2].match(/addr_of_mut!\(\(\*target\)\.(\w+)\)/)?.[1];
        expect(field).toBeDefined();
        const demand = demandArms.find(match => match[1] === arm[1])![2];
        if (arm[2].includes("initialize_absent_array_at")) {
            const length = Number(arm[2].match(/,\s*(\d+)>\(std::ptr/)?.[1]);
            expect(length).toBe(field === "snapshot_aliases" ? 8 : 2);
            expect(demand).toContain("absent_array_bytes");
            expect(demand).toContain("body.max(1)");
            values[field!] = Array(length).fill(null);
        } else if (arm[2].includes("initialize_absent_field_at")) {
            expect(demand).toContain("absent_field_bytes");
            expect(demand).toContain("body.max(1)");
            values[field!] = null;
        } else {
            expect(arm[2]).toContain("byte != 0 || count < 2 * size_of::<");
            expect(demand).toContain("2 * size_of::<");
            const expression = arm[2].match(/\.write\(([^;]+)\);/)?.[1];
            expect(expression).toBeDefined();
            values[field!] = initialValue(field!, expression!);
        }
    }
    if (owner.name === "ledger") {
        for (const port of [initialize, quote]) {
            expect(port).toContain("(10..10 + document_fields).contains(&field)");
            expect(port).toContain("field - 10, byte");
        }
        expect(source).toContain("INITIAL_FIELDS: usize = 26 + TimeTravelStoreState");
        expect(initialize).toContain("time_travel_mutation_label::<A>");
        values.document = "emptyDocument";
        expect([...Array.from({length: 10}, (_, index) => index),
            ...Array.from({length: 41}, (_, index) => index + 10),
            ...Array.from({length: 16}, (_, index) => index + 51)]).toEqual(Array.from({length: 67}, (_, index) => index));
    } else {
        expect(source).toContain("INITIAL_FIELDS: usize = 41");
        expect(initialize).toContain("snapshot_aliases).cast::<[Option<");
        expect(initialize).toContain("snapshot_issuers).cast::<Option<");
        expect(initialize).toContain("snapshot_close).cast::<Option<");
    }
    expect(values).toEqual(owner.initial);
    expect(initialize).not.toMatch(/target\.write|write\(Self|write\(None\)|ManuallyDrop::new\(None\)|undefined/);
    expect(quote).not.toContain("size_of::<Self>");
});
