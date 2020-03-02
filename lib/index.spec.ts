import { TelemetryClient, DEFAULT_PORT } from './index';
import { EventEmitter } from 'events';
import {
  PACKET_CAR_SETUP_DATA_BUFFER,
  PACKET_CAR_SETUP_DATA_PARSED,
  PACKET_CAR_STATUS_DATA_BUFFER,
  PACKET_CAR_STATUS_DATA_PARSED,
  PACKET_CAR_TELEMETRY_DATA_BUFFER,
  PACKET_CAR_TELEMETRY_DATA_PARSED,
  PACKET_EVENT_DATA_BUFFER,
  PACKET_EVENT_DATA_PARSED,
  PACKET_LAP_DATA_BUFFER,
  PACKET_LAP_DATA_PARSED,
  PACKET_PARTICIPANTS_DATA_BUFFER,
  PACKET_PARTICIPANTS_DATA_PARSED,
  PACKET_SESSION_DATA_BUFFER,
  PACKET_SESSION_DATA_PARSED,
} from './mocks';

describe('F1 Game UDP Parser', () => {
  describe('constructor', () => {
    let telemetryClient: TelemetryClient;

    describe('port is passed through parameters', () => {
      beforeAll(() => {
        telemetryClient = new TelemetryClient({ port: 4477 });
      });

      it('should set default port and should set up client', () => {
        expect(telemetryClient.port).toBe(4477);
        // tslint:disable-next-line: no-any
        expect((telemetryClient.socket as any).type).toBe('udp4');
      });
    });

    describe('port is not passed through parameters', () => {
      beforeAll(() => {
        telemetryClient = new TelemetryClient();
      });

      it('should set default port and should set up client', () => {
        expect(telemetryClient.port).toBe(DEFAULT_PORT);
        // tslint:disable-next-line: no-any
        expect((telemetryClient.socket as any).type).toBe('udp4');
      });
    });
  });

  describe('parseMessage', () => {
    let telemetryClient: TelemetryClient;
    // tslint:disable-next-line: no-any
    let buffer: any;

    beforeEach(() => {
      telemetryClient = new TelemetryClient();
      spyOn(EventEmitter.prototype, 'emit');
    });

    describe('CarSetupData', () => {
      beforeAll(() => {
        buffer = Buffer.from(PACKET_CAR_SETUP_DATA_BUFFER);
      });

      it('should parse message', () => {
        telemetryClient.parseMessage(buffer);
        expect(EventEmitter.prototype.emit).toHaveBeenCalledWith(
          'CarSetupData',
          PACKET_CAR_SETUP_DATA_PARSED
        );
      });
    });

    describe('CarStatusData', () => {
      beforeAll(() => {
        buffer = Buffer.from(PACKET_CAR_STATUS_DATA_BUFFER);
      });

      it('should parse message', () => {
        telemetryClient.parseMessage(buffer);
        expect(EventEmitter.prototype.emit).toHaveBeenCalledWith(
          'CarStatusData',
          PACKET_CAR_STATUS_DATA_PARSED
        );
      });
    });

    describe('EventData', () => {
      beforeAll(() => {
        buffer = Buffer.from(PACKET_EVENT_DATA_BUFFER);
      });

      it('should parse message', () => {
        telemetryClient.parseMessage(buffer);
        expect(EventEmitter.prototype.emit).toHaveBeenCalledWith(
          'EventData',
          PACKET_EVENT_DATA_PARSED
        );
      });
    });

    describe('LapData', () => {
      beforeAll(() => {
        buffer = Buffer.from(PACKET_LAP_DATA_BUFFER);
      });

      it('should parse message', () => {
        telemetryClient.parseMessage(buffer);
        expect(EventEmitter.prototype.emit).toHaveBeenCalledWith(
          'LapData',
          PACKET_LAP_DATA_PARSED
        );
      });
    });

    describe('SessionData', () => {
      beforeAll(() => {
        buffer = Buffer.from(PACKET_SESSION_DATA_BUFFER);
      });

      it('should parse message', () => {
        telemetryClient.parseMessage(buffer);
        expect(EventEmitter.prototype.emit).toHaveBeenCalledWith(
          'SessionData',
          PACKET_SESSION_DATA_PARSED
        );
      });
    });

    describe('ParticipantsData', () => {
      beforeAll(() => {
        buffer = Buffer.from(PACKET_PARTICIPANTS_DATA_BUFFER);
      });

      it('should parse message', () => {
        telemetryClient.parseMessage(buffer);
        expect(EventEmitter.prototype.emit).toHaveBeenCalledWith(
          'ParticipantsData',
          PACKET_PARTICIPANTS_DATA_PARSED
        );
      });
    });

    xdescribe('CarTelemetryData', () => {
      beforeAll(() => {
        buffer = Buffer.from(PACKET_CAR_TELEMETRY_DATA_BUFFER);
      });

      it('should parse message', () => {
        telemetryClient.parseMessage(buffer);
        expect(EventEmitter.prototype.emit).toHaveBeenCalledWith(
          'CarTelemetryData',
          PACKET_CAR_TELEMETRY_DATA_PARSED
        );
      });
    });
  });
});
