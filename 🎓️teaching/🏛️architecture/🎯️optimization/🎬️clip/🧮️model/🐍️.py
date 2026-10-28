"""🏛️ Transparent annual teaching model of one seminar room for six optimization examples."""

import copy
import functools
import itertools
import json
import math
from pathlib import Path

STUDY = Path(__file__).with_name("🔣️.json")
KEYS = ("orientation", "wwr", "overhang", "insulation")


def load_study():
    """📚 Read the authored study inputs without drawing-engine dependencies."""
    return json.loads(STUDY.read_text(encoding="utf-8"))


@functools.lru_cache(maxsize=1)
def _frozen():
    """🧊 Keep one read-only copy of the study for repeated evaluation."""
    study = load_study()
    values = {section: {name: item["value"] for name, item in items.items()} for section, items in study["parameters"].items()}
    return study, values


def _memo(function):
    """🧠 Cache deterministic results and hand out independent copies."""
    cached = functools.lru_cache(maxsize=None)(function)

    @functools.wraps(function)
    def wrapper(*arguments):
        """🪞 Return an independent copy of the cached result."""
        return copy.deepcopy(cached(*arguments))
    return wrapper


def _design(design=None):
    """🧰 Complete a partial design from the baseline and reject unknown or impossible values."""
    study, _ = _frozen()
    design = {**study["baseline"], **(design or {})}
    if set(design) != set(KEYS) or any(not math.isfinite(design[key]) for key in KEYS):
        raise ValueError("A design needs finite orientation, wwr, overhang and insulation")
    if not 0 < design["wwr"] <= 1 or design["overhang"] < 0 or design["insulation"] <= 0:
        raise ValueError("A design needs 0 < wwr <= 1, overhang >= 0 and insulation > 0")
    return design


def _utilization(ratio, exponent):
    """♻️ Share of gains (or losses) that is useful, after the monthly method of EN ISO 13790."""
    if math.isclose(ratio, 1.0, abs_tol=1e-9):
        return exponent / (exponent + 1)
    return (1 - ratio ** exponent) / (1 - ratio ** (exponent + 1))


def _irradiance(season, phi):
    """☀️ Vertical irradiance on the glazed façade as a harmonic of its bearing."""
    _, values = _frozen()
    climate = values["climate"]
    mean, first, second = (climate[f"{season}_irradiance_{name}"] for name in ("mean", "cos1", "cos2"))
    return max(0.0, mean + first * math.cos(phi) + second * math.cos(2 * phi))


@functools.lru_cache(maxsize=None)
def _peak(season):
    """🌞 Strongest vertical irradiance of the season over all bearings."""
    return max(_irradiance(season, math.radians(angle)) for angle in range(360))


@functools.lru_cache(maxsize=None)
def _sun_track(season):
    """🛤️ Solstice sun path as (compass bearing, altitude) pairs while the sun stands above the minimum altitude."""
    study, _ = _frozen()
    sun = study["sun"]
    phi, delta = math.radians(sun["latitude"]), math.radians(sun["declination"] if season == "summer" else -sun["declination"])
    track = []
    for step in range(-900, 901):
        hour = math.radians(step / 5)
        altitude = math.degrees(math.asin(math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.cos(hour)))
        if altitude >= sun["minimum_altitude"]:
            bearing = (180 + math.degrees(math.atan2(math.sin(hour), math.cos(hour) * math.sin(phi) - math.tan(delta) * math.cos(phi)))) % 360
            track.append((bearing, altitude))
    return tuple(track)


@functools.lru_cache(maxsize=None)
def _sun_point(orientation, season):
    """📌 Cached best-matching sun-path point of one façade bearing."""
    bearing, altitude = min(_sun_track(season), key=lambda point: abs((point[0] - orientation + 180) % 360 - 180))
    across = abs((bearing - orientation + 180) % 360 - 180)
    front = across < 89
    profile = min(85.0, math.degrees(math.atan(math.tan(math.radians(altitude)) / max(math.cos(math.radians(across)), 1e-3)))) if front else altitude
    return {"bearing": bearing, "altitude": altitude, "profile": profile, "front": front, "across": across}


def sun_state(orientation, season):
    """🌞 The sun-path point that meets the façade best: bearing, altitude, angle across the façade normal and profile angle in section."""
    return dict(_sun_point(float(orientation), season))


