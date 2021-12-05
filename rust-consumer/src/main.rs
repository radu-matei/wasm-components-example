wit_bindgen_rust::import!("../cache.wit");

fn main() {
    let key = "five-good-emperors";
    let value = "Nerva, Trajan, Hadrian, Pius, and Marcus Aurelius";
    cache::set(key, value.as_bytes(), None).unwrap();
    let ret = cache::get(key).unwrap();
    assert_eq!(ret, value.as_bytes());
    println!(
        "Retrieved from {}: {}",
        key,
        std::str::from_utf8(value.as_bytes()).unwrap()
    );
}
