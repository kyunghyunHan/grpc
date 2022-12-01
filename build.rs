fn main() {
    println!("dd");

    tonic_build::configure()
        .type_attribute(".", "#[derive(Hash,Eq)]")
        .out_dir("src/pb")
        .compile(&["./abi.proto"], &["./"])
        .unwrap();
}
