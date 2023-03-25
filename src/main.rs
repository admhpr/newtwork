use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::time::Duration;
use std::thread;

fn main() {
    let ip_range = "192.168.1.1-192.168.1.10"; // The IP range to scan
    let port = 80; // The port number to scan

    // Parse the IP range and generate a vector of IP addresses to scan
    let mut addrs:Vec<IpAddr> = Vec::new();
    for addr_str in ip_range.split('-') {
        let addr = addr_str.parse::<Ipv4Addr>().unwrap().to_string();
        addrs.push(addr.parse().unwrap());
    }

    let mut threads = vec![];

    for addr in addrs {
        let port = port;
        let socket_addr = (addr, port).into();

        // Set a timeout value for the socket to avoid hanging indefinitely
        let timeout = Duration::from_secs(5);

        // Spawn a new thread for each IP address to scan in parallel
        let handle = thread::spawn(move || {
            // Use the socket_addr and timeout values to scan the specified IP address and port
            match TcpStream::connect_timeout(&socket_addr, timeout) {
                Ok(_) => println!("{}:{} is open", addr, port),
                Err(_) => println!("{}:{} is closed", addr, port),
            }
        });
        threads.push(handle);
    }

    // Wait for all threads to finish before exiting
    for handle in threads {
        handle.join().unwrap();
    }
}