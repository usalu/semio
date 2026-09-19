"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SEMIO_APP_ROLE_ENV = exports.SEMIO_DEFAULT_EXAMPLE_ENV = exports.SEMIO_BRAND_ENV = exports.SEMIO_LOCKED_APPEARANCE_ENV = exports.SEMIO_LOCKED_THEME_ENV = exports.SEMIO_LOCKED_TERMINOLOGY_ENV = exports.SEMIO_LOCKED_LOCALE_ENV = exports.PLAYGROUND_LOCKED_EXAMPLE_ENV = void 0;
exports.frameworkOsLockedPrefsEnv = frameworkOsLockedPrefsEnv;
var ____json_1 = require("./\uD83D\uDD23\uFE0F.json");
exports.PLAYGROUND_LOCKED_EXAMPLE_ENV = "PLAYGROUND_LOCKED_EXAMPLE_ID";
exports.SEMIO_LOCKED_LOCALE_ENV = "SEMIO_LOCKED_LOCALE";
exports.SEMIO_LOCKED_TERMINOLOGY_ENV = "SEMIO_LOCKED_TERMINOLOGY";
exports.SEMIO_LOCKED_THEME_ENV = "SEMIO_LOCKED_THEME";
exports.SEMIO_LOCKED_APPEARANCE_ENV = "SEMIO_LOCKED_APPEARANCE";
exports.SEMIO_BRAND_ENV = "SEMIO_BRAND";
exports.SEMIO_DEFAULT_EXAMPLE_ENV = "SEMIO_DEFAULT_EXAMPLE";
/** 👁️✏️ Boot-time surface role. The shells read `VITE_SEMIO_APP_ROLE` (`🐚️Shell/🟦️.tsx`'s
 * `resolveBootAppRole`, `🧑‍💻dev/🟦️.ts`'s `appRole`), so without this projection the ONLY way to open a
 * plugin's viewer surface was to hand-export the `VITE_`-prefixed name before starting the dev server —
 * a launch row could not reach it, and no artifact's read-only surface was reachable from any launch
 * entry (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 2). */
exports.SEMIO_APP_ROLE_ENV = "SEMIO_APP_ROLE";
/** 🔒️ Projects explicitly set shell preferences into Vite's public environment. */
function frameworkOsLockedPrefsEnv(env) {
    if (env === void 0) { env = process.env; }
    return Object.fromEntries(Object.entries(____json_1.default).flatMap(function (_a) {
        var _b;
        var source = _a[0], target = _a[1];
        var value = (_b = env[source]) === null || _b === void 0 ? void 0 : _b.trim();
        return value ? [[target, value]] : [];
    }));
}
