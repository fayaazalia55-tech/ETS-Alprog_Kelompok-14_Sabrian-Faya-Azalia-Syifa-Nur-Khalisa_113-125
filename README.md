

Repositori ini disusun untuk memenuhi tugas Evaluasi Tengah Semester (ETS) mata kuliah Algoritma dan Pemrograman.

Sistem ini merupakan simulasi monitoring dan kontrol level cairan dalam tangki industri. Masalah utama yang diselesaikan adalah risiko kegagalan operasional akibat *human error*, seperti *overflow* dan *dry running*. 

Sistem ini menggunakan:
- **Rust Language:** Memberikan jaminan *memory safety* dan performa tinggi.
- **Moving Average:** Algoritma komputasi numerik untuk menyaring noise data sensor agar kontrol pompa lebih stabil.
- **Logika Kontrol:** Otomatisasi pompa (ON pada level <20% dan OFF pada level >90%).

1. Pastikan Anda telah menginstal **Rust** dan **Cargo**.
2. Clone repositori ini atau download file `main.rs`.
3. Buka terminal di direktori project.
4. Jalankan perintah:
   ```bash
   cargo run
