import "../../../📐️parametric/📝️assignment/🧪️quality/🟦️.ts";
import { verifyParametrizationAssignmentContentV1 } from "../../../📐️parametric/📝️assignment/🧪️tests/🟦️.ts";
import { courseAssignmentPdfDocumentsV1 } from "../../🏗️builder/🌐️vite/📃️documents/🟦️.ts";
import Ajv from "ajv";
import fixture from "./🧫️cases.json";
import channelSchema from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/📥️environment/🏃️runtime/🧬️schema/🔣️.json";
import { readViteRuntimeChannelsV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/📥️environment/🏃️runtime/🟦️.ts";
import type { OwnedBuildMiddleware } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { courseAssignmentPdfsVitePlugin } from "../../🏗️builder/🌐️vite/📖️assignments/🟦️.ts";
import { expect, test } from "vitest";
import { createServer } from "node:http";
import { assignmentPdfBytes, readCourseAssignmentPdfsV1 } from "../../🏗️builder/🌐️vite/📖️assignments/🟦️.ts";

test("PDF transport requires explicit published input and rejects non-PDF or oversized bodies", async () => {
  await expect(readCourseAssignmentPdfsV1({})).rejects.toThrow("Published assignment PDF transport required");
  expect(() => assignmentPdfBytes(new TextEncoder().encode("<html>"))).toThrow();
  expect(() => assignmentPdfBytes(new Uint8Array(16 * 1024 * 1024 + 1))).toThrow();
});
test("both language PDFs are fetched through the authenticated owned runtime channel", async () => {
  const requests: string[] = [], secret = "a".repeat(64), bytes = new TextEncoder().encode("%PDF-1.7\nassignment\n%%EOF");
  const server = createServer((request, response) => {
    expect(request.headers.authorization).toBe("Bearer " + secret);
    requests.push(request.url!);
    response.setHeader("content-length", bytes.length);
    response.end(bytes);
  });
  await new Promise<void>(accept => server.listen(0, "127.0.0.1", accept));
  try {
    const address = server.address();
    if (!address || typeof address === "string") throw Error("Test listener required");
    const documents = await readCourseAssignmentPdfsV1({ SEMIO_VITE_RUNTIME_CHANNELS_V1: JSON.stringify([{ name: "course-assignment-pdfs", endpoint: "http://127.0.0.1:" + address.port + "/pdfs", secret }]) });
    expect(requests).toEqual(["/pdfs/architecture-parametric-parametrization-en.pdf", "/pdfs/architecture-parametric-parametrization-de.pdf"]);
    expect([...documents.keys()]).toEqual(["architecture-parametric-parametrization-en.pdf", "architecture-parametric-parametrization-de.pdf"]);
    for (const document of documents.values()) expect(document).toEqual(bytes);
  } finally { await new Promise<void>(accept => { server.close(() => accept()); server.closeAllConnections(); }); }
});

for (const row of fixture.channels) test(row.name + " matches independent JSON Schema validation", () => {
  const oracle = new Ajv({ strict: false }).compile(channelSchema);
  const owned = (() => { try { readViteRuntimeChannelsV1(row.value); return true; } catch { return false; } })();
  expect(owned).toBe(row.valid);
  expect(owned).toBe(Boolean(oracle(row.value)));
});
test("development GET and HEAD share the published PDF media type and length without writing static output", async () => {
  const bytes = new TextEncoder().encode("%PDF-1.7\nassignment\n%%EOF"), secret = "b".repeat(64);
  const upstream = createServer((_request, response) => response.end(bytes));
  await new Promise<void>(accept => upstream.listen(0, "127.0.0.1", accept));
  let middleware: OwnedBuildMiddleware | undefined;
  let listener: ReturnType<typeof createServer> | undefined;
  try {
    const address = upstream.address();
    if (!address || typeof address === "string") throw Error("Test upstream required");
    const environment = { SEMIO_VITE_RUNTIME_CHANNELS_V1: JSON.stringify([{ name: "course-assignment-pdfs", endpoint: "http://127.0.0.1:" + address.port + "/pdfs", secret }]) };
    const plugin = courseAssignmentPdfsVitePlugin(environment);
    await plugin.config?.({}, { command: "serve", mode: "prototype" });
    await plugin.configureServer?.({ middlewares: { use(handler: OwnedBuildMiddleware) { middleware = handler; } } } as never);
    listener = createServer((request, response) => { void middleware!(request, response, () => { response.statusCode = 404; response.end(); }); });
    await new Promise<void>(accept => listener!.listen(0, "127.0.0.1", accept));
    const port = listener.address();
    if (!port || typeof port === "string") throw Error("Test site required");
    const url = "http://127.0.0.1:" + port.port + "/assignments/architecture-parametric-parametrization-en.pdf";
    const get = await fetch(url), head = await fetch(url, { method: "HEAD" });
    expect(get.headers.get("content-type")).toBe("application/pdf");
    expect(head.headers.get("content-type")).toBe(get.headers.get("content-type"));
    expect(head.headers.get("content-length")).toBe(String(bytes.length));
    expect(new Uint8Array(await get.arrayBuffer())).toEqual(bytes);
    expect((await head.arrayBuffer()).byteLength).toBe(0);
    await plugin.closeBundle?.();
  } finally {
    for (const server of [listener, upstream]) if (server) await new Promise<void>(accept => { server.close(() => accept()); server.closeAllConnections(); });
  }
});

for (const row of fixture.publications) test(row.name, () => {
  if (row.expected) expect(courseAssignmentPdfDocumentsV1(row.value)).toEqual(row.expected);
  else expect(() => courseAssignmentPdfDocumentsV1(row.value)).toThrow(row.error ?? "Conflicting assignment PDF producers");
});

test("course assignment semantics agree with independent schema, D3 and arithmetic oracles", () => verifyParametrizationAssignmentContentV1());
