fn main() {
    let mut v = Vec::new(); // create a vector
    v.push('R'); // set R as the first element of vector
    v.push('U');
    v.push('B');
    v.push('Y');
    for n in v {
        print!("{}", n);
    }
}
