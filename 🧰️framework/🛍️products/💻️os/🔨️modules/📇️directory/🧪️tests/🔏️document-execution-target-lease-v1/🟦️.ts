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
