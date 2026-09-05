/** 📤️ Strict schema and source-roster join; native Syn tests independently parse the same sources. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";

//#region 🧬️ExportRoster
const fixture = await Bun.file(new URL("./🔣️.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./📐️.schema.json", import.meta.url)).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
const facade = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
const exports = /pub use dsl_derive::\{([^}]+)\};/.exec(facade)?.[1].split(",").map((name) => name.trim()).filter(Boolean).sort();
assert.deepEqual(exports, fixture.facadeExports);
const owner = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
assert.deepEqual([...owner.matchAll(/#\[proc_macro_derive\((\w+)/g)], [], "the implementation owner must not register crate-root derives");
const source = await Bun.file(new URL("../../📦️packages/🦀️rust/🦀️.rs", import.meta.url)).text();
const names = [...source.matchAll(/#\[proc_macro_derive\((\w+)/g)].map((match) => match[1]).sort();
assert.deepEqual(names, fixture.registeredDerives);
assert(fixture.facadeExports.every((name: string) => names.includes(name)));
assert(fixture.traitOnly.every((name: string) => !names.includes(name)));
for (const mutant of [{ ...fixture, extra: true }, { ...fixture, facadeExports: ["DslRecord", "DslRecord"] }, { ...fixture, traitOnly: ["invalid-name"] }]) assert(!validate(mutant));
console.log(`[DEBUG] DSL macro export source facade=${fixture.facadeExports.length} registered=${fixture.registeredDerives.length} hostileRejections=3 nativeOracle=Syn`);
//#endregion 🧬️ExportRoster
