// File: frontend/src/services/websocket.ts

class WebSocketService {
  private socket: WebSocket | null = null;
  private messageCallbacks: ((message: any) => void)[] = [];
  private reconnectInterval: number = 5000; // 5 seconds
  private url: string;

  constructor(url: string) {
    this.url = url;
  }

  connect(): void {
    this.socket = new WebSocket(this.url);

    this.socket.onopen = () => {
      console.log('WebSocket connection established');
    };

    this.socket.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.messageCallbacks.forEach(callback => callback(message));
    };

    this.socket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    this.socket.onclose = () => {
      console.log('WebSocket connection closed. Attempting to reconnect...');
      setTimeout(() => this.connect(), this.reconnectInterval);
    };
  }

  sendMessage(message: any): void {
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(message));
    } else {
      console.error('WebSocket is not connected');
    }
  }

  onMessage(callback: (message: any) => void): void {
    this.messageCallbacks.push(callback);
  }

  disconnect(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }
  }
}

export default WebSocketService;