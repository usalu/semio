/** 🔘️ The utility, interaction, tool, tool run and command definition types, the framework-owned action ids, the dialog definition and choice and the shell brand types. */
import type { IntroductionDefinition, TutorialDefinition } from "@semio-tech/framework-onboarding/declaration";
import type { JobKindId as GeneratedJobKindId, ToolRunCounterDefinition as GeneratedToolRunCounterDefinition, ToolRunDefinition as GeneratedToolRunDefinition, ToolRunReasonDefinition as GeneratedToolRunReasonDefinition, ToolRunRebasePolicy as GeneratedToolRunRebasePolicy, ToolRunReconfigurePolicy as GeneratedToolRunReconfigurePolicy, ToolRunSettingsReads as GeneratedToolRunSettingsReads, ToolRunStageDefinition as GeneratedToolRunStageDefinition, ToolRunTraceKind as GeneratedToolRunTraceKind, ToolRunVerdict as GeneratedToolRunVerdict, UtilityDefinition as GeneratedUtilityDefinition, UtilityRef as GeneratedUtilityRef, InteractionDefinition as GeneratedInteractionDefinition, GranularityDefinition as GeneratedGranularityDefinition, HierarchyProvider as GeneratedHierarchyProvider, HoverSpec as GeneratedHoverSpec, SelectionSpec as GeneratedSelectionSpec, SelectionMode as GeneratedSelectionMode, SelectionMethod as GeneratedSelectionMethod, MergeMode as GeneratedMergeMode, InteractionRef as GeneratedInteractionRef, DomainSelection as GeneratedDomainSelection, ToolDefinition as GeneratedToolDefinition, ToolRef as GeneratedToolRef, CommandDefinition as GeneratedCommandDefinition, CommandOwnerAddress as GeneratedCommandOwnerAddress, CommandAddress as GeneratedCommandAddress, CommandInvocation as GeneratedCommandInvocation, OsDefinition as GeneratedOsDefinition, Platform as GeneratedPlatform, PlatformKeybinding as GeneratedPlatformKeybinding, DialogChoice as GeneratedDialogChoice, DialogDefinition as GeneratedDialogDefinition } from "../🤖️generated/🪪️manifest/🟦️.ts";
import { type ShellLocale, type ShellTerminology } from "../🤖️generated/🎚️ui-axes/🟦️.ts";
import type { ActionArgDef } from "../🎬️action-types-and-semantics/🟦️.ts";

// #region 🛂️Manifest
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
export type UtilityDefinition = GeneratedUtilityDefinition;
export type UtilityRef = GeneratedUtilityRef;

/** 🕹️ Generated from Rust `InteractionDefinition` family (`🕹️interaction/🦀️.rs`) — see
 * `js/generated/manifest.ts`. Mirrors `ActionDefinition`/`ActionRef`'s import shape above. */
export type InteractionDefinition = GeneratedInteractionDefinition;
export type GranularityDefinition = GeneratedGranularityDefinition;
export type HierarchyProvider = GeneratedHierarchyProvider;
export type HoverSpec = GeneratedHoverSpec;
export type SelectionSpec = GeneratedSelectionSpec;
export type SelectionMode = GeneratedSelectionMode;
export type SelectionMethod = GeneratedSelectionMethod;
export type MergeMode = GeneratedMergeMode;
export type InteractionRef = GeneratedInteractionRef;
export type DomainSelection = GeneratedDomainSelection;

/** 🛠️ Generated from Rust `ToolDefinition`/`ToolRef` (`framework/core/rs/lib.rs`) — a mode-level,
 * activatable capability (e.g. puzzle3d fill), distinct from a per-window `UtilityDefinition`. See
 * `js/generated/manifest.ts`. */
export type ToolDefinition = GeneratedToolDefinition;

/** ⏯️ Generated from Rust `ToolRunDefinition` (`⏯️tool-run/🦀️.rs`, schema `⏯️tool-run/🧬️schema/🔣️.json`) — the static
 * run declaration a `ToolDefinition`/`UtilityDefinition` carries as `run`. */
