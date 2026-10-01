#!/usr/bin/env bun
/** 🗄️ General caller-authored Stdio assembly library tasks. */
import { copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { devToolingEnv } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { runArtifactRustPackageMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { cargoRepositoryPackages, cargoWorkspaceForManifest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { runAssemblyChecks } from "../../🧪️tests/📦️assembly/🟦️.ts";
/** 🗑️ Compiles the general assembly in a regular-file workspace with the concrete fleet absent. */
class DeletionProofScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const requestedOwners: string[] = [];
    for (let index = 0; index < segments.length; index += 2) {
      if (segments[index] !== "--owner" || !/^[a-z0-9][a-z0-9-]*$/u.test(segments[index + 1] ?? "")) throw new Error("deletion-proof accepts repeated --owner <Cargo-package> inputs");
      requestedOwners.push(segments[index + 1]!);
    }
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("deletion-proof requires SEMIO_TEST_ARTIFACT_DIR");
    const output = resolve(this.repoRoot, process.env.SEMIO_TEST_ARTIFACT_DIR);
    mkdirSync(output, { recursive: true });
    const workspaceRoot = mkdtempSync(join(output, "stdio-deletion-"));
    type Owner = { path: string; manifest: any; workspace: any; directory: string };
    const members = new Map<string, Owner>();
    const owner = (manifestPath: string): Owner => {
      const scope = cargoWorkspaceForManifest(this.repoRoot, relative(this.repoRoot, manifestPath).split("\\").join("/"));
      return { path: dirname(manifestPath), manifest: Bun.TOML.parse(readFileSync(manifestPath, "utf8")), workspace: Bun.TOML.parse(readFileSync(resolve(this.repoRoot, scope.manifest), "utf8")).workspace, directory: resolve(this.repoRoot, scope.directory) };
    };
    for (const row of cargoRepositoryPackages(this.repoRoot)) members.set(row.name, owner(resolve(this.repoRoot, row.manifest)));
    const closure = new Map<string, Owner>();
    const visit = (name: string): void => {
      if (closure.has(name)) return;
      const member = members.get(name);
      if (!member) throw new Error(`first-party dependency has no workspace owner: ${name}`);
      if (name.startsWith("semio-s-artifact-stdio-") && name !== "semio-s-artifact-stdio-contract") throw new Error(`general assembly reaches concrete artifact ${name}`);
      if (name.startsWith("semio-hub-")) throw new Error(`general assembly reaches outward composition ${name}`);
      closure.set(name, member);
      for (const table of [member.manifest, ...Object.values(member.manifest.target ?? {})] as any[]) {
        for (const group of ["dependencies", "dev-dependencies", "build-dependencies"]) {
          for (const [alias, value] of Object.entries(table[group] ?? {}) as [string, any][]) {
            const declaration = value?.workspace ? member.workspace.dependencies[alias] : value;
            if (declaration?.path) {
              const dependencyPath = resolve(value?.workspace ? member.directory : member.path, declaration.path);
              const dependency = Bun.TOML.parse(readFileSync(join(dependencyPath, "Cargo.toml"), "utf8")) as any;
              if (!members.has(dependency.package.name)) members.set(dependency.package.name, owner(join(dependencyPath, "Cargo.toml")));
              visit(dependency.package.name);
            }
          }
        }
      }
    };
    visit("semio-s-plugin-stdio");
    for (const owner of new Set(requestedOwners)) visit(owner);
    const skipped = new Set(["dist", "target", "node_modules", ".git", "🗑️generated", ".cache", "🔌️plugin-modules", "🧩️extension-modules", "🧶️bundles"]);
    let files = 0;
    const copy = (source: string): void => {
      const entry = lstatSync(source);
      if (entry.isSymbolicLink()) return;
      if (entry.isDirectory()) {
        if (skipped.has(basename(source))) return;
        for (const item of readdirSync(source)) copy(join(source, item));
      } else if (entry.isFile() && !source.endsWith(".wasm")) {
        const target = join(workspaceRoot, relative(this.repoRoot, source));
        mkdirSync(dirname(target), { recursive: true });
        copyFileSync(source, target);
        files++;
      }
    };
    copy(join(this.repoRoot, "🧰️framework"));
    const core = resolve(this.root, "../..");
    copy(join(core, "🦀️.rs"));
    for (const folder of ["📦️packages", "📇️registry", "🧬️schema", "🧫️fixtures", "🧪️tests"]) copy(join(core, folder));
    for (const member of closure.values()) {
      const path = relative(this.repoRoot, member.path).split("\\").join("/");
      if (!path.startsWith("🧰️framework/") && !path.startsWith(relative(this.repoRoot, core).split("\\").join("/") + "/")) copy(resolve(member.path, "../.."));
    }
    const literal = (value: any): string => typeof value === "string" ? JSON.stringify(value) : Array.isArray(value) ? `[${value.map(literal).join(", ")}]` : value && typeof value === "object" ? `{ ${Object.entries(value).map(([key, item]) => `${JSON.stringify(key)} = ${literal(item)}`).join(", ")} }` : String(value);
    const table = (header: string, values: Record<string, any>): string => `[${header}]\n${Object.entries(values).map(([key, value]) => `${JSON.stringify(key)} = ${literal(value)}`).join("\n")}\n`;
    const dependencies: Record<string, any> = {};
    const original = members.get("semio-s-plugin-stdio")!.workspace;
    for (const member of closure.values()) {
      if (!isDeepStrictEqual(member.workspace.package, original.package) || !isDeepStrictEqual(member.workspace.lints, original.lints)) throw new Error("deletion-proof owners disagree on inherited package or lint authority");
      for (const row of [member.manifest, ...Object.values(member.manifest.target ?? {})] as any[]) {
        for (const group of ["dependencies", "dev-dependencies", "build-dependencies"]) {
          for (const [alias, value] of Object.entries(row[group] ?? {}) as [string, any][]) {
            if (!value?.workspace) continue;
            const declaration = member.workspace.dependencies[alias];
            if (declaration === undefined) throw new Error(`missing inherited dependency ${alias}`);
            const resolved = declaration?.path ? { ...declaration, path: relative(this.repoRoot, resolve(member.directory, declaration.path)).split("\\").join("/") } : declaration;
            if (alias in dependencies && !isDeepStrictEqual(dependencies[alias], resolved)) throw new Error(`deletion-proof owners disagree on inherited dependency ${alias}`);
            dependencies[alias] = resolved;
          }
        }
      }
      const target = join(workspaceRoot, relative(this.repoRoot, member.path), "Cargo.toml");
      const inherited = relative(member.path, this.repoRoot).split("\\").join("/") || ".";
      const source = readFileSync(target, "utf8");
      writeFileSync(target, source.replace(/\[package\]([^]*?)(?=\n\[|$)/u, (_, body: string) => `[package]${body.replace(/^workspace\s*=.*\n?/mu, "").trimEnd()}\nworkspace = ${JSON.stringify(inherited)}\n`));
    }
    const manifest = table("workspace", { members: [...closure.values()].map((member) => relative(this.repoRoot, member.path).split("\\").join("/")), resolver: "2" }) + table("workspace.package", original.package) + table("workspace.dependencies", dependencies) + Object.entries(original.lints).map(([name, values]) => table(`workspace.lints.${name}`, values as Record<string, any>)).join("") + table("profile.dev", { debug: 0, incremental: false });
    writeFileSync(join(workspaceRoot, "Cargo.toml"), manifest);
    copy(join(this.repoRoot, "Cargo.lock"));
    copy(join(this.repoRoot, "nx.json"));
    copy(join(this.repoRoot, "📋️project.json"));
    const artifactsAbsent = !existsSync(join(workspaceRoot, relative(this.repoRoot, core), "🗿️artifacts"));
    if (!artifactsAbsent) throw new Error("deletion-proof copied concrete Stdio artifacts");
    const env = devToolingEnv({ CARGO_TARGET_DIR: join(workspaceRoot, "target"), CARGO_BUILD_BUILD_DIR: join(workspaceRoot, "cargo-build"), CARGO_INCREMENTAL: "0" });
    const checkedPackages = [...new Set(["semio-s-plugin-stdio", ...(closure.has("semio-framework-3d") ? ["semio-framework-3d"] : []), ...requestedOwners])];
    await runOwnedCommand("cargo", ["check", "--offline", ...checkedPackages.flatMap((name) => ["-p", name]), "--lib"], workspaceRoot, "stdio-deletion-proof", undefined, { env });
    const probeRoot = join(workspaceRoot, "🧪️sqlite-required");
    mkdirSync(probeRoot);
    writeFileSync(join(probeRoot, "Cargo.toml"), '[package]\nname = "semio-stdio-sql-required-probe"\nversion = "0.1.0"\nedition = "2021"\n[lib]\npath = "🦀️.rs"\n[dependencies]\nsemio-framework-os-kernel.workspace = true\n');
    writeFileSync(join(probeRoot, "🦀️.rs"), `use semio_framework_os_kernel::{ArtifactCodec, ArtifactDsl, ArtifactPack, ToValue, FromValue, OpText, OpBinary, Mutation};
pub fn required<P, M>(schema: &str) -> ArtifactCodec
where P: Clone + PartialEq + ToValue + FromValue + ArtifactDsl + ArtifactPack + Send + Sync + 'static,
M: Mutation<P> + PartialEq + ToValue + FromValue + OpText + OpBinary + Send + Sync + 'static {
    ArtifactCodec::of::<P, M>(schema)
}
`);
    writeFileSync(join(workspaceRoot, "Cargo.toml"), manifest.replace('"members" = [', '"members" = ["🧪️sqlite-required", '));
    const probe = Bun.spawn(["cargo", "check", "--offline", "-p", "semio-stdio-sql-required-probe"], { cwd: workspaceRoot, env, stdout: "ignore", stderr: "pipe" });
    const timeout = setTimeout(() => probe.kill(), 60_000);
    const cancel = () => probe.kill();
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    let diagnostic: string;
    let status: number;
    try { diagnostic = await new Response(probe.stderr).text(); status = await probe.exited; }
    finally { clearTimeout(timeout); process.off("SIGINT", cancel); process.off("SIGTERM", cancel); }
    writeFileSync(join(workspaceRoot, "sqlite-required-refusal.txt"), diagnostic);
    if (status === 0 || !diagnostic.includes("ArtifactSqliteSnapshot") || !diagnostic.includes("E0277")) throw new Error("explicit SQLite constructor did not refuse a pack-only generic caller with the required bound");
    writeFileSync(join(output, "stdio-deletion-proof.json"), JSON.stringify({ schema: "semio.stdio.deletion-proof/v1", artifactsAbsent, regularFiles: files, owners: [...closure.keys()].sort(), checkedPackages, sqliteRequiredRefusal: "E0277 ArtifactSqliteSnapshot" }, null, 2) + "\n");
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-plugin-stdio", { twins: [{ name: "authored-assembly", run: runAssemblyChecks }], commands: { "deletion-proof": DeletionProofScript } });
