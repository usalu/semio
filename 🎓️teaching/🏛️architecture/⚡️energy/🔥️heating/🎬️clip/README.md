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
`V · n` (Lüftung Kapitel 2), `c_Luft` and `Δθ = 20 K` (Kapitel 3), shows `Φ_V ≈ 1500 W` next to
the transmission loss `Φ_T ≈ 2300 W` of Modul 2, applies `(1 − η_WRG)` (Kapitel 5) and links
back with chips.

Entry file: [`🐍️.py`](./🐍️.py)

---

## Render

From the workspace root, the repository entry provisions the pinned Python environment:

```sh
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --video heating --quality l --jobs 4 --output-root ./videos --no-play
```

Use `--quality m` for 720p30 or `--quality h` for 1080p60. The renderer constructs each canonical source beat once, selects the declared narration sequences, assembles ordered chapters, and presents the complete lesson inside its virtual window. Video and optional source audio use identical cut boundaries; silent zoom transitions pad both ends. Source-aware complete-clip caches support cancellation and restarting; Manim partial caches stay disabled. Add `--chapter <chapter-identifier>` to render a selected chapter inside the same virtual window. Pass --language en for English chrome; the authored teaching content retains its recorded language. Chapter identifiers are declared in the curriculum JSON; lesson entry files expose `compose(language="de", chapters=[...], **settings)`.

The final low-quality movie is `<output-root>/rendered/Full_Heating_Demand_de_480p15.mp4`. Private raw segment media and logical chapter clips stay under the chosen output root.

## Chapter Identifiers and Beats

| Chapter identifier | Content |
|-------------|---------|
| `Heating_01_Introduction` | Modul 1 |
| `Heating_02_Conduction` | Modul 2 |
| `Heating_03_Convection` | Modul 3 — Lüftung im Winter |
| `Heating_04_InternalGains` | Modul 4 |
| `Heating_05_Solar` | Modul 5, with canonical physics thermal-mass excerpts |
| `Heating_06_FinalCalculation` | Final calculation |

## Authoring

Each named chapter owns its scene at `🐍️.py`, optional narration tool at `🔊️speech/🐍️.py`, composition tool at `🏗️composition/🐍️.py`, and timing/trace JSON under their respective semantic directories. The language-neutral curriculum descriptor at `../../🎬️clip/🔣️.json` controls the complete source order. Shared fonts, visual primitives and tool interfaces live under `../../🏗️authoring`.

Narration synthesis requires explicitly configured provider credentials. The complete silent lesson needs no external TTS account. Keep measured narration timing aligned when changing a beat. Set `LAYOUT_CHECK=1` in the environment to enable the authored layout guard while rendering.
