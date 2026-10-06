/** 📤️ Strict schema and source-roster join; native Syn tests independently parse the same sources. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";

//#region 🧬️ExportRoster
const fixture = await Bun.file(new URL("../../🧫️fixtures/📤️macro-exports/🔣️.json", import.meta.url)).json();
const document = await Bun.file(new URL("../../🧬️schema/🔣️.json", import.meta.url)).json();
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(document);


const facade = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
const exports = /pub use dsl_derive::\{([^}]+)\};/.exec(facade)?.[1].split(",").map((name) => name.trim()).filter(Boolean).sort();
assert.deepEqual(exports, fixture.facadeExports);
const owner = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
assert.deepEqual([...owner.matchAll(/#\[proc_macro_derive\((\w+)/g)], [], "the implementation owner must not register crate-root derives");
const source = await Bun.file(new URL("../../📦️packages/🦀️rust/🦀️.rs", import.meta.url)).text();
const names = [...source.matchAll(/#\[proc_macro_derive\((\w+)/g)].map((match) => match[1]).sort();
assert.deepEqual(names, fixture.registeredDerives);
const macros = [...source.matchAll(/#\[proc_macro\]\s*(?:\/\/[^\n]*\n\s*)?pub fn (\w+)/g)].map((match) => match[1]).sort();
assert.deepEqual(macros, fixture.registeredMacros);
assert(fixture.facadeExports.every((name: string) => [...names,...macros].includes(name)));
assert(fixture.traitOnly.every((name: string) => !names.includes(name)));

//#endregion 🧬️ExportRoster

await import("../🪆️record-owner/🟦️.ts");

await import("../🚪️diff-codecs/🟦️.ts");
