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

### Using as a dependency in another project

```
$ npm install tcn-node
```

```js
import tcn from "tcn-node";
console.log(tcn.tcn_example()); // should print "symptom data"
```

### Releasing

Suggestion: only make full releases from master, otherwise make a pre-release

1. Make sure your working copy is clean
1. `OLD_VERSION=$(node -p -e "require('./package.json').version")`
1. Update node version, e.g.:
    - `npm version --git-tag-version=false preminor --preid=alpha`
    - `npm version --git-tag-version=false minor)`
1. `VERSION=$(node -p -e "require('./package.json').version")`
1. `sed -i "s/$OLD_VERSION/$VERSION/" native/Cargo.toml`
1. `git commit -am "$VERSION" && git tag "v$VERSION"`
1. `git push && git push --tags`
1. CI will create a GitHub release, upload binaries, and publish to npm