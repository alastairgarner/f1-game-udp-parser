import * as dgram from 'dgram';
import * as constants from './constants';

import EventEmitter from 'events';

import { AddressInfo } from 'net';
import { Options } from './types';

const parser = require('../native/index.node');

const DEFAULT_PORT = 20777;

class TelemetryClient extends EventEmitter {
  port: number;
  socket: dgram.Socket | undefined;

  constructor(opts: Options = {}) {
    super();

    const { port = DEFAULT_PORT } = opts;

    this.port = port;
    this.socket = dgram.createSocket('udp4');
  }

  parseMessage(m: Buffer) {
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

      const address: AddressInfo = this.socket.address() as AddressInfo;
      console.log(`UDP socket listening on ${address.address}:${address.port}`);
      this.socket.setBroadcast(true);
    });

    this.socket.on('message', this.parseMessage);
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

export { TelemetryClient, constants };
