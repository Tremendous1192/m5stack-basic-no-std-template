//! 組込みRustのおまじない
#![no_std]
#![no_main]
extern crate alloc; // 配列を扱う
use esp_backtrace as _; // エラーハンドリング
use esp_println::println; // Espressif devicesでprintlnマクロを扱う
use hal::prelude::*; // tomlファイルにて as 構文でesp32-halから名前を変えている

#[global_allocator] // ヒープ
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

// ヒープの初期化
fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;

    // C言語を扱う
    extern "C" {
        static mut _heap_start: u32;
        static mut _heap_end: u32;
    }

    // メモリ割り当て
    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        let heap_end = &_heap_end as *const _ as usize;
        assert!(
            heap_end - heap_start > HEAP_SIZE,
            "Not enough available heap memory."
        );
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

#[hal::prelude::entry]
fn main() -> ! {
    // 初期化
    // 必須
    init_heap();
    let peripherals = hal::peripherals::Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = hal::clock::ClockControl::boot_defaults(system.clock_control).freeze();
    // Low-power Management
    let mut rtc = hal::Rtc::new(peripherals.RTC_CNTL);

    // メモリ番地の異なる2箇所に保存した時刻を基にしてタイマーを設定する
    let timer_group0 = hal::timer::TimerGroup::new(
        peripherals.TIMG0, // 0x3ff5f000
        &clocks,
        &mut system.peripheral_clock_control,
    );
    // Watchdog timer
    let mut wdt0 = timer_group0.wdt;

    // もう一組
    let timer_group1 = hal::timer::TimerGroup::new(
        peripherals.TIMG1, // 0x3ff60000
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    // それぞれのタイマーの停止(初期化に当たるのか?)
    rtc.rwdt.disable(); // RTC Watchdog Timer
    wdt0.disable(); // Watchdog timer
    wdt1.disable();
    println!("Hello world!");
    // ここまで 初期化

    // 組込みはloop必須
    loop {}
    // ここまでloop処理
}
// ここまでmain関数
