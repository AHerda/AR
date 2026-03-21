use client::RpcServer;
use common::SeekFrom;

fn main() {
    let mut client = RpcServer::new("100.76.125.64:4444", 5000).unwrap();

    // Open a file
    match client.open("test.txt".to_string(), "w+".to_string()) {
        Ok(_) => println!("File opened!"),
        Err(e) => println!("Failed to open: {:?}", e),
    }

    // Write data
    client.write(b"Hello RPC!".to_vec()).ok();

    // Seek to start and read
    client.lseek(SeekFrom::Start(0)).ok();
    let data = client.read(10).unwrap();
    println!("Read: {}", String::from_utf8(data.unwrap()).unwrap());
}
