import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

export const files: Record<string, string> = {
  "Cargo.toml": '[workspace]\nresolver = "2"\nmembers = [ "📦️packages/🦀️rust" ]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n',
  "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-artifact"\nversion = "0.1.0"\n',
  "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-artifact"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
  "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-artifact",{snapshotSqliteTests:[]});\n',
  "🦀️.rs": [
    "//! 🧪️ Demo artifact root.",
    "pub const DOCUMENT: &str = \"demo\";",
    "#[path = \".\"]",
    "pub mod schema {",
    "    #[path = \"🧬️schema/🦀️.rs\"]",
    "    mod component;",
    "    pub use component::*;",
    "    #[path = \"🧬️schema/📸️snapshot/🦀️.rs\"]",
    "    pub mod snapshot;",
    "    #[path = \"🧬️schema/🔺️diff/🦀️.rs\"]",
    "    pub mod diff;",
    "}",
    "#[path = \".\"]",
    "pub mod io {",
    "    #[path = \"🚪️io/🦀️.rs\"]",
    "    mod component;",
    "    pub use component::*;",
    "}",
    "",
  ].join("\n"),
  "🧬️schema/🦀️.rs": "//! 🧬️ schema component\npub fn describe() -> &'static str { crate::DOCUMENT }\n",
  "🧬️schema/📸️snapshot/🦀️.rs": "//! 📸️ snapshot\n#[derive(Clone, Debug, Default, PartialEq)]\npub struct Snap { pub id: String }\n",
  "🧬️schema/🔺️diff/🦀️.rs": "//! 🔺️ diff\nuse crate::schema::snapshot::Snap;\npub struct Diff { pub before: Snap }\n",
  "🚪️io/🦀️.rs": "//! 🚪️ io\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n",
  "🚪️io/📝️text/🦀️.rs": "//! 📝️ text codec\nuse crate::schema::snapshot::Snap;\nimpl std::fmt::Display for Snap {\n    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, \"{}\", self.id) }\n}\n",
};

export function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "semio-crate-tree-"));
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
  return root;
}

export const weaveFiles = (deep: boolean): Record<string, string> => ({
  "Cargo.toml": files["Cargo.toml"]!,
  "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-weave"\nversion = "0.1.0"\n',
  "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-weave"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
  "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-weave",{snapshotSqliteTests:[]});\n',
  "🦀️.rs": '//! root\npub const DOCUMENT: &str = "d";\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n#[path = "."]\npub mod schema {\n    #[path = "🧬️schema/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
  "🚪️io/🦀️.rs": "//! io\npub fn helper() -> u8 { 1 }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n",
  "🚪️io/📝️text/🦀️.rs": `//! text\npub fn helper_text() -> u8 { crate::io::helper() + crate::schema::value() }\n${deep ? '#[path = "🧱️deep/🦀️.rs"]\npub mod deep;\n' : ""}`,
  ...(deep ? { "🚪️io/📝️text/🧱️deep/🦀️.rs": "//! deep\npub fn run() -> u8 { crate::io::text::helper_text() }\n" } : {}),
  "🧬️schema/🦀️.rs": "//! schema\npub fn value() -> u8 { 2 }\n",
});

export const tiny = { target: { files: 1, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } };

export const writeTree = (prefix: string, tree: Record<string, string>): string => {
  const root = mkdtempSync(join(tmpdir(), prefix));
  for (const [path, content] of Object.entries(tree)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
  return root;
};

export const scaffold = (name: string, rootSource: string): Record<string, string> => ({
  "Cargo.toml": files["Cargo.toml"]!,
  "Cargo.lock": `version = 4\n\n[[package]]\nname = "${name}"\nversion = "0.1.0"\n`,
  "📦️packages/🦀️rust/Cargo.toml": `[package]\nworkspace = "../.."\nname = "${name}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n`,
  "📦️packages/🦀️rust/📜️script.ts": `await runArtifactRustPackageMain(import.meta.dir, "${name}",{snapshotSqliteTests:[]});\n`,
  "🦀️.rs": rootSource,
});
