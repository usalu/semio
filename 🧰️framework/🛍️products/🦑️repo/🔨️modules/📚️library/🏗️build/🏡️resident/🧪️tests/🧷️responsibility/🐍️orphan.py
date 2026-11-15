"""Orphan experiment on macOS, no repo code: A is spawned as its own responsible app (disclaim), A spawns a detached B and ends; B reports who is responsible for it before and after A ended, as JSON into the file named by the last argument."""
import ctypes, json, os, subprocess, sys, time

libc = ctypes.CDLL(None)
who = libc.responsibility_get_pid_responsible_for_pid
who.argtypes, who.restype = [ctypes.c_int], ctypes.c_int

if sys.argv[1] == "B":
    before = who(os.getpid())
    time.sleep(1.5)
    json.dump({"a": int(sys.argv[2]), "b": os.getpid(), "before": before, "after": who(os.getpid())}, open(sys.argv[3], "w"))
elif sys.argv[1] == "A":
    subprocess.Popen([sys.executable, "-I", sys.argv[0], "B", str(os.getpid()), sys.argv[2]], start_new_session=True, stdout=subprocess.DEVNULL)
    time.sleep(0.5)
else:
    attributes = ctypes.create_string_buffer(8)
    libc.posix_spawnattr_init(attributes)
    libc.responsibility_spawnattrs_setdisclaim(attributes, 1)
    pid = ctypes.c_int()
    argv = (ctypes.c_char_p * 6)(sys.executable.encode(), b"-I", sys.argv[0].encode(), b"A", sys.argv[2].encode(), None)
    sys.exit(0 if libc.posix_spawn(ctypes.byref(pid), sys.executable.encode(), None, attributes, argv, ctypes.POINTER(ctypes.c_char_p).in_dll(libc, "environ")) == 0 else 1)
