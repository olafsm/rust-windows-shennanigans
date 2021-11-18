#![allow(non_camel_case_types)]

type c_int = i32;
type c_uint = u32;
type HANDLE = PVOID;
type HBRUSH = HANDLE;
type HCURSOR = HICON;
type HICON = HANDLE;
type HINSTANCE = HANDLE;
type HWND = HANDLE;
type LONG_PTR = isize;
type LPARAM = LONG_PTR;
type LPCWSTR = *const WCHAR;
type LRESULT = LONG_PTR;
type PVOID = *mut core::ffi::c_void;
type UINT = c_uint;
type UINT_PTR = usize;
type WCHAR = wchar_t;
type wchar_t = u16;
type WPARAM = UINT_PTR;

type WNDPROC = Option<
  unsafe extern "system" fn(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
  ) -> LRESULT,
>;

#[repr(C)]
pub struct WNDCLASSW {
  style: UINT,
  lpfnWndProc: WNDPROC,
  cbClsExtra: c_int,
  cbWndExtra: c_int,
  hInstance: HINSTANCE,
  hIcon: HICON,
  hCursor: HCURSOR,
  hbrBackground: HBRUSH,
  lpszMenuName: LPCWSTR,
  lpszClassName: LPCWSTR,
}

impl Default for WNDCLASSW {
    #[inline]
    #[must_use]
    fn default() -> Self {
      unsafe { core::mem::zeroed() }
    }
  }
  
macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
      impl Default for $t {
        #[inline]
        #[must_use]
        fn default() -> Self {
          unsafe { core::mem::zeroed() }
        }
      }
    };
  }

unsafe extern "system" fn dummy_window_procedure(
    hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM,
) -> LRESULT {
    unimplemented!()
}
#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
}

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

type ATOM = WORD;
type WORD = c_ushort;
type c_ushort = u16;
#[link(name = "User32")]
extern "system" {
    /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
} 

type DWORD = c_ulong;
type c_ulong = u32;

#[link(name = "Kernel32")]
extern "system" {
    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
}

type HMENU = HANDLE;
type LPVOID = *mut core::ffi::c_void;
#[link(name = "User32")] 
extern "system" {
    /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
      dwExStyle: DWORD, 
      lpClassName: LPCWSTR, 
      lpWindowName: LPCWSTR,
      dwStyle: DWORD, 
      X: c_int, 
      Y: c_int, 
      nWidth: c_int, 
      nHeight: c_int,
      hWndParent: HWND, 
      hMenu: HMENU, 
      hInstance: HINSTANCE, 
      lpParam: LPVOID,
    ) -> HWND;
  }
  
fn main() {
    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");

    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = Some(dummy_window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = sample_window_class_wn.as_ptr();

    let atom = unsafe { RegisterClassW(&wc) };
    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }


}

