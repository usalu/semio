/** 🧪️ Runs the UiDocumentStore typed-wire topic suites as a plain suite, so the store module imports no test module. */
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import * as vitest from "vitest";
import { RetainedUiPatchCursor, RetainedUiSnapshotCursor, RetainedUiSurfaceOwner } from "@semio-tech/framework";
import { DEFAULT_UI_DOCUMENT_LIMITS, UiDocumentStore, applyUiPatch, emitIntent, uiDocumentStateFromSnapshot, validateUiDocumentCore } from "../../🟦️.tsx";
import { registerTypedWireTests } from "../🧪️typedwire/🟦️.tsx";

const directory = resolve(import.meta.dirname, "..", "..");

await registerTypedWireTests(vitest as never, { DEFAULT_UI_DOCUMENT_LIMITS, RetainedUiPatchCursor, RetainedUiSnapshotCursor, RetainedUiSurfaceOwner, UiDocumentStore, applyUiPatch, emitIntent, uiDocumentStateFromSnapshot, validateUiDocumentCore }, { directory, url: pathToFileURL(resolve(directory, "🟦️.tsx")).href });
