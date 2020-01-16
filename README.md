# F1 Game UDP Parser

The F1 series of games by Codemasters outputs a data stream via UDP. This data can be used for external displays, HUDs or performance analysis.

The purpose of this project was to port [Jonathan Bursztyn's](https://github.com/jonybur) fantastic TypeScript parser [f1-telemetry-client](https://github.com/jonybur/f1-telemetry-client) into [Rust](https://www.rust-lang.org/) and [neon](https://neon-bindings.com/) in order to create a native node module, in an effort to speed up the packet parsing, which becomes a bottle neck at 60Hz using JS Binary-Parser. Benchmarks locally show a 10x speed up parsing packets using the Rust implementation over JS Binary-Parser.

## Installing
```
$ npm install f1-telemetry-client
```
or
```
$ yarn add f1-telemetry-client
```
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
