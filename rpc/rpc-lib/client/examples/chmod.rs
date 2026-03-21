use client::RpcServer;

fn main() {
    let mut client = RpcServer::new("malinka.tailb2454f.ts.net:4444", 5000).unwrap();

    let data = client.chmod("test.txt".to_string(), 0o777).unwrap();
}
