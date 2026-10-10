import schema from "./🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 📥️ Declares the actual native caller's selected command and finite child authority. */
export interface NativeOwnerCapabilities {
  readonly version: 1;
  readonly repositoryRoot: string;
  readonly artifactDirectory: string;
  readonly command: Readonly<{ kind: "native-owner-command"; manifest: string; workingDirectory: string; program: string; arguments: readonly string[] }>;
  readonly transport: Readonly<{ maximumBytes: number; maximumLines: number }>;
  readonly child: Readonly<{ maximumElapsedMilliseconds: number }>;
  readonly network: Readonly<{ offline: boolean }>;
}

/** 🔐️ Refuses missing caller grants before native input or child acquisition. */
export function readNativeOwnerCapabilities(value: unknown): NativeOwnerCapabilities {
  if (validateJsonSchemaSubset(schema, value).length) throw Error("Original native caller capabilities required");
  return value as NativeOwnerCapabilities;
}
