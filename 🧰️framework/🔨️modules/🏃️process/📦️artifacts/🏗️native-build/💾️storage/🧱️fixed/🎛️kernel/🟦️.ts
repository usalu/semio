/** 🪟️ Emits only the fixed owner's in-memory system ABI roster; no compiler or assembly artifact is created.
 * @see https://learn.microsoft.com/en-us/dotnet/api/system.reflection.emit.typebuilder.definepinvokemethod
 */
export function nativeFixedWindowsKernelV1():string{return String.raw`
if($ExecutionContext.SessionState.LanguageMode -ne 'FullLanguage'){throw 'FullLanguage system ABI required'}
$system=[Environment]::SystemDirectory
$kernel=[IO.Path]::Combine($system,'kernel32.dll')
if(![IO.Path]::IsPathRooted($kernel)-or ![IO.File]::Exists($kernel)){throw 'Actual OS system ABI required'}
$assembly=[AppDomain]::CurrentDomain.DefineDynamicAssembly((New-Object Reflection.AssemblyName('SemioNativeFixed')),[Reflection.Emit.AssemblyBuilderAccess]::Run)
$module=$assembly.DefineDynamicModule('SemioNativeFixed')
$type=$module.DefineType('SemioNativeFixed',[Reflection.TypeAttributes]'Public,Sealed,Abstract')
$roster=@(
 @('GetFileInformationByHandle',[int],[Type[]]@([IntPtr],[IntPtr])),
 @('GetFileInformationByHandleEx',[int],[Type[]]@([IntPtr],[int],[IntPtr],[uint32])),
 @('GetFinalPathNameByHandleW',[uint32],[Type[]]@([IntPtr],[IntPtr],[uint32],[uint32])),
 @('GetVolumePathNameW',[int],[Type[]]@([string],[IntPtr],[uint32])),
 @('GetVolumeNameForVolumeMountPointW',[int],[Type[]]@([string],[IntPtr],[uint32])),
 @('GetVolumeInformationByHandleW',[int],[Type[]]@([IntPtr],[IntPtr],[uint32],[IntPtr],[IntPtr],[IntPtr],[IntPtr],[uint32])),
 @('CreateFileW',[IntPtr],[Type[]]@([string],[uint32],[uint32],[IntPtr],[uint32],[uint32],[IntPtr])),
 @('CloseHandle',[int],[Type[]]@([IntPtr]))
)
foreach($entry in $roster){$method=$type.DefinePInvokeMethod($entry[0],$kernel,[Reflection.MethodAttributes]'Public,Static,PinvokeImpl',[Reflection.CallingConventions]::Standard,$entry[1],$entry[2],[Runtime.InteropServices.CallingConvention]::Winapi,[Runtime.InteropServices.CharSet]::Unicode);$method.SetImplementationFlags($method.GetMethodImplementationFlags()-bor [Reflection.MethodImplAttributes]::PreserveSig)}
$native=$type.CreateType()
`;}
