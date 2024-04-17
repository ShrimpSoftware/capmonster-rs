use serde::Serialize;

#[derive(Debug)]
pub enum ProxyType {
    Http,
    Socks4,
    Socks5,
}

#[derive(Debug, Serialize, Default)]
pub struct Proxy {
    proxy_type: String,
    proxy_address: String,
    proxy_port: u32,
    proxy_login: Option<String>,
    proxy_password: Option<String>,
}

pub trait Proxiable {
    fn set_proxy(&mut self, proxy: Proxy);
}

impl Proxy {
    pub fn new(
        ptype: ProxyType,
        address: String,
        port: u32,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        Proxy {
            proxy_type: match ptype {
                ProxyType::Http => String::from("http"),
                ProxyType::Socks4 => String::from("socks4"),
                ProxyType::Socks5 => String::from("socks5"),
            },
            proxy_address: address,
            proxy_port: port,
            proxy_login: username,
            proxy_password: password,
        }
    }
}
