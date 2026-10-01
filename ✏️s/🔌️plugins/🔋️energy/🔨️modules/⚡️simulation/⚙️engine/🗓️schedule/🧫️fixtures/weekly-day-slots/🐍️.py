"""🐝 Honeybee reference for the Sunday-first weekly slot fixture.

EnergyPlus `Schedule:Week:Daily` lists Sunday through Saturday. Honeybee stores that
order in `week_apply_tuple` and addresses it as day-of-week 1 = Sunday … 7 = Saturday
(`does_rule_apply` indexes `tuple[dow - 1]`). Each fixture case must reproduce that
value, and the emitted week object must list the seven day identifiers in `slotOrder`.
"""

import json
import sys
from pathlib import Path

from honeybee_energy.schedule.day import ScheduleDay
from honeybee_energy.schedule.rule import ScheduleRule
from honeybee_energy.schedule.ruleset import ScheduleRuleset
from ladybug.dt import Date


def main() -> None:
    fixture = json.loads(Path(__file__).with_name("🔣️.json").read_text())
    order = fixture["slotOrder"]
    values = fixture["dailyValueBySlot"]
    hour = fixture["hour"]
    days = [ScheduleDay(name, [value]) for name, value in zip(order, values)]
    rules = [ScheduleRule(day, **{f"apply_{candidate}": candidate == day.identifier for candidate in order}) for day in days]
    ruleset = ScheduleRuleset("weekly-slots", ScheduleDay("unused", [-1.0]), rules)
    _year, weeks = ruleset.to_idf()
    week = _week_daily_fields("\n".join(weeks or []))
    if [field.lower() for field in week] != order:
        raise SystemExit(f"Schedule:Week:Daily fields {week} are not {order}")
    for case in fixture["cases"]:
        date = case["date"]
        series = ruleset.values(1, Date(date["month"], date["day"]), Date(date["month"], date["day"]), start_dow=case["weekday"], leap_year=False)
        got = series[hour]
        if abs(got - case["value"]) > 1e-9:
            raise SystemExit(f"{case['weekday']} honeybee {got} fixture {case['value']}")
    print(f"honeybee weekly slots {len(fixture['cases'])}/{len(fixture['cases'])}")


def _week_daily_fields(idf: str) -> list[str]:
    start = idf.lower().find("schedule:week:daily")
    if start < 0:
        raise SystemExit("honeybee emitted no Schedule:Week:Daily")
    chunk = idf[start:idf.find(";", start)]
    fields = [part.split("!")[0].strip().rstrip(",") for part in chunk.splitlines()[1:]]
    return fields[1:8]


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        print(error, file=sys.stderr)
        raise
