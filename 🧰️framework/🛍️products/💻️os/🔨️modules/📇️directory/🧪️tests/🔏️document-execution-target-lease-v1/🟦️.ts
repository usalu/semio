/** 🧱️ Retains the fixture law at its concrete product contract owner. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../..");
const fixture = <T = Record<string, unknown>>(path: string): T => JSON.parse(readFileSync(resolve(root, path), "utf8"));

test("the lease corpus binds both peers to one framework contract witness", () => {
  const vectors = fixture<{ schema: string; plan: { package: Record<string, string> }; manifest: { package: Record<string, string> } }>("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json");
  expect(vectors.schema).toBe("semio.os.document-execution-target-lease-corpus/v1");
  expect(vectors.plan.package.componentSha256).toBe(vectors.manifest.package.componentSha256);
  expect(vectors.plan.package.descriptorByteSha256).toBe(vectors.manifest.package.descriptorByteSha256);
});

test("a lease package is admitted only with the shipped receipt binding its exact component and semantic descriptor", async () => {
  const { parseDocumentExecutionTargetLeaseFieldsV1 } = await import("../../🧬️schema/🟦️.ts");
  const { default: Ajv } = await import("ajv");
  const corpus = fixture<{ manifest: Record<string, any>; componentReceiptCases: { id: string; path: string[] | null; value: unknown; accepted: boolean }[] }>("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json");
  const directorySchema = fixture("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json") as { $id: string };
  const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(fixture("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json")).addSchema(directorySchema);
  const shape = ajv.getSchema(`${directorySchema.$id}#/$defs/DocumentShippedComponentReceiptV1`)!;
  const shapeOnly = new Set(["iterate-profile", "over-shipped-bound", "absent"]);
  expect(corpus.componentReceiptCases.map((row) => row.id)).toEqual(["exact", "foreign-plugin", "foreign-package", "foreign-component", "foreign-descriptor", "iterate-profile", "component-length", "over-shipped-bound", "absent"]);
  for (const row of corpus.componentReceiptCases) {
    const lease = structuredClone(corpus.manifest);
    if (row.path === null) delete lease.package.componentReceipt;
    else if (row.path.length === 1) lease.package.componentReceipt[row.path[0]!] = row.value;
    let admitted = true;
    try {
      parseDocumentExecutionTargetLeaseFieldsV1(lease);
    } catch {
      admitted = false;
    }
    expect(admitted, row.id).toBe(row.accepted);
    expect(Boolean(shape(lease.package.componentReceipt)), `${row.id} Ajv shape`).toBe(!shapeOnly.has(row.id));
  }
});
