"""⏱️ Independent oracle of the native Windows boot-time read: the layout of SYSTEM_TIMEOFDAY_INFORMATION (ctypes field list of the
phnt declaration) and the FILETIME conversion through datetime arithmetic, with the 100 ns text PowerShell prints for the same
instant (`ToString('o')`). usage: python3 -I 🐍️.py layout | convert <filetime>..."""
import ctypes, datetime, json, sys


class TimeOfDay(ctypes.Structure):
    _fields_ = [("BootTime", ctypes.c_int64), ("CurrentTime", ctypes.c_int64), ("TimeZoneBias", ctypes.c_int64),
                ("TimeZoneId", ctypes.c_uint32), ("Reserved", ctypes.c_uint32), ("BootTimeBias", ctypes.c_uint64),
                ("SleepTimeBias", ctypes.c_uint64)]


def convert(ticks):
    origin = datetime.datetime(1601, 1, 1, tzinfo=datetime.timezone.utc)
    moment = origin + datetime.timedelta(microseconds=ticks // 10)
    unix = moment - datetime.datetime(1970, 1, 1, tzinfo=datetime.timezone.utc)
    milliseconds = (unix.days * 86400 + unix.seconds) * 1000 + unix.microseconds // 1000
    text = moment.strftime("%Y-%m-%dT%H:%M:%S") + ".%07dZ" % (ticks % 10_000_000)
    return {"milliseconds": milliseconds, "text": text}


if sys.argv[1] == "layout":
    print(json.dumps({"bytes": ctypes.sizeof(TimeOfDay), "bootTime": TimeOfDay.BootTime.offset}))
else:
    print(json.dumps([convert(int(value)) for value in sys.argv[2:]]))
