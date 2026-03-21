use client::RpcServer;

fn main() {
    let mut client = RpcServer::new("malinka.tailb2454f.ts.net:4444", 5000).unwrap();

    let name1 = "test.txt".to_string();
    let name2 = "test2.txt".to_string();

    let data = client.rename(name1.clone(), name2.clone()).unwrap();
    match client.open(name1, "r".to_string()) {
        Ok(rpc_result) => match rpc_result {
            Ok(()) => println!("open successful"),
            Err(e) => eprintln!("open error: {:?} - should be Open", e),
        },
        Err(e) => eprintln!("client error: {:?}", e),
    }
    match client.open(name2, "r".to_string()) {
        Ok(rpc_result) => match rpc_result {
            Ok(()) => println!("open successful"),
            Err(e) => eprintln!("open error: {:?} - should be Open", e),
        },
        Err(e) => eprintln!("client error: {:?}", e),
    }
}
