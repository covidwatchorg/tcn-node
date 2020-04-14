# tcn-node

Node.js Module that wraps the [TCN Protocol Rust Implementation](https://github.com/TCNCoalition/TCN)

## Development

### Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install Node Build Tools
    - Mac:
        1. Install Xcode
        2. Install the Command Line Tools via Xcode under the menu
        *Xcode → Preferences → Downloads.*
    - Windows:  
    `npm install --global --production windows-build-tools`
    - Linux/WSL:  
    `sudo apt install -y make gcc g++`

### Build and Run

```
$ npm run install && node lib/index.js
```

### TODO

- [ ] Publish a proof of concept node module
- [ ] Automatically build binaries for various platforms so that we can run on GCP
- [ ] Add tests
- [ ] Expose full TCN API
- [ ] Use Typescript
- [ ] Use Prettier & ESLint
- [ ] Automatically compile/run on changes (nodemon + cargo-watch)