@functools.lru_cache(maxsize=None)
def _beam_samples(orientation, season):
    """🔆 Hourly solstice sun positions in front of the façade plane (angle across the normal below 90°), each with its beam weight cos(altitude) × cos(incidence), which is 0 at 90°."""
    study, _ = _frozen()
    sun = study["sun"]
    phi, delta = math.radians(sun["latitude"]), math.radians(sun["declination"] if season == "summer" else -sun["declination"])
    samples = []
    for hour in range(-sun["hours"], sun["hours"] + 1):
        angle = math.radians(15 * hour)
        altitude = math.degrees(math.asin(math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.cos(angle)))
        bearing = (180 + math.degrees(math.atan2(math.sin(angle), math.cos(angle) * math.sin(phi) - math.tan(delta) * math.cos(phi)))) % 360
        across = abs((bearing - orientation + 180) % 360 - 180)
        if altitude >= sun["minimum_altitude"] and across < 90:
            profile = min(85.0, math.degrees(math.atan(math.tan(math.radians(altitude)) / math.cos(math.radians(across)))))
            samples.append({"hour": hour, "bearing": bearing, "altitude": altitude, "across": across, "profile": profile,
                            "weight": math.cos(math.radians(altitude)) * math.cos(math.radians(across))})
    return tuple(samples)


@functools.lru_cache(maxsize=None)
def _sun_samples(orientation, season):
    """🕘 Hourly sun positions of the solstice that shine on the façade (angle across the normal below 89°), with their share of the beam (cos of incidence)."""
    return tuple(item for item in _beam_samples(orientation, season) if item["across"] < 89)


def sun_samples(orientation, season):
    """🕘 Hourly solstice sun positions on the façade: bearing, altitude, angle across the façade normal, profile angle in section and beam weight."""
    return [dict(item) for item in _sun_samples(float(orientation), season)]


def _shading(season, orientation, overhang, window_height, gap):
    """⛱️ Shaded beam and delivered beam: the shaded share of each hour's window height (depth × tan(profile angle) minus the wall gap above the window) weighted by the beam it delivers, over the larger of the delivered beam and the climate beam reference."""
    samples = _beam_samples(orientation, season)
    _, values = _frozen()
    beam = sum(item["weight"] for item in samples)
    shaded = 0.0 if overhang <= 0 else sum(item["weight"] * min(1.0, max(0.0, (overhang * math.tan(math.radians(item["profile"])) - gap) / window_height)) for item in samples)
    return shaded / max(beam, values["climate"]["beam_reference"]), beam


@functools.lru_cache(maxsize=None)
def _evaluate(orientation, wwr, overhang, insulation):
    """🔬 Evaluate one complete design with every intermediate quantity."""
    _, values = _frozen()
    room, climate, envelope, daylight, systems, lighting, prices, carbon = (
        values[name] for name in ("room", "climate", "envelope", "daylight", "systems", "lighting", "prices", "carbon"))
    phi = math.radians(orientation - 180)
    facade = room["width"] * room["height"]
    floor = room["width"] * room["depth"]
    window = wwr * facade
    wall = facade - window
    window_height = min(wwr * room["height"], room["head"] - 0.05)
    gap = room["height"] - room["head"]
    u_wall = 1 / (envelope["r_si"] + envelope["r_se"] + envelope["r_masonry"] + insulation / envelope["conductivity_insulation"])
    conductance = u_wall * wall + envelope["u_window"] * window + envelope["h_other"]
    shaded = {season: _shading(season, orientation, overhang, window_height, gap) for season in ("winter", "summer")}
    shading, beam = ({season: shaded[season][index] for season in shaded} for index in (0, 1))
    sky = 1 - daylight["sky_angle_loss"] * overhang ** 2 / (overhang ** 2 + daylight["overhang_half_depth"] ** 2)
    angle = daylight["sky_angle"] * sky
    direct = {season: climate[f"direct_share_{season}"] for season in shading}
    reach = {season: direct[season] * min(1.0, beam[season] / climate["beam_reference"]) for season in shading}
    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (1 - direct[season] * shading[season]) for season in shading}
    losses = conductance * climate["heating_degree_hours"] / 1000
    gains = solar["winter"] + climate["internal_gains_winter"]
    heating_eta = _utilization(gains / losses, climate["utilization_exponent_heating"])
    heating_demand = max(0.0, losses - heating_eta * gains)
    removable = conductance * climate["cooling_degree_hours"] / 1000
    heat = solar["summer"] + climate["internal_gains_summer"]
    cooling_eta = _utilization(removable / heat, climate["utilization_exponent_cooling"])
    cooling_demand = max(0.0, heat - cooling_eta * removable)
    factor = daylight["transmittance"] * window * angle / (daylight["surface_area"] * (1 - daylight["reflectance"] ** 2))
    autonomy = daylight["max_autonomy"] * (1 - math.exp(-factor * (1 + daylight["orientation_gain"] * math.cos(phi)) / daylight["reference_factor"]))
    heating = heating_demand / systems["cop"]
    cooling = cooling_demand / systems["eer"]
    light = lighting["power_density"] * floor * lighting["hours"] * (1 - autonomy) / 1000
    electricity = heating + cooling + light
    return {
        "conductance": conductance,
        "heating_losses": losses, "heating_gains": gains, "heating_eta": heating_eta, "heating_used": heating_eta * gains, "cop": systems["cop"],
        "cooling_gains": heat, "cooling_removable": removable, "cooling_eta": cooling_eta, "cooling_removed": cooling_eta * removable, "eer": systems["eer"],
        "internal_winter": climate["internal_gains_winter"], "internal_summer": climate["internal_gains_summer"],
        "beam_winter": beam["winter"], "beam_summer": beam["summer"], "direct_winter": reach["winter"], "direct_summer": reach["summer"],
        "lighting_power": lighting["power_density"] * floor / 1000, "lighting_hours": lighting["hours"], "lamps_on": 1 - autonomy,
        "window_area": window, "wall_area": wall, "u_wall": u_wall,
        "heating_demand": heating_demand, "cooling_demand": cooling_demand,
        "heating": heating, "cooling": cooling, "lighting": light, "electricity": electricity,
        "intensity": electricity / floor,
        "solar_winter": solar["winter"], "solar_summer": solar["summer"],
        "shading_winter": shading["winter"], "shading_summer": shading["summer"],
        "sun_winter": _irradiance("winter", phi) / _peak("winter"), "sun_summer": _irradiance("summer", phi) / _peak("summer"),
        "daylight": factor, "autonomy": autonomy,
        "cost": prices["fixed"] + wall * (prices["wall"] + prices["insulation_per_cm"] * 100 * insulation) + window * prices["window"] + prices["overhang"] * room["width"] * overhang,
        "carbon": wall * (carbon["wall"] + carbon["insulation_per_cm"] * 100 * insulation) + window * carbon["window"] + carbon["overhang"] * room["width"] * overhang,
    }


