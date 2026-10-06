/** 🏗️ Publishes the concrete Stdio composition from present artifact-owned contributions. */
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";
import { admitCompositionContributionV1, selectCompositionContributionsV1, resolveCompositionNativeFactoriesV1, resolveCompositionOpenTargetsV1, type CompositionNativeReceiptV1, type CompositionContributionV1 } from "../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧩️composition/🟦️.ts";

type OwnerV1 = { id: string; root: string; enum: string; label: string; main: boolean };
type InputV1 = { directory: string; manifest: string; contribution: CompositionContributionV1; observed: Map<string, string>; nativeReceipts: CompositionNativeReceiptV1[] };
const boundary = 4 * 1024 * 1024;

function physical(root: string, path: string, file = true): string {
  const absolute = resolve(root, path), local = relative(root, absolute);
  if (local.startsWith(".." + sep) || local === "..") throw new Error(`Composition input escapes its owner: ${path}`);
  let current = root;
  for (const part of local.split(sep)) {
    current = resolve(current, part);
    if (lstatSync(current).isSymbolicLink()) throw new Error(`Composition input follows a symlink: ${path}`);
  }
  const info = lstatSync(absolute);
  if (file ? !info.isFile() || info.size > boundary : !info.isDirectory()) throw new Error(`Composition input has an invalid physical shape: ${path}`);
  return absolute;
}

function read(root: string, path: string): string {
  return readFileSync(physical(root, path), "utf8");
}

function owners(value: unknown): OwnerV1[] {
  const document = value as { schema: string; owners: OwnerV1[] };
  if (!document || Object.keys(document).sort().join(",") !== "owners,schema" || document.schema !== "semio.stdio.composition-owners/v1" || !Array.isArray(document.owners)) throw new Error("Invalid Stdio composition owner authority");
  for (const owner of document.owners) if (!owner || Object.keys(owner).sort().join(",") !== "enum,id,label,main,root" || !/^[a-z][a-z0-9-]*$/.test(owner.id) || !/^[A-Za-z_][A-Za-z0-9_]*$/.test(owner.enum) || typeof owner.label !== "string" || !owner.label.length || typeof owner.main !== "boolean" || typeof owner.root !== "string" || owner.root.split("/").some((part) => !part || part === "." || part === "..") || !owner.root.endsWith("/🦀️.rs")) throw new Error("Invalid Stdio composition owner");
  for (const key of ["id", "root", "enum"] as const) if (new Set(document.owners.map((owner) => owner[key])).size !== document.owners.length) throw new Error(`Duplicate composition owner ${key}`);
  if (document.owners.filter((owner) => owner.main).length !== 1) throw new Error("Stdio composition requires one authored catalog owner");
  return document.owners;
}

