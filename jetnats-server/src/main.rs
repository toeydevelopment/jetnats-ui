use jetnats_server::nats::{INats, Nats};

#[tokio::main]
async fn main() {
    let nc = nats::Options::default()
        .with_name("My Rust NATS App")
        .connect("172.31.6.227:4222")
        .unwrap();

    let n = Nats::new(nc);
    n.list_streams().unwrap().iter().for_each(|stream| {
        println!("{}", stream);
    });
}