def evaluate(design=None):
    """⚙️ Evaluate one design; missing keys fall back to the baseline design."""
    design = _design(design)
    return dict(_evaluate(*(float(design[key]) for key in KEYS)))


def _objective(wwr, overhang, base):
    """🎯 Electricity of the south wall design at one point of the algorithm surface."""
    return _evaluate(base["orientation"], float(wwr), float(overhang), base["insulation"])["electricity"]


@_memo
def _orientation_study():
    """🔄 Scan the bearing of the glazed façade on a 1° grid for optima, the courtyard constraint and a greedy descent."""
    study, values = _frozen()
    settings = study["orientation"]
    low, high = settings["range"]
    design = {**settings["design"]}
    energy = lambda angle: _evaluate(float(angle), design["wwr"], design["overhang"], design["insulation"])["electricity"]
    angles = list(range(low, high + 1))
    scan = [energy(angle) for angle in angles]
    minima, maxima = [], []
    for index in range(1, len(angles) - 1):
        for sign, found in ((1, minima), (-1, maxima)):
            if sign * scan[index] < sign * scan[index - 1] and sign * scan[index] <= sign * scan[index + 1]:
                found.append(angles[index])
    constraint = settings["constraint"]
    candidates = lambda bounds: [angle for angle in (*minima, *bounds) if bounds[0] <= angle <= bounds[1]]
    reference, tariff, step = energy(settings["reference"]), values["economics"]["tariff"], settings["neighbour_step"]
    current = settings["start"]
    descent = []
    while True:
        neighbours = [{"theta": angle, "electricity": energy(angle)} for angle in (current - step, current + step) if low <= angle <= high]
        descent.append({"theta": current, "electricity": energy(current), "neighbours": neighbours})
        best = min(neighbours, key=lambda item: item["electricity"])
        if best["electricity"] >= energy(current):
            break
        current = best["theta"]
    return {
        "design": design, "range": [low, high],
        "local_minima": minima, "local_minima_electricity": [energy(angle) for angle in minima],
        "local_maxima": maxima, "local_maxima_electricity": [energy(angle) for angle in maxima],
        "global_minimum": min(candidates([low, high]), key=energy), "constraint": constraint,
        "feasible_minimum": min(candidates(constraint), key=energy),
        "reference": settings["reference"], "reference_electricity": reference, "tariff": tariff,
        "samples": [[angle, value] for angle, value in zip(angles, scan, strict=True)],
        "savings": [[angle, reference - value, (reference - value) * tariff] for angle, value in zip(angles, scan, strict=True)],
        "start": settings["start"], "neighbour_step": step, "descent": descent, "sun": study["sun"],
    }


def orientation_study():
    """📉 Local and global optima of electricity over the façade bearing, with constraint and savings."""
    return _orientation_study()


