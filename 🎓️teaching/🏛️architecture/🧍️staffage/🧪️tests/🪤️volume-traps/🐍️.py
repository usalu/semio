"""🪤️ Oracle of `volume-revolve-implied-closure`: shapely decides whether the profile of a revolve solid is an open polyline that stays off the axis at both ends (farther than 2 % of its largest radius) and whose chord from the last to the first point closes it into a valid polygon of non-zero area.

Input: the path of a JSON document `{"cases": [{"name": ..., "profile": [[axial, radial], ...]}]}`. Output: JSON `{name: {"shell": bool}}`; a profile whose first point equals its last is a loop and never `shell`.
"""
import json
import sys

from shapely.geometry import Polygon


def shell(profile):
    if len(profile) >= 4 and profile[0] == profile[-1]:
        return False
    reach = max(p[1] for p in profile)
    if profile[0][1] <= 0.02 * reach or profile[-1][1] <= 0.02 * reach:
        return False
    if len(profile) < 3:
        return False
    polygon = Polygon(profile)
    return bool(polygon.is_valid and polygon.area > 0)


if __name__ == "__main__":
    with open(sys.argv[1], encoding="utf8") as handle:
        cases = json.load(handle)["cases"]
    print(json.dumps({case["name"]: {"shell": shell(case["profile"])} for case in cases}))
