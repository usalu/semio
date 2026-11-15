# Cooling Load — Full Video

Presents the **Cooling load series** (Kühllast, VDI 2078:2015-06 design power rather than annual cooling energy) inside the shared virtual semio UI. Its lesson window zooms in from the five-window canvas and returns to the identical overview; navbar and footer remain visible.

1. Teil 1 — Heizwärmebedarf vs. Kühllast
2. Teil 2 — Interne Wärmegewinne
3. Teil 3 — Transmission & Lüftungslast
4. Teil 4 — Solare Einstrahlung
5. Teil 5 — Systemauslegung
6. Teil 6 — Sommerstrategie

Entry file: [`🐍️.py`](./🐍️.py)

This is the fourth video in the curriculum: Physical Fundamentals → **Lüftung** (`../2_ventilation/`) → Heizung →
**Kühlung** → Energiebilanz. Ventilation is not explained here any more — see the note at the end.

---

## Render

From the workspace root, the repository entry provisions the pinned Python environment:

```sh
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --video cooling --profile preview --jobs 4 --output-root ./videos --no-play
```

Use `--profile preview` (480p15, swap morphs) or `--profile final` (1080p30, full morphs); `--fps` and `--resolution` remain explicit overrides. The renderer constructs each canonical source beat once, selects the declared narration sequences, assembles ordered chapters, and presents the complete lesson inside its virtual window. Video and optional source audio use identical cut boundaries; silent zoom transitions pad both ends. Source-aware complete-clip caches support cancellation and restarting; Manim partial caches stay disabled. Add `--chapter <chapter-identifier>` to render a selected chapter inside the same virtual window. Pass --language en for English chrome; the authored teaching content retains its recorded language. Chapter identifiers are declared in the curriculum JSON; lesson entry files expose `compose(language="de", chapters=[...], **settings)`.

The movie of a preview job is `<output-root>/preview/<language>/<audience>/rendered/Full_Cooling_Demand_<audience>_<language>_480p15.mp4` and that of a final job `<output-root>/final/<language>/<audience>/rendered/Full_Cooling_Demand_<audience>_<language>_1080p30.mp4`, each with `.render.json` and `.export.json` provenance. Private raw segment media and logical chapter clips stay under the chosen output root.

## Chapter Identifiers and Beats

| Chapter identifier | Content |
|-------------|---------|
| `Cooling_01_HeatingVsCooling` | Teil 1 |
| `Cooling_02_InternalGains` | Teil 2 |
| `Cooling_03_TransmissionHumidity` | Teil 3 — opake Transmission, canonical physics thermal-mass excerpts, Brückenbeat `Beat3_LueftungslastImSommer` |
| `Cooling_04_SolarRadiation` | Teil 4 |
| `Cooling_05_Systemauslegung` | Teil 5 — `Beat2_VolumeFlowEquation`, `Beat3_IsolateAirflow` (Last → Luftstrom → Kanal, gegen den Hygieneluftstrom) |
| `Cooling_06_Sommerstrategie` | Teil 6 — `Beat1_LastZuerstSenken`, `Beat2_SorptionsKuehlung`, `Beat3_KomfortStrategie` |

The first segment of `Cooling_01_HeatingVsCooling` is `🗺️roadmap` `Beat1_Fahrplan`, the Fahrplan of the whole video: six cards in chapter order (Heizen und Kühlen `Q̇_H, Q̇_C` → interne Lasten `Q̇_i` → Transmission `Q̇_T, Q̇_V`, with `Q̇_V` taken over from the ventilation video → solare Last `Q̇_S, F_C` → Auslegung `Q̇_C, V̇` → Sommerstrategie `Q_c`, handed to the energy balance). It is declared as data on the shared `🏗️authoring/🗺️roadmap` builder; keep its steps in sync with renamed chapters.

## Notation

The cooling load is the sum $\dot{Q}_\mathrm{C} = \dot{Q}_\mathrm{T} + \dot{Q}_\mathrm{V} + \dot{Q}_\mathrm{S} + \dot{Q}_i$ in W (transmission, ventilation, solar, internal), the same $\dot{Q}$ notation as in the heating video; index `V` always means ventilation, with the latent part `V,lat` and the sensible part `V,sens`. Internal sources are `i,p` (people), `i,e` (equipment), `i,l` (lighting), the latent internal part `i,lat`. Solar load: $\dot{Q}_\mathrm{S} = A \cdot F_\mathrm{F} \cdot F_\mathrm{C} \cdot g_\perp \cdot I_\mathrm{s,max}$ with `F_C` the sun-protection factor of DIN 4108-2:2026-05 and DIN V 4108-6:2003-06 (never `F_sh`, which is the notation of ISO 52016-1:2017). Units appear as `Q in W` or as a value with unit, never as a bare `[W]`; every number of a substitution carries its unit.

References on screen (all strings come from the registry `🏗️authoring/📚️references`): VDI 2078:2015-06 (cooling load, keyword locators; no revision date is announced in any source found), DIN 4108-2:2026-05 (section 8 summer heat protection, sun-protection factor `F_C`; the GModG § 14 still names DIN 4108-2:2013-02), DIN/TS 18599-2:2025-10 (internal heat sources; replaces DIN V 18599-2:2018-09, which GModG § 20 still names), DIN EN 16798-1:2022-03 (annex B, indoor air quality categories), VDI 6022 Blatt 1:2018-01 (hygiene), and in `Beat1_WinterGains` the status chip of DIN V 4108-6:2003-06 (withdrawn, kept for the notation).

## Authoring

Each named chapter owns its scene at `🐍️.py`, optional narration tool at `🔊️speech/🐍️.py`, composition tool at `🏗️composition/🐍️.py`, and timing/trace JSON under their respective semantic directories. The language-neutral curriculum descriptor at `../../🎬️clip/🔣️.json` controls the complete source order. Shared fonts, visual primitives and tool interfaces live under `../../🏗️authoring`.

Narration synthesis requires explicitly configured provider credentials. The complete silent lesson needs no external TTS account. Keep measured narration timing aligned when changing a beat. Set `LAYOUT_CHECK=1` in the environment to enable the authored layout guard while rendering.
