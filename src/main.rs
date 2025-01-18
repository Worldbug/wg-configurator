use clap::Parser;
use std::fs::{self, File};
use std::io::{BufWriter, Error as IOError, Write};
use std::path::PathBuf;
use toml::de::Error as TomlError;

mod config;
use config::GeneralConfig;

mod client_config;
use client_config::ClientConfig;

mod server_config;
use server_config::ServerConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short = 'c', long, default_value = "./config.toml")]
    config: PathBuf,

    /// Output directory for generated configs
    #[arg(short = 'o', long, default_value = "./output")]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    let config_path = args.config.to_str().expect("Invalid config path");
    let output_path = args.output.to_str().expect("Invalid output path");

    GeneralConfig::from_file(config_path)
        .unwrap()
        .process()
        .with_clients(|c| {
            c.iter()
                .for_each(|client| client.to_file(output_path).unwrap())
        })
        .with_server(|s| s.to_file(output_path).unwrap())
        .to_file(config_path)
        .unwrap();
}

impl GeneralConfig {
    fn from_file(path: &str) -> Result<GeneralConfig, TomlError> {
        let file_contents = fs::read_to_string(path).unwrap();
        toml::from_str(&file_contents)
    }

    fn to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let config_file = File::create(path)?;
        let mut config_writer = BufWriter::new(config_file);
        config_writer.write_all(
            toml::to_string(self)
                .expect("Failed to serialize config")
                .as_bytes(),
        )?;
        config_writer.flush()
    }

    fn with_clients<F>(&mut self, mut f: F) -> &GeneralConfig
    where
        F: FnMut(&Vec<ClientConfig>),
    {
        f(&self.get_clients());
        self
    }

    fn with_server<F>(&self, mut f: F) -> &GeneralConfig
    where
        F: FnMut(&ServerConfig),
    {
        f(&self.get_server());
        self
    }
}

impl ClientConfig {
    fn to_file(&self, output_dir: &str) -> Result<(), IOError> {
        let client_dir = format!("{}/{}", output_dir, self.name);
        fs::create_dir_all(&client_dir)?;

        let client_file = File::create(format!("{}/wg.conf", client_dir))?;
        let mut client_writer = BufWriter::new(client_file);
        client_writer.write_all(self.to_string().as_bytes())?;
        client_writer.flush()
    }
}

impl ServerConfig {
    fn to_file(&self, output_dir: &str) -> Result<(), IOError> {
        let client_dir = format!("{}", output_dir);
        fs::create_dir_all(&client_dir)?;

        let client_file = File::create(format!("{}/wg.conf", client_dir))?;
        let mut client_writer = BufWriter::new(client_file);
        client_writer.write_all(self.to_string().as_bytes())?;
        client_writer.flush()
    }
}
