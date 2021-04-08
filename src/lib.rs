use std::os::windows::prelude::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::{ptr, mem};
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn get_windows_keys(ptr: *const c_char, key_arr: *mut * mut c_char,key_count: *mut i32) {
    let s = unsafe {
        assert!(!ptr.is_null());
        CStr::from_ptr(ptr)
    };
    let r_str = s.to_str().unwrap();
    println!("String {}", r_str);
}

#[no_mangle]
unsafe  extern "C" fn process_string(ptr: *const c_char, str_arr: *mut *mut *mut c_char, outlen: *mut c_int) -> *mut *mut c_char {
    println!("Function process_string=====================================");

    let s = unsafe {
        assert!(!ptr.is_null());
        CStr::from_ptr(ptr)
    };
    let r_str = s.to_str().unwrap();
    println!("String {}", r_str);
    let text = r_str;

    // let text = "2GWD2-NC9C3-T2GR8-3FHYF-8448X,10,0,V
    // 28BPN-66GDT-FRVJC-D6RGW-HMJWX,10,0,H";
    //println!("{}",text);

    let v: Vec<&str> = text.split("\n").collect();
    //println!("{:?}",v);
    let mut key_vector =  Vec::new();
    for item in v.iter(){
        let i:Vec<&str> = item.trim().split(",").collect();
        let ii = i[0];
        //println!("{}",ii);
        //key_vector.push(ii);
        key_vector.push(CString::new(ii).unwrap());
        //v.push(CString::new("Hello").unwrap());
    }
    //println!("{:?}",key_vector);

    // Turning each null-terminated string into a pointer.
    // `into_raw` takes ownershop, gives us the pointer and does NOT drop the data.
    let mut out = key_vector
        .into_iter()
        .map(|s| s.into_raw())
        .collect::<Vec<_>>();

        // Make sure we're not wasting space.
        out.shrink_to_fit();
        assert!(out.len() == out.capacity());
    
        // Get the pointer to our vector.
        let len = out.len();
        let ptr = out.as_mut_ptr();
        mem::forget(out);

         // Let's write back the length the caller can expect
        ptr::write(outlen, len as c_int);

        *str_arr = ptr;
    
        // Finally return the data
        ptr


    
}

#[no_mangle]
unsafe extern "C" fn free_string_array(ptr: *mut *mut c_char, len: c_int) {
    let len = len as usize;

    // Get back our vector.
    // Previously we shrank to fit, so capacity == length.
    let v = Vec::from_raw_parts(ptr, len, len);

    // Now drop one string at a time.
    for elem in v {
        let s = CString::from_raw(elem);
        mem::drop(s);
    }

    // Afterwards the vector will be dropped and thus freed.
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn hello_world(){
        println!("Hello world");
        //process_string();
    }
}


//////////////////////////////////////////////////////////////////////
/// In C file
/// 
/// 
/// 
/// #pragma comment(lib,"WS2_32")
// #pragma comment(lib,"advapi32")
// #pragma comment(lib,"iphlpapi")
// #pragma comment(lib,"psapi")
// #pragma comment(lib,"shell32")
// #pragma comment(lib,"user32")
// #pragma comment(lib,"userenv")



// extern "C" char** get_strings(int* outlen);
// extern "C" void free_string_array(char **ptr, int len);


// int main(int argc, char** argv)
// {
// 	int len;
// 	char** s = get_strings(&len);

// 	for (int i = 0; i<len; i++) {
// 		printf("String %d: %s\n", i, s[i]);
// 	}

// 	free_string_array(s, len);

// 	return 0;
// }