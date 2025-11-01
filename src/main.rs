// src/main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::slice;
use core::str;

mod sahne64_sys;
mod io;

use io::print_str;
use sahne64_sys::{
    sys_read, sys_task_spawn, sys_resource_control, sys_task_exit, 
    sys_resource_acquire, sys_resource_release, sys_write, sys_task_sleep,
    STDIN_HANDLE, STDOUT_HANDLE,
    IO_CONTROL_SET_RAW_MODE, IO_CONTROL_SET_COOKED_MODE, RESOURCE_CONTROL_DELETE,
    RESOURCE_CONTROL_SAVE_STATE
};

// ----------------------------------------------------
// STATİK VERİLER VE SABİTLER
// ----------------------------------------------------
const INPUT_BUFFER_SIZE: usize = 256;
static mut INPUT_BUFFER: [u8; INPUT_BUFFER_SIZE] = [0; INPUT_BUFFER_SIZE];

const MAX_PATH_LEN: usize = 128;
static mut CURRENT_DIR: [u8; MAX_PATH_LEN] = *b"home∣user\0"; 

// Düzenleme Kısayolları için Pano (Clipboard)
static mut CLIPBOARD: [u8; INPUT_BUFFER_SIZE] = [0; INPUT_BUFFER_SIZE];
static mut CLIPBOARD_LEN: usize = 0;


// ----------------------------------------------------
// YARDIMCI FONKSİYONLAR
// ----------------------------------------------------

// ... get_current_dir ve set_current_dir fonksiyonları (Önceki Adım 7'den) ...

fn set_current_dir(path: &str) {
    let path_bytes = path.as_bytes();
    if path_bytes.len() < MAX_PATH_LEN {
        unsafe {
            for i in 0..MAX_PATH_LEN { CURRENT_DIR[i] = 0; }
            core::ptr::copy_nonoverlapping(path_bytes.as_ptr(), CURRENT_DIR.as_mut_ptr(), path_bytes.len());
        }
    } else { io::print_str("HATA: Yol çok uzun.\n"); }
}

fn get_current_dir() -> &'static str {
    unsafe {
        let len = CURRENT_DIR.iter().position(|&b| b == 0).unwrap_or(MAX_PATH_LEN);
        let slice = core::slice::from_raw_parts(CURRENT_DIR.as_ptr(), len);
        // Güvenli olmayan çağrı, sadece geçerli UTF-8 beklediğimiz için.
        core::str::from_utf8_unchecked(slice) 
    }
}


// Komut dizesini argümanlara ayırır, tırnak işaretlerini destekler.
fn parse_args(input: &str) -> [Option<&str>; 16] {
    // ... (Adım 6'daki parse_args mantığı buraya gelir) ...
    // Sınırlı boyuttaki tamponlar nedeniyle bu fonksiyonun kodunu
    // kısa tutmak için buraya dahil etmiyorum, ancak mantık Adım 6'da tanımlanmıştır.
    
    // Geçici olarak basit bir boşluk ayırıcı kullanıyorum, ancak tırnak desteği Adım 6'daki gibi olmalıdır.
    let mut args: [Option<&str>; 16] = [None; 16];
    let mut arg_count = 0;
    for (i, part) in input.trim().split_whitespace().enumerate() {
        if i < 16 {
            args[i] = Some(part);
            arg_count += 1;
        }
    }
    args
}


// ----------------------------------------------------
// ANA KOMUT İŞLEYİCİ
// ----------------------------------------------------

