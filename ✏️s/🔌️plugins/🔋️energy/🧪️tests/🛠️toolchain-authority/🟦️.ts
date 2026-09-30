import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileSha256, oracleManifest, validateOracleManifest } from "../../🔮️oracles/🛠️toolchain/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../..");

test("Energy portable toolchain authority preserves its manifest and digest contracts", async () => {
    const packageRoot = resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python");
    const manifestPath = resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🔣️.json");
    const manifest = oracleManifest(repoRoot);
    expect(manifest.tools[0]!.platforms[process.platform === "darwin" ? `darwin-${process.arch}` : `${process.platform}-${process.arch}`]).toBeDefined();
    const hostile = structuredClone(manifest) as any;
    hostile.tools[0].platforms["linux-x64"].sha256 = "0".repeat(64);
    expect(() => validateOracleManifest(hostile)).toThrow();
    const bytes = readFileSync(manifestPath);
    expect(await fileSha256(manifestPath)).toBe(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex"));
    expect(existsSync(resolve(packageRoot, "🔣️.json"))).toBe(false);
    const contribution = JSON.parse(readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🔣️.json"), "utf8"));
    expect(contribution.oracleHostPackages).toContainEqual(expect.objectContaining({ implementation: "python", path: "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution", module: "🐍️" }));
    const execution = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts"), "utf8");
    expect(execution).toContain("entry.path === PYTHON_OWNER");
    expect(execution).toContain("import_module");
    expect(execution).toContain("env.PYTHONPATH");
    expect(readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🐍️.py"), "utf8")).toContain("@see ../🛠️toolchain/🔣️.json");
    expect(existsSync(resolve(packageRoot, "🔮️oracles/🐍️.py"))).toBe(false);
    const registrations = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json"), "utf8");
    expect(registrations).toContain("✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🐍️.py");
    expect(registrations).toContain("✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🔣️.json");
    expect(registrations).not.toContain("📦️packages/🐍️python/🔮️oracles/🐍️.py");
    expect(registrations).not.toContain("🔮️oracles/📦️packages/🐍️python/🔣️.json");
});
