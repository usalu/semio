import { open } from "node:fs/promises";
import { closeSync, fstatSync, lstatSync, openSync, readSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { validateJsonSchemaSubset } from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import type { ProctorExecutable } from "../../../../🛂️proctor/🏗️bootstrap/🟦️.ts";

/** 📋️ Names one bounded, link-free runtime context leaf without admitting host path selectors. */
export interface CourseImageEntry { readonly path: string; readonly bytes: number; readonly mode: number; readonly kind: "file" | "directory" }
/** 📤️ Supplies one entry's exact bytes while its original producer custody remains held. */
export interface CourseImageSource extends CourseImageEntry { chunks(): AsyncIterable<Uint8Array> }
/** 🧬️ Admits relative portable archive names and the finite runtime payload envelope. */
export function readCourseImageEntry(value: unknown): CourseImageEntry {
  if (validateJsonSchemaSubset(schema, value).length) throw Error("Closed bounded course image entry required");
  const row = value as CourseImageEntry, parts = row.path.split("/");
  if (isAbsolute(row.path) || row.path.startsWith("/") || parts.some(part => part === "." || part === "..") || parts.slice(0, -1).some(part => !part) || row.path.includes(":") || (row.kind === "directory" ? row.bytes !== 0 || !row.path.endsWith("/") : !parts.at(-1))) throw Error("Portable runtime context entry required");
  return Object.freeze({ ...row });
}

function tarHeader(row: CourseImageEntry): Buffer {
  let name = row.path, prefix = "";
  if (Buffer.byteLength(name) > 100) {
    const cuts = [...name.matchAll(/\//gu)].map(match => match.index).reverse();
    const cut = cuts.find(index => Buffer.byteLength(name.slice(0, index)) <= 155 && Buffer.byteLength(name.slice(index + 1)) <= 100);
    if (cut === undefined) throw Error("Runtime context path exceeds the portable tar header");
    prefix = name.slice(0, cut); name = name.slice(cut + 1);
  }
  const header = Buffer.alloc(512), text = (value: string, offset: number, size: number): void => { if (Buffer.byteLength(value) > size) throw Error("Runtime tar field overflow"); header.write(value, offset, size, "utf8"); };
  const octal = (value: number, offset: number, size: number): void => text(value.toString(8).padStart(size - 1, "0") + "\0", offset, size);
  text(name, 0, 100); octal(row.mode, 100, 8); octal(0, 108, 8); octal(0, 116, 8); octal(row.bytes, 124, 12); octal(0, 136, 12); text("        ", 148, 8); text(row.kind === "file" ? "0" : "5", 156, 1); text("ustar\0", 257, 6); text("00", 263, 2); text(prefix, 345, 155);
  text([...header].reduce((sum, byte) => sum + byte, 0).toString(8).padStart(6, "0") + "\0 ", 148, 8);
  return header;
}

/** 🧳️ Encodes a finite POSIX runtime tar directly from retained sources, with exact lengths and cancellation between chunks. */
export async function courseImageContext(sources: readonly CourseImageSource[], signal: AbortSignal, progress: (bytes: number) => void | Promise<void> = () => undefined): Promise<Uint8Array> {
  signal.throwIfAborted();
  if (!sources.length || sources.length > 64) throw Error("Finite runtime context roster required");
  const entries = sources.map(({ chunks: _chunks, ...row }) => readCourseImageEntry(row));
  if (new Set(entries.map(row => row.path.replace(/\/$/u, ""))).size !== entries.length) throw Error("Unique runtime context entries required");
  const length = entries.reduce((sum, row) => sum + 512 + Math.ceil(row.bytes / 512) * 512, 1024);
  if (length > 268435456) throw Error("Runtime context exceeds 256 MiB");
  const archive = Buffer.alloc(length);
  let cursor = 0;
  for (let index = 0; index < entries.length; index++) {
    signal.throwIfAborted();
    const row = entries[index]!;
    tarHeader(row).copy(archive, cursor); cursor += 512;
    let written = 0;
    for await (const chunk of sources[index]!.chunks()) {
      signal.throwIfAborted();
      if (!(chunk instanceof Uint8Array) || chunk.length > row.bytes - written) throw Error("Runtime entry exceeds its declared bytes");
      archive.set(chunk, cursor + written); written += chunk.length;
      await progress(cursor + written); signal.throwIfAborted();
    }
    if (written !== row.bytes) throw Error("Runtime entry did not supply its declared bytes");
    cursor += Math.ceil(row.bytes / 512) * 512;
  }
  return archive;
}

/** 📁️ Reads one physical source in bounded chunks without moving checked-in files into build staging. */
export function courseImageFile(path: string, source: string, signal: AbortSignal): CourseImageSource {
  source = resolve(source);
  signal.throwIfAborted();
  for (let parent = dirname(source);; parent = dirname(parent)) { const facts = lstatSync(parent); if (!facts.isDirectory() || facts.isSymbolicLink()) throw Error("Physical image content ancestry required"); if (dirname(parent) === parent) break; }
  const facts = lstatSync(source);
  if (!facts.isFile() || facts.isSymbolicLink() || facts.nlink !== 1 || facts.size > 16777216) throw Error("Bounded physical image content required");
  const row = readCourseImageEntry({ path, bytes: facts.size, mode: 0o444, kind: "file" });
  return { ...row, chunks: async function* () {
    signal.throwIfAborted();
    const file = await open(source, "r");
    try {
      const before = await file.stat();
      if (before.dev !== facts.dev || before.ino !== facts.ino || before.size !== facts.size || before.mtimeMs !== facts.mtimeMs) throw Error("Image content changed before capture");
      let read = 0;
      for (;;) { signal.throwIfAborted(); const chunk = Buffer.allocUnsafe(65536), result = await file.read(chunk, 0, chunk.length, null); if (!result.bytesRead) break; read += result.bytesRead; if (read > facts.size) throw Error("Image content grew during capture"); yield chunk.subarray(0, result.bytesRead); }
      const after = await file.stat();
      if (read !== facts.size || after.size !== facts.size || after.mtimeMs !== facts.mtimeMs) throw Error("Image content changed during capture");
    } finally { await file.close(); }
  } };
}

/** 📚️ Selects exactly the authored course and its referenced quizzes in their original relative layout. */
export function courseImageContent(root: string, course: string, signal: AbortSignal): CourseImageSource[] {
  const teaching = resolve(root, "🎓️teaching"), named = resolve(root, course), admission = courseImageFile("catalog.json", named, signal), original = lstatSync(named);
  if (!original.isFile() || original.isSymbolicLink() || original.nlink !== 1 || original.size !== admission.bytes) throw Error("Physical course catalog required");
  const file = openSync(named, "r");
  let catalog: { quizzes?: unknown };
  try {
    const before = fstatSync(file), bytes = Buffer.alloc(admission.bytes + 1);
    if (before.dev !== original.dev || before.ino !== original.ino || before.size !== admission.bytes || !before.isFile() || before.nlink !== 1) throw Error("Bounded physical course catalog required");
    let count = 0;
    for (;;) { signal.throwIfAborted(); const read = readSync(file, bytes, count, bytes.length - count, null); count += read; if (!read || count === bytes.length) break; }
    const after = fstatSync(file);
    if (count !== admission.bytes || after.size !== before.size || after.mtimeMs !== before.mtimeMs) throw Error("Course catalog changed during bounded capture");
    catalog = JSON.parse(bytes.subarray(0, count).toString("utf8")) as { quizzes?: unknown };
  } finally { closeSync(file); }
  if (!Array.isArray(catalog.quizzes) || catalog.quizzes.length > 57 || catalog.quizzes.some(quiz => typeof quiz !== "string")) throw Error("Course image requires its finite explicit quiz roster");
  const selected = [named, ...catalog.quizzes.map(quiz => resolve(dirname(named), quiz as string))];
  if (new Set(selected).size !== selected.length) throw Error("Unique course image quiz paths required");
  return selected.map(source => {
    const path = relative(teaching, source);
    if (!path || isAbsolute(path) || path === ".." || path.startsWith(".." + sep)) throw Error("Course image content must belong to teaching");
    return courseImageFile("content/" + path.split(sep).join("/"), source, signal);
  });
}

/** 🐧️ Refuses foreign executable formats or targets before feeding a Linux runtime context. */
export function courseImageExecutable(executable: ProctorExecutable): CourseImageSource {
  if (executable.platform !== "linux" || executable.mode !== "release" || !["x64", "arm64"].includes(executable.architecture)) throw Error("Native Linux release proctor required for image packaging");
  const row = readCourseImageEntry({ path: "proctor", bytes: executable.bytes, mode: 0o555, kind: "file" });
  if (row.bytes < 64) throw Error("Complete ELF64 proctor executable required");
  return { ...row, chunks: async function* () {
    await executable.recheck();
    let header = Buffer.alloc(0), checked = false;
    for await (const chunk of executable.chunks()) {
      if (!checked) {
        header = Buffer.concat([header, chunk.subarray(0, 64 - header.length)]);
        if (header.length === 64) {
          const machine = executable.architecture === "x64" ? 62 : 183;
          if (header.subarray(0, 4).toString("hex") !== "7f454c46" || header[4] !== 2 || header[5] !== 1 || header.readUInt16LE(18) !== machine) throw Error("Linux runtime proctor architecture does not match its genuine producer");
          checked = true;
        }
      }
      yield chunk;
    }
    if (!checked) throw Error("Complete ELF64 proctor executable required");
    await executable.recheck();
  } };
}
