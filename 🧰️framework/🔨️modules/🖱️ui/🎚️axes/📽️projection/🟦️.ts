import type { UiAxes, UiAxisEntry } from "../📥️source/🟦️.ts";

function emitRustEnum(name: string, entries: readonly UiAxisEntry[]): string {
  const variants = entries.map((entry, index) => `    ${index === 0 ? "#[default]\n    " : ""}${entry.variant},`).join("\n");
  const asStrArms = entries.map((entry) => `            ${name}::${entry.variant} => ${JSON.stringify(entry.id)},`).join("\n");
  const indexArms = entries.map((entry, index) => `            ${name}::${entry.variant} => ${index},`).join("\n");
  const parseArms = entries.map((entry) => `            ${JSON.stringify(entry.id)} => Some(${name}::${entry.variant}),`).join("\n");
  const allList = entries.map((entry) => `${name}::${entry.variant}`).join(", ");
  return `#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ${name} {
${variants}
}

impl ${name} {
    pub const ALL: [${name}; ${entries.length}] = [${allList}];
    pub const COUNT: usize = ${entries.length};

    pub const fn as_str(self) -> &'static str {
        match self {
${asStrArms}
        }
    }

    pub const fn index(self) -> usize {
        match self {
${indexArms}
        }
    }

    pub fn parse(value: &str) -> Option<${name}> {
        match value {
${parseArms}
            _ => None,
        }
    }
}

impl std::fmt::Display for ${name} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
`;
}

/** 🦀️ Projects the shared axes into the native enum registry. */
export function emitUiAxesRust(axes: UiAxes): string {
  return `// @generated from 🖱️ui/🎚️axes/🔣️.json — do not edit.

use serde::{Deserialize, Serialize};

/// 🌐️ Chrome-complete UI locales — generated. Adding a locale here breaks every exhaustive
/// \`match\` on \`Locale\` (every \`app_labels!\` cell resolver, \`resolve_labels\`, chrome bundles)
/// until the new variant is covered — that failure list is the correctness guarantee.
${emitRustEnum("Locale", axes.locales)}
/// 🗣️ Terminology ids — generated. Same contract as \`Locale\`.
${emitRustEnum("Terminology", axes.terminologies)}
`;
}

/** 🟦️ Projects the shared axes into TypeScript literals and membership guards. */
export function emitUiAxesTypeScript(axes: UiAxes): string {
  const localeIds = axes.locales.map((entry) => JSON.stringify(entry.id)).join(", ");
  const terminologyIds = axes.terminologies.map((entry) => JSON.stringify(entry.id)).join(", ");
  return `/** @generated from 🖱️ui/🎚️axes/🔣️.json — do not edit. */

export const SHELL_LOCALES = [${localeIds}] as const;
export type ShellLocale = (typeof SHELL_LOCALES)[number];

export const SHELL_TERMINOLOGIES = [${terminologyIds}] as const;
export type ShellTerminology = (typeof SHELL_TERMINOLOGIES)[number];

/** 🧭️ Membership guard — the runtime twin of the Rust \`Locale::parse\`/exhaustive-match guarantee. */
export const isShellLocale = (value: string): value is ShellLocale => (SHELL_LOCALES as readonly string[]).includes(value);
/** 🧭️ Membership guard — the runtime twin of the Rust \`Terminology::parse\`/exhaustive-match guarantee. */
export const isShellTerminology = (value: string): value is ShellTerminology => (SHELL_TERMINOLOGIES as readonly string[]).includes(value);

/** 🗺️ Full locale×terminology matrix for a manifest label, mirroring Rust \`LocalizedLabel\`. */
export type LocalizedLabel = Readonly<Record<ShellTerminology, Readonly<Record<ShellLocale, string>>>>;
`;
}
