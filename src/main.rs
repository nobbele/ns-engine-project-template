fn main() {
    nsengine::run(Some(include_bytes!("../resources.zip").to_vec())).unwrap();
}
