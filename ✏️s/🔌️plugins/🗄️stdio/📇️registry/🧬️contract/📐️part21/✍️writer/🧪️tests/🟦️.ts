/** 🧪️ Header/instance/footer composition uses the shared neutral Part21 fixture. */
import assert from "node:assert/strict";
import fixture from "../🧪️fixtures/🔣️stream.json";
import { parsePart21, writePart21, writePart21Header, writePart21Instance, writePart21Footer } from "../../🟦️.ts";
export function verifyPart21WriterParts(): { cases: number } { const doc = parsePart21(fixture.input); const source = writePart21Header(doc.header) + doc.instances.map((instance) => writePart21Instance(instance)).join("") + writePart21Footer(); assert.deepEqual(parsePart21(source), doc); assert.equal(doc.instances.length, fixture.expected.instances); assert.equal(doc.instances[0].entities[0].args[0].kind === "string" && doc.instances[0].entities[0].args[0].value, fixture.expected.unicode); assert.deepEqual(parsePart21(writePart21(doc)), doc); return { cases: 1 }; }
