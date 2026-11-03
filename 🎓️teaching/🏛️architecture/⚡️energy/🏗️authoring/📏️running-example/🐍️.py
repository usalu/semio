"""📏 The running example of the energy course: one house, one set of numbers, every unit in the name.

The raw numbers live in ``🔣️.json``; this module derives the powers every clip quotes, so heating, cooling and ventilation cannot drift apart.
"""

from __future__ import annotations

import json
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
THETA_E_WINTER_C = float(_DATA["winter"]["outdoorDesignC"])
D_THETA_WINTER_K = THETA_I_WINTER_C - THETA_E_WINTER_C
THETA_I_SUMMER_C = float(_DATA["summer"]["indoorC"])
THETA_E_SUMMER_C = float(_DATA["summer"]["outdoorC"])
D_THETA_SUMMER_K = THETA_E_SUMMER_C - THETA_I_SUMMER_C
X_OUT_SUMMER_G_KG = float(_DATA["summer"]["outdoorHumidityGKg"])
X_IN_SUMMER_G_KG = float(_DATA["summer"]["indoorHumidityGKg"])
D_X_G_KG = X_OUT_SUMMER_G_KG - X_IN_SUMMER_G_KG

ENVELOPE = [(item["nameDe"], float(item["areaM2"]), float(item["uValueWM2K"]), float(item["deltaK"])) for item in _DATA["envelope"]]
ENVELOPE_EN = {item["nameDe"]: item["nameEn"] for item in _DATA["envelope"]}
UA_OPAQUE_W_K = sum(item["areaM2"] * item["uValueWM2K"] for item in _DATA["envelope"] if item["id"] in ("roof", "wall"))

SEASON_DAYS = int(_DATA["season"]["heatingDays"])
GT_K_D_PER_A = float(_DATA["season"]["degreeDaysKdPerA"])
THETA_E_SEASON_MEAN_C = THETA_I_WINTER_C - GT_K_D_PER_A / SEASON_DAYS
Q_SOL_KWH_PER_A = float(_DATA["gains"]["solarKWhPerA"])
Q_INT_KWH_PER_A = float(_DATA["gains"]["internalKWhPerA"])

PHI_T_W = round(sum(area * u * delta for _name, area, u, delta in ENVELOPE), -2)
PHI_V_W = round(V_DOT_M3_H * C_LUFT_WH_M3_K * D_THETA_WINTER_K, -2)
PHI_V_WRG_W = round(PHI_V_W * (1.0 - ETA_WRG), -2)
H_T_W_K = PHI_T_W / D_THETA_WINTER_K
H_V_W_K = PHI_V_W / D_THETA_WINTER_K
PHI_LOSS_W = PHI_T_W + PHI_V_W
Q_LOSS_KWH_PER_A = (H_T_W_K + H_V_W_K) * GT_K_D_PER_A * 24.0 / 1000.0

Q_SENS_WINTER_W = V_DOT_M3_H * C_LUFT_WH_M3_K * D_THETA_WINTER_K
Q_SENS_SUMMER_W = V_DOT_M3_H * C_LUFT_WH_M3_K * D_THETA_SUMMER_K
Q_LAT_SUMMER_W = V_DOT_M3_S * RHO_AIR_KG_M3 * R_LAT_KJ_KG * D_X_G_KG
