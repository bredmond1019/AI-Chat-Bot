// File: frontend/src/services/websocket.ts

import { AIMessage, ChatMessage, SentChatMessage, WebSocketMessage } from '../types/ChatMessage';


class WebSocketService {
  private socket: WebSocket | null = null;
  private messageCallbacks: ((message: AIMessage) => void)[] = [];
  private reconnectInterval = 5000; // 5 seconds
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

    this.socket.onmessage = this.handleMessage;

    this.socket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    this.socket.onclose = () => {
      console.log('WebSocket connection closed. Attempting to reconnect...');
      setTimeout(() => this.connect(), this.reconnectInterval);
    };
  }

  private handleMessage = (event: MessageEvent): void => {
    const message: WebSocketMessage = JSON.parse(event.data)
    if (message.type === 'chat_session_started') {
      this.sessionId = message.session_id;
      console.log('Chat session started with ID:', this.sessionId);
    } else {
      this.messageCallbacks.forEach(callback => callback(message));
    }
  }

  sendMessage(message: ChatMessage): void {
    if (
      this.socket &&
      this.socket.readyState === WebSocket.OPEN &&
      this.sessionId !== null
    ) {
      const fullMessage: SentChatMessage = {
        message: message.content,
        session_id: this.sessionId,
      };
      this.socket.send(JSON.stringify(fullMessage));
    } else {
      console.error('WebSocket is not connected');
    }
  }

  onMessage(callback: (message: AIMessage) => void): void {
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