import { parse } from "expo-linking";

export default class WebSocketClient {
    private socket: WebSocket;
    private eventListeners: { [key: string]: ((body: any) => void)[] } = {};

    constructor(url: string) {
        this.socket = new WebSocket(url);

        this.socket.onopen = () => {
            console.log('WebSocket connection established.');
        };

        this.socket.onmessage = (event: MessageEvent) => {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
        };

        this.socket.onclose = () => {
            console.log('WebSocket connection closed.');
        };

        this.socket.onerror = (error: Event) => {
            console.error('WebSocket error:', error);
        };
    }

    private handleMessage(message: { ev: string; body: string }) {
        const { ev, body } = message;
        const parsedBody = JSON.parse(body);

        if (this.eventListeners[ev]) {
            this.eventListeners[ev].forEach(listener => listener(parsedBody));
        }
    }

    public on(event: string, listener: (body: any) => void) {
        if (!this.eventListeners[event]) {
            this.eventListeners[event] = [];
        }
        this.eventListeners[event].push(listener);
    }

    public send(event: string, body: any) {
        const message = JSON.stringify({ ev: event, body: JSON.stringify(body) });
        this.socket.send(message);
    }

    public close() {
        this.socket.close();
    }
}

// Example usage:

console.log('strtd ws')
// wsClient.on('Cam_Motion', (body) => {
//     console.log('Received someEvent with body:', body);
// });

// To send a message
// wsClient.send('someEvent', { key: 'value' });

// To close the connection
// wsClient.close();