function rustSource(owner: OwnerV1, rows: CompositionContributionV1[]): string {
  const hosted = rows.filter((row) => row.apps.some((app) => app.owner === owner.id));
  const apps = hosted.flatMap((row) => row.apps.filter((app) => app.owner === owner.id)).sort((a, b) => a.order - b.order);
  const crate = (row: CompositionContributionV1) => row.package.replaceAll("-", "_");
  const variants = apps.map((app) => `        ${app.variant}(VcsArtifactApp<${app.role === "editor" ? "EditorApp" : "ViewerApp"}<${app.type}>>),`).join("\n");
  const registrations = apps.map((app) => `    builder = builder.${app.role}::<${app.type}>(${app.factory}());${app.mutationRoster ? `\n    builder = builder.${app.role}_mutation_roster::<${app.type}>();` : ""}`).join("\n");
  const activations = hosted.map((row) => `    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: ${crate(row)}::artifact_kind().id });`).join("\n");
  const assembly = owner.main ? `    let registry = crate::catalog::selected_registry()?;\n    let plan = semio_s_plugin_stdio::AssemblyPlan::new(&registry, semio_s_plugin_stdio::AssemblyOwner { plugin_id: ${JSON.stringify(owner.id)}, package_id: crate::catalog::component_package_id()?, package_version: env!("CARGO_PKG_VERSION") })?;\n    let catalog = crate::catalog::artifact_catalog_contribution(plan.assemblies())?;\n    builder = plan.apply(builder);` : hosted.map((row) => `    builder = builder.host_artifact(${crate(row)}::declaration(${crate(row)}::definition()?).map_err(PluginAssemblyError::definition)?);`).join("\n");
  const source = `//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.\n\n#![allow(async_fn_in_trait)]\n#![allow(long_running_const_eval)]\n\nuse semio_framework_plugin::__semio_dispatch_PluginApp;\nuse semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};\nuse semio_framework_plugin::plugin_app_close_prelude::*;\nuse semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};\n\nsemio_framework_dispatch_macros::dyn_enum_close! {\n    /// 🗃️ The present owner-authored runtime app contributions.\n    pub enum ${owner.enum}: PluginApp {\n${variants}\n    }\n}\n\n/// 🔌️ Builds the exact present artifact and app contributions.\npub fn plugin() -> Result<Plugin<${owner.enum}>, PluginAssemblyError> {\n    let mut builder = Plugin::<${owner.enum}>::builder(${JSON.stringify(owner.id)}).label(${JSON.stringify(owner.label)}).version(env!("CARGO_PKG_VERSION")).package_id(${owner.main ? "crate::catalog::component_package_id()?" : JSON.stringify(`semio:${owner.id}`)}).schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS)${owner.main ? '.schema_documents("stdio", crate::catalog::CATALOG_SCHEMA_DOCUMENTS)' : '.depends_on("stdio", semio_framework::tree_pin!())'};\n${assembly}\n${registrations}\n${activations}\n    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });\n    ${owner.main ? "builder.contributes_topic(catalog)" : "builder"}.try_build()\n}\n${owner.main ? "" : `\n#[cfg(feature = "plugin-root")]\nsemio_framework_plugin::plugin_exports!(plugin, ${owner.enum});\n`}`;
  return source;
}

function region(source: string, name: string, content: string): string {
  const begin = `# 🧩️ ${name}\n`, end = `# /🧩️ ${name}`;
  const start = source.indexOf(begin), finish = source.indexOf(end);
  if (start < 0 || finish < start || source.indexOf(begin, start + begin.length) >= 0 || source.indexOf(end, finish + end.length) >= 0) throw new Error(`Missing or repeated composition output region: ${name}`);
  return source.slice(0, start + begin.length) + content + (content ? "\n" : "") + source.slice(finish);
}

