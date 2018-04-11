extern crate rust_rpm;
extern crate libc;
use rust_rpm::*;
use libc::c_char;
use std::ffi::CStr;
use std::str;

fn main() {
	let specfile = match std::ffi::CString::new("/home/peroyvind/efivar.omv/efivar.spec") {
		Ok(s)  => s,
			Err(e) => return,
	};
	let topdir = match std::ffi::CString::new("_topdir /home/peroyvind/efivar.omv") {
		Ok(s)  => s,
			Err(e) => return,
	};
	let sourcedir = match std::ffi::CString::new("_sourcedir /home/peroyvind/efivar.omv") {
		Ok(s)  => s,
			Err(e) => return,
	};


	let fmt = match std::ffi::CString::new("%|EPOCH?{%{EPOCH}:}|%{VERSION}-%{RELEASE}") {
		Ok(s)  => s,
			Err(e) => return,
	};

	unsafe {
		rpmReadConfigFiles(std::ptr::null_mut(), std::ptr::null_mut());
		rpmDefineMacro(std::ptr::null_mut(), topdir.as_ptr(),0);
		rpmDefineMacro(std::ptr::null_mut(), sourcedir.as_ptr(),0);

		let spec = rpmSpecParse(specfile.as_ptr(), 0, std::ptr::null());
		let header = rpmSpecSourceHeader(spec);
		let c_buf: *const c_char = headerFormat(header, fmt.as_ptr(), std::ptr::null_mut());
		let c_str: &CStr = CStr::from_ptr(c_buf);
		let str_slice: &str = c_str.to_str().unwrap();
		let str_buf: String = str_slice.to_owned();  // if necessary
		println!("s == {:?}", str_buf);
	}

}

