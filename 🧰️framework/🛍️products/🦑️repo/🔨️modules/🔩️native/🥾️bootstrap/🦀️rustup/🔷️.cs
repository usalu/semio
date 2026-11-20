using System;
using System.ComponentModel;
using System.Runtime.InteropServices;
using System.Text;
using Microsoft.Win32.SafeHandles;

namespace Semio.Bootstrap
{
    public static class RustupProcess
    {
        [StructLayout(LayoutKind.Sequential)]
        private struct StartupInfo
        {
            public int Size;
            public IntPtr Reserved, Desktop, Title;
            public int X, Y, Width, Height, Columns, Rows, Fill, Flags;
            public short ShowWindow, ReservedSize;
            public IntPtr ReservedData, Input, Output, Error;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct StartupInfoEx
        {
            public StartupInfo Startup;
            public IntPtr Attributes;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct ProcessInformation
        {
            public IntPtr Process, Thread;
            public uint ProcessId, ThreadId;
        }

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern IntPtr GetStdHandle(int handle);

        [DllImport("kernel32.dll")]
        private static extern IntPtr GetCurrentProcess();

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool DuplicateHandle(IntPtr sourceProcess, IntPtr source, IntPtr targetProcess, out SafeFileHandle target, uint access, bool inherit, uint options);

        [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode, ExactSpelling = true)]
        private static extern SafeFileHandle CreateFileW(string path, uint access, uint share, IntPtr security, uint disposition, uint flags, IntPtr template);

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool InitializeProcThreadAttributeList(IntPtr attributes, int count, int flags, ref IntPtr size);

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool UpdateProcThreadAttribute(IntPtr attributes, uint flags, IntPtr attribute, IntPtr value, IntPtr size, IntPtr previous, IntPtr returned);

        [DllImport("kernel32.dll")]
        private static extern void DeleteProcThreadAttributeList(IntPtr attributes);

        [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode, ExactSpelling = true)]
        private static extern bool CreateProcessW(string application, StringBuilder command, IntPtr processSecurity, IntPtr threadSecurity,
            bool inherit, uint flags, IntPtr environment, string directory, ref StartupInfoEx startup, out ProcessInformation process);

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern uint WaitForSingleObject(IntPtr handle, uint milliseconds);

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool GetExitCodeProcess(IntPtr process, out uint exit);

        [DllImport("kernel32.dll")]
        private static extern bool CloseHandle(IntPtr handle);

        private static SafeFileHandle Inherit(IntPtr source)
        {
            var parent = GetCurrentProcess();
            SafeFileHandle copy;
            if (!DuplicateHandle(parent, source, parent, out copy, 0, true, 2))
                throw new Win32Exception();
            return copy;
        }

        private static SafeFileHandle StandardHandle(int identifier, uint access)
        {
            var handle = GetStdHandle(identifier);
            if (handle != IntPtr.Zero && handle != new IntPtr(-1))
                return Inherit(handle);
            using (var empty = CreateFileW("NUL", access, 3, IntPtr.Zero, 3, 0, IntPtr.Zero))
            {
                if (empty.IsInvalid)
                    throw new Win32Exception();
                return Inherit(empty.DangerousGetHandle());
            }
        }

        /// <summary>🔒 Gives the pinned installer and its original output streams an explicitly inherited lock until the installer exits.</summary>
        public static int Run(SafeFileHandle held, string program, string directory, bool bootstrap)
        {
            using (var installerLock = Inherit(held.DangerousGetHandle()))
            using (var input = StandardHandle(-10, 0x80000000))
            using (var output = StandardHandle(-11, 0x40000000))
            using (var error = StandardHandle(-12, 0x40000000))
            {
                var size = IntPtr.Zero;
                InitializeProcThreadAttributeList(IntPtr.Zero, 1, 0, ref size);
                if (size == IntPtr.Zero)
                    throw new Win32Exception();
                var attributes = Marshal.AllocHGlobal(size);
                var handles = IntPtr.Zero;
                var initialized = false;
                try
                {
                    if (!InitializeProcThreadAttributeList(attributes, 1, 0, ref size))
                        throw new Win32Exception();
                    initialized = true;
                    var allowed = new[] { installerLock.DangerousGetHandle(), input.DangerousGetHandle(), output.DangerousGetHandle(), error.DangerousGetHandle() };
                    var handleBytes = allowed.Length * IntPtr.Size;
                    handles = Marshal.AllocHGlobal(handleBytes);
                    Marshal.Copy(allowed, 0, handles, allowed.Length);
                    if (!UpdateProcThreadAttribute(attributes, 0, new IntPtr(0x00020002), handles, new IntPtr(handleBytes), IntPtr.Zero, IntPtr.Zero))
                        throw new Win32Exception();
                    var startup = new StartupInfoEx
                    {
                        Startup = new StartupInfo { Size = Marshal.SizeOf(typeof(StartupInfoEx)), Flags = 0x100, Input = allowed[1], Output = allowed[2], Error = allowed[3] },
                        Attributes = attributes
                    };
                    var arguments = bootstrap ? "-y --no-modify-path --profile minimal --default-toolchain none" : "toolchain install --no-self-update";
                    var command = new StringBuilder("\"" + program + "\" " + arguments);
                    ProcessInformation process;
                    if (!CreateProcessW(program, command, IntPtr.Zero, IntPtr.Zero, true, 0x00080000, IntPtr.Zero, directory, ref startup, out process))
                        throw new Win32Exception();
                    CloseHandle(process.Thread);
                    try
                    {
                        uint exit;
                        if (WaitForSingleObject(process.Process, uint.MaxValue) != 0 || !GetExitCodeProcess(process.Process, out exit))
                            throw new Win32Exception();
                        return unchecked((int)exit);
                    }
                    finally
                    {
                        CloseHandle(process.Process);
                    }
                }
                finally
                {
                    if (initialized)
                        DeleteProcThreadAttributeList(attributes);
                    if (handles != IntPtr.Zero)
                        Marshal.FreeHGlobal(handles);
                    Marshal.FreeHGlobal(attributes);
                }
            }
        }
    }
}
