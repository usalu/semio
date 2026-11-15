# Heating Demand — Full Video

Presents the **Heating demand series** inside the shared virtual semio UI. The navbar and footer stay visible while its lesson window zooms in from the five-window canvas and returns to the identical overview.

1. Modul 1 — Grundlagen der Bauphysik
2. Modul 2 — Transmission
3. Modul 3 — Lüftung im Winter (bridge beat to the Lüftung video)
4. Modul 4 — Interne Wärmegewinne
5. Modul 5 — Solarer Wärmegewinn
6. Final calculation — Heizwärmebedarf

Ventilation is explained once, season-neutral, in the second video of the curriculum
(`🎓️teaching/🏛️architecture/⚡️energy/2_ventilation/`). Modul 3 is only the short winter bridge: it recalls
`V̇ = V · n` (Lüftung Kapitel 2), `c_Luft` and `Δθ = 20 K` (Kapitel 3), shows the ventilation load `Q̇_V ≈ 1500 W` next to
the transmission load `Q̇_T ≈ 2300 W` of Modul 2 (both are the heat flow `Φ`, written as loads at the room; values come from the shared running example), applies `(1 − η_WRG)` (Kapitel 5) and links
back with chips.

Notation of Modul 1 to 3: `Φ` is the generic heat flow (`Φ_k`, `Φ_c`, `Φ_r` for conduction, convection, radiation; `Φ = U · A · Δθ`), `R_T = R_si + Σ d/λ + R_se` the total resistance and `U = 1/R_T` in W/(m²K), introduced once in Modul 1 (Beat 8) and recalled in Modul 2; `ψ` in W/(mK) is the linear thermal bridge coefficient. Units are never bracketed inside equations; compound units are set with a fraction bar. Modul 1 Beat 1 names the second law of thermodynamics, Beat 4 shows the spectrum strip (visible light vs infrared) linking thermal radiation to electromagnetic waves.

Entry file: [`🐍️.py`](./🐍️.py)

---

## Render

From the workspace root, the repository entry provisions the pinned Python environment:

```sh
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --video heating --profile preview --jobs 4 --output-root ./videos --no-play
```

Use `--profile preview` (480p15, swap morphs) or `--profile final` (1080p30, full morphs); `--fps` and `--resolution` remain explicit overrides. The renderer constructs each canonical source beat once, selects the declared narration sequences, assembles ordered chapters, and presents the complete lesson inside its virtual window. Video and optional source audio use identical cut boundaries; silent zoom transitions pad both ends. Source-aware complete-clip caches support cancellation and restarting; Manim partial caches stay disabled. Add `--chapter <chapter-identifier>` to render a selected chapter inside the same virtual window. Pass --language en for English chrome; the authored teaching content retains its recorded language. Chapter identifiers are declared in the curriculum JSON; lesson entry files expose `compose(language="de", chapters=[...], **settings)`.

The movie of a preview job is `<output-root>/preview/<language>/<audience>/rendered/Full_Heating_Demand_<audience>_<language>_480p15.mp4` and that of a final job `<output-root>/final/<language>/<audience>/rendered/Full_Heating_Demand_<audience>_<language>_1080p30.mp4`, each with `.render.json` and `.export.json` provenance. Private raw segment media and logical chapter clips stay under the chosen output root.

## Chapter Identifiers and Beats

| Chapter identifier | Content |
|-------------|---------|
| `Heating_01_Introduction` | Modul 1 |
| `Heating_02_Conduction` | Modul 2 |
| `Heating_03_Convection` | Modul 3 — Lüftung im Winter |
| `Heating_04_InternalGains` | Modul 4 |
| `Heating_05_Solar` | Modul 5, with canonical physics thermal-mass excerpts |
| `Heating_06_FinalCalculation` | Final calculation |

The first segment of `Heating_01_Introduction` is `🗺️roadmap` `Beat1_Fahrplan`, the Fahrplan of the whole video: six cards in chapter order (Grundlagen `Φ, Δθ` → Wärmeleitung `U, λ, ψ` → Lüftung `Q̇_V`, taken over from the ventilation video → interne Gewinne `Q̇_i` → solare Gewinne `Q̇_S, g⊥, F_C` → Bilanz `Q_h`, handed to the energy balance). It is declared as data on the shared `🏗️authoring/🗺️roadmap` builder; keep its steps in sync with renamed chapters.

## Authoring

Each named chapter owns its scene at `🐍️.py`, optional narration tool at `🔊️speech/🐍️.py`, composition tool at `🏗️composition/🐍️.py`, and timing/trace JSON under their respective semantic directories. The language-neutral curriculum descriptor at `../../🎬️clip/🔣️.json` controls the complete source order. Shared fonts, visual primitives and tool interfaces live under `../../🏗️authoring`.

Narration synthesis requires explicitly configured provider credentials. The complete silent lesson needs no external TTS account. Keep measured narration timing aligned when changing a beat. Set `LAYOUT_CHECK=1` in the environment to enable the authored layout guard while rendering.
