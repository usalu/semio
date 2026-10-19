/** 🪢️ The native codec projection script and the snapshot sqlite test script of the hub stdio package. */
import { resolveTestLevel } from "@semio-tech/process/test-budget";
import { buildBudgetMs } from "@semio-tech/process/budget";
import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { devToolingEnv, runCmd, runRepositoryCommand } from "@semio-tech/process/execution";
import { runAdmittedCargoPackageTestsV1 } from "@semio-tech/process/cargo-test";
import { runRepositoryTestCommand } from "@semio-tech/process/test-execution";
import { BundleScript } from "@semio-tech/process/routing";
import { projectNativeCodecReceiptPublicationV1, selectNativeCodecPublicationFactoryV1, type NativeCodecPublicationReceiptV1 } from "../../../📇️publication/♻️native-receipt/🟦️.ts";
import { PACKAGE_NAME } from "../🧮️component-constants/🟦️.ts";

/** 🧬️ Publishes current artifact-scoped native receipts or the complete live codec pack-hash column. */
export class NativeCodecProjectionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "refresh") {
      const factoryId = segments[1];
      if (!factoryId || segments.length !== 2) throw new Error("native-codec-projection refresh requires one authored factory identity");
      const factoryPath = resolve(this.root, "../../🔌️plugin/📇️catalog/📜️native-codec-factories.json");
      const catalogPath = resolve(this.root, "../../📇️publication/📜️native-catalog.json");
      const authored = selectNativeCodecPublicationFactoryV1(JSON.parse(readFileSync(factoryPath, "utf8")).receipts, factoryId);
      const artifact = authored.artifact;
      const control = new AbortController();
      const interrupt = () => control.abort();
      process.on("SIGINT", interrupt);
      process.on("SIGTERM", interrupt);
      try {
        const receipts: NativeCodecPublicationReceiptV1[] = [];
        const ownerScript=resolve(this.repoRoot,dirname(authored.definition_path),"📦️packages/🦀️rust/📜️script.ts");
        await runRepositoryCommand(this.invocation,process.execPath,["repo", "exec", `--projects=@semio-tech/stdio-${artifact}-rs`, "--exclude-dependencies", "--", process.execPath, ownerScript, "test", "long", "--lib", "owned_fixture_publication_reports_canonical_logical_carriers", "--", "--nocapture"],this.repoRoot,"native-codec-publication",{version:1,maximumElapsedMilliseconds:buildBudgetMs(),output:null},{env: { ...process.env, NEXTEST_SUCCESS_OUTPUT: "immediate" },onLine: line => { const prefix = "native-codec-publication="; const index = line.indexOf(prefix); if (index >= 0) receipts.push(JSON.parse(line.slice(index + prefix.length))); }});
        if (control.signal.aborted || receipts.length !== 1) throw new Error("native-codec-projection requires one successful current native receipt");
        const receipt = receipts[0];if(!receipt)throw Error("Actual native codec receipt required");
        if (receipt.factoryId !== authored.factory_id) throw new Error("native-codec-projection native factory disagrees with its owner");
        const definitionPath = resolve(this.repoRoot, authored.definition_path);
        const protocolPath = resolve(this.repoRoot, authored.protocol_path);
        const paths = [factoryPath, definitionPath, catalogPath] as const;
        const originals = [readFileSync(factoryPath,"utf8"),readFileSync(definitionPath,"utf8"),readFileSync(catalogPath,"utf8")] as const;
        const protocol = readFileSync(protocolPath, "utf8");
        const projected = projectNativeCodecReceiptPublicationV1({ receipt, protocol, factories: JSON.parse(originals[0]), definition: JSON.parse(originals[1]), catalog: JSON.parse(originals[2]) });
        const documents = [projected.factories, projected.definition, projected.catalog];
        if (control.signal.aborted || readFileSync(protocolPath, "utf8") !== protocol || paths.some((path, index) => readFileSync(path, "utf8") !== originals[index])) throw new Error("native-codec-projection source changed before publication");
        for (const [index,path] of paths.entries()) writeFileSync(path, `${JSON.stringify(documents[index], null, 2)}\n`);
      } finally {
        process.off("SIGINT", interrupt);
        process.off("SIGTERM", interrupt);
      }
      return;
    }
    if (segments.length) throw new Error("native-codec-projection accepts refresh <factory-id>");
    runCmd("cargo", ["test", "--manifest-path", join(this.root, "Cargo.toml"), "-p", "semio-hub-stdio", "--no-default-features", "--test", "native_openable_provider", "--", "native_codec_projection_pack_schema_hashes_equal_live_receipts", "--exact"], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_NATIVE_CODEC_PROJECTION: "write", CARGO_INCREMENTAL: "0" }), budgetMs: buildBudgetMs() });
  }
}

