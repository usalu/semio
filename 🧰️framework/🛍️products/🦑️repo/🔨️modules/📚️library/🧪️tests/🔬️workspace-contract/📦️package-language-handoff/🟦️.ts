import { describe } from "bun:test";
import { registerSchemaFirstSixLanguageTests } from "./🔶️schema-first-six-language/🟦️.ts";
import { registerResidentNativeMetadataRouterTests } from "./🔸️resident-native-metadata-router/🟦️.ts";
import { registerUiHostMetadataRouterTests } from "./🟠️ui-host-metadata-router/🟦️.ts";

describe("package language semantic handoff", () => {
  registerSchemaFirstSixLanguageTests();
  registerResidentNativeMetadataRouterTests();
  registerUiHostMetadataRouterTests();
});