export type ToolRunDefinition = GeneratedToolRunDefinition;
export type ToolRunStageDefinition = GeneratedToolRunStageDefinition;
export type ToolRunCounterDefinition = GeneratedToolRunCounterDefinition;
export type ToolRunReasonDefinition = GeneratedToolRunReasonDefinition;
export type ToolRunRebasePolicy = GeneratedToolRunRebasePolicy;
export type ToolRunReconfigurePolicy = GeneratedToolRunReconfigurePolicy;
export type ToolRunSettingsReads = GeneratedToolRunSettingsReads;
export type ToolRunTraceKind = GeneratedToolRunTraceKind;
export type ToolRunVerdict = GeneratedToolRunVerdict;
export type JobKindId = GeneratedJobKindId;
export type ToolRef = GeneratedToolRef;

/** 🎛️ Generated command ownership, invocation, and platform-aware keybinding contracts. */
export type CommandDefinition = GeneratedCommandDefinition;
export type CommandOwnerAddress = GeneratedCommandOwnerAddress;
export type CommandAddress = GeneratedCommandAddress;
export type CommandInvocation = GeneratedCommandInvocation;
export type OsDefinition = GeneratedOsDefinition;
export type Platform = GeneratedPlatform;
export type PlatformKeybinding = GeneratedPlatformKeybinding;

/** 🧰️ The framework-owned action id apps dispatch to activate a utility — mirrors `SET_ACTIVE_UTILITY_ACTION_ID`. */
export const SET_ACTIVE_UTILITY_ACTION_ID = "setActiveUtility";

/** 🔧️ The framework-owned action id apps dispatch to activate a mode-level tool — mirrors Rust `SET_ACTIVE_TOOL_ACTION_ID`. */
export const SET_ACTIVE_TOOL_ACTION_ID = "setActiveTool";

/** 🖱️ The six framework-owned Interaction action ids (`interaction_action_definitions`), auto-injected
 * into any app that declares at least one `InteractionDefinition` — mirrors `HISTORY_ACTION_IDS`.
 * `interactionSelect`/`interactionHover` are raw dispatch verbs renderers translate clicks/marquee/
 * hover into (never in the palette); the rest are user-facing and drive the per-domain Select controls. */
export const INTERACTION_SELECT_ACTION_ID = "interactionSelect";
export const INTERACTION_HOVER_ACTION_ID = "interactionHover";
export const CLEAR_SELECTION_ACTION_ID = "clearSelection";
export const SELECT_ALL_ACTION_ID = "selectAll";
export const SET_SELECTION_MODE_ACTION_ID = "setSelectionMode";
export const SET_INTERACTION_GRANULARITY_ACTION_ID = "setInteractionGranularity";

/** 📤️ The framework-owned Export Document action id, shell-intercepted in every app — mirrors Rust
 * `EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`. */
export const EXPORT_ARTIFACT_DOCUMENT_ACTION_ID = "exportArtifactDocument";

/** 📥️ The framework-owned Import Document action id, shell-intercepted in every app — mirrors Rust
 * `IMPORT_ARTIFACT_DOCUMENT_ACTION_ID`. */
export const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID = "importArtifactDocument";

/** 💾️ The shell-intercepted action that writes a mounted natural file codec's bytes. */
export const SAVE_ARTIFACT_FILE_ACTION_ID = "saveArtifactFile";

/** 📂️ The shell-intercepted action that opens natural bytes as a new document owner. */
export const OPEN_ARTIFACT_FILE_ACTION_ID = "openArtifactFile";

/** 📄️ The ABI media port reserved for an editor's declared natural-file representation. */
export const NATURAL_FILE_PORT_ID = "artifact:native";

/** 🗨️ Generated from Rust `DialogDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type DialogDefinition = GeneratedDialogDefinition;

/** 🔀️ Generated from Rust `DialogChoice` — one decision button of a {@link DialogDefinition}. */
export type DialogChoice = GeneratedDialogChoice;

/** ⌨️ The arg key a {@link DialogChoice} dispatch carries its own id under — mirrors Rust `DIALOG_CHOICE_ARG`. */
export const DIALOG_CHOICE_ARG = "choice";

