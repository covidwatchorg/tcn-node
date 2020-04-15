# tcn-node

Node.js Module that wraps the [TCN Protocol Rust Implementation](https://github.com/TCNCoalition/TCN)

## TODO

- [x] Publish a proof of concept node module
- [x] Automatically build binaries for various platforms so that we can run on GCP
- [ ] Add tests
- [ ] Expose full TCN API
- [ ] Use Typescript
- [ ] Use Prettier & ESLint
- [ ] Automatically compile/run on changes (nodemon + cargo-watch)
- [x] Put release process into a shell script

## Installation

```
$ npm install covid19risk/tcn-node
```

## Usage

```js
import tcn from "tcn-node";
console.log(tcn.tcn_example()); // should print "symptom data"
```

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

### Building and running from source

```
$ git clone https://github.com/covid19risk/tcn-node.git && cd tcn-node
$ npm run dev && node lib/index.js
```

### Releasing

Suggestion: only make full releases from master, otherwise make a pre-release

Full release: `./scripts/release.sh major | minor | patch`  
Pre-release: `./scripts/release.sh premajor | preminor | prepatch --preid=alpha|beta|rc`  

Use minor/preminor for functional changes, patch/prepatch for bug fixes.  
Major/premajor for large breaking changes / overhauls, and 1.0.0 release.