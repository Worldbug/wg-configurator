# WireGuard Configuration Generator

A tool for generating WireGuard configurations for both server and clients.

## Features

- Automatic key generation for new clients
- Server and client configuration generation
- Configurable via TOML files
- Output directory customization
- Persistent key storage

## Installation

```bash
git clone https://github.com/worldbug/wg-configurator
cd wg-configurator
cargo build --release
```

## Usage

```bash
# Using default config path and output directory
./wg-configurator

# Specifying custom config and output paths
./wg-configurator -c /path/to/config.toml -o /path/to/output
```

## Configuration File (config.toml)

```toml
[server]
ip = "123.45.67.89"              # Your server's public IP
port = 51820                     # WireGuard port
priv_key = "YOUR_PRIVATE_KEY"    # Server's private key
address = "10.0.0.1/24"          # VPN network address
post_up = "iptables -A FORWARD -i %i -j ACCEPT"    # Post up script
post_down = "iptables -D FORWARD -i %i -j ACCEPT"  # Post down script

[[client]]
name = "laptop"
address = "10.0.0.2/24"

[[client]]
name = "phone"
address = "10.0.0.3/24"

[[keys]]
client_name = "laptop"
priv_key = "EXISTING_PRIVATE_KEY"    # Optional, will be generated if not provided
```

## Output Structure

```
output/
├── wg.conf           # Server configuration
├── laptop/
│   └── wg.conf      # Laptop client configuration
└── phone/
    └── wg.conf      # Phone client configuration
```

## Sample Output Configurations

### Server Configuration (output/wg.conf)
```ini
[Interface]
PrivateKey = SERVER_PRIVATE_KEY
Address = 10.0.0.1/24
ListenPort = 51820
PostUp = iptables -A FORWARD -i %i -j ACCEPT
PostDown = iptables -D FORWARD -i %i -j ACCEPT

[Peer]
PublicKey = LAPTOP_PUBLIC_KEY
AllowedIPs = 10.0.0.2/24

[Peer]
PublicKey = PHONE_PUBLIC_KEY
AllowedIPs = 10.0.0.3/24
```

### Client Configuration (output/laptop/wg.conf)
```ini
[Interface]
PrivateKey = LAPTOP_PRIVATE_KEY
Address = 10.0.0.2/24
DNS = 8.8.8.8

[Peer]
PublicKey = SERVER_PUBLIC_KEY
Endpoint = 123.45.67.89:51820
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
```

## Command Line Arguments

- `-c, --config`: Path to config file (default: "./config.toml")
- `-o, --output`: Output directory for generated configs (default: "./output")
- `-h, --help`: Display help information
- `-V, --version`: Display version information

## Notes

- Private keys will be automatically generated for clients that don't have them specified
- Generated keys are saved back to the config file for future use
- Each client gets its own subdirectory in the output folder
- DNS is hardcoded to 8.8.8.8 for clients
- PersistentKeepalive is set to 25 seconds for clients
