const parser = require('../native/index.node');
const dgram = require('dgram');
const EventEmitter = require('events');

const constants = require('./constants');

const DEFAULT_PORT = 20777;

class TelemetryClient extends EventEmitter {
  constructor(opts = {}) {
    super();

    const {port = DEFAULT_PORT} = opts;

    this.port = port;
    this.socket = dgram.createSocket('udp4');
  }

  parseMessage(m) {
    const message = parser.parseMessage(m);
    this.emit(message.packetType, message.packetData[message.packetType]);
  }

  start() {
    if (!this.socket) {
      return;
    }

    this.socket.on('listening', () => {
      if (!this.socket) {
        return;
      }

      const address = this.socket.address();
      console.log(
          `UDP socket listening on ${address.address}:${address.port}`);
      this.socket.setBroadcast(true);
    });

    this.socket.on('message', m => this.parseMessage(m));
    this.socket.bind(this.port);
  }

  stop() {
    if (!this.socket) {
      return;
    }

    return this.socket.close(() => {
      console.log(`UDP socket closed`);
      this.socket = undefined;
    });
  }
}

exports.TelemetryClient = TelemetryClient;
exports.constants = constants;