@_memo
def _surface(count):
    """🗻 Electricity over WWR × overhang for the south wall."""
    study, _ = _frozen()
    surface = study["surface"]
    base = _design(surface["design"])
    wwr = [surface["wwr"][0] + index * (surface["wwr"][1] - surface["wwr"][0]) / (count - 1) for index in range(count)]
    overhang = [surface["overhang"][0] + index * (surface["overhang"][1] - surface["overhang"][0]) / (count - 1) for index in range(count)]
    grid = [[_objective(x, y, base) for y in overhang] for x in wwr]
    value, i, j = min((value, i, j) for i, row in enumerate(grid) for j, value in enumerate(row))
    return {"design": base, "wwr": wwr, "overhang": overhang, "electricity": grid,
            "minimum": {"design": {**base, "wwr": wwr[i], "overhang": overhang[j]}, "electricity": value}}


def surface(n=41):
    """⛰️ Electricity [wwr index][overhang index] over a dense n × n grid."""
    return _surface(n)


def _sample(wwr, overhang):
    """📍 One sample point with its evaluated consequences."""
    study, _ = _frozen()
    return {"wwr": wwr, "overhang": overhang, **evaluate({**study["surface"]["design"], "wwr": wwr, "overhang": overhang})}


def levels():
    """🪜 Grid levels per design axis: wwr and overhang of the 3 × 3 grid, four bearings, three insulation thicknesses."""
    study, _ = _frozen()
    return copy.deepcopy(study["sampling"]["levels"])


def lhs_strata():
    """🧱 Stratum edges of the Latin hypercube on both axes and the stratum index pair of every point."""
    study, _ = _frozen()
    permutation = study["sampling"]["latin_permutation"]
    count = len(permutation)
    edges = lambda low, high: [low + index * (high - low) / count for index in range(count + 1)]
    return {"count": count, "wwr": edges(*study["surface"]["wwr"]), "overhang": edges(*study["surface"]["overhang"]),
            "index": [[index, stratum] for index, stratum in enumerate(permutation)]}


@_memo
def _samples(kind):
    """🎰 Equal-budget grid, recorded random and Latin-hypercube samples of the surface."""
    study, _ = _frozen()
    surface_study, sampling = study["surface"], study["sampling"]
    (wlow, whigh), (olow, ohigh) = surface_study["wwr"], surface_study["overhang"]
    if kind == "grid":
        points = [(*point, None) for point in itertools.product(sampling["levels"]["wwr"], sampling["levels"]["overhang"])]
    elif kind == "random":
        points = [(wlow + u * (whigh - wlow), olow + v * (ohigh - olow), None) for u, v in sampling["random"]]
    elif kind == "lhs":
        strata = lhs_strata()
        points = [((strata["wwr"][i] + strata["wwr"][i + 1]) / 2, (strata["overhang"][j] + strata["overhang"][j + 1]) / 2, [i, j])
                  for i, j in strata["index"]]
    else:
        raise ValueError("Unknown sampling kind")
    return [{**_sample(wwr, overhang), **({} if index is None else {"strata_index": index})} for wwr, overhang, index in points]


def samples(kind):
    """🎲 Return grid, random or lhs samples of the south wall surface with their consequences."""
    return _samples(kind)


def sampling_diagnostics(kind="random"):
    """🔍 Largest empty box and closest pair of one sample set, measured on axes normalized to their ranges."""
    study, _ = _frozen()
    (wlow, whigh), (olow, ohigh) = study["surface"]["wwr"], study["surface"]["overhang"]
    points = [((item["wwr"] - wlow) / (whigh - wlow), (item["overhang"] - olow) / (ohigh - olow)) for item in samples(kind)]
    best = (0.0, 0.0, 1.0, 0.0, 1.0)
    for left, right in itertools.combinations(sorted({0.0, 1.0, *(u for u, _ in points)}), 2):
        inside = sorted({0.0, 1.0, *(v for u, v in points if left < u < right)})
        gap, bottom = max((high - low, low) for low, high in itertools.pairwise(inside))
        area = (right - left) * gap
        if area > best[0]:
            best = (area, left, right, bottom, bottom + gap)
    pairs = [(math.dist(points[i], points[j]), i, j) for i, j in itertools.combinations(range(len(points)), 2)]
    distance, i, j = min(pairs)
    box = {"wwr": [wlow + best[1] * (whigh - wlow), wlow + best[2] * (whigh - wlow)],
           "overhang": [olow + best[3] * (ohigh - olow), olow + best[4] * (ohigh - olow)]}
    return {"largest_empty_box": box, "area": best[0], "closest_pair": [i, j, distance],
            "closest_delta": {"wwr": abs(points[i][0] - points[j][0]) * (whigh - wlow), "overhang": abs(points[i][1] - points[j][1]) * (ohigh - olow)}}


