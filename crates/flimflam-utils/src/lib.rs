use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use std::net::IpAddr;

#[cfg(target_os = "macos")]
pub fn get_ip() -> anyhow::Result<IpAddr> {
    (|| {
        let interfaces = datalink::interfaces();
        let default_interface = interfaces.iter().find(|iface| iface.name == "en0")?;

        if let IpNetwork::V4(i) = default_interface.ips[1] {
            Some(IpAddr::V4(i.ip()))
        } else {
            None
        }
    })()
    .ok_or_else(|| anyhow::anyhow!("could not determine IP address"))
}
