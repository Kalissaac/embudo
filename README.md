# embudo

Lightweight reverse proxy

## Usage

Embudo will read configuration by default from `/etc/embudo/config.toml`, or you can specify the configuration as an arugment when running the executable (`embudo /path/to/config.toml`). If you'd like it to run in the background continuously, add it as a daemon to be run on startup.

### Example Config
For now, you will still need to manually set your DNS to resolve any special hosts to `127.0.0.1`, this can be done through your hosts file or something like dnsmasq.
```toml
listen_addr = "127.0.0.1:80"

[[hosts]]
source = "localhost"
destination = "127.0.0.1:3000"

[[hosts]]
source = "special_service"
destination = "127.0.0.1:8080"
```
