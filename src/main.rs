use std::net::SocketAddr;

use clap;
use etherparse;
use socket2::{Domain, Protocol, Socket, Type};

fn main() {
    let matches = clap::App::new("deadping")
        .version("0.1.0")
        .author("Steve")
        .about("timing with TTL")
        .arg(clap::Arg::with_name("ip-address")
            .long("ip")
            .value_name("ADDRESS")
            .takes_value(true)
            .help("The IP Address to send the packet to")
            .required(true)
        )
        .arg(clap::Arg::with_name("ttl")
            .long("ttl")
            .value_name("NUMBER")
            .takes_value(true)
            .help("The Time To Live of the packet")
            .required(true)
        )
        .arg(clap::Arg::with_name("interval")
            .short("i")
            .long("interval")
            .value_name("MILLISECONDS")
            .takes_value(true)
            .help("The time between sending packets")
            .default_value("1000")
        )
        .get_matches();
    let ip = matches.value_of("ip-address").unwrap();
    let ttl: u32 = matches.value_of("ttl").unwrap().parse().expect("TTL is not a number");
    let interval: u32 = matches.value_of("interval").unwrap().parse().expect("unable to convert interval to number");
    let interval = std::time::Duration::new(0, 1_000_000 * interval);
    let socket = Socket::new(Domain::ipv4(), Type::raw(), Some(Protocol::icmpv4())).unwrap();
    socket.set_ttl(ttl).expect("Unable to set custom TTL");
    let addr = &format!("{}:1234", ip).parse::<SocketAddr>().expect("IP address is invalid").into();
    let icmp_packet = [0x8, 0x0, 0x44, 0x49, 0xaa, 0xaa, 1, 2, 3, 4, 5, 6].as_ref();
    let mut receive_buffer = [0u8; 1024];
    loop {
        let time = std::time::Instant::now();
        socket.send_to(icmp_packet, addr).expect("Unknown socket error");
        socket.recv(&mut receive_buffer).expect("Unknown socket error");
        let elapsed = time.elapsed().as_millis();
        let source = etherparse::Ipv4HeaderSlice::from_slice(&receive_buffer).expect("Unable to parse response packet").source_addr();
        println!("{}: Time: {}ms", source, elapsed);
        std::thread::sleep(interval);
    }
}
