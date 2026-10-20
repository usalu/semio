import Ajv from "ajv";
import { createRequire } from "node:module";
import { Readable, type Writable } from "node:stream";
import { closeSync, ftruncateSync, mkdirSync, mkdtempSync, openSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { expect, it } from "vitest";
import fixtures from "../../🧫️fixtures/🐳️runtime/🔣️.json" with { type: "json" };
import schema from "../../🚀️deploy/📦️image/🧬️schema/🔣️.json" with { type: "json" };
import { courseImageContent, courseImageContext, courseImageFile, readCourseImageEntry } from "../../🚀️deploy/📦️image/🟦️.ts";

const validate = new Ajv().compile(schema);
const tarStream = createRequire(import.meta.url)("tar-stream") as { extract(): Writable };
const bytes = async function* (text: string): AsyncGenerator<Uint8Array> { yield Buffer.from(text); };
it("encodes portable runtime payloads with exactly the names, modes and bytes decoded by tar-stream", async () => {
  const entries = fixtures.entries.map(({ text, ...row }) => ({ ...row, kind: row.kind as "file" | "directory", bytes: Buffer.byteLength(text), chunks: () => bytes(text) }));
  for (const { chunks: _chunks, ...row } of entries) expect(validate(row)).toBe(true);
  const archive = await courseImageContext(entries, AbortSignal.timeout(5000));
  const reader = tarStream.extract(), actual: { path: string; mode: number; kind: string; text: string }[] = [];
  const finished = new Promise<void>((resolve, reject) => { reader.once("finish", resolve); reader.once("error", reject); });
  reader.on("entry", (header: { name: string; mode: number; type: string }, stream: Readable, next: (error?: Error) => void) => { const parts: Buffer[] = []; stream.on("data", chunk => parts.push(Buffer.from(chunk))); stream.once("error", next); stream.once("end", () => { actual.push({ path: header.name, mode: header.mode, kind: header.type, text: Buffer.concat(parts).toString("utf8") }); next(); }); stream.resume(); });
  Readable.from([archive]).pipe(reader);
  await finished;
  expect(actual).toEqual(fixtures.entries.map(row => ({ path: row.path, mode: row.mode, kind: row.kind, text: row.text })));
});
for (const row of fixtures.invalid) it(`refuses unsafe image context entry ${JSON.stringify(row)}`, () => {
  expect(() => readCourseImageEntry(row)).toThrow();
});
it("rejects content whose actual streamed byte count differs from the declared entry", async () => {
  for (const count of [1, 3]) await expect(courseImageContext([{ path: "proctor", bytes: count, mode: 365, kind: "file", chunks: () => bytes("ab") }], AbortSignal.timeout(5000))).rejects.toThrow();
});

function scratch(): string {
  const parent = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated", "image-inputs") : join(realpathSync(tmpdir()), "image-inputs");
  mkdirSync(parent, { recursive: true });
  return mkdtempSync(join(parent, "run-"));
}

for (const row of fixtures.catalogs) it(`bounds the authored catalog roster ${row.id}`, () => {
  const root = scratch(), teaching = join(root, "🎓️teaching"), course = join(teaching, "course.json");
  mkdirSync(teaching); writeFileSync(course, JSON.stringify({ quizzes: row.quizzes })); writeFileSync(join(teaching, "quiz.json"), "{}"); writeFileSync(join(root, "outside.json"), "{}");
  try {
    if (row.valid) expect(courseImageContent(root, course, AbortSignal.timeout(5000)).map(entry => entry.path)).toEqual(["content/course.json", "content/quiz.json"]);
    else expect(() => courseImageContent(root, course, AbortSignal.timeout(5000))).toThrow();
  } finally { rmSync(root, { recursive: true, force: true }); }
});

it("refuses an oversized catalog before reading or parsing it", () => {
  const root = scratch(), file = join(root, "oversized.json"), descriptor = openSync(file, "w");
  try { ftruncateSync(descriptor, 16777217); } finally { closeSync(descriptor); }
  try { expect(() => courseImageContent(root, file, AbortSignal.timeout(5000))).toThrow("Bounded physical image content required"); }
  finally { rmSync(root, { recursive: true, force: true }); }
});

it("bounds the quiz roster before resolving or opening any referenced file", () => {
  const root = scratch(), course = join(root, "course.json");
  writeFileSync(course, JSON.stringify({ quizzes: Array.from({ length: 58 }, (_, index) => `absent-${index}.json`) }));
  try { expect(() => courseImageContent(root, course, AbortSignal.timeout(5000))).toThrow("Course image requires its finite explicit quiz roster"); }
  finally { rmSync(root, { recursive: true, force: true }); }
});

it("refuses linked source ancestry before opening payload bytes", () => {
  const root = scratch(), physical = join(root, "physical"), linked = join(root, "linked");
  mkdirSync(physical); writeFileSync(join(physical, "Dockerfile"), "FROM scratch\n");
  try {
    symlinkSync(physical, linked, process.platform === "win32" ? "junction" : "dir");
    expect(() => courseImageFile("Dockerfile", join(linked, "Dockerfile"), AbortSignal.timeout(5000))).toThrow("Physical image content ancestry required");
  } finally { rmSync(root, { recursive: true, force: true }); }
});

it("honors original context cancellation before consuming source bytes", async () => {
  const controller = new AbortController(), reason = Error("original image export cancelled"); controller.abort(reason);
  await expect(courseImageContext([{ path: "proctor", bytes: 2, mode: 365, kind: "file", chunks: () => bytes("ab") }], controller.signal)).rejects.toBe(reason);
});
