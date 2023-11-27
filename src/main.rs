mod udp_server;

fn main() {
    udp_server::start_server("127.0.0.1:7777");
}