/** 📇️ Reads all physically present declared contribution owners and validates their source bindings. */
export function readStdioCompositionInputs(repoRoot: string, hubRoot: string): InputV1[] {
  const manifest = Bun.TOML.parse(read(hubRoot, "📦️packages/🦀️rust/Cargo.toml")) as any;
  const roots = manifest.package?.metadata?.semio?.sources?.artifacts;
  if (!Array.isArray(roots) || roots.some((root) => typeof root !== "string")) throw new Error("Stdio composition has no authored artifact source authority");
  const inputs: InputV1[] = [];
  for (const root of roots) {
    const directory = physical(repoRoot, relative(repoRoot, resolve(hubRoot, "📦️packages/🦀️rust", root)), false);
    for (const entry of readdirSync(directory, { withFileTypes: true }).sort((a, b) => Buffer.compare(Buffer.from(a.name), Buffer.from(b.name)))) {
      if (entry.isSymbolicLink()) throw new Error(`Stdio contribution owner is a symlink: ${entry.name}`);
      if (!entry.isDirectory()) continue;
      const artifactRoot = resolve(directory, entry.name), packagePath = resolve(artifactRoot, "📦️packages/🦀️rust/Cargo.toml");
      if (!existsSync(packagePath) && !existsSync(resolve(artifactRoot, "🧩️composition/🔣️.json")) && !existsSync(resolve(artifactRoot, "📜️artifact-definition.json"))) continue;
      if (!existsSync(packagePath)) throw new Error(`Present Stdio artifact has no Cargo owner: ${entry.name}`);
      const observed = new Map<string, string>();
      const capture = (path: string): string => {
        const bytes = readFileSync(physical(artifactRoot, relative(artifactRoot, path)));
        observed.set(path, createHash("sha256").update(bytes).digest("hex"));
        return bytes.toString("utf8");
      };
      const packageSource = capture(packagePath), packageManifest = Bun.TOML.parse(packageSource) as any;
      const authority = packageManifest.package?.metadata?.semio?.composition;
      if (!authority || Object.keys(authority).join(",") !== "manifest" || typeof authority.manifest !== "string") throw new Error(`Present Stdio artifact has no declared composition contribution: ${entry.name}`);
      const contribution = admitCompositionContributionV1(JSON.parse(capture(resolve(dirname(packagePath), authority.manifest))));
      if (contribution.catalogFeatures.some((feature) => !Object.hasOwn(packageManifest.features ?? {}, feature))) throw new Error(`Contribution catalog feature is absent from its Cargo owner: ${entry.name}`);
      if (contribution.package !== packageManifest.package.name) throw new Error(`Contribution package does not match its physical Cargo owner: ${entry.name}`);
      const prefix = contribution.package.replaceAll("-", "_") + "::";
      if (contribution.apps.some((app) => !app.type.startsWith(prefix) || !app.factory.startsWith(prefix))) throw new Error(`Contribution app symbols escape their Cargo owner: ${entry.name}`);
      const ownInput = (path: string): string => {
        const absolute = resolve(repoRoot, path), local = relative(artifactRoot, absolute);
        if (local === ".." || local.startsWith(".." + sep)) throw new Error("Native factory export escapes its actual artifact owner");
        return absolute;
      };
      const nativeReceipts = resolveCompositionNativeFactoriesV1(contribution, (path, factoryId) => {
        const definition = JSON.parse(capture(ownInput(path)));
        const matches = definition.codecs?.filter((codec: any) => codec.native_factory?.factory_id === factoryId);
        if (!matches || matches.length !== 1 || matches[0].status !== "implemented" || matches[0].executable_registration !== true) throw new Error(`Native factory export is missing, ambiguous or unavailable: ${factoryId}`);
        return { descriptorCodecId: matches[0].id, factory: matches[0].native_factory };
      }, path => {
        const absolute = ownInput(path);
        capture(absolute);
        return observed.get(absolute)!;
      });
      inputs.push({ directory: artifactRoot, manifest: packagePath, contribution, observed, nativeReceipts });
    }
  }
  selectCompositionContributionsV1(inputs.map((input) => input.contribution), "full");
  return inputs;
}

