import { courseAssignmentPdfDocumentsV1 } from "../📃️documents/🟦️.ts";
import { randomBytes } from "node:crypto";
import { readPrintMaterialFileV1 } from "@semio-tech/print/artifacts/material/producer";
import { createServer } from "node:http";
import { join } from "node:path";
import { withPrintExecutionV1, type PrintExecutionV1 } from "@semio-tech/print/artifacts/operation";
import { withPublishedPrintDocumentV1 } from "@semio-tech/print/modules/tectonic-template-compilation";
import { createIssuedResourceControlV1, type ScriptInvocation } from "@semio-tech/process/routing";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import course from "../../../🔣️.json" with { type: "json" };

type Channel = Readonly<{ name: string; endpoint: string; secret: string }>;
const documents = courseAssignmentPdfDocumentsV1(course.assignments.flatMap(assignment => Object.values(assignment.documents)));

/** 🔐️ Retains exact published producer artifacts until the consumer compiler or listener joins. */
export async function withCourseAssignmentPdfsV1<T>(workspace: string, invocation: ScriptInvocation, body: (channels: readonly Channel[], control: ResourceLeaseControl) => Promise<T>): Promise<T> {
  const phaseControl = createIssuedResourceControlV1(invocation, "architecture-course:assignment-pdfs", 0);
  return withPrintExecutionV1({ workspace, invocation, phaseControl, kind: "document" }, execution => retain(execution, 0, new Map(), body));
}
async function retain<T>(execution: PrintExecutionV1, index: number, files: Map<string, Uint8Array>, body: (channels: readonly Channel[], control: ResourceLeaseControl) => Promise<T>): Promise<T> {
  const document = documents[index];
  if (document) return withPublishedPrintDocumentV1(execution, document.namespace, document.id, document.producer, async artifact => {
    const path = join(artifact.path, document.file);
    const bytes = await readPrintMaterialFileV1(path, 16 * 1024 * 1024, artifact.control);
    if (bytes.length > 16 * 1024 * 1024 || new TextDecoder().decode(bytes.subarray(0, 5)) !== "%PDF-") throw Error("Finite published assignment PDF required: " + document.file);
    files.set(document.file, bytes);
    await artifact.recheck();
    const result = await retain({ ...execution, phaseControl: artifact.control }, index + 1, files, body);
    await artifact.recheck();
    return result;
  });
  const secret = randomBytes(32).toString("hex");
  const server = createServer((request, response) => {
    if (request.headers.authorization !== "Bearer " + secret) { response.statusCode = 403; return void response.end(); }
    const path = new URL(request.url ?? "/", "http://localhost").pathname;
    const bytes = files.get(path.startsWith("/pdfs/") ? path.slice(6) : "");
    if (!bytes || request.method !== "GET") { response.statusCode = 404; return void response.end(); }
    response.setHeader("content-type", "application/pdf");
    response.setHeader("content-length", bytes.length);
    response.end(bytes);
  });
  await new Promise<void>((accept, reject) => { server.once("error", reject); server.listen(0, "127.0.0.1", accept); });
  try {
    const address = server.address();
    if (!address || typeof address === "string") throw Error("Assignment PDF transport listener required");
    return await body([{ name: "course-assignment-pdfs", endpoint: "http://127.0.0.1:" + address.port + "/pdfs", secret }], execution.phaseControl);
  } finally { await new Promise<void>((accept, reject) => { server.close(error => error ? reject(error) : accept()); server.closeAllConnections(); }); }
}
