// File: frontend/src/services/websocket.ts

class WebSocketService {
  private socket: WebSocket | null = null;
  private messageCallbacks: ((message: any) => void)[] = [];
  private reconnectInterval: number = 5000; // 5 seconds
  private url: string;
  private sessionId: string | null = null;

  constructor(url: string) {
    this.url = url;
  }

  connect(): void {
    this.socket = new WebSocket(this.url);

    this.socket.onopen = () => {
      console.log('WebSocket connection established');
    };

    this.socket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'chat_session_started') {
        this.sessionId = data.session_id;
        console.log('Chat session started with ID:', this.sessionId);
      } else {
        this.messageCallbacks.forEach(callback => callback(data));
      }
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
      const fullMessage = {
        ...message,
        id: this.sessionId,
      };
      this.socket.send(JSON.stringify(fullMessage));
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

  getSessionId(): string | null {
    return this.sessionId;
  }
}

export default WebSocketService;