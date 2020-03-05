# F1 Game UDP Parser

![Travis (.com)](https://img.shields.io/travis/com/msansoni/f1-game-udp-parser?style=for-the-badge)
![GitHub package.json version](https://img.shields.io/github/package-json/v/msansoni/f1-game-udp-parser?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/msansoni/f1-game-udp-parser?style=for-the-badge)

The F1 series of games by Codemasters outputs a data stream via UDP. This data can be used for external displays, HUDs or performance analysis.
This project leverages [Rust](https://www.rust-lang.org/)'s performance to speed up packet parsing to cope with higher frequency (60Hz) UDP streams from the game.

The purpose of this project was to port [Jonathan Bursztyn's](https://github.com/jonybur) comprehensive TypeScript parser [f1-telemetry-client](https://github.com/jonybur/f1-telemetry-client) into [Rust](https://www.rust-lang.org/) and [neon](https://neon-bindings.com/) in order to create a native node module, in an effort to speed up the packet parsing, which becomes a bottle neck at 60Hz using JS Binary-Parser. There are also some limitations in JS's binary-parser, especially when handling 64bit integers. Benchmarks locally show a 10x speed up parsing packets using the Rust implementation over JS Binary-Parser.

## Installing

```
$ npm install f1-game-udp-parser
```

or

```
$ yarn add f1-game-udp-parser
```

### Rebuilding the module

This module is developed in Rust and interfaces with Node as a Node Native Module. Travis-CI is used to package and distribute targeted binaries for operating system and node version. The build targets are currently set to Windows, OSX and Linux, supporting Node 12 (LTS) and the current node release. In some cases a targeted binary will not exist for your OS and node combination, or you'll want to use the package in an Electron application. Therefore you need to build the module from source. This will require Rust to be installed on the client machine and Node build tools - [guide here](https://neon-bindings.com/docs/getting-started/).

Run the follow commands to build the package:

```
$ npm i
$ npm run install
$ npm run build
```

You can then execute `$ npm pack` to generate a .tgz file that you can install in a local project for trying out the library.

## Usage

```
import { TelemetryClient, constants } from "f1-game-udp-parser";
// or: const { TelemetryClient, constants } = require('f1-game-udp-parser');
const { PACKETS } = constants;

const client = new F1TelemetryClient();
client.on(PACKETS.MotionData, console.log);
client.on(PACKETS.SessionData, console.log);
client.on(PACKETS.LapData, console.log);
client.on(PACKETS.EventData, console.log);
client.on(PACKETS.ParticipantsData, console.log);
client.on(PACKETS.CarSetupData, console.log);
client.on(PACKETS.CarTelemetryData, console.log);
client.on(PACKETS.CarStatusData, console.log);

// to start listening:
client.start();

// and when you want to stop:
client.stop();
```

## Documentation

The F1 2019 UDP specifications, are posted on the official Codemaster's forums on [this post](https://forums.codemasters.com/topic/38920-f1-2019-udp-specification/).

## License

This project has been developed from [jonybur's](https://github.com/jonybur) [f1-telemetry-client](https://github.com/jonybur/f1-telemetry-client) which in turn was originally a fork from [irvingswiftj's](https://github.com/irvingswiftj) [f1-2018-udp](https://github.com/irvingswiftj/f1-2018-udp).
All of which are licensed under the MIT License, as is this project.

## ToDo

- [x] Convert JS interop to TypeScript
- [ ] Fix CarTelemetryData parser
- [ ] Move UDP socket and EventEmitter to Rust
- [ ] Improve packet dependent logic inside Rust parser
