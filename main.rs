use std::io::{self, Write};
use std::thread;
use std::time::Duration;


struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        // Seed dari waktu sistem (pakai nilai hardcoded jika tidak ada std::time)
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as u64;
        Self { state: seed ^ 0xDEADBEEF }
    }

    // Generate float antara -max dan +max
    fn gen_range_f32(&mut self, min: f32, max: f32) -> f32 {
        // LCG formula standar
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let normalized = (self.state >> 33) as f32 / (u32::MAX as f32);
        min + normalized * (max - min)
    }
}

// --- 1. KONSEP OOP: Struct untuk Sensor ---
struct Sensor {
    nama: String,
    current_level: f32,
    rng: SimpleRng, // <-- RNG disimpan di dalam Sensor
}

impl Sensor {
    fn new(nama: &str) -> Self {
        Self {
            nama: nama.to_string(),
            current_level: 50.0, // mulai dari tengah (50%)
            rng: SimpleRng::new(),
        }
    }

    // DIUBAH: Tidak lagi minta input user,
    // tapi generate nilai otomatis berdasarkan level sebelumnya
    fn baca_data(&mut self) -> Result<f32, String> {
        // Simulasi perubahan level: naik/turun pelan (-5% sampai +5%)
        let delta = self.rng.gen_range_f32(-5.0, 5.0);
        let new_level = (self.current_level + delta).clamp(0.0, 100.0);
        self.current_level = new_level;
        Ok(new_level)
    }
}

// --- 2. KONSEP OOP: Struct untuk Controller ---
struct Controller {
    status_pompa: bool,
    threshold_bawah: f32,
    threshold_atas: f32,
}

impl Controller {
    fn new(bawah: f32, atas: f32) -> Self {
        Self {
            status_pompa: false,
            threshold_bawah: bawah,
            threshold_atas: atas,
        }
    }

    // LOGIKA PENGAMBILAN KEPUTUSAN (tidak berubah)
    fn kontrol_pompa(&mut self, level: f32) {
        if level < self.threshold_bawah {
            self.status_pompa = true;
            println!("[KONTROL] Status: AIR RENDAH -> Pompa otomatis NYALA (ON)");
        } else if level > self.threshold_atas {
            self.status_pompa = false;
            println!("[KONTROL] Status: TANGKI PENUH -> Pompa otomatis MATI (OFF)");
            println!("[ALARM]   !!! PERINGATAN: LEVEL AIR KRITIS !!!");
        } else {
            println!(
                "[KONTROL] Status: Aman. Pompa tetap {}",
                if self.status_pompa { "ON" } else { "OFF" }
            );
        }
    }
}

// --- 3. KONSEP OOP: Struct Monitoring System ---
struct MonitoringSystem {
    sensor_tangki: Sensor,
    pompa_controller: Controller,
    riwayat_data: Vec<f32>,
    interval_detik: u64, // BARU: interval antar pembacaan
}

impl MonitoringSystem {
    fn new(interval_detik: u64) -> Self {
        Self {
            sensor_tangki: Sensor::new("Sensor Ultrasonik HC-SR04"),
            pompa_controller: Controller::new(20.0, 90.0),
            riwayat_data: Vec::new(),
            interval_detik,
        }
    }

    // KOMPUTASI NUMERIK: Menghitung Rata-rata
    fn hitung_rata_rata(&self) -> f32 {
        if self.riwayat_data.is_empty() {
            return 0.0;
        }
        let total: f32 = self.riwayat_data.iter().sum();
        total / self.riwayat_data.len() as f32
    }

    // BARU: Visualisasi bar sederhana di terminal
    fn tampilkan_bar(&self, level: f32) {
        let filled = (level / 5.0) as usize; // 20 karakter = 100%
        let empty = 20 - filled;
        print!("Level: [");
        print!("{}", "█".repeat(filled));
        print!("{}", "░".repeat(empty));
        println!("] {:.1}%", level);
    }

    fn mulai_monitoring(&mut self) {
        println!("===========================================");
        println!(" SISTEM MONITORING LEVEL TANGKI - KELOMPOK X");
        println!(" Sensor Aktif : {}", self.sensor_tangki.nama);
        println!(" Interval     : {} detik", self.interval_detik);
        println!(" Threshold    : Bawah=20% | Atas=90%");
        println!("===========================================");
        println!(" Tekan Ctrl+C untuk menghentikan sistem.");
        println!("===========================================\n");

        // DIUBAH: loop otomatis, tidak perlu input manual
        loop {
            println!("--- PEMBACAAN #{} ---", self.riwayat_data.len() + 1);

            match self.sensor_tangki.baca_data() {
                Ok(nilai) => {
                    self.riwayat_data.push(nilai);
                    self.tampilkan_bar(nilai);          // tampilkan bar
                    self.pompa_controller.kontrol_pompa(nilai);
                    println!("Rata-rata : {:.2}%", self.hitung_rata_rata());
                }
                Err(pesan_error) => println!("ERROR: {}", pesan_error),
            }

            println!();

            // Tunggu sesuai interval sebelum pembacaan berikutnya
            thread::sleep(Duration::from_secs(self.interval_detik));
        }
    }
}

fn main() {
    // Tanya user mau interval berapa detik
    print!("Masukkan interval monitoring (detik, contoh: 2): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let interval: u64 = input.trim().parse().unwrap_or(2); // default 2 detik

    let mut app = MonitoringSystem::new(interval);
    app.mulai_monitoring();
}