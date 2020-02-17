import { TelemetryClient, DEFAULT_PORT } from './index';

describe('F1 Game UDP Parser', () => {
  describe('constructor', () => {
    describe('when no port is passed through parameters', () => {
      let telemetryClient: TelemetryClient;

      beforeAll(() => {
        telemetryClient = new TelemetryClient();
      });

      it('should set default port and should set up client', () => {
        //let telemetryClient: TelemetryClient;
        expect(telemetryClient.port).toBe(DEFAULT_PORT);
      });

      /*
      it('should set up client as udp4 client', () => {
        expect(telemetryClient.client).toBeDefined();
        // tslint:disable-next-line:no-any
        expect((telemetryClient.client as any).type).toBe('udp4');
      });
      */
    });
  });
});
