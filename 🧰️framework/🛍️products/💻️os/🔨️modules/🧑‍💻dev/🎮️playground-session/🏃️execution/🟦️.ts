import { parseGeneratorPreviewProgressPolicyV1, serializeGeneratorPreviewProgressV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/📈️progress/🟦️.ts";
import { readPhysicalFileV1 } from "../../../../../../🔨️modules/📁️filesystem/🧾️observation/🟦️.ts";
import { parseDeployedRegistryEntryV1 } from "../../../🔌️plugin/📇️registry/🔎️discovery/🧬️schema/🟦️.ts";
import { basename, dirname, join, relative, resolve } from "node:path";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { renderPlaygroundSessionTypeScript } from "../../../🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts";
import { registryModuleDirectories, type GeneratedCatalogProjection } from "../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { PLAYGROUND_SESSION_OUTPUT_ROOT_ENV, playgroundSessionOutputPath } from "../../♻️activation/🟦️.ts";
import { parsePlaygroundSessionPublicationRequestV1, type PlaygroundSessionPublicationRequestV1 } from "../🧬️schema/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";

export class PlaygroundSessionGenerateScript extends BundleScript {
  readonly playgroundSessionPath: string;
  readonly request: PlaygroundSessionPublicationRequestV1;
  constructor(root: string, request: PlaygroundSessionPublicationRequestV1) {
    super(root);
    this.request = parsePlaygroundSessionPublicationRequestV1(request);
    const configured = process.env[PLAYGROUND_SESSION_OUTPUT_ROOT_ENV];
    this.playgroundSessionPath = playgroundSessionOutputPath(configured ? resolve(this.repoRoot, configured) : resolve(this.repoRoot, this.request.outputRoot));
  }
  protected async readProjectionV1(): Promise<GeneratedCatalogProjection> {
    if (!this.request.catalogPath) return { entries: [], playgrounds: [] };
    const operation = new AbortController(), retire = () => operation.abort(), started = performance.now(), policy = parseGeneratorPreviewProgressPolicyV1(schema["x-semio-preview-progress"]);
    if (process.env.SEMIO_GENERATOR_PREVIEW === "1" && JSON.stringify(parseGeneratorPreviewProgressPolicyV1(JSON.parse(process.env.SEMIO_GENERATOR_PREVIEW_PROGRESS ?? "null"))) !== JSON.stringify(policy)) throw new Error("Session preview progress authority was not explicitly admitted");
    let sourceIndex = 0, work = 0, progressBytes = 0;
    process.once("SIGINT", retire); process.once("SIGTERM", retire);
    const control = { maxBytes: schema["x-semio-admission"].maxBytes, maxWork: 65536, chunkBytes: 65536, cancelled: () => operation.signal.aborted, remainingMs: () => schema["x-semio-admission"].maxDurationMs - (performance.now() - started), onProgress: (event: { bytes: number; totalBytes: number }) => { const frame = serializeGeneratorPreviewProgressV1({ protocol: policy.protocol, sourceIndex, bytes: event.bytes, totalBytes: event.totalBytes, work: ++work, elapsedMs: Math.floor(performance.now() - started) }, policy); progressBytes += new TextEncoder().encode(frame).byteLength; if (work > policy.maxEvents || progressBytes > policy.maxBytes) throw new Error("Session source progress exceeds finite authority"); process.stderr.write(frame); } };
    try {
      const rows: unknown[][] = [];
      for (const name of ["🔌️plugins.json", "🎠️playgrounds.json"]) {
        const observed = await readPhysicalFileV1(resolve(this.repoRoot, this.request.catalogPath, name), control);
        try { const value: unknown = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(observed.bytes)); if (!Array.isArray(value) || value.length > schema["x-semio-admission"].maxRows) throw new Error("Session source exceeds admitted row authority"); rows.push(value); }
        finally { observed.bytes.fill(0); sourceIndex++; }
      }
      if (rows[0]!.length + rows[1]!.length > schema["x-semio-admission"].maxRows) throw new Error("Session source exceeds admitted row authority");
      const projection = { entries: rows[0]!.map(value => parseDeployedRegistryEntryV1(value)), playgrounds: rows[1]! as GeneratedCatalogProjection["playgrounds"] };
      registryModuleDirectories(projection.entries);
      if (this.request.variant && !projection.playgrounds.some(row => row.variant === this.request.variant || row.aliases.includes(this.request.variant))) throw new Error("Session selected variant is absent: " + this.request.variant);
      return projection;
    } finally { process.off("SIGINT", retire); process.off("SIGTERM", retire); }
  }
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments[0] !== undefined && segments[0] !== "check")) throw new Error("Session generation accepts only its check phase");
    const expected = renderPlaygroundSessionTypeScript(this.request.variant || undefined, await this.readProjectionV1());
    if (segments[0] === "check") {
      if (!existsSync(this.playgroundSessionPath) || readFileSync(this.playgroundSessionPath, "utf8") !== expected) throw new Error("Generated playground session is stale");
      console.log("[DEBUG] playground session generated source is fresh");
      return;
    }
    mkdirSync(dirname(this.playgroundSessionPath), { recursive: true });
    writeFileSync(this.playgroundSessionPath, expected);
    console.log(`[DEBUG] playground session generated source refreshed -> ${this.playgroundSessionPath}`);
  }
}

export class PlaygroundSessionPreviewScript extends PlaygroundSessionGenerateScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Session preview accepts no extra phase");
    const rootPath = relative(this.repoRoot, dirname(this.playgroundSessionPath)).replaceAll("\\", "/").normalize("NFC");
    const nodes = [
      { bytesBase64: Buffer.from(renderPlaygroundSessionTypeScript(this.request.variant || undefined, await this.readProjectionV1())).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/${basename(this.playgroundSessionPath).normalize("NFC")}` },
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const staleRemovals: readonly string[] = [];
    process.stdout.write(`${JSON.stringify({ contractId: this.request.contractId, nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}