/** 🪶️ Concrete Stdio snapshot codec fleet with independent SQLite interoperability oracles. */
export class SnapshotSqliteTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "census") {
      if (segments[1] === "check") {
        await runRepositoryTestCommand(process.execPath, ["x", "tsc", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts"), "--noEmit", "--allowImportingTsExtensions", "--module", "ESNext", "--moduleResolution", "Bundler", "--resolveJsonModule", "--allowSyntheticDefaultImports", "--strict", "--skipLibCheck", "--target", "ES2022"], { cwd: this.repoRoot });
        return;
      }
      if (segments[1] === "source") {
        await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts")], { cwd: this.repoRoot });
        return;
      }
      if (segments[1] === "all") resolveTestLevel(segments.slice(2), "long");
      await runAdmittedCargoPackageTestsV1([PACKAGE_NAME], this.repoRoot, this.invocation, ["--test", "shipped_fleet", ...(segments[1] === "all" ? [] : ["sqlite_snapshot_primary_"]), "--no-fail-fast", "--", "--nocapture"]);
      return;
    }
    if (segments[0] === "foundation") {
      await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-binary", "semio-s-artifact-stdio-txt", "semio-s-artifact-stdio-csv", "semio-s-artifact-stdio-tsv", "semio-s-artifact-stdio-json", "semio-s-artifact-stdio-xml"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runRepositoryTestCommand(process.execPath, ["test", ...[
        "💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any",
        "🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any",
        "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any",
        "📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any",
        "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base",
        "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base",
      ].map(root => join(artifacts, root, "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "catalog" || segments[0] === "catalog-contract") {
      const directory = resolve(this.root, "../../🧪️tests/🧫️fixtures/✏️editor-catalog/🪶️sqlite");
      const law = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
      
      assert.deepEqual(law, { format: 1, schemaVersion: 1, minimumDomainTables: 1, integrity: [{ integrity_check: "ok" }], foreignKeys: [], nativeEncodings: ["binary", "text"] });
      if (segments[0] === "catalog-contract") return;
      await runAdmittedCargoPackageTestsV1(["semio-hub-stdio"], this.repoRoot, this.invocation, ["--test", "editor_catalog", "sqlite_snapshot_", "--no-fail-fast"]);
      return;
    }
    if (segments[0] === "geometry") {
      await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-stl", "semio-s-artifact-stdio-obj", "semio-s-artifact-stdio-ply"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runRepositoryTestCommand(process.execPath, ["test", ...[
        "🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any",
        "🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry",
        "🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any",
      ].map(root => join(artifacts, root, "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "images") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-bmp", "semio-s-artifact-stdio-png"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runRepositoryTestCommand(process.execPath, ["test", ...[
        "🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any",
        "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any",
      ].map(root => join(artifacts, root, "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "archives") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-deflate", "semio-s-artifact-stdio-zip"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runRepositoryTestCommand(process.execPath, ["test", ...[
        "🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any",
        "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base",
      ].map(root => join(artifacts, root, "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "gif") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-gif"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards");
      await runRepositoryTestCommand(process.execPath, ["test", ...[
        "7️⃣87a/🪆️subsets/✳️any",
        "9️⃣89a/🪆️subsets/🧱️base",
      ].map(root => join(artifacts, root, "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "jpg") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-jpg"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "md") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-md"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mp3") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-mp3"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      if (segments[1] !== "native") await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "semio") {
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets");
      const owners = ["✉️base", "🔤️text", "🔢️value", "🔊️audio", "🎬️video", "🖼️image", "🌊️flow", "📊️table", "🕸️graph", "🎞️animation", "📦️object", "🧰️kit", "🔺️mesh", "🏛️model", "📐️cad", "📑️document", "🖊️drawing", "📽️presentation", "🧊️brep"];
      const selected = segments[2] === undefined ? owners : owners.filter(owner => owner.endsWith(segments[2]!));
      if (selected.length === 0) throw new Error("Unknown Semio SQLite snapshot owner");
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-semio"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      await runRepositoryTestCommand(process.execPath, ["test", ...selected.map(owner => join(artifacts, owner, "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "tiff") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-tiff"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "wav") {
      if (segments[1] !== "source") await runAdmittedCargoPackageTestsV1(["semio-s-artifact-stdio-wav"], this.repoRoot, this.invocation, ["--lib", "sqlite_snapshot_"]);
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    throw new Error("Unknown authored Stdio SQLite snapshot group");
  }
}
