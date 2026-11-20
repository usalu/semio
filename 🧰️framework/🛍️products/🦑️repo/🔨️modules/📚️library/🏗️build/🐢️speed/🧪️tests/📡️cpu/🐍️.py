import datetime
import json
import sys


def milliseconds(row):
    if row["pid"] <= 0:
        return None
    if row["platform"] == "darwin":
        minutes, seconds = row["text"].strip().split(":")
        return round(datetime.timedelta(minutes=int(minutes), seconds=float(seconds)).total_seconds() * 1000)
    if row["platform"] == "linux":
        fields = row["text"].rpartition(")")[2].split()
        if len(fields) < 13:
            return None
        return (int(fields[11]) + int(fields[12])) * 10
    return None


with open(sys.argv[1], encoding="utf-8") as source:
    print(json.dumps([milliseconds(row) for row in json.load(source)]))
