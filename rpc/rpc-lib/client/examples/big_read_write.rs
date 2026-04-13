use client::RpcServer;
use common::SeekFrom;

fn main() {
    let mut client = RpcServer::new("malinka.tailb2454f.ts.net:4444", 5000).unwrap();

    // Open a file
    match client.open("test.txt".to_string(), "w+".to_string()) {
        Ok(_) => println!("File opened!"),
        Err(e) => println!("Failed to open: {:?}", e),
    }

    // Write data
    client.write(vec![b'a'; 4096 * 3]).ok();

    // Seek to start and read
    client.lseek(SeekFrom::Start(0)).ok();
    let data = client.read(4096 * 3).unwrap();
    println!("Read: {}", String::from_utf8(data.unwrap()).unwrap());
}
