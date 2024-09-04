interface BaseChatMessage {
  type: 'ai_message' | 'chat_session_started'
}

export interface ChatSessionStarted extends BaseChatMessage {
  type: 'chat_session_started'; 
  session_id: string;
}

export interface AIMessage extends BaseChatMessage {
  type: 'ai_message';
  message: ReceivedChatMessage;
}

export type WebSocketMessage = ChatSessionStarted | AIMessage;

export interface ReceivedChatMessage {
  content: string;
  isComplete: boolean;
}

export interface SentChatMessage {
  session_id: string;
  message: string;
}

export interface ChatMessage {
  content: string;
  sender: 'user' | 'AI'
  isComplete: boolean;
}