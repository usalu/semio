/** 🏷️ A descriptor whose manifest label is a localized label decodes; a plain string label is refused. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { validateSemanticCatalogDescriptorValue } from "../../✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";

const pdf = new URL("../../../../../../../../🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/🔣️.json", import.meta.url);
const identity = { pluginId: "stdio-pdf", packageId: "semio:stdio-pdf", role: "plugin", dependsOn: ["stdio"] } as const;

test("localized manifest label decodes and a string label is refused", () => {
  const descriptor = JSON.parse(readFileSync(pdf, "utf8"));
  expect(() => validateSemanticCatalogDescriptorValue(identity, descriptor)).not.toThrow();
  const plain = { ...descriptor, manifest: { ...descriptor.manifest, label: "PDF" } };
  expect(() => validateSemanticCatalogDescriptorValue(identity, plain)).toThrow();
});
