"""📏 The running example of the energy course: one house, one set of numbers, every unit in the name.

The raw numbers live in ``🔣️.json``; this module derives the powers every clip quotes, so heating, cooling and ventilation cannot drift apart.
"""

from __future__ import annotations

import importlib
import json
import numpy as np
from pathlib import Path

_DATA = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))

RHO_AIR_KG_M3 = float(_DATA["air"]["densityKgM3"])
CP_AIR_J_KG_K = float(_DATA["air"]["specificHeatJKgK"])
C_LUFT_WH_M3_K = float(_DATA["air"]["volumetricHeatCapacityWhM3K"])
R_LAT_KJ_KG = float(_DATA["air"]["latentHeatKJKg"])

V_NET_M3 = float(_DATA["ventilation"]["netVolumeM3"])
N_AIR_PER_H = float(_DATA["ventilation"]["airChangeRatePerH"])
ETA_WRG = float(_DATA["ventilation"]["heatRecoveryEfficiency"])
V_DOT_M3_H = V_NET_M3 * N_AIR_PER_H
V_DOT_M3_S = V_DOT_M3_H / 3600.0

THETA_I_WINTER_C = float(_DATA["winter"]["indoorC"])
THETA_E_WINTER_C = float(_DATA["winter"]["outdoorC"])
D_THETA_WINTER_K = THETA_I_WINTER_C - THETA_E_WINTER_C
THETA_E_DESIGN_C = float(_DATA["design"]["outdoorC"])
D_THETA_DESIGN_K = THETA_I_WINTER_C - THETA_E_DESIGN_C
DESIGN_REFERENCE_ID = str(_DATA["design"]["referenceId"])

G_PERP = float(_DATA["glazing"]["gPerp"])
G_SHARE_DIRECT = float(_DATA["glazing"]["directTransmission"])
G_SHARE_SECONDARY_IN = float(_DATA["glazing"]["secondaryInward"])
G_SHARE_REFLECTED = float(_DATA["glazing"]["reflected"])
G_SHARE_SECONDARY_OUT = float(_DATA["glazing"]["secondaryOutward"])
F_F = float(_DATA["glazing"]["frameFactorFF"])
F_W = float(_DATA["glazing"]["incidenceFW"])
F_S_WINTER = float(_DATA["glazing"]["shadingFSWinter"])
F_C_BLIND = float(_DATA["glazing"]["sunProtectionFCBlind"])
F_C_NONE = float(_DATA["glazing"]["sunProtectionFCNone"])
WINDOW_W_M = float(_DATA["glazing"]["windowWidthM"])
WINDOW_H_M = float(_DATA["glazing"]["windowHeightM"])
WINDOW_A_M2 = WINDOW_W_M * WINDOW_H_M
I_S_WINTER_W_M2 = float(_DATA["glazing"]["winterIrradianceWM2"])
I_S_CLEAR_W_M2 = float(_DATA["glazing"]["clearSkyIrradianceWM2"])


THETA_I_SUMMER_C = float(_DATA["summer"]["indoorC"])
THETA_E_SUMMER_C = float(_DATA["summer"]["outdoorC"])
D_THETA_SUMMER_K = THETA_E_SUMMER_C - THETA_I_SUMMER_C
X_OUT_WINTER_G_KG = float(_DATA["winter"]["outdoorHumidityGKg"])
X_IN_WINTER_G_KG = float(_DATA["winter"]["indoorHumidityGKg"])
D_X_WINTER_G_KG = X_IN_WINTER_G_KG - X_OUT_WINTER_G_KG
X_OUT_SUMMER_G_KG =float(_DATA["summer"]["outdoorHumidityGKg"])
X_IN_SUMMER_G_KG = float(_DATA["summer"]["indoorHumidityGKg"])
D_X_G_KG = X_OUT_SUMMER_G_KG - X_IN_SUMMER_G_KG

ENVELOPE = [(item["nameDe"], float(item["areaM2"]), float(item["uValueWM2K"]), float(item["deltaK"])) for item in _DATA["envelope"]]
ENVELOPE_EN = {item["nameDe"]: item["nameEn"] for item in _DATA["envelope"]}
UA_OPAQUE_W_K = sum(item["areaM2"] * item["uValueWM2K"] for item in _DATA["envelope"] if item["id"] in ("roof", "wall"))

SEASON_DAYS = int(_DATA["season"]["heatingDays"])
GT_K_D_PER_A = float(_DATA["season"]["degreeDaysKdPerA"])
THETA_HG_C = float(_DATA["season"]["heatingLimitC"])
THETA_E_SEASON_MEAN_C = THETA_I_WINTER_C - GT_K_D_PER_A / SEASON_DAYS
Q_SOL_KWH_PER_A = float(_DATA["gains"]["solarKWhPerA"])
Q_INT_KWH_PER_A = float(_DATA["gains"]["internalKWhPerA"])

