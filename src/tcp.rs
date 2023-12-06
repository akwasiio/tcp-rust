use std::net::{Ipv6Addr, TcpStream};

struct TcpHeader {
    source: u16,
    dest: u16,
    sq_number: u32,
    ack_number: u32,
    data_offset: u8,
    rsrvd: u8, // 0 in generated segments, ignored in received segments
    flags: u8, // control bits
    window: u32,
    checksum: u16,
    urgent_pointer: u16,


}

struct Ipv6Header {
    source_addr: u32,
    dest_addr: u32,
    zero: u8,
    ptcl: u8,
    tcp_length: u32,
    s: Ipv6Addr
}

enum Connection {
    /// represents waiting for a connection request from any remote TCP peer and port
    Listen,
    /// represents waiting for a matching connection request after having sent a connection request
    SynSent,
    ///represents waiting for a confirming connection request acknowledgment after having both received and sent a connection request
    SynReceived,
    /// represents an open connection, data received can be delivered to the user. The normal state for the data transfer phase of the connection
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    LastAct,
    TimeWait,
    Closed,
}

impl Connection {
    pub fn open() { }

    pub fn send() {}

    pub fn receive() {}

    pub fn close() {}

    pub fn abort() { }

    pub fn status() { }

}