/** 💬️ The dispatch args of `choice`: the seed context of `effective` (keys no arg of `defs` declares), the args the choice requires, and {@link DIALOG_CHOICE_ARG} naming it — the twin of Rust `DialogChoice::dispatch_args`. */
export function dialogChoiceArgs(choice: DialogChoice, defs: readonly ActionArgDef[], effective: Readonly<Record<string, unknown>>): Record<string, unknown> {
  const requires = choice.requires ?? [];
  const kept = Object.entries(effective).filter(([key]) => key !== DIALOG_CHOICE_ARG && (requires.includes(key) || !defs.some((def) => def.id === key)));
  return { ...Object.fromEntries(kept), [DIALOG_CHOICE_ARG]: choice.id };
}

//#region 🏷️ShellBrand
// 🌐️ ShellLocale/ShellTerminology are generated from 🖱️ui/🎚️axes/🔣️.json (the same source of
// truth Rust's Locale/Terminology enums derive from), imported/re-exported above — so a locale
// added there and here can never drift. The single source `UiLocale` (`framework/ui/js/react`),
// `ShellBrandLocks.locale`, and `resolveShellLocks` all derive from this.
/** 🔒️ Shell preferences a brand pins at boot: each set axis is fixed and its in-app switcher hidden (validated by the renderer's `resolveShellLocks`). */
export type ShellBrandLocks = {
  readonly exampleId?: string;
  readonly locale?: ShellLocale;
  readonly terminology?: ShellTerminology;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🎨️ Shell preferences a brand seeds at boot without pinning them: the value applies on first launch but the in-app switcher stays visible. */
export type ShellBrandDefaults = {
  readonly exampleId?: string;
};

/** 🏛️ Localized partner credits contributed by a shell owner. */
export type ShellFooterItem = {
  readonly id: string;
  readonly placement: "leading" | "trailing";
  readonly caption: Readonly<Record<ShellLocale, string>>;
  readonly separator?: Readonly<Record<ShellLocale, string>>;
  readonly logos: readonly { readonly href: string; readonly src: string; readonly darkSrc?: string; readonly alt: string }[];
};

/** 🏷️ Boot-time branding for a standalone shell artifact — identity (window title, logo mark, favicon), locked and defaulted shell preferences, and an optional brand-owned {@link IntroductionDefinition} replacing the app's own (already localized, rendered verbatim). */
export type ShellBrand = {
  readonly id: string;
  readonly windowTitle: string;
  /** 📱️ What the navbar shows below the tablet breakpoint instead of {@link windowTitle}; defaults to `windowTitle`. */
  readonly shortWindowTitle?: string;
  readonly footerItems?: readonly ShellFooterItem[];
  readonly logoSvg?: string;
  readonly faviconIcoPath?: string;
  readonly locks?: ShellBrandLocks;
  readonly defaults?: ShellBrandDefaults;
  readonly introduction?: IntroductionDefinition;
  /** 🎬️ Brand-owned tutorials shown ALONGSIDE the app's own declared ones (never replacing them, unlike `introduction`). */
  readonly tutorials?: readonly TutorialDefinition[];
  /** 🎓️ When true, auto-starts the brand introduction on every window load and never persists a device-local "seen" flag. */
  readonly replayIntroductionOnLoad?: boolean;
  /** 🧊️ When true, the shell never reads or writes device-local shell state (dock, panes, named layouts, chrome prefs, introduction seen) — every refresh boots from brand locks/defaults only. */
  readonly ephemeral?: boolean;
  /** 🗂️ Repo-root-relative directory of this brand's own static assets (logos, etc.) — the dev/build server mounts it as a static route at `/<assetsDir>` alongside the shared `framework/ui/asset` mount. */
  readonly assetsDir?: string;
  /** 📦️ Repo-root-relative directory this brand's build output lands in instead of the shared playground `dist/` — keeps a brand's specialization (including its build artifact) self-contained. */
  readonly distDir?: string;
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `CNAME` file at the build root. */
  readonly cnameHost?: string;
};
//#endregion 🏷️ShellBrand
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