PHI_T_W = round(sum(area * u * delta for _name, area, u, delta in ENVELOPE), -2)
PHI_V_W = round(V_DOT_M3_H * C_LUFT_WH_M3_K * D_THETA_WINTER_K, -2)
PHI_V_WRG_W = round(PHI_V_W * (1.0 - ETA_WRG), -2)
H_T_W_K = PHI_T_W / D_THETA_WINTER_K
H_V_W_K = PHI_V_W / D_THETA_WINTER_K
PHI_LOSS_W = PHI_T_W + PHI_V_W
Q_H_DESIGN_W = (H_T_W_K + H_V_W_K) * D_THETA_DESIGN_K
G_PERP_SPLIT = G_SHARE_DIRECT + G_SHARE_SECONDARY_IN
F_SOLAR_WINTER = G_PERP * F_S_WINTER * F_C_NONE * F_F * F_W
Q_LOSS_KWH_PER_A = (H_T_W_K + H_V_W_K) * GT_K_D_PER_A * 24.0 / 1000.0

Q_SENS_WINTER_W = V_DOT_M3_H * C_LUFT_WH_M3_K * D_THETA_WINTER_K
Q_SENS_SUMMER_W = V_DOT_M3_H * C_LUFT_WH_M3_K * D_THETA_SUMMER_K
Q_LAT_SUMMER_W = V_DOT_M3_S * RHO_AIR_KG_M3 * R_LAT_KJ_KG * D_X_G_KG
Q_LAT_WINTER_W = V_DOT_M3_S * RHO_AIR_KG_M3 * R_LAT_KJ_KG * D_X_WINTER_G_KG


def _tex_degrees(value: float) -> str:
    return f"{value:g}".replace("-", "{-}") + r"\,^{\circ}\mathrm{C}"


def winter_day_label(language: str) -> str:
    """📅 TeX of the example winter day (θ_e = 0 °C, Δθ = 20 K) that every winter loss of the running example refers to."""
    day = {"de": "Beispiel-Wintertag", "en": "Example winter day"}[language]
    return rf"\text{{{day}}}\;\theta_{{e}} = {_tex_degrees(THETA_E_WINTER_C)}"


def design_day_label(language: str) -> str:
    """📅 TeX of the design day of the standard heating load (θ_e = −12 °C, Δθ = 32 K)."""
    day = {"de": "Auslegungstag", "en": "Design day"}[language]
    return rf"\text{{{day}}}\;\theta_{{e}} = {_tex_degrees(THETA_E_DESIGN_C)}"


def design_day_source(language: str) -> str:
    """📅 TeX of the source line under the design day: the standard outdoor temperature, named after its registry entry."""
    references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
    note = {"de": "Norm-Außentemperatur", "en": "standard outdoor temperature"}[language]
    return rf"\text{{{note}, {references.ref(DESIGN_REFERENCE_ID, language=language)}}}"


def year_outdoor_profile() -> np.ndarray:
    """🌡️ Daily mean outdoor temperatures θ_e of a stylised year in °C: exactly ``SEASON_DAYS`` days lie below the heating limit and their θ_i − θ_e add up to ``GT_K_D_PER_A``."""
    days = np.arange(365)
    phase = -np.cos(2.0 * np.pi * (days - 15) / 365.0)
    noise = np.convolve(np.random.default_rng(18599).normal(0.0, 2.2, 365), np.ones(5) / 5.0, mode="same")
    winter = np.exp(-((np.abs((days - 15 + 182) % 365 - 182)) / 110.0) ** 2)

    def raw(mean, cold):
        return mean + 8.0 * phase - cold * winter + noise

    def heating(theta):
        return theta < THETA_HG_C

    def fitted(cold):
        lo, hi = -30.0, 40.0
        for _ in range(100):
            mid = (lo + hi) / 2.0
            lo, hi = (mid, hi) if heating(raw(mid, cold)).sum() > SEASON_DAYS else (lo, mid)
        return raw(hi, cold)

    def degree_days(theta):
        return float((THETA_I_WINTER_C - theta)[heating(theta)].sum())

    lo, hi = 0.0, 30.0
    for _ in range(60):
        mid = (lo + hi) / 2.0
        lo, hi = (mid, hi) if degree_days(fitted(mid)) < GT_K_D_PER_A else (lo, mid)
    theta = fitted((lo + hi) / 2.0)
    theta[~heating(theta)] = THETA_HG_C + 0.5 * (theta[~heating(theta)] - THETA_HG_C)
    return theta
