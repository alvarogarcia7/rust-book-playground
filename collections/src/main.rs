
fn main() {
    let mut v = vec![100, 32, 57];

    v.push(1);

    for i in &v {
        println!("{}", i);
    }
}
