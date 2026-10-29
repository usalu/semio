/** 📚️ The archive app of the staffage site: language gate, navigation bar, the nested overview and the footer; `mountArchive` renders it on a root element.
 *
 * No language is assumed. The reader's stored choice, else the first language of the browser's list the app speaks, decides; with neither the app asks, in every language at once, before it shows anything else.
 * Everything the reader chooses is kept locally (`💾️persistence`); everything a link must reproduce is in the address (`🗺️routing`). Light, dark and system themes come from the design system's surface chrome.
 *
 * @see ../../../🔨️modules/🥞️levels/🟦️.tsx — the nested layered overview
 * @see ../../../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/📚️app/🟦️.tsx — the course client this shell follows
 * @see ../../../../../🌐️site/🟦️.ts — the entry that mounts it
 */
import { StrictMode, useEffect, useMemo, useRef, useState, type ReactElement } from "react";
import { createPortal } from "react-dom";
import { createRoot } from "react-dom/client";
import { DEFAULT_UI_DRIVER, Navbar, UI_AVAILABLE_HEIGHT, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, bootstrapElementsSurfaceChromeDocument, elementsSurfaceDeviceForMatches, navbarFillItem, useElementsSurfaceChrome, useMediaQuery, type NavbarItem } from "@semio-tech/ui-react/chrome";
import { VIEWS } from "@teaching/staffage";
import "../../../🎨️.css";
import { ArchiveProvider, useArchive, useLocal } from "../../../🔨️modules/🧭️context/🟦️.tsx";
import { LANGUAGE_NAMES, OFFERED, inEveryLanguage, resolveLanguage, translator } from "../../../🔨️modules/🌐️i18n/🟦️.ts";
import { Button, ButtonLink, Segmented, cn } from "../../../🔨️modules/🪟️chrome/🟦️.tsx";
import { RootLevel } from "../../../🔨️modules/🏠️overview/🟦️.tsx";
import { ArchiveStore, STORAGE_VERSION, browserStorage, useArchiveState, type StorageArea, type Theme } from "../../../🔨️modules/💾️persistence/🟦️.ts";
import { CatalogStore, bundledCatalog, type CatalogSource } from "../../../🔨️modules/📚️catalog/🟦️.ts";
import { formatAddress, parentRoute, recallParams, useAddress } from "../../../🔨️modules/🗺️routing/🟦️.ts";

/** ⚙️ What a host may replace: storage (tests), the browser's language list, the catalog source. */
export type ArchiveOptions = { readonly storage?: StorageArea; readonly durable?: boolean; readonly languages?: readonly string[]; readonly catalog?: CatalogSource };

const ROUTE_TITLES = { overview: "nav.overview", all: "all.title", category: "nav.category", asset: "nav.asset", collections: "collections.title", compare: "compare.title", playground: "playground.title" } as const;

function LanguageGate(props: { readonly onChoose: (language: (typeof OFFERED)[number]) => void }): ReactElement {
  return (
    <div className="staffage-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }}>
      <main id="staffage-main" className="relative flex min-h-0 flex-1 flex-col items-center justify-center gap-double overflow-auto p-4">
        <div className="flex w-full max-w-xl flex-col gap-double border border-border bg-panel p-4">
          {inEveryLanguage("brand.title").map(({ language, text }) => (
            <h1 key={language} lang={language} className="m-0 font-serif text-3xl leading-tight">
              {text}
            </h1>
          ))}
          <ul role="list" className="m-0 flex list-none flex-col gap-double p-0">
            {OFFERED.map((language) => (
              <li key={language} lang={language} className="flex flex-wrap items-center gap-double">
                <Button variant="primary" onClick={() => props.onChoose(language)}>
                  {LANGUAGE_NAMES[language]}
                </Button>
                <span className="text-sm">{translator(language)("language.invite")}</span>
              </li>
            ))}
          </ul>
        </div>
      </main>
    </div>
  );
}

const THEMES: readonly Theme[] = ["system", "light", "dark"];

