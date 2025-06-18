// fn dangling_ref() -> &String {
//     let s = String::from("Rust");
//     &s // ❌ error: s akan di-drop saat keluar fungsi
// }