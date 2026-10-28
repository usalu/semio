"""🐍️ Independent oracle of the Windows physical lock tests: the layout of the Win32 structures (ctypes field lists of the SDK
declarations) and the extended-path rule (`ntpath`, the documented Windows path grammar), computed without the code under test.
usage: python3 -I 🐍️.py layout | extended <path>..."""
import ctypes, json, ntpath, sys

DWORD = ctypes.c_uint32  # the SDK defines DWORD as a 32-bit unsigned integer on every Windows ABI (ctypes.wintypes follows the host C long)


class FILETIME(ctypes.Structure):
    _fields_ = [("dwLowDateTime", DWORD), ("dwHighDateTime", DWORD)]


class ByHandleFileInformation(ctypes.Structure):
    _fields_ = [("dwFileAttributes", DWORD), ("ftCreationTime", FILETIME), ("ftLastAccessTime", FILETIME),
                ("ftLastWriteTime", FILETIME), ("dwVolumeSerialNumber", DWORD), ("nFileSizeHigh", DWORD),
                ("nFileSizeLow", DWORD), ("nNumberOfLinks", DWORD), ("nFileIndexHigh", DWORD),
                ("nFileIndexLow", DWORD)]


class OffsetPair(ctypes.Structure):
    _fields_ = [("Offset", DWORD), ("OffsetHigh", DWORD)]


class OffsetUnion(ctypes.Union):
    _fields_ = [("pair", OffsetPair), ("Pointer", ctypes.c_void_p)]


class Overlapped(ctypes.Structure):
    _fields_ = [("Internal", ctypes.c_size_t), ("InternalHigh", ctypes.c_size_t), ("u", OffsetUnion), ("hEvent", ctypes.c_void_p)]


def extended(path):
    drive, rest = ntpath.splitdrive(path)
    if path.startswith("\\\\?\\"):
        return path
    if drive.startswith("\\\\"):
        return "\\\\?\\UNC\\" + path[2:]
    if drive and rest.startswith("\\"):
        return "\\\\?\\" + path
    return None


if sys.argv[1] == "layout":
    info = ByHandleFileInformation
    print(json.dumps({"bytes": ctypes.sizeof(info), "attributes": info.dwFileAttributes.offset, "volume": info.dwVolumeSerialNumber.offset,
                      "links": info.nNumberOfLinks.offset, "indexHigh": info.nFileIndexHigh.offset, "indexLow": info.nFileIndexLow.offset,
                      "overlapped": ctypes.sizeof(Overlapped)}))
else:
    print(json.dumps([extended(path) for path in sys.argv[2:]]))