function SettingsMenu(): ReactElement {
  const { t, store } = useArchive();
  const { preferences } = useLocal();
  const [open, setOpen] = useState(false);
  const [confirm, setConfirm] = useState(false);
  const root = useRef<HTMLDivElement>(null);
  const panel = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!open) return;
    const onKey = (event: KeyboardEvent): void => {
      if (event.key === "Escape") {
        event.stopPropagation();
        setOpen(false);
        root.current?.querySelector<HTMLElement>("button[aria-expanded]")?.focus();
      }
    };
    const onPointer = (event: PointerEvent): void => {
      if (root.current !== null && panel.current !== null && event.target instanceof Node && !root.current.contains(event.target) && !panel.current.contains(event.target)) setOpen(false);
    };
    const onFocus = (event: FocusEvent): void => {
      if (root.current !== null && panel.current !== null && event.target instanceof Node && !root.current.contains(event.target) && !panel.current.contains(event.target)) setOpen(false);
    };
    panel.current?.querySelector<HTMLElement>("a, button, input, select")?.focus();
    window.addEventListener("keydown", onKey, true);
    window.addEventListener("pointerdown", onPointer);
    window.addEventListener("focusin", onFocus);
    return () => {
      window.removeEventListener("focusin", onFocus);
      window.removeEventListener("keydown", onKey, true);
      window.removeEventListener("pointerdown", onPointer);
    };
  }, [open]);
  const patch = (next: Partial<typeof preferences>): void => store.dispatch({ type: "preferences-changed", patch: next });
  return (
    <div ref={root} className="relative">
      <Button variant="quiet" icon="settings" aria-expanded={open} aria-haspopup="true" aria-controls={open ? "staffage-settings" : undefined} aria-label={t("settings.title")} title={t("settings.title")} onClick={() => setOpen(!open)}>
        <span className="hidden lg:inline">{t("settings.title")}</span>
      </Button>
      {open ? createPortal(
        <div id="staffage-settings" ref={panel} role="group" aria-label={t("settings.title")} className="staffage-app fixed right-double top-[calc(var(--size-workbench)+0.5rem)] z-50 flex max-h-[85vh] w-72 max-w-[90vw] flex-col gap-double overflow-auto border border-border bg-panel p-double text-foreground">
          <nav aria-label={t("nav.label")} className="flex flex-col gap-single md:hidden">
            {[{ href: "#/all", label: t("all.title") }, { href: "#/collections", label: t("collections.title") }, { href: "#/compare", label: t("compare.title") }, { href: "#/playground", label: t("playground.title") }].map((link) => (
              <ButtonLink key={link.href} href={link.href} variant="quiet" className="justify-start">{link.label}</ButtonLink>
            ))}
          </nav>
          <Segmented className="sm:hidden" label={t("settings.theme")} value={preferences.theme} onChange={(theme) => patch({ theme })} options={THEMES.map((theme) => ({ value: theme, label: t(`settings.theme.${theme}`) }))} />
          <Segmented label={t("settings.textSize")} value={preferences.textSize} onChange={(textSize) => patch({ textSize })} options={[{ value: "normal", label: t("settings.textSize.normal") }, { value: "large", label: t("settings.textSize.large") }]} />
          <Segmented label={t("settings.motion")} value={preferences.motion} onChange={(motion) => patch({ motion })} options={[{ value: "system", label: t("settings.motion.system") }, { value: "reduced", label: t("settings.motion.reduced") }, { value: "full", label: t("settings.motion.full") }]} />
          <Segmented label={t("settings.playful")} value={preferences.playful ? "on" : "off"} onChange={(value) => patch({ playful: value === "on" })} options={[{ value: "on", label: t("settings.on") }, { value: "off", label: t("settings.off") }]} />
          <div className="flex flex-col gap-single border-t border-border pt-double">
            <p className="m-0 text-xs text-muted-foreground">{t("settings.clearBody")}</p>
            {confirm ? (
              <div className="flex gap-single">
                <Button variant="primary" onClick={() => { store.dispatch({ type: "everything-cleared" }); setConfirm(false); }}>{t("settings.clearConfirm")}</Button>
                <Button onClick={() => setConfirm(false)}>{t("action.cancel")}</Button>
              </div>
            ) : (
              <Button icon="trash-2" onClick={() => setConfirm(true)}>{t("settings.clear")}</Button>
            )}
          </div>
        </div>,
        document.body,
      ) : null}
    </div>
  );
}

