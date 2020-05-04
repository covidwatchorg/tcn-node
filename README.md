# tcn-node

Node.js module that wraps the [TCN Protocol Rust Implementation](https://github.com/TCNCoalition/TCN)

![npm](https://img.shields.io/npm/v/tcn-node)
![Build](https://github.com/covid19risk/tcn-node/workflows/Build/badge.svg)

## TODO

- [x] Publish a proof of concept node module
- [x] Automatically build binaries for various platforms so that we can run on GCP
- [x] Add tests
- [ ] Expose full TCN API
- [ ] Conversions from Strings etc. to/from byte arrays for convenience
- [ ] Serialize TCN Errors
- [x] Use Typescript
- [x] Use Prettier & ESLint
- [ ] Automatically compile/run on changes (nodemon + cargo-watch)
- [x] Put release process into a shell script

## Installation

Pre-built binaries available for Node v10, v12, v13, and v14.

As long as you are using one of those versions, installing via npm should automatically download the binary for your platform:

```
$ npm install tcn-node
```

In case you are using a different version of Node or an unusual system architecture, you may need to also install [Rust](https://www.rust-lang.org/tools/install) and the Node build tools so that it can to build the native addon during install.

## Usage

### Using the TCN native module directly

The API of the Rust [tcn crate](https://docs.rs/tcn/0.4.1/tcn/) is wrapped in a native Node addon and can be used in much the same way as if you were using that crate in Rust code:

```js
const { ReportAuthorizationKey, MemoType } = require("tcn-node").native;
const assert = require("assert");

// Generate a report authorization key.  This key represents the capability
// to publish a report about a collection of derived temporary contact numbers.
let rak = new ReportAuthorizationKey();

// Use the temporary contact key ratchet mechanism to compute a list
// of temporary contact numbers.
let tck = rak.initial_temporary_contact_key(); // tck <- tck_1
let tcns = [];
for (let i = 0; i < 100; i++) {
  tcns.push(tck.temporary_contact_number());
  tck = tck.ratchet();
}

// Prepare a report about a subset of the temporary contact numbers.
let signed_report = rak.create_report(
  MemoType.CoEpiV1, // The memo type
  Buffer.from("symptom data"), // The memo data
  20, // Index of the first TCN to disclose
  90 // Index of the last TCN to check
);

// Verify the source integrity of the report...
let report = signed_report.verify();
// ...allowing the disclosed TCNs to be recomputed.
let recomputed_tcns = report.temporary_contact_numbers();

// Check that the recomputed TCNs match the originals.
// The slice is offset by 1 because tcn_0 is not included.
assert.deepEqual(recomputed_tcns, tcns.slice(20 - 1, 90 - 1));
```

### JavaScript API

The JS API is a work in progress and currently consists of a few example functions only:

```js
const { tcnExample, signedReportExample, validateReport } = require("tcn-node");

console.log(tcnExample()); // => "symptom data"

console.log(signedReportExample()); // => generates a signed report as JSON

console.log(validateReport(signedReportExample())); // => true

console.log(
  validateReport({
    report: {
      rvk: [
        205,
        234,
        147,
        231,
        210,
        96,
        99,
        128,
        241,
        255,
        168,
        61,
        243,
        222,
        144,
        41,
        194,
        92,
        112,
        118,
        140,
        98,
        90,
        38,
        156,
        32,
        216,
        117,
        171,
        14,
        206,
        117,
      ],
      tck_bytes: [
        5,
        44,
        47,
        43,
        14,
        249,
        162,
        165,
        139,
        157,
        225,
        217,
        38,
        77,
        151,
        140,
        247,
        198,
        138,
        23,
        208,
        188,
        229,
        189,
        20,
        101,
        126,
        83,
        216,
        18,
        194,
        19,
      ],
      j_1: 20,
      j_2: 90,
      memo_type: "CoEpiV1",
      memo_data: [115, 121, 109, 112, 116, 111, 109, 32, 100, 97, 116, 97],
    },
    sig: {
      R_bytes: [
        171,
        0,
        174,
        55,
        138,
        201,
        100,
        209,
        69,
        98,
        176,
        85,
        27,
        240,
        129,
        22,
        204,
        209,
        89,
        245,
        9,
        31,
        170,
        4,
        1,
        69,
        243,
        251,
        36,
        31,
        249,
        192,
      ],
      s_bytes: [
        250,
        99,
        139,
        105,
        167,
        126,
        136,
        208,
        253,
        158,
        225,
        46,
        81,
        179,
        50,
        90,
        113,
        63,
        235,
        163,
        172,
        193,
        251,
        86,
        76,
        118,
        188,
        170,
        16,
        252,
        132,
        8,
      ],
    },
  })
); // => true
```

## Development

This project uses:

- [Neon](https://neon-bindings.com/) for compiling Rust code to a native Node.js addon
- [TSDX](https://github.com/jaredpalmer/tsdx) for typescript tooling
- GitHub Actions for CI/CD

### Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install Node Build Tools
   - Mac:
     1. Install Xcode
     2. Install the Command Line Tools via Xcode under the menu
        _Xcode → Preferences → Downloads._
   - Windows:  
     `npm install --global --production windows-build-tools`
   - Linux/WSL:  
     `sudo apt install -y make gcc g++`

### Building and running from source

```
$ git clone https://github.com/covid19risk/tcn-node.git && cd tcn-node
$ npm install --build-from-source
$ node
> const tcn = require('.')
undefined
> tcn.tcnExample()
'symptom data'
```

You can also use `npm start` to run the typescript build in development/watch mode. This is handy for catching errors while you work.

### Testing

```
$ npm run native:dev && npm test
```

### Releasing

Suggestion: only make full releases from master, otherwise make a pre-release

Full release: `./scripts/release.sh major | minor | patch`  
Pre-release: `./scripts/release.sh premajor | preminor | prepatch | prerelease --preid=alpha|beta|rc`

Use minor/preminor for functional changes, patch/prepatch for bug fixes.  
Use prerelease when already on a alpha/beta/rc version to just bump the last part.
Major/premajor for large breaking changes / overhauls, and 1.0.0 release.
