// #region 🧲️Header
/** 📎️ The offer to reconnect a remembered folder is accessible in both shell languages: one polite status region named
 * "Folder of this document" / "Ordner dieses Dokuments" that names the folder, a "Reconnect folder" button that is the
 * person's own gesture and a "Forget folder" button; while reconnecting both are disabled and the reconnect is busy.
 * The shared `🧫️local-folder-bindings` corpus (the one the wgpu shell asserts) holds on React: validated by Ajv against its
 * schema and the two leaf payload schemas, its commits, stored logs, offers, folder names and copy are React's.
 * Accessible names are computed by `dom-accessibility-api` (third party), clicks are driven by `@testing-library/user-event`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import Ajv from "ajv";
import * as React from "react";
import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterAll, describe, expect, it, vi } from "vitest";
import type * as DomAccessibilityApi from "dom-accessibility-api" with { "resolution-mode": "require" };
import { argControl, createMemoryStoragePort, mutationInputDefs, OsShellConfig } from "@semio-tech/framework";
import { shellLabel, syncAttachDocumentIdV1, syncShellLabelLocale } from "../../../../🛠️ShellHelpers/🟦️.tsx";
import { commitLocalFoldersConfigMutationV1, LocalFolderReconnectBand, localFolderNameV1, localFolderReconnectOfferV1, readLocalFolderBindingsV1, readLocalFolderEventsV1 } from "../../🟦️.tsx";
import { LOCAL_FOLDERS_CONFIG_SCHEMA, type LocalFolderBinding, type LocalFoldersConfigMutation } from "../../../../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";

const { computeAccessibleName } = createRequire(import.meta.url)("dom-accessibility-api") as typeof DomAccessibilityApi;
const here = dirname(fileURLToPath(import.meta.url));
const engine = join(here, "..", "..", "..", "..", "..");
const mutations = join(engine, "..", "..", "..", "🎚️config", "🧬️schema", "🧬️mutations");
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
type Identity = { readonly documentId: string; readonly pluginId: string; readonly appId: string };
type Corpus = {
  readonly schema: string;
  readonly commits: { readonly steps: readonly { readonly name: string; readonly mutation: LocalFoldersConfigMutation; readonly recorded: boolean; readonly bindings: readonly LocalFolderBinding[] }[]; readonly log: unknown };
  readonly logs: readonly { readonly name: string; readonly raw: string | null; readonly bindings: readonly LocalFolderBinding[] }[];
  readonly offers: readonly { readonly name: string; readonly bindings: readonly LocalFolderBinding[]; readonly identity: Identity | null; readonly attached: string | null; readonly offer: LocalFolderBinding | null }[];
  readonly names: readonly (readonly [string, string])[];
  readonly texts: { readonly folder: string } & Readonly<Record<"en" | "de", { readonly label: string; readonly message: string; readonly attach: string; readonly forget: string }>>;
};
const corpus = readJson(join(engine, "🧫️fixtures", "📎️local-folder-bindings", "🔣️.json")) as Corpus;
// #endregion 🔌️Adapters

//#region 🧪️Laws
const EXPECTED = {
  en: { label: "Folder of this document", message: "This document was attached to the folder “puzzles”.", reconnect: "Reconnect folder", forget: "Forget folder" },
  de: { label: "Ordner dieses Dokuments", message: "Dieses Dokument war mit dem Ordner „puzzles“ verbunden.", reconnect: "Ordner wieder verbinden", forget: "Ordner vergessen" },
} as const;

function band(busy: boolean, onReconnect: () => void, onForget: () => void): React.ReactElement {
  return (
    <LocalFolderReconnectBand
      label={String(shellLabel("ui.sync.reconnect.label"))}
      message={String(shellLabel("ui.sync.reconnect.message", { folder: localFolderNameV1("/Users/ada/Documents/puzzles/") }))}
      reconnect={String(shellLabel("ui.sync.reconnect.attach"))}
      forget={String(shellLabel("ui.sync.reconnect.forget"))}
      busy={busy}
      onReconnect={onReconnect}
      onForget={onForget}
    />
  );
}

describe("local folder reconnect band", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("names the remembered folder in a polite status region and offers reconnect and forget, in both languages", async () => {
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      const reconnect = vi.fn();
      const forget = vi.fn();
      const view = render(band(false, reconnect, forget));
      const region = view.getByRole("status");
      expect(region.getAttribute("aria-live")).toBe("polite");
      expect(computeAccessibleName(region)).toBe(EXPECTED[locale].label);
      expect(view.container.querySelector("#s-folder-reconnect-message")?.textContent).toBe(EXPECTED[locale].message);
      const reconnectButton = view.container.querySelector<HTMLButtonElement>("#s-folder-reconnect")!;
      const forgetButton = view.container.querySelector<HTMLButtonElement>("#s-folder-forget")!;
      expect([computeAccessibleName(reconnectButton), computeAccessibleName(forgetButton)]).toEqual([EXPECTED[locale].reconnect, EXPECTED[locale].forget]);
      const user = userEvent.setup();
      await user.click(reconnectButton);
      await user.click(forgetButton);
      expect([reconnect.mock.calls.length, forget.mock.calls.length]).toEqual([1, 1]);
      view.unmount();
    }
  });

  it("disables both actions and marks the reconnect busy while the folder reconnects", async () => {
    syncShellLabelLocale("en");
    const reconnect = vi.fn();
    const view = render(band(true, reconnect, vi.fn()));
    const reconnectButton = view.container.querySelector<HTMLButtonElement>("#s-folder-reconnect")!;
    expect([reconnectButton.disabled, reconnectButton.getAttribute("aria-busy"), view.container.querySelector<HTMLButtonElement>("#s-folder-forget")!.disabled]).toEqual([true, "true", true]);
    await userEvent.setup().click(reconnectButton);
    expect(reconnect).not.toHaveBeenCalled();
    view.unmount();
  });

  it("fits a phone: the band wraps within 90 % of the viewport and both actions are touch-size targets", () => {
    syncShellLabelLocale("en");
    const view = render(band(false, vi.fn(), vi.fn()));
    const region = view.getByRole("status");
    expect([region.classList.contains("max-w-[90vw]"), region.classList.contains("flex-wrap"), [...region.querySelectorAll("button")].map((button) => button.classList.contains("min-h-medium"))]).toEqual([true, true, [true, true]]);
    view.unmount();
  });

  it("names a folder by its last path segment", () => {
    expect([localFolderNameV1("/Users/ada/Documents/puzzles"), localFolderNameV1("C:\\Users\\ada\\drawings\\"), localFolderNameV1("/")]).toEqual(["puzzles", "drawings", "/"]);
  });

  it("edits a remembered folder in time travel through its path, a text field labelled in both languages, its kind no input", () => {
    const inputs = mutationInputDefs(readFileSync(join(mutations, "📎️attach-local-folder", "🧬️schema", "🔣️.json"), "utf8"), () => undefined);
    const folder = inputs.find((input) => input.id === "/folder")!;
    const fields = folder.schema.kind === "object" ? folder.schema.fields : [];
    expect(fields.map((field) => [field.id, field.schema.kind, argControl(field).kind])).toEqual([["/path", "string", "text"]]);
    expect(["Path", "Pfad"].every((text) => JSON.stringify(fields[0]!.label).includes(`"${text}"`))).toBe(true);
  });
});

describe("📎️ the shared local-folder corpus holds on React", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("validates against its schema and the attach and detach payload schemas", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    ajv.addSchema(readJson(join(mutations, "📎️attach-local-folder", "🧬️schema", "🔣️.json")));
    ajv.addSchema(readJson(join(mutations, "✂️detach-local-folder", "🧬️schema", "🔣️.json")));
    const validate = ajv.compile(readJson(join(engine, "🧬️schema", "🔣️local-folder-bindings", "🔣️.json")));
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...corpus, commits: { ...corpus.commits, log: { version: 1, events: [{ mutation: "shareLocalFolder", documentId: "doc-a" }] } } }), "an unknown mutation is not a facet event").toBe(false);
    expect(corpus.schema).toBe(LOCAL_FOLDERS_CONFIG_SCHEMA);
  });

  it("records exactly the changing mutations, folds each step and stores the corpus's log", () => {
    const device = createMemoryStoragePort();
    for (const step of corpus.commits.steps) {
      const before = readLocalFolderEventsV1(device).length;
      expect(commitLocalFoldersConfigMutationV1(device, step.mutation).bindings, step.name).toEqual(step.bindings);
      expect(readLocalFolderEventsV1(device).length - before, step.name).toBe(step.recorded ? 1 : 0);
    }
    expect(JSON.parse(new OsShellConfig(device).getPreference(LOCAL_FOLDERS_CONFIG_SCHEMA)!)).toEqual(corpus.commits.log);
  });

  it("replays every stored log, and a log that does not read whole reattaches nothing", () => {
    for (const log of corpus.logs) {
      const device = createMemoryStoragePort();
      if (log.raw !== null) new OsShellConfig(device).setPreference(LOCAL_FOLDERS_CONFIG_SCHEMA, log.raw);
      expect(readLocalFolderBindingsV1(device).bindings, log.name).toEqual(log.bindings);
    }
  });

  it("offers the remembered folder only to the same document in the same program and app while it is not attached", () => {
    for (const offer of corpus.offers) expect(localFolderReconnectOfferV1({ bindings: offer.bindings }, offer.identity, new Set(offer.attached === null ? [] : [offer.attached])), offer.name).toEqual(offer.offer);
  });

  it("names folders and words the band exactly as the corpus, in both languages", () => {
    for (const [path, name] of corpus.names) expect(localFolderNameV1(path), path).toBe(name);
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      const expected = corpus.texts[locale];
      expect({
        label: String(shellLabel("ui.sync.reconnect.label")),
        message: String(shellLabel("ui.sync.reconnect.message", { folder: corpus.texts.folder })),
        attach: String(shellLabel("ui.sync.reconnect.attach")),
        forget: String(shellLabel("ui.sync.reconnect.forget")),
      }, locale).toEqual(expected);
    }
  });
});
//#endregion 🧪️Laws

describe("🪪️ a sync attach addresses the program's own document (e2e R2-4)", () => {
  it("attaches a folder or a file's folder under the id the program's store stamps, a hub target under its own, and nothing for a program without a document", () => {
    const identity = { parent_document_id: "board.ports.directed.v1" };
    expect([
      syncAttachDocumentIdV1({ kind: "folder" }, identity),
      syncAttachDocumentIdV1({ kind: "file" }, identity),
      syncAttachDocumentIdV1({ kind: "remote", documentId: null }, identity),
      syncAttachDocumentIdV1({ kind: "remote", documentId: "space-doc" }, identity),
      syncAttachDocumentIdV1({ kind: "folder" }, { parent_document_id: null }),
      syncAttachDocumentIdV1({ kind: "folder" }, { parent_document_id: "" }),
      syncAttachDocumentIdV1({ kind: "folder" }, null),
    ]).toEqual(["board.ports.directed.v1", "board.ports.directed.v1", "board.ports.directed.v1", "space-doc", null, null, null]);
  });
});
