use libloading::{Library, Symbol};
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::shared::ntdef::HANDLE;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;

fn main() {
    let library = unsafe { Library::new("kernel32.dll") }.expect("Failed to load kernel32.dll");

    // Define the OpenProcess function signature
    type OpenProcessFunc = extern "system" fn(DWORD, BOOL, DWORD) -> HANDLE;

    // Load the OpenProcess function from the library
    let open_process: Symbol<OpenProcessFunc> = unsafe {
        library
            .get(b"OpenProcess")
            .expect("Failed to get OpenProcess function")
    };

    let process_handle = open_process(PROCESS_QUERY_INFORMATION, 0, std::process::id() as DWORD);

    if process_handle.is_null() {
        eprintln!("Failed to open the process");
    } else {
        println!(
            "Successfully opened current process with handle: {:?}",
            process_handle
        );

        // You can perform additional operations with the process handle here

        // https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle#return-value
        let sucess = unsafe { winapi::um::handleapi::CloseHandle(process_handle) } != 0;
        if sucess {
            println!("Successfully closed the process handle");
        } else {
            eprintln!("Failed to close the process handle");
        }
    }
}
