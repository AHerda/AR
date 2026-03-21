use client::RpcServer;

fn main() {
    let mut client = RpcServer::new("malinka.tailb2454f.ts.net:4444", 5000).unwrap();

    let data = client.unlink("test2.txt".to_string()).unwrap();
}
