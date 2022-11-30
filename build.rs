fn main() {
    println!("dd");

    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["./abi.proto"], &["./"])
        .unwrap();
}
