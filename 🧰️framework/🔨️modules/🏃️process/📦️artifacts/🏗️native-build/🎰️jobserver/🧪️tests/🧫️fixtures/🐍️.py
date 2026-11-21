import ctypes
import os
import sys


endpoint = os.environ["CARGO_MAKEFLAGS"].split("--jobserver-auth=")[-1]
restore = sys.argv[1] == "return"
if os.name == "nt":
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel.OpenSemaphoreW.argtypes = [ctypes.c_uint32, ctypes.c_int, ctypes.c_wchar_p]
    kernel.OpenSemaphoreW.restype = ctypes.c_void_p
    kernel.WaitForSingleObject.argtypes = [ctypes.c_void_p, ctypes.c_uint32]
    kernel.ReleaseSemaphore.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_void_p]
    kernel.CloseHandle.argtypes = [ctypes.c_void_p]
    handle = kernel.OpenSemaphoreW(0x100002, False, endpoint)
    if not handle:
        raise ctypes.WinError(ctypes.get_last_error())
    status = kernel.WaitForSingleObject(handle, 0)
    if status not in (0, 258):
        raise ctypes.WinError(ctypes.get_last_error())
    acquired = status == 0
    if acquired and restore and not kernel.ReleaseSemaphore(handle, 1, None):
        raise ctypes.WinError(ctypes.get_last_error())
    kernel.CloseHandle(handle)
else:
    descriptor = os.open(endpoint.removeprefix("fifo:"), os.O_RDWR | os.O_NONBLOCK)
    try:
        token = os.read(descriptor, 1)
    except BlockingIOError:
        token = b""
    acquired = bool(token)
    if acquired and restore:
        os.write(descriptor, token)
    os.close(descriptor)

print("acquired" if acquired else "unavailable")
