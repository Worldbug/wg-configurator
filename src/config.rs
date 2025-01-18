use std::collections::HashMap;
use std::str::FromStr;

use crate::client_config::ClientConfig;
use crate::server_config::{Peer, ServerConfig};
use serde::{Deserialize, Serialize};
use wireguard_keys::Privkey;

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneralConfig {
    pub server: ServerInfo,
    pub client: Vec<Client>,
    pub keys: Vec<ClientKey>,
}

impl GeneralConfig {
    pub fn new() -> GeneralConfig {
        GeneralConfig {
            server: ServerInfo {
                ip: String::new(),
                port: 51820,
                priv_key: String::new(),
                address: String::from("10.0.0.1/24"),
                post_up: String::new(),
                post_down: String::new(),
            },
            client: vec![],
            keys: vec![],
        }
    }

    pub fn process(self) -> GeneralConfig {
        // let myPub = Pubkey::from_str(&self.server.priv_key).unwrap();

        self
    }

    pub fn get_clients(&mut self) -> Vec<ClientConfig> {
        let mut updated_keys = Vec::with_capacity(self.client.len());
        let mut clients = Vec::with_capacity(self.client.len());
        let keys_map: HashMap<String, String> = self
            .keys
            .iter()
            .map(|key| (key.client_name.clone(), key.priv_key.clone()))
            .collect();

        let server_pub_key = Privkey::from_str(&self.server.priv_key)
            .expect("Invalid private key")
            .pubkey()
            .to_string();

        for client in &self.client {
            let priv_key = match keys_map.get(&client.name) {
                Some(key) => key.clone(),
                _ => Privkey::generate().to_string(),
            };

            clients.push(ClientConfig {
                name: client.name.clone(),
                address: client.address.clone(),
                port: self.server.port,
                dns: String::from("8.8.8.8"),
                priv_key: priv_key.clone(),
                pub_key: server_pub_key.clone(),
                allowed_ips: String::from("0.0.0.0/0"),
                endpoint: self.server.ip.clone(),
                persistent_keepalive: 25,
            });

            updated_keys.push(ClientKey {
                client_name: client.name.clone(),
                priv_key: priv_key.clone(),
            });
        }

        self.keys = updated_keys;
        clients
    }

    pub fn get_server(&self) -> ServerConfig {
        let mut sconfig = ServerConfig::new(
            self.server.priv_key.clone(),
            self.server.address.clone(),
            self.server.port,
            self.server.post_up.clone(),
            self.server.post_down.clone(),
            self.client.len(),
        );

        let keys_map: HashMap<String, String> = self
            .keys
            .iter()
            .map(|key| (key.client_name.clone(), key.priv_key.clone()))
            .collect();

        for client in &self.client {
            let priv_key = match keys_map.get(&client.name) {
                Some(key) => key.clone(),
                _ => Privkey::generate().to_string(),
            };

            let pub_key = Privkey::from_str(&priv_key)
                .expect("Invalid private key")
                .pubkey()
                .to_string();

            sconfig.add_peer(Peer {
                public_key: pub_key,
                allowed_ips: client.address.clone(),
            });
        }

        sconfig
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerInfo {
    pub ip: String,
    pub port: i32,
    pub priv_key: String,
    pub address: String,
    pub post_up: String,
    pub post_down: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientKey {
    pub client_name: String,
    pub priv_key: String,
}