function Navigation(): ReactElement {
  const { t, store, language } = useArchive();
  const { preferences } = useLocal();
  const { route, navigate } = useAddress();
  const [query, setQuery] = useState("");
  const search = useRef<HTMLInputElement>(null);
  const up = parentRoute(route);
  useEffect(() => {
    const onKey = (event: KeyboardEvent): void => {
      if (event.key !== "/" || event.ctrlKey || event.metaKey || event.altKey) return;
      if (event.target instanceof Element && event.target.closest("input, textarea, select, [contenteditable]") !== null) return;
      event.preventDefault();
      search.current?.focus();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);
  const links = [
    { href: formatAddress({ page: "all" }), label: t("all.title"), icon: "layout-grid", active: route.page === "all" },
    { href: formatAddress({ page: "collections", collection: null }), label: t("collections.title"), icon: "folder", active: route.page === "collections" },
    { href: formatAddress({ page: "compare" }), label: t("compare.title"), icon: "combine", active: route.page === "compare" },
    { href: formatAddress({ page: "playground" }), label: t("playground.title"), icon: "move", active: route.page === "playground" },
  ] as const;
  const items: NavbarItem[] = [
    {
      key: "brand",
      content: (
        <a href="#/" aria-label={t("nav.home")} aria-current={route.page === "overview" ? "page" : undefined} className="inline-flex min-h-6 items-center gap-single px-single text-sm font-semibold text-foreground no-underline">
          <span aria-hidden="true" className="font-serif text-lg leading-none">⌂</span>
          <span className="hidden sm:inline">{t("brand.short")}</span>
        </a>
      ),
    },
    { key: "up", content: up === null ? <span /> : <Button variant="quiet" icon="arrow-left" aria-label={t("nav.up")} title={t("nav.up")} onClick={() => navigate(up, recallParams(up))}><span className="hidden md:inline">{t("nav.up")}</span></Button> },
    ...links.map((link): NavbarItem => ({ key: link.href, className: "hidden md:flex", content: <ButtonLink href={link.href} icon={link.icon} variant="quiet" aria-label={link.label} className={cn(link.active && "underline underline-offset-4")}>{link.label}</ButtonLink> })),
    navbarFillItem("fill"),
    {
      key: "search",
      className: "min-w-0 shrink",
      content: (
        <form role="search" aria-label={t("nav.search")} className="flex min-w-0 items-center gap-single" onSubmit={(event) => { event.preventDefault(); navigate({ page: "all" }, new URLSearchParams(query === "" ? "" : { q: query })); }}>
          <input ref={search} type="search" value={query} onChange={(event) => setQuery(event.target.value)} aria-label={t("nav.search")} placeholder={t("nav.searchPlaceholder")} className="min-h-6 w-28 min-w-0 border border-border bg-panel px-double py-single text-xs text-foreground sm:w-44" />
          <Button type="submit" variant="quiet" icon="search" aria-label={t("nav.searchSubmit")} title={t("nav.searchSubmit")} />
        </form>
      ),
    },
    { key: "language", content: <Segmented hideLabel label={t("settings.language")} value={language} onChange={(chosen) => store.dispatch({ type: "preferences-changed", patch: { language: chosen } })} options={OFFERED.map((option) => ({ value: option, label: option.toUpperCase(), title: LANGUAGE_NAMES[option], name: LANGUAGE_NAMES[option], lang: option }))} /> },
    { key: "theme", className: "hidden sm:flex", content: <Segmented hideLabel label={t("settings.theme")} value={preferences.theme} onChange={(theme) => store.dispatch({ type: "preferences-changed", patch: { theme } })} options={THEMES.map((theme) => ({ value: theme, label: <span className="sr-only">{t(`settings.theme.${theme}`)}</span>, title: t(`settings.theme.${theme}`), icon: theme === "system" ? "monitor" : theme === "light" ? "sun" : "moon" }))} /> },
    { key: "settings", content: <SettingsMenu /> },
  ];
  return <Navbar label={t("nav.label")} items={items} showFullscreenToggle={false} />;
}

function Shell(): ReactElement {
  const { t, store, language, reducedMotion } = useArchive();
  const { route } = useAddress();
  const { preferences } = useLocal();
  const main = useRef<HTMLElement>(null);
  const seen = useRef(route.page);
  useEffect(() => {
    document.title = `${t(ROUTE_TITLES[route.page])} · ${t("brand.title")}`;
  }, [route.page, t]);
  useEffect(() => {
    if (seen.current !== route.page) main.current?.focus({ preventScroll: true });
    seen.current = route.page;
  }, [route.page]);
  return (
    <div className="staffage-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={language} data-motion={reducedMotion ? "reduced" : "full"} data-playful={preferences.playful ? "on" : "off"}>
      <a href="#staffage-main" className="sr-only focus:not-sr-only focus:absolute focus:left-double focus:top-double focus:z-50 focus:bg-background focus:p-double focus:text-sm focus:text-foreground" onClick={(event) => { event.preventDefault(); main.current?.focus(); }}>
        {t("nav.skip")}
      </a>
      <header className="relative z-40 shrink-0">
        <Navigation />
      </header>
      <main id="staffage-main" ref={main} tabIndex={-1} className="flex min-h-0 flex-1 flex-col outline-none">
        {route.page === "overview" ? <h1 className="sr-only">{t("brand.title")}</h1> : null}
        <div className="relative min-h-0 flex-1">
          <RootLevel />
        </div>
      </main>
      <footer className="flex shrink-0 flex-wrap items-center justify-between gap-double border-t border-border px-double py-single text-[0.65rem] text-muted-foreground">
        <span>{t("footer.text")}</span>
        {store.mode === "volatile" ? <span role="status">{t("footer.volatile")}</span> : <span>{t("footer.local", { version: STORAGE_VERSION })}</span>}
      </footer>
    </div>
  );
}

function Client(props: { readonly store: ArchiveStore; readonly catalog: CatalogStore; readonly languages: readonly string[] }): ReactElement {
  const { store, catalog, languages } = props;
  const state = useArchiveState(store);
  const { language } = resolveLanguage(state.preferences.language, languages);
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  useElementsSurfaceChrome({ appearance: state.preferences.theme, device: elementsSurfaceDeviceForMatches({ mobile, tablet }), driver: DEFAULT_UI_DRIVER, browserDefaults: "native" });
  useEffect(() => {
    document.documentElement.style.fontSize = state.preferences.textSize === "large" ? "112.5%" : "";
  }, [state.preferences.textSize]);
  useEffect(() => {
    if (language === undefined) document.documentElement.removeAttribute("lang");
    else document.documentElement.lang = language;
  }, [language]);
  useEffect(() => {
    void catalog.loadManifest();
  }, [catalog]);
  useEffect(() => store.watch(window), [store]);
  const t = useMemo(() => (language === undefined ? undefined : translator(language)), [language]);
  if (language === undefined || t === undefined) {
    document.title = inEveryLanguage("brand.title").map((entry) => entry.text).join(" · ");
    return <LanguageGate onChoose={(chosen) => store.dispatch({ type: "preferences-changed", patch: { language: chosen } })} />;
  }
  return (
    <ArchiveProvider language={language} t={t} catalog={catalog} store={store}>
      <Shell />
    </ArchiveProvider>
  );
}

/** 🚀️ Renders the archive on `root` and returns the function that unmounts it. */
export function mountArchive(root: HTMLElement, options: ArchiveOptions = {}): () => void {
  const host = options.storage === undefined ? browserStorage() : { area: options.storage, durable: options.durable ?? true };
  const store = new ArchiveStore(host.area, VIEWS, host.durable);
  const catalog = new CatalogStore(options.catalog ?? bundledCatalog);
  bootstrapElementsSurfaceChromeDocument(store.getSnapshot().preferences.theme);
  const languages = options.languages ?? (typeof navigator === "undefined" ? [] : navigator.languages);
  const reactRoot = createRoot(root);
  reactRoot.render(
    <StrictMode>
      <Client store={store} catalog={catalog} languages={languages} />
    </StrictMode>,
  );
  return () => reactRoot.unmount();
}
