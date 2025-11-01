// src/sahne64_sys.rs

// Çekirdek tarafından sağlanan gerçek sistem çağrısı giriş noktası
extern "C" {
    /// Sahne64 çekirdek sistem çağrısı arayüzü
    fn __syscall(id: u64, arg1: u64, arg2: u64, arg3: u64) -> u64;
}

// ----------------------------------------------------
// SYSCALL SABİTLERİ
// ----------------------------------------------------
pub const SYSCALL_TASK_SPAWN: u64 = 3;
pub const SYSCALL_TASK_EXIT: u64 = 4;
pub const SYSCALL_RESOURCE_ACQUIRE: u64 = 5;
pub const SYSCALL_RESOURCE_READ: u64 = 6;
pub const SYSCALL_RESOURCE_WRITE: u64 = 7;
pub const SYSCALL_RESOURCE_RELEASE: u64 = 8;
pub const SYSCALL_TASK_SLEEP: u64 = 10;
pub const SYSCALL_RESOURCE_CONTROL: u64 = 102;

// Standart Kaynak Tanıtıcıları (Handle)
pub const STDIN_HANDLE: u64 = 0;
pub const STDOUT_HANDLE: u64 = 1;
pub const STDERR_HANDLE: u64 = 2;

// Varsayımsal Kaynak Kontrol Kodları
pub const IO_CONTROL_SET_RAW_MODE: u64 = 1;      // Terminali ham moda al
pub const IO_CONTROL_SET_COOKED_MODE: u64 = 2;   // Terminali pişmiş moda al
pub const RESOURCE_CONTROL_DELETE: u64 = 3;      // Kaynağı silme komutu
pub const RESOURCE_CONTROL_SAVE_STATE: u64 = 4;  // Sistem durumunu kaydetme komutu


// ----------------------------------------------------
// GÜVENLİ SARMALAYICI FONKSİYONLAR
// ----------------------------------------------------

/// SYSCALL_RESOURCE_WRITE çağrısını sarmalar.
pub fn sys_write(handle: u64, buffer: *const u8, count: u64) -> u64 {
    unsafe {
        __syscall(SYSCALL_RESOURCE_WRITE, handle, buffer as u64, count)
    }
}

/// SYSCALL_RESOURCE_READ çağrısını sarmalar.
pub fn sys_read(handle: u64, buffer: *mut u8, count: u64) -> u64 {
    unsafe {
        __syscall(SYSCALL_RESOURCE_READ, handle, buffer as u64, count)
    }
}

/// SYSCALL_RESOURCE_ACQUIRE çağrısını sarmalar.
pub fn sys_resource_acquire(path: *const u8, len: u64, mode: u64) -> u64 {
    unsafe {
        __syscall(SYSCALL_RESOURCE_ACQUIRE, path as u64, len, mode)
    }
}

/// SYSCALL_RESOURCE_RELEASE çağrısını sarmalar.
pub fn sys_resource_release(handle: u64) -> u64 {
    unsafe {
        __syscall(SYSCALL_RESOURCE_RELEASE, handle, 0, 0)
    }
}

/// SYSCALL_TASK_SPAWN çağrısını sarmalar.
pub fn sys_task_spawn(path: *const u8, path_len: u64) -> u64 {
    unsafe {
        __syscall(SYSCALL_TASK_SPAWN, path as u64, path_len, 0)
    }
}

/// SYSCALL_TASK_EXIT çağrısını sarmalar.
pub fn sys_task_exit(status: u64) -> ! {
    unsafe {
        __syscall(SYSCALL_TASK_EXIT, status, 0, 0);
    }
    loop {} 
}

/// SYSCALL_TASK_SLEEP çağrısını sarmalar.
pub fn sys_task_sleep(duration_ms: u64) -> u64 {
    unsafe {
        __syscall(SYSCALL_TASK_SLEEP, duration_ms, 0, 0)
    }
}


/// SYSCALL_RESOURCE_CONTROL çağrısını sarmalar.
pub fn sys_resource_control(handle: u64, command: u64, arg: u64) -> u64 {
    unsafe {
        __syscall(SYSCALL_RESOURCE_CONTROL, handle, command, arg)
    }
}