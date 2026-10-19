import type { OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";

/** 🎬️ Inline shell paint before Tailwind finishes compiling the play stylesheet. */
export const PLAYGROUND_PLAY_BOOT_INLINE_STYLE =
  "html{color-scheme:light dark}html,body,#root{height:var(--ui-available-height,100dvh);margin:0}body{background-color:#f7f3e3;color:#001117}html.dark body{background-color:#001117;color:#f7f3e3}html:not([data-semio-styled]) body{visibility:hidden}";

/** 📐 Sizes the host to the available viewport before first paint.
 *
 * `100vh` is the screen. On Android Firefox the browser navbar is drawn over that screen and a
 * `100vh` page with `overflow: hidden` is clipped underneath it. `visualViewport.height` is the
 * available height; the same choice as {@link availableViewportHeightPx}. Listeners keep the
 * variable live as the toolbar shows and hides. */
export const PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT = `(function(){var last="";function sync(){var vv=window.visualViewport;var h=vv&&vv.height>0?vv.height:window.innerHeight;if(!(h>0)||!isFinite(h))return;var px=Math.round(h)+"px";if(px===last)return;last=px;document.documentElement.style.setProperty("--ui-available-height",px)}sync();var vv=window.visualViewport;if(vv&&vv.addEventListener){vv.addEventListener("resize",sync);vv.addEventListener("scroll",sync)}window.addEventListener("resize",sync);window.addEventListener("orientationchange",sync)})();`;

/**
 * 🌓️ Synchronous appearance bootstrap for every semio host `🌐️.html` head.
 *
 * Reads the ONE durable document the OS shell actually writes — `localStorage["semio.os.config"]`,
 * whose `preferences["os.config.ui-preferences"]` holds the append-only UI-preference event log — and
 * REPLAYS it, because that lane is event-sourced (`commitUiPreferencesConfigMutation`,
 * `🧑‍🎨engine/🧱️elements/🎚️UiPreferences/🟦️.ts`) and the last `setAppearance` is the only appearance there is.
 * It used to read `ui.chrome.appearance`, a key nothing has written since the shell moved to that
 * document: with appearance = dark, `<html>` got no `.dark` class and `<body>` stayed light until
 * React mounted, which is a guaranteed light flash on every reload of a dark session
 * (`📓️react-i18n-a11y-customization-2026-09-13.md` §3.2, measured there and fixed here).
 *
 * 🔁️ Deliberately a hand-replayed literal and not an import: this is an inline `<script>` in a head
 * that runs before any module is fetched, so it can share no code with the engine. What keeps it
 * honest is `🧪️tests/🧫️fixtures/🌓️appearance-boot.json`, answered by this script in a DOM-less sandbox and
 * independently by the engine's own projection.
 *
 * @see ../../../../../🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/🟦️.ts
 */
export const PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT = `(function(){var d=document.documentElement,m=window.matchMedia("(prefers-color-scheme: dark)");var a=null;try{var c=JSON.parse(localStorage.getItem("semio.os.config")||"null");var l=c&&c.preferences&&c.preferences["os.config.ui-preferences"];var g=l?JSON.parse(l):null;if(g&&g.version===1&&g.events&&g.events.length){for(var i=0;i<g.events.length;i++){var e=g.events[i];if(e&&e.mutation==="setAppearance")a=e.appearance||null}}}catch(e){}var dark=a==="dark"||(a!=="light"&&m.matches);d.classList.toggle("dark",dark);d.dataset.uiAppearance=dark?"dark":"light";d.style.colorScheme=dark?"dark":"light";if(document.body){document.body.style.colorScheme=dark?"dark":"light";document.body.style.backgroundColor=dark?"#001117":"#f7f3e3";document.body.style.color=dark?"#f7f3e3":"#001117";}})();`;

/** 👁️ Reveals the play shell after the linked globals stylesheet finishes loading. */
export const PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT = `(function(){function reveal(){document.documentElement.dataset.semioStyled="ready"}var link=document.getElementById("semio-play-styles");if(link){if(link.sheet)reveal();else link.addEventListener("load",reveal,{once:true})}else{reveal()}setTimeout(reveal,8000)})();`;

/**
 * 🎨️ Synchronous active-theme bootstrap: reapplies the active CUSTOM theme's colors before
 * first paint so a user-authored theme does not flash the semio defaults. Runs after
 * {@link PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT} so its resolved light/dark class wins the appearance
 * choice; this script only overrides colors.
 *
 * Replays the same `semio.os.config` event log the appearance script replays, folding `setTheme` and
 * `setCustomTheme` — the two mutations that actually decide which theme is active — instead of the
 * dead `ui.chrome.theme.snapshot` key. One deliberate consequence: a BUILT-IN theme is not in storage
 * at all (only its id is), so it has nothing to pre-apply and boots on the head's inline defaults, as
 * it did before this fix in every session where the snapshot key was absent, which is all of them.
 */
export const PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = `(function(){try{var c=JSON.parse(localStorage.getItem("semio.os.config")||"null");var l=c&&c.preferences&&c.preferences["os.config.ui-preferences"];var g=l?JSON.parse(l):null;if(!g||g.version!==1||!g.events)return;var id=null,themes={};for(var i=0;i<g.events.length;i++){var e=g.events[i];if(!e)continue;if(e.mutation==="setTheme")id=e.themeId||null;if(e.mutation==="setCustomTheme"){if(e.theme)themes[e.themeId]=e.theme;else delete themes[e.themeId]}}var stored=id?themes[id]:null;var t=stored&&stored.config;if(!t||!t.colors)return;var d=document.documentElement;var dark=d.classList.contains("dark");for(var k in t.colors){d.style.setProperty("--color-"+k.replace(/_/g,"-"),t.colors[k])}if(t.spacing)for(var s in t.spacing){d.style.setProperty("--spacing-"+s.replace(/_/g,"-"),t.spacing[s])}d.dataset.uiTheme=id;var appearance=t.appearances&&t.appearances[dark?"dark":"light"];var chrome=appearance&&appearance.chrome;function resolveSimple(ref){return ref&&ref.token&&t.colors[ref.token]?t.colors[ref.token]:undefined}var base=chrome&&resolveSimple(chrome.base);var fg=chrome&&resolveSimple(chrome.foreground);if(document.body){if(base)document.body.style.backgroundColor=base;if(fg)document.body.style.color=fg}}catch(e){}})();`;

/** 🧬️ Boot-time head tags every semio host document shares (color-scheme inline style + synchronous
 * appearance/theme scripts) — single source both {@link semioHostHtmlString} and
 * {@link playgroundPlayBootHtmlPlugin} inject from, so the generalized host and playground never drift. */
export function semioHostBootHeadTags(): { readonly tag: string; readonly attrs?: Record<string, string>; readonly children?: string; readonly injectTo: "head-prepend" | "head" }[] {
  return [
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT, injectTo: "head-prepend" },
    { tag: "style", children: PLAYGROUND_PLAY_BOOT_INLINE_STYLE, injectTo: "head-prepend" },
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, injectTo: "head-prepend" },
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, injectTo: "head-prepend" },
  ];
}

/** 🎬️ Vite: inject early appearance + theme + stylesheet link into play `🌐️.html` to avoid unstyled flashes — additive tag injection onto each play's own hand-authored `🌐️.html`, sharing its boot-head fragment ({@link semioHostBootHeadTags}) with {@link semioHostHtmlVitePlugin} instead of duplicating the style/script assembly. */
export function playgroundPlayBootHtmlPlugin(): OwnedBuildPlugin {
  return {
    name: "playground-play-boot-html",
    transformIndexHtml: {
      order: "pre",
      handler() {
        return {
          tags: [
            ...semioHostBootHeadTags(),
            { tag: "link", attrs: { rel: "stylesheet", href: "./🎨️.css", id: "semio-play-styles" }, injectTo: "head" },
            { tag: "script", children: PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, injectTo: "head" },
          ],
        };
      },
    },
  };
}
