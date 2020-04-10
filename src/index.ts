import * as dgram from 'dgram';
import * as constants from './constants';

import { EventEmitter } from 'events';

import { AddressInfo } from 'net';
import { Options } from './types';

import * as parser from '../native/index.node';

export const DEFAULT_PORT = 20777;
export const CLIENT_STARTED_ERROR_MESSAGE = 'Client was already started';

class TelemetryClient extends EventEmitter {
  port: number;
  socket: dgram.Socket | undefined;
  verbose: boolean;
  isRunning: boolean;

  constructor(opts: Options = {}) {
    super();

    const { port = DEFAULT_PORT, verbose = true } = opts;

    this.port = port;
    this.verbose = verbose;
    this.isRunning = false;
  }

  parseMessage(m: Buffer) {
    const message = parser.parseMessage(m);
    this.emit(message.packetType, message.packetData[message.packetType]);
  }

  start() {
    if (this.isRunning) {
      throw CLIENT_STARTED_ERROR_MESSAGE;
    }

    this.socket = dgram.createSocket('udp4');

    this.socket.on('listening', () => {
      if (!this.socket) {
        return;
      }

      const address: AddressInfo = this.socket.address() as AddressInfo;

      if (this.verbose) {
        console.log(
          `UDP socket listening on ${address.address}:${address.port}`
        );
      }

      this.socket.setBroadcast(true);
    });

    this.socket.on('message', m => this.parseMessage(m));

    this.socket.bind(this.port);

    this.isRunning = true;
  }

  stop() {
    if (!this.isRunning) {
      return;
    }

    return (
      this.socket &&
      this.socket.close(() => {
        if (this.verbose) {
          console.log(`UDP socket closed`);
        }

        this.socket = undefined;

        this.isRunning = false;
      })
    );
  }
}

export { TelemetryClient, constants };
