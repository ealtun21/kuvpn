# KUVPN

KUVPN is a simple Rust program to retrieve the DSID cookie and execute the OpenConnect command to connect to the VPN for Koç University.

## Getting Started

These instructions will help you set up and run the project on your local machine.

### Prerequisites

Ensure you have the following installed on your system:

1. **Rust and Cargo**: Install Rust and Cargo using [rustup](https://rustup.rs/).

2. **ChromeDriver or GeckoDriver**:
   - **ChromeDriver**: Required if you choose to use the Chrome browser.
   - **GeckoDriver**: Required if you choose to use the Gecko browser (Firefox).


### Installation

1. **Clone and Install KUVPN**:
    ```bash
    cargo install --git https://github.com/ealtun21/kuvpn
    ```

### Usage

Run the program with default parameters:

```bash
kuvpn
```

Or specify the URL, browser, and port:

```bash
kuvpn --url https://vpn.ku.edu.tr --browser gecko --port 9515
```

### Example

```bash
kuvpn --url https://vpn.ku.edu.tr --browser chrome --port 9515
```

This command will start the Chrome WebDriver, navigate to the specified URL, retrieve the DSID cookie, and construct the OpenConnect command.

### Command Line Options

```
Simple program to retrieve DSID cookie and execute OpenConnect command

Usage: kuvpn [OPTIONS]

Options:
  -u, --url <URL>          URL to visit [default: https://vpn.ku.edu.tr]
  -b, --browser <BROWSER>  Browser to use [default: chrome] [possible values: chrome, gecko, none]
  -p, --port <PORT>        Port to use for WebDriver [default: 9515]
  -h, --help               Print help
  -V, --version            Print version
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


# Ubuntu QuickInstall

Install rustup & Restart terminal (Open new one)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
``` 

Install openssl-dev
```
sudo apt-get install libssl-dev
```

Install chromium-chromedriver
```
sudo apt-get install chromium-chromedriver
```

Install kuvpn
```
cargo install --git https://github.com/ealtun21/kuvpn
```

That is it! Enjoy!


## TODO List

- [ ] Add automatic typing of email and password via environment variable as well as clicking next
- [ ] Add Nix build for more reliable building
- [ ] Create AppImage
- [ ] Create another version with an embedded browser for those who prefer

## Contributing

We welcome contributions from everyone! If you have an idea, fix, or improvement, please feel free to get involved.

