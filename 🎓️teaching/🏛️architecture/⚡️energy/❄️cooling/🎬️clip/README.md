# Cooling Load — Full Video

Plays the **Cooling load series** (Kühllast, VDI 2078 design power — not annual
Kühlenergiebedarf) in curriculum order, prepended with the series intro card
``Demo_Intro_Kuehllast`` from ``🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py``:

0. Intro — Kühllast
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
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video cooling --quality l --jobs 4 --output-root ./videos --no-play
```

Use `--quality m` for 720p30 or `--quality h` for 1080p60. The renderer constructs every authored beat independently, assembles ordered chapters, and joins the institutional intro and all chapters into the complete silent lesson. Source-aware complete-clip caches support cancellation and restarting; Manim partial caches stay disabled.

The final low-quality movie is `<output-root>/rendered/Full_Cooling_Demand_480p15.mp4`. Private raw segment media and logical chapter clips stay under the chosen output root.

## Chapter Scenes and Beats

| Scene class | Content |
|-------------|---------|
| `Cooling_01_HeatingVsCooling` | Teil 1 |
| `Cooling_02_InternalGains` | Teil 2 |
| `Cooling_03_TransmissionHumidity` | Teil 3 — opake Transmission, Time Lag, Brückenbeat `Beat3_LueftungslastImSommer` |
| `Cooling_04_SolarRadiation` | Teil 4 |
| `Cooling_05_Systemauslegung` | Teil 5 — `Beat2_VolumeFlowEquation`, `Beat3_IsolateAirflow` (Last → Luftstrom → Kanal, gegen den Hygieneluftstrom) |
| `Cooling_06_Sommerstrategie` | Teil 6 — `Beat1_LastZuerstSenken`, `Beat2_SorptionsKuehlung`, `Beat3_KomfortStrategie` |

## Authoring

Each named chapter owns its scene at `🐍️.py`, optional narration tool at `🔊️speech/🐍️.py`, composition tool at `🏗️composition/🐍️.py`, and timing/trace JSON under their respective semantic directories. The language-neutral curriculum descriptor at `../../🎬️clip/🔣️.json` controls the complete source order. Shared fonts, visual primitives and tool interfaces live under `../../🏗️authoring`.

Narration synthesis requires explicitly configured provider credentials. The complete silent lesson needs no external TTS account. Keep measured narration timing aligned when changing a beat. Set `LAYOUT_CHECK=1` in the environment to enable the authored layout guard while rendering.
