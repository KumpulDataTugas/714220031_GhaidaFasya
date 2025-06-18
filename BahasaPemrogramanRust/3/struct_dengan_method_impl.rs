struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    println!("Area: {}", c.area());
}
// Output: Area: 78.5
// Penjelasan:  Struct `Circle` memiliki field `radius` dan method `area` yang menghitung luas lingkaran. Method `area` menggunakan `&self` untuk meminjam instance dari `Circle`, sehingga kita bisa mengakses field `radius` tanpa mengambil kepemilikan dari instance tersebut. Ini memungkinkan kita untuk menggunakan instance `c` setelah memanggil method `area`.
// Dengan cara ini, kita bisa membuat method yang beroperasi pada data yang disimpan dalam struct tanpa kehilangan kepemilikan atas data tersebut.