/** 📦️ Refreshes only this composition's authored output regions and source projections. */
export function prepareStdioComposition(repoRoot: string, hubRoot: string): { contributions: number; apps: number; receipts: number } {
  const inputs = readStdioCompositionInputs(repoRoot, hubRoot), rows = selectCompositionContributionsV1(inputs.map((input) => input.contribution), "full").map(row => ({ ...row, nativeReceipts: inputs.find(input => input.contribution.package === row.package)!.nativeReceipts }));
  const ownerSource = read(hubRoot, "🧩️composition/🔣️.json"), config = owners(JSON.parse(ownerSource)), ids = new Set(config.map((owner) => owner.id));
  if (rows.some((row) => row.apps.some((app) => !ids.has(app.owner)) || row.playgrounds.some((entry) => !ids.has(entry.owner)))) throw new Error("Artifact app contribution targets an undeclared composition owner");
  const outputs = new Map<string, { previous: string | undefined; next: string }>();
  const publish = (path: string, next: string) => {
    const absolute = resolve(hubRoot, path), local = relative(hubRoot, absolute);
    if (local.startsWith(".." + sep) || local === "..") throw new Error("Composition output escapes its owner");
    let current = hubRoot;
    for (const part of local.split(sep)) { current = resolve(current, part); if (existsSync(current) && lstatSync(current).isSymbolicLink()) throw new Error("Composition output follows a symlink"); }
    outputs.set(absolute, { previous: existsSync(absolute) ? read(hubRoot, path) : undefined, next });
  };
  for (const owner of config) {
    publish(owner.root, rustSource(owner, rows));
    const path = owner.main ? "📦️packages/🦀️rust/Cargo.toml" : relative(hubRoot, resolve(hubRoot, dirname(owner.root), "📦️packages/🦀️rust/Cargo.toml"));
    const hosted = rows.filter((row) => row.apps.some((app) => app.owner === owner.id));
    const dependencies = (owner.main ? rows : hosted).map((row) => `${row.package} = { workspace = true, ${owner.main ? "optional = true" : 'features = ["component-app-assembly"]'} }`).join("\n");
    let manifest = region(read(hubRoot, path), "Composition Dependencies", dependencies);
    if (owner.main) {
      const full = rows.map((row) => `dep:${row.package}`), home = selectCompositionContributionsV1(inputs.map((input) => input.contribution), "home").map((row) => `dep:${row.package}`);
      for (const row of rows) for (const feature of row.catalogFeatures) full.push(`${row.package}/${feature}`);
      manifest = region(manifest, "Composition Features", `component-app-assembly = ${JSON.stringify(["full-artifact-catalog", ...hosted.map((row) => `${row.package}/component-app-assembly`)])}\nfull-artifact-catalog = ${JSON.stringify(full)}\nhome-io = ${JSON.stringify(home)}`);
    }
    const playgrounds = rows.flatMap((row) => row.playgrounds.filter((entry) => entry.owner === owner.id));
    const playgroundSource = playgrounds.map((entry) => `[[package.metadata.semio.playground]]\nvariant = ${JSON.stringify(entry.variant)}\napp = ${JSON.stringify(entry.app)}\naliases = ${JSON.stringify(entry.aliases)}\nports = { react = ${entry.ports.react}, wgpu = ${entry.ports.wgpu} }`).join("\n\n");
    manifest = region(manifest, "Composition Playgrounds", playgroundSource);
    publish(path, manifest);
  }
  const selected = (selection: string) => `vec![${selectCompositionContributionsV1(inputs.map((input) => input.contribution), selection).map((row) => `${row.package.replaceAll("-", "_")}::contribution()`).join(", ")}]`;
  publish("🤖️generated/🧩️composition/🦀️.rs", `#[cfg(feature = "full-artifact-catalog")]\nfn selected_contributions() -> Vec<ArtifactContribution> { ${selected("full")} }\n#[cfg(all(feature = "home-io", not(feature = "full-artifact-catalog")))]\nfn selected_contributions() -> Vec<ArtifactContribution> { ${selected("home")} }\n#[cfg(not(any(feature = "full-artifact-catalog", feature = "home-io")))]\nfn selected_contributions() -> Vec<ArtifactContribution> { Vec::new() }\n`);
  const mutationDescriptors = rows.flatMap(row => row.mutationDescriptors);
  for (const row of rows) {
    const prefix = row.package.replaceAll("-", "_") + "::";
    if (row.mutationDescriptors.some(entry => !entry.snapshot.startsWith(prefix) || !entry.mutation.startsWith(prefix))) throw new Error("Mutation descriptor escapes its actual artifact package");
    for (const coordinate of row.mutationCoordinates) {
      const directory = physical(repoRoot, coordinate.owner, false);
      const source = inputs.find(input => input.contribution.package === row.package)!;
      const local = relative(source.directory, directory);
      if (local === ".." || local.startsWith(".." + sep)) throw new Error("Mutation coordinate escapes its actual artifact owner");
      if (!lstatSync(directory).isDirectory() || coordinate.artifact !== row.nativeReceipts[0]?.artifact_kind && coordinate.artifact !== `s.stdio.${row.artifact}`) throw new Error("Invalid artifact-owned mutation coordinate");
    }
  }
  const aggregates = mutationDescriptors.map(entry => `    (${JSON.stringify(entry.name)}, descriptors::<${entry.snapshot}, ${entry.mutation}>),`).join("\n");
  const coordinates = rows.flatMap(row => row.mutationCoordinates).map(entry => `    (${[entry.artifact, entry.standard, entry.subset, entry.surface, entry.owner, entry.prefix].map(value => JSON.stringify(value)).join(", ")}),`).join("\n");
  publish("🤖️generated/🧩️mutations/🦀️.rs", `const AGGREGATES: &[(&str, fn() -> &'static [MutationLeafDescriptor])] = &[\n${aggregates}\n];\nconst COORDINATES: &[(&str, &str, &str, &str, &str, &str)] = &[\n${coordinates}\n];\n`);
  const editors = rows.flatMap(row => row.apps).filter(app => app.role === "editor");
  const editorLaws = editors.map(app => `    (${app.laws!.editing}, ${app.laws!.sqliteSnapshot}, ${app.type}, ${app.factory}),`).join("\n");
  publish("🤖️generated/🧪️editor-laws/🦀️.rs", editors.length ? `editor_catalog_laws! {\n${editorLaws}\n}\n` : "const EDITOR_COUNT: usize = 0;\n");
  const receipts = rows.flatMap((row) => row.nativeReceipts).sort((a, b) => Buffer.compare(Buffer.from(a.factory_id), Buffer.from(b.factory_id)));
  publish("🔌️plugin/📇️catalog/📜️native-codec-factories.json", JSON.stringify({ schema: "semio.stdio.native-openable-catalog-provider/v1", provider_id: "stdio/native-codecs/v1", plugin_id: "stdio", package_id: "semio:stdio", receipts }, null, 2) + "\n");
  const payload = JSON.parse(read(hubRoot, "📇️publication/📜️native-catalog.json"));
  payload.nativeCodecs = receipts.map((row) => ({ artifactKind: row.artifact_kind, artifactSchema: row.artifact_schema, packSchemaHash: row.pack_schema_hash, factoryId: row.factory_id, extension: row.extension, protocolSourceSha256: row.protocol_source_sha256 }));
  payload.openTargets = resolveCompositionOpenTargetsV1(rows, receipts);
  publish("📇️publication/📜️native-catalog.json", JSON.stringify(payload, null, 2) + "\n");
  const declaration = JSON.parse(read(hubRoot, "📇️publication/🔣️.json"));
  const publication = declaration.publications.find((row: { id: string }) => row.id === "stdio-native-codecs");
  if (!publication) throw new Error("Stdio publication declaration is missing");
  publication.integrity = receipts.map((row, index) => ({ pointer: `/nativeCodecs/${index}/protocolSourceSha256`, path: relative(resolve(hubRoot, "📇️publication"), resolve(repoRoot, row.protocol_path)).split(sep).join("/") }));
  payload.openTargets.forEach((target: Record<string, string>, index: number) => {
    const receipt = receipts.find((row) => row.factory_id === target.factoryId);
    if (!receipt || target.protocolSourceSha256 !== receipt.protocol_source_sha256 || target.packSchemaHash !== receipt.pack_schema_hash) throw new Error("Stdio open target has no exact native receipt authority");
    publication.integrity.push({ pointer: `/openTargets/${index}/protocolSourceSha256`, path: relative(resolve(hubRoot, "📇️publication"), resolve(repoRoot, receipt.protocol_path)).split(sep).join("/") });
  });
  publish("📇️publication/🔣️.json", JSON.stringify(declaration, null, 2) + "\n");
  if (read(hubRoot, "🧩️composition/🔣️.json") !== ownerSource) throw new Error("Composition owner authority changed during publication");
  for (const input of inputs) for (const [path, digest] of input.observed) if (createHash("sha256").update(readFileSync(physical(repoRoot, relative(repoRoot, path)))).digest("hex") !== digest) throw new Error(`Composition input changed during publication: ${path}`);
  for (const [path, output] of outputs) if ((existsSync(path) ? readFileSync(path, "utf8") : undefined) !== output.previous) throw new Error(`Composition output changed during publication: ${path}`);
  for (const [path, output] of outputs) if (output.previous !== output.next) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, output.next); }
  return { contributions: rows.length, apps: rows.reduce((sum, row) => sum + row.apps.length, 0), receipts: receipts.length };
}
