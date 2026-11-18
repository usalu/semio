// #region 🧲️Header
/** 🎡️ semio-tech play — composition root of the landing ({@link PlayLanding}): binds the host ports, boots the user's locale and hosts each app's live shell on its own storage namespace. */
// #endregion 🧲️Header

import { mountUiRoot, useUiMemo as useMemo, useUiState as useState } from "@semio-tech/ui-react/runtime";
import {
  LanguageChoice,
  bootstrapElementsSurfaceChromeDocument,
  initUiLocaleSync,
  readStoredUiChromeAppearance,
  readStoredUiChromeLocale,
  type LanguageChoiceOption,
} from "@semio-tech/ui-react";
import { resolvePlaygroundBoot, type ShellLocale } from "@semio-tech/framework";
import { composeSpecificOsCatalogV1 } from "../../✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts";
import { FrameworkOsShell, resolveShellLocks, resolveShellDefaults } from "@semio-tech/framework-renderer-react";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-2d";
import { createFrameworkOsHostPortsV1 } from "@semio-tech/framework-os-dev/host-ports";
import { PlayLanding, playStorage } from "./🏠️play-landing.tsx";
import { resolvePlayLocale, type PlayPaneSpec } from "./🪧️brand.ts";
import "./🎨️globals.css";

/** 🪝️ The one set of ports every pane's shell reads: the page's composition root binds them once. */
const FRAMEWORK_OS_HOST_PORTS = createFrameworkOsHostPortsV1();

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(playStorage));
const requestedLocale = resolvePlayLocale(readStoredUiChromeLocale(playStorage), typeof navigator === "undefined" ? [] : navigator.languages);
if (requestedLocale) initUiLocaleSync(requestedLocale);

//#region 🎡️PlayPages
/** 🎡️ One app's live shell on its own storage namespace; its own tour only while it is opened. */
function PlayShell({ pane, opened, locale }: { readonly pane: PlayPaneSpec; readonly opened: boolean; readonly locale: ShellLocale }) {
  const catalog = useMemo(() => composeSpecificOsCatalogV1(window.location.href, { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} }).catalog, []);
  const boot = useMemo(() => resolvePlaygroundBoot(catalog, pane.variant), [catalog, pane.variant]);
  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand]);
  const defaults = useMemo(() => resolveShellDefaults(pane.brand, undefined), [pane.brand]);
  return (
    <FrameworkOsShell catalog={catalog}
      initialLocale={locale}
      pluginFilter={pane.variant}
      plugins={boot.plugins}
      surfaceSessionFactories={PUZZLE_BOARD_SESSION_FACTORIES}
      hostPorts={FRAMEWORK_OS_HOST_PORTS}
      appId={boot.defaultAppId}
      locks={locks}
      defaults={defaults}
      brand={pane.brand}
      shellId={pane.id}
      storageNamespace={pane.id}
      suppressAutoIntroduction={!opened}
    />
  );
}

const renderPlayPane = (pane: PlayPaneSpec, opened: boolean, locale: ShellLocale) => <PlayShell pane={pane} opened={opened} locale={locale} />;
//#endregion 🎡️PlayPages

const PLAY_LANGUAGE_OPTIONS: readonly LanguageChoiceOption<ShellLocale>[] = [{value:"en",endonym:"English"},{value:"de",endonym:"Deutsch"}];

/** 🗣️ An absent supported user source renders an accessible bilingual choice before any app boots. */
function PlayBoot() {
  const [locale, setLocale] = useState<ShellLocale | null>(requestedLocale);
  if (locale) return <PlayLanding locale={locale} renderPane={renderPlayPane}/>;
  const choose = (language: ShellLocale) => {initUiLocaleSync(language);setLocale(language);};
  return <main className="flex min-h-screen flex-col items-center justify-center gap-single" data-slot="play-language-choice" aria-labelledby="play-language-title">
    <h1 id="play-language-title"><span lang="en">Choose your language</span> / <span lang="de">Sprache auswählen</span></h1>
    <LanguageChoice label="Language · Sprache" options={PLAY_LANGUAGE_OPTIONS} onChoose={choose}/>
  </main>;
}

mountUiRoot(document.getElementById("root")!, <PlayBoot />);
