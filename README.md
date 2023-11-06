# Example of a mixed WinApi and libloading usage

This example demonstrates how [`winapi`](https://crates.io/crates/winapi) can be used together with [`libloading`](https://crates.io/crates/libloading) where:

* `libloading` is used to load the [`kernel32.dll`](https://en.wikipedia.org/wiki/Microsoft_Windows_library_files#KERNEL32.DLL) library and get [`OpenProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) (callable) symbol from it.
* [`winapi`](https://crates.io/crates/winapi) is used to call [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) function.
