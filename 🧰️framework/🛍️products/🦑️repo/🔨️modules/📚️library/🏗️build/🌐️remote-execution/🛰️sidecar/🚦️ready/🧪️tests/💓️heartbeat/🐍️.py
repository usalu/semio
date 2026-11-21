"""💓️ Checks Ready heartbeat publications with an independent date-offset model."""
import json
from datetime import datetime, timedelta, timezone
from pathlib import Path

from dateutil.relativedelta import relativedelta

directory = Path(__file__).parent
fixture = json.loads((directory / "🔣️.json").read_text(encoding="utf-8"))
schema = json.loads((directory.parents[1] / "🧬️schema/🔣️.json").read_text(encoding="utf-8"))
heartbeat = timedelta(milliseconds=schema["definitions"]["Policy"]["const"]["statusHeartbeatMilliseconds"])
assert heartbeat == timedelta(milliseconds=fixture["heartbeatMs"])
started = datetime(2026, 11, 21, tzinfo=timezone.utc)
stamps = [started]
for offset in fixture["stage"]["movementsMs"]:
    observed = started + relativedelta(microseconds=1000 * offset)
    if observed >= stamps[-1] + heartbeat:
        stamps.append(observed)
def elapsed(stamp):
    return (stamp - started) // timedelta(milliseconds=1)
assert [elapsed(stamp) for stamp in stamps] == fixture["stage"]["expectedPublicationsMs"]
backoff = fixture["loopBackoff"]
deadline = started + relativedelta(microseconds=1000 * backoff["pauseMs"])
stamps = [started]
while stamps[-1] + heartbeat < deadline:
    stamps.append(stamps[-1] + heartbeat)
assert len(stamps) == backoff["expectedPublications"]
assert elapsed(stamps[-1]) == backoff["expectedLastMs"]
print("stage and retry heartbeat publications matched")
