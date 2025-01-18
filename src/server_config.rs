use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub private_key: String,
    pub address: String,
    pub listen_port: i32,
    pub post_up: String,
    pub post_down: String,
    pub peers: Vec<Peer>,
}

#[derive(Deserialize, Serialize)]
pub struct Peer {
    pub public_key: String,
    pub allowed_ips: String,
}

impl ServerConfig {
    pub fn add_peer(&mut self, peer: Peer) {
        self.peers.push(peer);
    }

    pub fn to_string(&self) -> String {
        let mut server_config = String::new();
        server_config.push_str("[Interface]\n");
        server_config.push_str(&format!("PrivateKey = {}\n", self.private_key));
        server_config.push_str(&format!("Address = {}\n", self.address));
        server_config.push_str(&format!("ListenPort = {}\n", self.listen_port));
        server_config.push_str(&format!("PostUp = {}\n", self.post_up));
        server_config.push_str(&format!("PostDown = {}\n", self.post_down));
        server_config.push_str("\n");

        for client in &self.peers {
            server_config.push_str("[Peer]\n");
            server_config.push_str(&format!("PublicKey = {}\n", client.public_key));
            server_config.push_str(&format!("AllowedIPs = {}\n\n", self.address));
        }

        server_config
    }

    pub fn new(
        private_key: String,
        address: String,
        listen_port: i32,
        post_up: String,
        post_down: String,
        clients_count: usize,
    ) -> Self {
        Self {
            private_key,
            address,
            listen_port,
            post_up,
            post_down,
            peers: Vec::with_capacity(clients_count),
        }
    }
}
