import { readFileSync } from "node:fs";
import { join } from "node:path";
import { deepStrictEqual } from "node:assert";
import Ajv from "ajv";
import { runRepositoryExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";

/** 🧬️ One compiled export of the GIS plugin's own module contract (`🌎️hub/🧩️compositions/🌍️gis/🧬️schema/🔣️.json`).
 * The GIS scope owns these shapes; this crate is a reader, never a second declaration site. */
function gisContract(repoRoot: string, exportId: string) {
  const schema = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "🧬️schema", "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`the GIS scope publishes no ${exportId} export`);
  return validate;
}

/** 🗺️ Independently validates the committed GIS roster without granting execution authority. */
export class InferenceDiscoveryOracleScript extends BundleScript {
  run(): void {
    const identityRoot = join(this.repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "🧫️fixtures", "🪪️artifact-identity");
    const identity = JSON.parse(readFileSync(join(identityRoot, "🔣️.json"), "utf8"));
    
    
    const kinds = new Set<string>();
    for (const artifact of identity.artifacts) {
      const segments = artifact.kind.split(".");
      if (segments.length !== 3 || segments[0] !== "s" || segments[1] !== identity.pluginId || kinds.has(artifact.kind)) throw new Error("GIS artifact identity must have one exact plugin owner");
      kinds.add(artifact.kind);
      if (artifact.nativeDialect !== `${artifact.kind}@1/*` || artifact.documentSchema !== (segments[2] === "gismap" ? "gis.map" : "gis.terrain")) throw new Error("GIS native identity and payload schema were conflated");
      if (artifact.extension !== segments[2] || artifact.codecExtension !== `${Buffer.byteLength(artifact.documentSchema, "utf8")}:${artifact.documentSchema}:${artifact.extension}`)
        throw new Error("GIS codec extension must bind its exact payload schema");
    }
    for (const kind of identity.hostileKinds) {
      const candidate = structuredClone(identity);
      candidate.artifacts[0].kind = kind;
      
    }
    console.log(`gis-artifact-identity-oracle: canonical=${kinds.size} hostile=${identity.hostileKinds.length}; native assembly still requires Rust law`);
    const controlRoot = join(this.repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "🧫️fixtures", "💡️inference-control");
    const control = JSON.parse(readFileSync(join(controlRoot, "🔣️.json"), "utf8"));
    
    
    const checkpoints = [0, 1];
    const coordinates: number[][] = [];
    let work = 1;
    const scan = (value: any): void => {
      checkpoints.push(++work);
      if (Array.isArray(value)) {
        if (value.length === 2 && value.every((item) => typeof item === "number")) coordinates.push(value);
        else value.forEach(scan);
      } else if (value !== null && typeof value === "object") {
        if (typeof value.lon === "number" && typeof value.lat === "number") coordinates.push([value.lon, value.lat]);
        Object.values(value).forEach(scan);
      }
    };
    for (const feature of [...control.snapshot.positions, ...control.snapshot.routes, ...control.snapshot.regions]) scan(feature.data);
    checkpoints.push(work);
    deepStrictEqual(checkpoints, control.checkpoints);
    deepStrictEqual(
      {
        positionCount: control.snapshot.positions.length,
        routeCount: control.snapshot.routes.length,
        regionCount: control.snapshot.regions.length,
        bounds: {
          lonMin: Math.min(...coordinates.map(([lon]) => lon!)),
          lonMax: Math.max(...coordinates.map(([lon]) => lon!)),
          latMin: Math.min(...coordinates.map(([, lat]) => lat!)),
          latMax: Math.max(...coordinates.map(([, lat]) => lat!)),
        },
      },
      control.expected,
    );
    for (const interruption of control.interruptions) {
      if (checkpoints.indexOf(interruption.at) + 1 !== interruption.calls) throw new Error(`control does not stop at first interruption ${interruption.name}`);
    }
    const { lonMin, lonMax, latMin, latMax } = control.expected.bounds;
    deepStrictEqual(control.proposal, {
      CreateRegion: {
        index: control.snapshot.regions.length,
        item: {
          id: `inference-${control.proposalJobId}`,
          data: {
            id: `inference-${control.proposalJobId}`,
            kind: "inference-bounds",
            ring: [
              [lonMin, latMin],
              [lonMax, latMin],
              [lonMax, latMax],
              [lonMin, latMax],
              [lonMin, latMin],
            ],
          },
        },
      },
    });
    console.log(`gis-inference-control-oracle: checkpoints=${checkpoints.length} interruptions=${control.interruptions.length} typed-proposal=1; no hub execution claim`);
    const fixtureRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🌉️mcp/🧪️tests/🧫️fixtures/🗺️discovery");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧬️schema/🔣️.json"), "utf8"));
    const inferenceAjv = new Ajv({ strict: true, allErrors: true });
    inferenceAjv.addSchema(schema);
    const validate = inferenceAjv.getSchema(`${schema.$id}#/$defs/InferenceDiscoveryRosterV1`)!;
    if (!validate(fixture.expected)) throw new Error(`invalid neutral GIS roster: ${JSON.stringify(validate.errors)}`);
    const matchesRoster = (candidate: unknown): boolean => JSON.stringify(candidate) === JSON.stringify(fixture.expected);
    if (!matchesRoster(fixture.expected)) throw new Error("the neutral GIS roster did not match itself");
    let shapeRejections = 0;
    let identityRejections = 0;
    for (const hostile of fixture.hostile) {
      const candidate = structuredClone(fixture.expected);
      if (hostile.operation === "remove") candidate.declared = [];
      else if (hostile.operation === "duplicate") candidate.declared.push(structuredClone(candidate.declared[0]));
      else candidate.declared[0][hostile.field] = hostile.value;
      if (!validate(candidate)) shapeRejections += 1;
      else if (!matchesRoster(candidate)) identityRejections += 1;
      else throw new Error(`GIS discovery oracle admitted ${hostile.name}`);
    }
    const descriptor = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "🔣️.json"), "utf8"));
    const contributions = descriptor.contributions;
    const declared = [...(contributions.inferenceServices ?? []), ...(contributions.artifactContributions ?? []).flatMap((row: { inferences?: unknown[] }) => row.inferences ?? [])].map((row: Record<string, unknown>) => ({
      ...row,
      dependsOn: row.dependsOn ?? [],
    }));
    const actual = { declared };
    if (!validate(actual)) throw new Error(`committed GIS descriptor discovery drift: ${JSON.stringify(validate.errors)}`);
    deepStrictEqual(actual, fixture.expected);
    console.log(`gis-inference-discovery-oracle: exact=1 hostile=${fixture.hostile.length} shape-rejected=${shapeRejections} identity-rejected=${identityRejections} execution-authority=0`);
  }
}

/** 🌉️ Executes the actual composition identity, controlled work and public MCP discovery laws. */
export class InferenceDiscoveryCheckScript extends BundleScript {
  async run(): Promise<void> {
    await new InferenceDiscoveryOracleScript(this.root,this.repoRoot).run();
    await runRepositoryExactCargoLaws({ cwd:this.repoRoot, artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs:3_600_000, lawBudgetMs:60_000,
      groups:[
        {package:"semio-s-artifact-gis-gismap",target:{kind:"lib"},cargoArgs:["--features","mcp-service"],laws:["inference_mcp::tests::installed_gis_descriptor_discovery_uses_the_registered_tool_without_granting_execution"]},
        {package:"semio-hub-gis",target:{kind:"lib"},laws:["surface_tests::gis_component_assembly_declares_exact_package_identity_before_descriptor_emission"]},
        {package:"semio-hub-gis",target:{kind:"test",name:"native_codecs"},cargoArgs:["--no-default-features"],laws:["gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace"]}
      ],progress(event){console.log("GIS discovery "+event.stage+": "+(event.law??event.package));}});
    console.log("gis-inference-discovery-check: exact MCP discovery, composition identity, finite controlled work passed");
  }
}
