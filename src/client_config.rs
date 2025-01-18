pub struct ClientConfig {
    pub name: String,
    pub address: String,
    pub port: i32,
    pub dns: String,
    pub priv_key: String,
    pub pub_key: String,
    pub allowed_ips: String,
    pub endpoint: String,
    pub persistent_keepalive: i32,
}

impl ClientConfig {
    pub fn to_string(&self) -> String {
        let mut client_config = String::new();
        client_config.push_str("[Interface]\n");
        client_config.push_str(&format!("Address = {}\n", self.address));
        client_config.push_str(&format!("DNS = {}\n", self.dns));
        client_config.push_str(&format!("PrivateKey = {}\n\n", self.priv_key));
        client_config.push_str("[Peer]\n");
        client_config.push_str(&format!("PublicKey = {}\n", self.pub_key));
        client_config.push_str(&format!("AllowedIPs = {}\n", self.allowed_ips));
        client_config.push_str(&format!("Endpoint = {}:{}\n", self.endpoint, self.port));
        client_config.push_str(&format!(
            "PersistentKeepalive = {}\n",
            self.persistent_keepalive
        ));

        client_config
    }
}
