use client::RpcServer;
use common::{PACKET_SIZE, SeekFrom};

fn main() {
    let mut client = RpcServer::new("malinka.tailb2454f.ts.net:4444", 5000).unwrap();

    // Open a file
    match client.open("test.txt".to_string(), "w+".to_string()) {
        Ok(_) => println!("File opened!"),
        Err(e) => println!("Failed to open: {:?}", e),
    }

    // Write data
    match client.write(vec![b'a'; PACKET_SIZE * 3]) {
        Ok(Ok(_)) => println!("Data written!"),
        Ok(Err(e)) => println!("Failed to write: {:?}", e),
        Err(e) => println!("Failed to write: {:?}", e),
    };

    // Seek to start and read
    client.lseek(SeekFrom::Start(0)).ok();
    match client.read(PACKET_SIZE * 3) {
        Ok(Ok(val)) => {
            println!(
                "Read: {}\nLen: {}",
                String::from_utf8(val.clone()).unwrap(),
                val.len()
            )
        }
        Ok(Err(e)) => println!("Failed to read server: {:?}", e),
        Err(e) => println!("Failed to read client: {:?}", e),
    }
}
