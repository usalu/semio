import { courseAssignmentPdfDocumentsV1 } from "../📃️documents/🟦️.ts";
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { readViteRuntimeChannelsV1, VITE_RUNTIME_CHANNELS_ENV_V1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/📥️environment/🏃️runtime/🟦️.ts";
import type { OwnedBuildPlugin } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import course from "../../../🔣️.json" with { type: "json" };

const documents = courseAssignmentPdfDocumentsV1(course.assignments.flatMap(assignment => Object.values(assignment.documents)));
const files = [...new Set(documents.map(document => document.file))];
export const ASSIGNMENT_PDF_MAXIMUM_BYTES = 16 * 1024 * 1024;

/** 📖️ Validates bounded PDF transport data before publishing it to a browser. */
export function assignmentPdfBytes(bytes: Uint8Array): Uint8Array {
  if (bytes.length > ASSIGNMENT_PDF_MAXIMUM_BYTES || new TextDecoder().decode(bytes.subarray(0, 5)) !== "%PDF-") throw Error("Finite published assignment PDF required");
  return bytes;
}

/** 📥️ Reads actual PDFs from the retained parent transport, with no compiler or store-path authority. */
export async function readCourseAssignmentPdfsV1(environment: Readonly<Record<string, string | undefined>> = process.env): Promise<ReadonlyMap<string, Uint8Array>> {
  const channels = readViteRuntimeChannelsV1(JSON.parse(environment[VITE_RUNTIME_CHANNELS_ENV_V1] ?? "[]"));
  const channel = channels.find(row => row.name === "course-assignment-pdfs");
  if (!channel) throw Error("Published assignment PDF transport required");
  const result = new Map<string, Uint8Array>();
  for (const file of files) {
    const response = await fetch(channel.endpoint + "/" + file, { headers: { authorization: "Bearer " + channel.secret }, signal: AbortSignal.timeout(30000) });
    if (!response.ok || Number(response.headers.get("content-length")) > ASSIGNMENT_PDF_MAXIMUM_BYTES) throw Error("Published assignment PDF unavailable: " + file);
    const reader = response.body?.getReader();
    if (!reader) throw Error("Published assignment PDF body required");
    const chunks: Uint8Array[] = [];
    let length = 0;
    try {
      while (true) {
        const { value, done } = await reader.read();
        if (done) break;
        length += value.length;
        if (length > ASSIGNMENT_PDF_MAXIMUM_BYTES) throw Error("Finite published assignment PDF required");
        chunks.push(value);
      }
    } finally { await reader.cancel(); }
    const bytes = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.length; }
    result.set(file, assignmentPdfBytes(bytes));
  }
  return result;
}

/** 🌐️ Publishes retained assignment PDFs to the same paths in development and static builds. */
export function courseAssignmentPdfsVitePlugin(environment: Readonly<Record<string, string | undefined>> = process.env): OwnedBuildPlugin {
  let output = "", written = true, building = false;
  return {
    name: "architecture-course-assignment-pdfs",
    config(_config, environment) { building = environment.command === "build"; },
    configResolved(config) { output = resolve(config.root, config.build.outDir); written = config.build.write !== false; },
    async configureServer(server) {
      const documents = await readCourseAssignmentPdfsV1(environment);
      server.middlewares.use((request, response, next) => {
        const path = new URL(request.url ?? "/", "http://localhost").pathname;
        if (!path.startsWith("/assignments/")) return next();
        const bytes = documents.get(path.slice("/assignments/".length));
        if (!bytes) { response.statusCode = 404; return void response.end(); }
        if (request.method !== "GET" && request.method !== "HEAD") { response.statusCode = 405; return void response.end(); }
        response.setHeader("content-type", "application/pdf");
        response.setHeader("content-length", bytes.length);
        response.setHeader("cache-control", "no-cache");
        response.end(request.method === "HEAD" ? undefined : bytes);
      });
    },
    async closeBundle() {
      if (!building || !written) return;
      for (const [file, bytes] of await readCourseAssignmentPdfsV1(environment)) {
        mkdirSync(resolve(output, "assignments"), { recursive: true });
        writeFileSync(resolve(output, "assignments", file), bytes);
      }
    },
  };
}
