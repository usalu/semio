# Canonical Source Sequences

The render command produces two distinct videos for every selected lesson or chapter:

- **Speaker** (`speaker`) overlays the authored subtitles.
- **Spectator** (`spectator`) shows the same animation and narration without subtitles.

Omit `--audience` to render both. Use `--audience speaker` or `--audience spectator` to select one, or repeat the option to select both explicitly. Output filenames include the audience and language. Every explicit language/audience pair has its own source, trace, excerpt, chapter and presentation cache. Canonical speech and timing are isolated by source identity and language; source scene processes receive SEMIO_TEACHING_LANGUAGE. Timing and available narration audio remain identical.

Python composition requires explicit `language` and `audience`, for example `compose(language="de", audience="spectator")`. Direct Manim scene renders use `SEMIO_TEACHING_AUDIENCE=speaker|spectator` and show Speaker captions when the variable is absent. The language-neutral audience policy lives in `👥️audience/🔣️.json`.

The curriculum JSON is the source of truth for complete lessons, selected chapters and chapter composition tools. Lesson wrappers delegate to the shared runtime; they contain no independent Manim playlists.

A segment selects a complete source scene or an inclusive sequence of its narration clauses:

```json
{
  "script": "🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py",
  "scene": "Beat6_ThermischeMasse",
  "clauses": { "first": "day", "last": "lag" }
}
```

Clause keys must exist in the source scene's ordered `NARRATION`. The renderer records the source's actual subtitle clock, then cuts video and optional audio on the same interval. Reordered or edited narration therefore updates the selection without fixed media timestamps. Invalid, reversed or overlapping boundaries fail explicitly.

Every script/scene pair renders once per run, even when multiple lessons select different parts. Repeated selections share one excerpt. Source, settings and trace fingerprints prevent stale reuse; interrupted outputs never receive completed provenance. Progress and keyboard cancellation cover rendering and selection.

Use `--chapter Heating_05_Solar` with `--video heating` to render that declared chapter. The Python lesson wrapper exposes `compose(chapters=[...], **settings)` through the same execution plan. Omit chapter selection for the complete lesson with its institutional intro.

Thermal mass comes from the earlier physics video: heating selects the storage example and winter application; cooling selects the daily lag and summer application. The later energy-balance video selects the heating calculation's existing system-loss sequence before its distinct detailed system analysis. Their obsolete independent explanations are removed.

Generated media lives in `.🧬semio/🎓️teaching/⚡️cache/videos`, including direct scene imports and text construction. `SEMIO_TEACHING_CACHE_ROOT` relocates the cache; `SEMIO_TEACHING_OUTPUT_ROOT` overrides the video output root. Relative environment paths resolve from the workspace. The renderer also accepts `--output-root` and preserves explicitly configured Manim media directories. Tests use their own output root.

Compact Manim folders (`m/t` for Pango SVGs, `m/x` for TeX, `v` for videos, `p` for partial movies) keep paths short. Narration drafts, clause audio and measured timing live under `speech/<source identity>`; subtitle traces live under `trace/<source identity>/<scene>`. These artifacts are regenerated on demand and are never authored source files.

## Exports

Every export is one explicit render profile (`--profile preview|final`; there is no default and no inheritance from an editor toggle). Beside each exported chapter or lesson master the runtime publishes `<movie>.export.json` (schema `📤️export/🧬️schema/🔣️.schema.json`): effective profile and fingerprint, render settings (size, frame rate, effect budgets, overrides, seed), the summed per-scene `work.json` counters and the profile of every input. A preview job must show zero full-morph work; a final job whose clips declare transitions must show real full-morph work, otherwise the export fails instead of silently keeping a preview substitution. Joins, excerpts, presentations, posters, the course joiner and delivery refuse inputs whose certificate or provenance belongs to another profile.

Posters and thumbnails (`posters/<movie>.poster.png|thumb.png`, referenced from the provenance) come from the profile's output schedule: an integer frame index chosen by a declared policy (`frame`, `time`, `fraction`, or the terminal sample of a declared transition via the shared timeline), extracted with an accurate seek, never the last frame a decoder happened to deliver. `📤️export/🎞️course` joins the current lesson masters of one profile into a chaptered course movie and writes `delivery/delivery.json`.
