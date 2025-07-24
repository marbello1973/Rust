use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

fn main() {
    // Listar interfaces y seleccionar la activa
    let interfaces = datalink::interfaces();
    let target_interface = interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No hay interfaz activa para Internet");

    println!("Escuchando en {}...", target_interface.name);

    // Crear canal de captura
    let (_, mut rx) = match datalink::channel(&target_interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Solo se soporta Ethernet"),
        Err(e) => panic!("Error al crear el canal: {}", e),
    };

    // Capturar paquetes
    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet = EthernetPacket::new(packet).unwrap();
                match ethernet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
                            match ipv4.get_next_level_protocol() {
                                IpNextHeaderProtocols::Tcp => {
                                    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                        println!(
                                            "TCP: {}:{} -> {}:{} ({} bytes)",
                                            ipv4.get_source(),
                                            tcp.get_source(),
                                            ipv4.get_destination(),
                                            tcp.get_destination(),
                                            tcp.payload().len()
                                        );
                                    }
                                }
                                IpNextHeaderProtocols::Udp => {
                                    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                        println!(
                                            "UDP: {}:{} -> {}:{} ({} bytes)",
                                            ipv4.get_source(),
                                            udp.get_source(),
                                            ipv4.get_destination(),
                                            udp.get_destination(),
                                            udp.payload().len()
                                        );
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => continue, // Ignorar tráfico no IPv4
                }
            }
            Err(e) => eprintln!("Error al capturar paquete: {}", e),
        }
    }
}
