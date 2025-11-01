// src/io.rs

use core::slice;
use crate::sahne64_sys::{sys_write, STDOUT_HANDLE}; 

/// Bir dizeyi (string slice) STDOUT'a (Ekran) yazar.
pub fn print_str(s: &str) {
    let bytes = s.as_bytes();
    let ptr = bytes.as_ptr();
    let len = bytes.len() as u64;

    sys_write(STDOUT_HANDLE, ptr, len);
}

// NOTE: No-std ortamında sayıları (u64, usize) ekrana yazdırmak için
// özel bir fonksiyon yazılması gerekir (örneğin itoa algoritması ile),
// bu işlev şimdilik karmaşıklığı azaltmak için atlanmıştır.