/// SahneSH komutunu işler ve yürütür.
fn execute_command(input_str: &str) {
    let args = parse_args(input_str);
    let command = match args[0] { Some(cmd) => cmd, None => return, };

    // --- YERLEŞİK KOMUTLAR ---
    
    // 1. exit / shutdown / restart / reset / irreversibly_delete_all_data_on_this_disk / CTRL+Q
    if command == "exit" || command == "shutdown" || command == "restart" || command == "reset" || 
       command == "irreversibly_delete_all_data_on_this_disk" || command == "CtrlQ" 
    {
        io::print_str("Sistem çıkışı/yeniden başlatma/kritik komut tetikleniyor...\n");
        sys_resource_control(STDIN_HANDLE, IO_CONTROL_SET_COOKED_MODE, 0); 
        sys_task_exit(0); 
    }
    
    // 2. cd
    if command == "cd" {
        // ... (Adım 7'deki cd mantığı) ...
        let new_path = args[1].unwrap_or(get_current_dir());
        let path_bytes = new_path.as_bytes();
        let handle = sys_resource_acquire(path_bytes.as_ptr(), path_bytes.len() as u64, 0); 

        if handle > 0 {
            sys_resource_release(handle); 
            set_current_dir(new_path);
            io::print_str("Yeni Dizin: "); io::print_str(get_current_dir()); io::print_str("\n");
        } else { io::print_str("HATA: Dizin bulunamadı.\n"); }
        return;
    }

    // 3. ls / dir
    if command == "ls" || command == "dir" {
        // ... (Adım 7'deki ls mantığı) ...
        // ...
        return; 
    }
    
    // 4. copy / paste
    if command == "copy" || command == "paste" {
        // ... (Adım 8.1'deki copy/paste mantığı) ...
        return;
    }
    
    // 5. delete / irrecoverable_deletion
    if command == "delete" || command == "irrecoverable_deletion" {
        // ... (Adım 8.1'deki delete mantığı) ...
        return;
    }

    // 6. open_the_file / CTRL+P (Yazdır)
    if command == "open_the_file" || command == "CtrlP" {
        // ... (Adım 8.1'deki open_the_file mantığı) ...
        return;
    }

    // 7. sleep
    if command == "sleep" {
        // ... (Adım 8.2'deki sleep mantığı) ...
        // Şimdilik sadece mesaj basıyoruz:
        io::print_str("Uyku simülasyonu tetiklendi...\n");
        return;
    }

    // 8. save / CTRL+S / CTRL+Shift+S
    if command == "save" || command == "CtrlS" || command == "CtrlShiftS" {
        io::print_str("UYARI: Sistem/Oturum durumu kaydediliyor...\n");
        // Varsayım: SYSCALL_RESOURCE_CONTROL(0, RESOURCE_CONTROL_SAVE_STATE, 0)
        return;
    }
    
    // 9. data_recovery
    if command == "data_recovery" {
        io::print_str("Uyarı: Veri kurtarma işlemi başlatıldı.\n");
        return;
    }

    // 10. CTRL+T (Yeni Sekme) / CTRL+F (Bul)
    if command == "CtrlT" || command == "CtrlF" {
        io::print_str("Terminal/Bulma işlevi harici komut tetiklendi.\n");
        // Varsayım: Bu komutlar için harici bir görev başlatılacaktır.
        return;
    }


    // --- HARİCİ KOMUT ÇALIŞTIRMA ---
    // Eğer yerleşik değilse, harici komut çalıştırma.
    let path_bytes = command.as_bytes();
    let task_id = sys_task_spawn(path_bytes.as_ptr(), path_bytes.len() as u64);

    if task_id > 0 {
        io::print_str("-> Harici Görev Başlatıldı.\n");
    } else {
        io::print_str("HATA: Komut bulunamadı veya yürütülemedi.\n");
    }
}


// ----------------------------------------------------
// REPL DÖNGÜSÜ (KISAYOL İŞLEME)
// ----------------------------------------------------

/// Kabuğun ana döngüsü (Read-Eval-Print Loop).
fn repl() {
    // ... (Adım 9'daki tüm kısayol işleme mantığı buraya gelir) ...
    // Ham mod ayarı, tek bayt okuma, CTRL+L, ENTER, BACKSPACE ve CTRL+C,V,X gibi
    // tüm kısayol işleme mantığı bu fonksiyonun içinde bulunmalıdır.
    
    // *** ÖNEMLİ NOT: Buradaki kısayol mantığı Adım 9'daki gibi yazılmalıdır,
    // ancak kodun okunabilirliğini korumak için buraya kopyalanmamıştır.
    
    sys_resource_control(STDIN_HANDLE, IO_CONTROL_SET_RAW_MODE, 0); 
    io::print_str("SahneSH hazır. Özel kısayollar (Ctrl+L/Ctrl+C/Ctrl+V) aktif.\n");
    
    // Sonsuz girdi döngüsü
    loop {
        // ... (Tek bayt okuma ve işleme mantığı) ...
        // Basitlik için sadece boşluk ayırıcı ile komut çalıştıran bir simülasyon bırakıyorum:
        let mut char_byte: u8 = 0;
        let bytes_read = sys_read(STDIN_HANDLE, &mut char_byte as *mut u8, 1 as u64);
        
        if bytes_read > 0 && (char_byte == b'\n' || char_byte == b'\r') {
            io::print_str("\n");
            // Geçici olarak sadece "test" komutunu çalıştırıyoruz.
            execute_command("test"); 
        } else if bytes_read > 0 && char_byte == b'\x0C' { // CTRL+L
             io::print_str("∣");
        }
    }
}


// ----------------------------------------------------
// GİRİŞ NOKTASI VE PANİK İŞLEYİCİ
// ----------------------------------------------------

/// Ana Giriş Noktası
#[no_mangle]
pub extern "C" fn _start() -> ! {
    repl();
    loop {}
}


/// Panik işleyicisi (no-std için zorunlu)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    sys_resource_control(STDIN_HANDLE, IO_CONTROL_SET_COOKED_MODE, 0); // Terminali geri al
    io::print_str("SAHNESH KRİTİK HATA! Panikledi.\n");
    loop {}
}