def _contrast(responses, signs):
    """➗ Mean of the high-level runs minus mean of the low-level runs."""
    return sum(sign * value for sign, value in zip(signs, responses, strict=True)) / (len(responses) / 2)


@_memo
def _factorial():
    """🧫 Exact contrasts of a two-level full factorial on one response."""
    study, _ = _frozen()
    setup = study["factorial"]
    base = _design(setup["design"])
    (a_id, a_levels), (b_id, b_levels) = ((factor["id"], factor["levels"]) for factor in setup["factors"])
    runs = []
    for number, (a, b) in enumerate(itertools.product((-1, 1), repeat=2), 1):
        design = {**base, a_id: a_levels[(a + 1) // 2], b_id: b_levels[(b + 1) // 2]}
        result = evaluate(design)
        runs.append({"id": f"r{number}", "coded": [a, b], "design": design, "response": result[setup["response"]], **result})
    y = [run["response"] for run in runs]
    effects = {"a": _contrast(y, [run["coded"][0] for run in runs]), "b": _contrast(y, [run["coded"][1] for run in runs]),
               "ab": _contrast(y, [run["coded"][0] * run["coded"][1] for run in runs])}
    coefficients = [sum(y) / 4, effects["a"] / 2, effects["b"] / 2, effects["ab"] / 2]
    centre = {**base, a_id: sum(a_levels) / 2, b_id: sum(b_levels) / 2}
    actual = evaluate(centre)[setup["response"]]
    at = lambda depth: evaluate({**centre, b_id: depth})[setup["response"]]
    full, half = at(b_levels[0]) - at(b_levels[1]), at(b_levels[0]) - at(sum(b_levels) / 2)
    return {"factors": setup["factors"], "response": setup["response"], "runs": runs, "effects": effects, "coefficients": coefficients,
            "centre": {"design": centre, "predicted": coefficients[0], "actual": actual, "curvature": (actual - coefficients[0]) / coefficients[0],
                       "result": evaluate(centre)},
            "centre_savings": {"half": half, "full": full, "share": half / full}}


def factorial():
    """🧪 Four runs r1…r4 with full results, effects (a, b, ab), coefficients (effects / 2) and the centre check."""
    return _factorial()


@_memo
def _fractional():
    """🪓 Half fraction 2^(3-1) with C = AB, compared with the true effects of the full 2³ plan."""
    study, _ = _frozen()
    setup = study["factorial"]
    base = _design(setup["design"])
    factors = [*setup["factors"], setup["third"]]
    response = lambda codes: evaluate({**base, **{factor["id"]: factor["levels"][(code + 1) // 2] for factor, code in zip(factors, codes, strict=True)}})[setup["response"]]
    full = [(codes, response(codes)) for codes in itertools.product((-1, 1), repeat=3)]
    runs = []
    for a, b in itertools.product((-1, 1), repeat=2):
        codes = [a, b, a * b]
        runs.append({"coded": codes, "design": {**base, **{factor["id"]: factor["levels"][(code + 1) // 2] for factor, code in zip(factors, codes, strict=True)}},
                     "response": response(codes)})
    y = [run["response"] for run in runs]
    true = {"ab": _contrast([value for _, value in full], [a * b for (a, b, _), _ in full]),
            "c": _contrast([value for _, value in full], [c for (_, _, c), _ in full])}
    return {"generator": "C=AB", "factors": factors, "runs": runs, "alias_contrast": _contrast(y, [run["coded"][2] for run in runs]),
            "true": {**true, "sum": true["ab"] + true["c"]}, "full": [{"coded": list(codes), "response": value} for codes, value in full]}


def fractional():
    """🧩 Four runs of the half fraction, the confounded C = AB contrast and the true C and AB effects."""
    return _fractional()


@_memo
def _variants(extra):
    """🏘️ Named façade variants with all evaluated criteria."""
    study, _ = _frozen()
    return [{"id": item["id"], "title": item["title"], "design": _design(item["design"]), **({"extra": True} if item.get("extra") else {}), **evaluate(item["design"])}
            for item in study["variants"] if extra or not item.get("extra")]


def variants(extra=False):
    """🏠 The authored façade variants evaluated by the model; extra=True adds the dearer sixth one."""
    return _variants(bool(extra))


def brief():
    """📝 The client brief: the façade cost budget in CHF (a mandatory limit) and its bilingual note."""
    study, _ = _frozen()
    return copy.deepcopy(study["brief"])


def criteria():
    """📏 Criteria with direction, unit, bilingual title and the model key they read."""
    study, _ = _frozen()
    return copy.deepcopy(study["criteria"])


def reweigh(weights, criterion, share):
    """🎚️ Give one criterion a share of 1 and scale the others in their old proportions."""
    others = sum(value for key, value in weights.items() if key != criterion)
    if criterion not in weights or not 0 <= share <= 1 or others <= 0:
        raise ValueError("Unknown criterion, share outside 0…1 or no other weight left")
    return {key: share if key == criterion else value / others * (1 - share) for key, value in weights.items()}


def index(weights, ids=None):
    """⚖️ Min–max normalize each criterion over the compared variants (0 best … 1 worst) and rank by the lowest weighted score."""
    items = criteria()
    rows = [row for row in variants(extra=True) if (ids is None and not row.get("extra")) or (ids is not None and row["id"] in ids)]
    identifiers = [item["id"] for item in items]
    if (not isinstance(weights, dict) or set(weights) != set(identifiers)
            or any(not math.isfinite(weights[key]) or weights[key] < 0 for key in identifiers) or sum(weights.values()) <= 0 or not rows):
        raise ValueError("Weights need every criterion id, finite and nonnegative, with a positive sum, and at least one variant")
    total = sum(weights.values())
    scores = [{"id": row["id"], "raw": {item["id"]: row[item["key"]] for item in items}, "normalized": {}} for row in rows]
    for item in items:
        column = [score["raw"][item["id"]] for score in scores]
        low, high = min(column), max(column)
        for score, value in zip(scores, column, strict=True):
            bad = 0.0 if high == low else (value - low) / (high - low)
            score["normalized"][item["id"]] = 1 - bad if item["direction"] == "max" else bad
    for score in scores:
        score["score"] = sum(weights[key] * score["normalized"][key] for key in identifiers) / total
    for rank, position in enumerate(sorted(range(len(scores)), key=lambda position: scores[position]["score"]), 1):
        scores[position]["rank"] = rank
    return scores


def _dominates(a, b, keys):
    """🥊 Whether a is at least as good as b in every key and strictly better in one (minimization)."""
    return all(a[key] <= b[key] for key in keys) and any(a[key] < b[key] for key in keys)


@_memo
def _pareto():
    """🏅 Cost versus electricity for a design grid, its front, payback steps and knees."""
    study, values = _frozen()
    setup, economics = study["pareto"], values["economics"]
    base = _design(setup["design"])
    designs = []
    for wwr, insulation, overhang in itertools.product(setup["wwr"], setup["insulation"], setup["overhang"]):
        design = {**base, "wwr": wwr, "insulation": insulation, "overhang": overhang}
        result = evaluate(design)
        designs.append({"id": f"w{round(wwr * 100)}-t{round(insulation * 100)}-d{round(overhang * 100)}", "design": design,
                        **{key: result[key] for key in ("cost", "electricity", "heating", "cooling", "lighting", "daylight")}})
    for item in designs:
        item["antidaylight"] = -item["daylight"]
    objectives = ("cost", "electricity")
    front = sorted((item for item in designs if not any(_dominates(other, item, objectives) for other in designs)), key=lambda item: (item["cost"], item["electricity"]))
    front3 = [item["id"] for item in designs if not any(_dominates(other, item, (*objectives, "antidaylight")) for other in designs)]
    for item in designs:
        del item["antidaylight"]
    lookup = {item["id"]: item for item in designs}
    steps = []
    for start, end in itertools.pairwise(front):
        saved = start["electricity"] - end["electricity"]
        ratio = (end["cost"] - start["cost"]) / saved
        steps.append({"from": start["id"], "to": end["id"], "cost": end["cost"] - start["cost"], "electricity": -saved, "saved": saved,
                      "ratio": ratio, "payback": ratio / economics["tariff"], "payback_hi": ratio / economics["tariff_hi"]})
    knee = {}
    for name, key in (("tariff", "payback"), ("tariff_hi", "payback_hi")):
        affordable = [step["to"] for step in steps if step[key] <= economics["service_life"]]
        knee[name] = affordable[-1] if affordable else front[0]["id"]
    optimum = surface()["minimum"]["design"]
    refined = evaluate(optimum)
    candidate = {"cost": refined["cost"], "electricity": refined["electricity"]}
    refinement = {"member": setup["refinement"]["member"], "design": optimum, "cost": refined["cost"], "electricity": refined["electricity"],
                  "dominates": [item["id"] for item in front if _dominates(candidate, item, objectives)]}
    revived = [lookup[identifier] for identifier in front3 if identifier not in {item["id"] for item in front}]
    third = max(revived, key=lambda item: item["daylight"])
    dominators = [item for item in front if _dominates(item, third, objectives)]
    third_example = {"dominated_2d": third["id"], "dominator": min(dominators, key=lambda item: item["cost"])["id"]}
    start, end = (lookup[setup["step"][key]] for key in ("from", "to"))
    front = [lookup[item["id"]] for item in front]
    return {"designs": designs, "front": [item["id"] for item in front], "dominated_example": setup["dominated_example"],
            "step": {"from": start["id"], "to": end["id"], "cost": end["cost"] - start["cost"], "electricity": end["electricity"] - start["electricity"]},
            "steps": steps, "tariff": economics["tariff"], "tariff_hi": economics["tariff_hi"], "service_life": economics["service_life"], "knee": knee,
            "refinement": refinement, "third_example": third_example, "front3": front3,
            "picks": [{"weight": weight, "id": _pick(front, weight)} for weight in setup["weights"]]}


def pareto():
    """🏆 Evaluated design grid, front ids by cost, dominated pair, steps with payback, knees, refinement and third criterion."""
    return _pareto()


def _pick(front, weight):
    """🥇 Front id minimizing the weighted sum of min–max normalized cost and electricity."""
    ranges = {key: (min(item[key] for item in front), max(item[key] for item in front)) for key in ("cost", "electricity")}
    normal = lambda item, key: (item[key] - ranges[key][0]) / (ranges[key][1] - ranges[key][0])
    return min(front, key=lambda item: (1 - weight) * normal(item, "cost") + weight * normal(item, "electricity"))["id"]


def weighted_pick(weight):
    """🧲 Front id for a weighted sum of normalized cost and electricity; weight is the share of electricity."""
    if not 0 <= weight <= 1:
        raise ValueError("The electricity weight must lie between 0 and 1")
    data = _pareto()
    lookup = {item["id"]: item for item in data["designs"]}
    return _pick([lookup[identifier] for identifier in data["front"]], weight)


def _clip(value, bounds):
    """✂️ Keep a value inside its closed interval."""
    return min(bounds[1], max(bounds[0], value))


def _setup():
    """🛠️ Shared base design, bounds and ranges of the algorithm surface."""
    study, _ = _frozen()
    bounds = {"wwr": study["surface"]["wwr"], "overhang": study["surface"]["overhang"]}
    return study["algorithms"], _design(study["surface"]["design"]), bounds, {key: bounds[key][1] - bounds[key][0] for key in bounds}


def _evaluated(point, base):
    """🧾 A design dictionary of one algorithm point with its electricity."""
    return {"design": {**base, **point}, "electricity": _objective(point["wwr"], point["overhang"], base)}


@_memo
def _gradient_path():
    """⛷️ Numerical-gradient descent: probe four neighbours, step downhill in range-normalized coordinates."""
    setup, base, bounds, span = _setup()
    settings = setup["gradient"]
    point, previous, evaluations, path = dict(setup["start"]), None, 0, []
    for _ in range(settings["iterations"]):
        probes, gradient = [], {}
        for key in ("wwr", "overhang"):
            h = settings["difference"] * span[key]
            low, high = _clip(point[key] - h, bounds[key]), _clip(point[key] + h, bounds[key])
            pair = [_evaluated({**point, key: value}, base) for value in (low, high)]
            probes += [{"direction": f"{key}{sign}", **item} for sign, item in zip("-+", pair, strict=True)]
            gradient[key] = (pair[1]["electricity"] - pair[0]["electricity"]) / (high - low)
        evaluations += 5
        path.append({**_evaluated(point, base), "gradient": gradient, "slope": {"wwr": gradient["wwr"] / 100, "overhang": gradient["overhang"]},
                     "step": {key: 0.0 if previous is None else point[key] - previous[key] for key in point},
                     "probes": probes, "runs": 5, "evaluations": evaluations})
        previous = point
        point = {key: _clip(point[key] - settings["learning_rate"] * gradient[key] * span[key] ** 2, bounds[key]) for key in point}
    return path


def gradient_path():
    """📐 Iterates of fixed-rate descent: design, electricity, gradient (per unit), slope (per %-point and per m), step, probes, runs, evaluations."""
    return _gradient_path()


@_memo
def _pattern_path():
    """🪀 Compass search that polls four neighbours and halves the step when none improves."""
    setup, base, bounds, _ = _setup()
    settings = setup["pattern"]
    point, step = dict(setup["start"]), dict(settings["step"])
    current = _evaluated(point, base)
    path = [{**current, "step": dict(step), "probes": [], "moved": False, "evaluations": 1}]
    evaluations = 1
    for _ in range(settings["iterations"]):
        probes = [{"direction": f"{key}{'+' if sign > 0 else '-'}", **_evaluated({**point, key: _clip(point[key] + sign * step[key], bounds[key])}, base)}
                  for key, sign in itertools.product(("wwr", "overhang"), (1, -1))]
        best = min(probes, key=lambda probe: probe["electricity"])
        improved = best["electricity"] < current["electricity"] - settings["tolerance"]
        used, evaluations = dict(step), evaluations + len(probes)
        if improved:
            point, current = {key: best["design"][key] for key in point}, {"design": best["design"], "electricity": best["electricity"]}
        else:
            step = {key: step[key] * settings["contraction"] for key in step}
        path.append({**current, "step": used, "probes": probes, "moved": improved, "evaluations": evaluations})
    return path


def pattern_path():
    """🧭 Iterates of compass search with probes, the step used in each round, moved flag and cumulative evaluations."""
    return _pattern_path()


@_memo
def _genetic():
    """🦠 Elitist genetic algorithm with tournament selection, gene-wise crossover and mutation."""
    setup, base, bounds, _ = _setup()
    settings = setup["genetic"]
    lcg = settings["lcg"]
    state = lcg["seed"]

    def draw():
        """🎡 Advance the recorded Park–Miller generator and return a draw in (0, 1)."""
        nonlocal state
        state = (lcg["multiplier"] * state + lcg["increment"]) % lcg["modulus"]
        return state / lcg["modulus"]

    def individual(identifier, genes):
        """👤 One evaluated member of a population."""
        return {"id": identifier, **_evaluated(genes, base)}

    population = [individual(f"g0-{index}", {key: bounds[key][0] + draw() * (bounds[key][1] - bounds[key][0]) for key in bounds})
                  for index in range(1, settings["population"] + 1)]
    generations, evaluations = [], len(population)
    for generation in range(settings["generations"] + 1):
        entry = {"population": population, "parents": [], "children": [], "tournaments": [], "elites": [], "survivors": [],
                 "best": min(population, key=lambda item: item["electricity"]), "evaluations": evaluations}
        generations.append(entry)
        if generation == settings["generations"]:
            break
        elite = sorted(population, key=lambda item: item["electricity"])[:settings["elite"]]

        def tournament():
            """🤼 Draw two contenders at random and let the lower electricity win."""
            contenders = [population[int(draw() * len(population))] for _ in range(settings["tournament"])]
            winner = min(contenders, key=lambda item: item["electricity"])
            entry["tournaments"].append([*(item["id"] for item in contenders), winner["id"]])
            return winner
        for number in range(1, settings["population"] - settings["elite"] + 1):
            first = tournament()
            second = tournament()
            while second["id"] == first["id"]:
                second = tournament()
            donors = (first, second) if draw() < .5 else (second, first)
            inherited = {"wwr": donors[0]["design"]["wwr"], "overhang": donors[1]["design"]["overhang"]}
            genes, mutation = {}, {}
            for key in ("wwr", "overhang"):
                chance, size = draw(), draw()
                shift = (2 * size - 1) * settings["mutation"][key] if chance < settings["mutation"]["probability"] else 0.0
                genes[key] = _clip(inherited[key] + shift, bounds[key])
                mutation[key] = genes[key] - inherited[key]
            child = individual(f"g{generation + 1}-{number}", genes)
            entry["parents"].append([first["id"], second["id"]])
            entry["children"].append({"id": child["id"], "from": [first["id"], second["id"]],
                                      "genes": {"wwr": donors[0]["id"], "overhang": donors[1]["id"]},
                                      "mutation": mutation, "design": child["design"], "electricity": child["electricity"]})
        evaluations += len(entry["children"])
        population = [*elite, *({"id": child["id"], "design": child["design"], "electricity": child["electricity"]} for child in entry["children"])]
        entry["elites"] = [item["id"] for item in elite]
        entry["survivors"] = [item["id"] for item in population]
    return {"generations": generations, "lcg": lcg}


def genetic():
    """🧬 Generations: population, parents, tournaments [a, b, winner], children with genes and mutation, elites, survivors, best, evaluations."""
    return _genetic()


def convergence():
    """📈 Best-so-far electricity against simulation runs for gradient, pattern and genetic search on a shared budget."""
    study, _ = _frozen()
    budget = study["algorithms"]["budget"]
    orders = {
        "gradient": [value for item in _gradient_path() for value in (item["electricity"], *(probe["electricity"] for probe in item["probes"]))],
        "pattern": [_pattern_path()[0]["electricity"],
                    *(probe["electricity"] for item in _pattern_path()[1:] for probe in item["probes"])],
        "genetic": [item["electricity"] for item in _genetic()["generations"][0]["population"]]
                   + [child["electricity"] for entry in _genetic()["generations"] for child in entry["children"]],
    }
    series = {}
    for method, values in orders.items():
        best, rows = math.inf, []
        for runs, value in enumerate(values[:budget], 1):
            best = min(best, value)
            rows.append([runs, best])
        series[method] = rows
    optimum = surface()
    return {"budget": budget, "map_minimum": optimum["minimum"]["electricity"], "runs_map": len(optimum["wwr"]) * len(optimum["overhang"]), "series": series}
