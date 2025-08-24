# Tor Endpoint Monitor

A real-time monitoring application for Tor hidden services (.onion addresses) that displays the connectivity status of configured endpoints through a local SOCKS5 proxy.

## Overview

This application continuously monitors a configurable list of Tor hidden service endpoints by attempting connections through a local SOCKS5 proxy (typically Tor). It provides a web interface displaying the status, connectivity information, and last check time for each monitored endpoint.

## Prerequisites

1. **Tor Browser or Tor Service**: Running locally with SOCKS5 proxy enabled
2. **Rust**: Latest stable version
3. **Network Access**: Ability to connect to Tor network

## Installation & Usage

1. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd joinmarket-directory-checker
   ```

2. **Start Tor** (if not already running):
   ```bash
   # On most systems:
   tor
   # Or use Tor Browser
   ```

3. **Run the monitor**:
   ```bash
   cargo run --release
   ```

4. **Access dashboard**:
   Open `http://localhost:3000` in your browser

## Adding a new endpoint

Add the endpoint to `config.toml` in the format:
```toml
[[endpoints]]
address = "example.onion"
name = "Example"
port = 80
```

## License

Distributed under the AGPLv3 License. See [LICENSE.txt](./LICENSE.txt) for more information.

## Support

PRs are more than welcome!

Feeling generous? Leave me a tip! ⚡️w3irdrobot@getalby.com.

Think I'm an asshole but still want to tip? Please donate [to OpenSats](https://opensats.org/).

Want to tell me how you feel? Hit me up [on Nostr](https://njump.me/npub17q5n2z8naw0xl6vu9lvt560lg33pdpe29k0k09umlfxm3vc4tqrq466f2y).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the AGPLv3 license, shall
be licensed as above, without any additional terms or conditions.
