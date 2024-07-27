use std::io::Write;

fn write(name: str) {
   let mut file = std::fs::File::create(name).expect("create failed");
   file.write_all("Hello World".as_bytes()).expect("write failed");
   file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
   println!("data written to file" );
}
