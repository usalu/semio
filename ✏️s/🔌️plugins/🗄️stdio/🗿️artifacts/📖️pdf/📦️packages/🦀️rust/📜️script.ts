#!/usr/bin/env bun
/** 📦️ Runs PDF artifact laws and its owned recursive retirement contract. */
import { resolve } from "node:path";
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";

const snapshotSqliteSources = [
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧪️tests/🛫️output/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts",
  "../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🎯️navigation/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📇️metadata/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📄️document/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖼️resource/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🌈️color/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧩️cos/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖋️content/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔤️font/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📝️form/🟦️.ts",
  "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📌️annotation/🟦️.ts"
];

/** 🛂️ Checks actual PDF Snapshot facades, own14 mutation facets and independent Source oracle types. */
class VerifySnapshotSqliteSource extends BundleScript {
  async run(args: string[]): Promise<void> {
    if(args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("Expected verify snapshot-sqlite-source");
    const paths = [...snapshotSqliteSources, "../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts", "../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts", "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts"].map(path=>resolve(this.root,path));
    await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",...paths],this.repoRoot,"pdf:snapshot-sqlite-source-types",60_000);
  }
}

/** 🎛️ Preserves default artifact testing and owns the explicit retirement subcommand. */
class RecursiveRetirementTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    const mode = args.shift();
    if (args.length || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Expected test recursive-retirement [source|native]");
    if (mode !== "native") await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/♻️retirement/🟦️.ts")], this.repoRoot, "pdf:recursive-retirement:source", 15_000);
    if (mode !== "source") await runArtifactRustTests("semio-s-artifact-stdio-pdf", this.repoRoot, ["--lib", "pdf_recursive_retirement_"], this.invocation);
  }
}
/** 🪪️ Runs native admission and pure stream identity laws through the package runner. */
class StreamRolesTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test stream-roles");
    await runArtifactRustTests("semio-s-artifact-stdio-pdf", this.repoRoot, ["--lib", "pdf_stream_roles_", "--", "--nocapture"], this.invocation);
  }
}
/** 🖼️ Proves logical image words and independently admitted native foreign artifact custody. */
class SemanticBodyTest extends BundleScript {
  async run(args:string[]):Promise<void>{if(args.length)throw new Error("Expected test semantic-body");await runArtifactRustTests("semio-s-artifact-stdio-pdf", this.repoRoot, ["--lib","pdf_semantic_body_","--","--nocapture"], this.invocation);}
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-pdf", { packages: ["semio-s-artifact-stdio-pdf", "semio-s-artifact-stdio-pdf-conformance-support", "semio-s-artifact-stdio-pdf-diff", "semio-s-artifact-stdio-pdf-fonts", "semio-s-artifact-stdio-pdf-graph-projection", "semio-s-artifact-stdio-pdf-inferences", "semio-s-artifact-stdio-pdf-lift", "semio-s-artifact-stdio-pdf-lower", "semio-s-artifact-stdio-pdf-model", "semio-s-artifact-stdio-pdf-mut-base", "semio-s-artifact-stdio-pdf-mut-base-2", "semio-s-artifact-stdio-pdf-mut-base-3", "semio-s-artifact-stdio-pdf-mut-base-4", "semio-s-artifact-stdio-pdf-mut-base-5", "semio-s-artifact-stdio-pdf-mut-base-6", "semio-s-artifact-stdio-pdf-mut-base-7", "semio-s-artifact-stdio-pdf-mut-base-8", "semio-s-artifact-stdio-pdf-mut-base-9", "semio-s-artifact-stdio-pdf-mut-family", "semio-s-artifact-stdio-pdf-mut-family-10", "semio-s-artifact-stdio-pdf-mut-family-11", "semio-s-artifact-stdio-pdf-mut-family-12", "semio-s-artifact-stdio-pdf-mut-family-13", "semio-s-artifact-stdio-pdf-mut-family-14", "semio-s-artifact-stdio-pdf-mut-family-15", "semio-s-artifact-stdio-pdf-mut-family-2", "semio-s-artifact-stdio-pdf-mut-family-3", "semio-s-artifact-stdio-pdf-mut-family-4", "semio-s-artifact-stdio-pdf-mut-family-5", "semio-s-artifact-stdio-pdf-mut-family-6", "semio-s-artifact-stdio-pdf-mut-family-7", "semio-s-artifact-stdio-pdf-mut-family-8", "semio-s-artifact-stdio-pdf-mut-family-9", "semio-s-artifact-stdio-pdf-mut-vt", "semio-s-artifact-stdio-pdf-mut-x", "semio-s-artifact-stdio-pdf-mut-x-2", "semio-s-artifact-stdio-pdf-mutations", "semio-s-artifact-stdio-pdf-mutations-2", "semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests", "semio-s-artifact-stdio-pdf-schema", "semio-s-artifact-stdio-pdf-schema-2", "semio-s-artifact-stdio-pdf-schema-3", "semio-s-artifact-stdio-pdf-schema-4", "semio-s-artifact-stdio-pdf-schema-5", "semio-s-artifact-stdio-pdf-snapshot", "semio-s-artifact-stdio-pdf-snapshot-2", "semio-s-artifact-stdio-pdf-tables", "semio-s-artifact-stdio-pdf-testkit", "semio-s-artifact-stdio-pdf-tests", "semio-s-artifact-stdio-pdf-tests-2", "semio-s-artifact-stdio-pdf-viewer"], snapshotSqliteTests: snapshotSqliteSources, commands: { verify: VerifySnapshotSqliteSource }, testCommands: { "recursive-retirement": RecursiveRetirementTest, "stream-roles": StreamRolesTest, "semantic-body": SemanticBodyTest } });
