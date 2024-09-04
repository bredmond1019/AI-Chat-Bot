// File: frontend/src/components/ChatInterface.tsx

import React, { useState, useEffect, useRef } from 'react';
import Message from './Message';
import WebSocketService from '../services/websocket';

interface ChatMessage {
  // id: string;
  content: string;
  sender: 'user' | 'bot';
}

const ChatInterface: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const webSocketRef = useRef<WebSocketService | null>(null);

  useEffect(() => {
    // Initialize WebSocket connection
    webSocketRef.current = new WebSocketService('ws://localhost:8080/ws');
    webSocketRef.current.connect();

    // Set up message listener
    webSocketRef.current.onMessage((message) => {
      const newMessage: ChatMessage = {
        content: message.content,
        sender: message.sender,
      };
      setMessages((prevMessages) => [...prevMessages, newMessage]);
    });

    return () => {
      if (webSocketRef.current) {
        webSocketRef.current.disconnect();
      }
    };
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = () => {
    if (input.trim()) {
      const newMessage: ChatMessage = {
        content: input.trim(),
        sender: 'user',
      };
      setMessages([...messages, newMessage]);
      
      // Send message to WebSocket
      if (webSocketRef.current) {
        webSocketRef.current.sendMessage({
          msg: input.trim(),
          id: webSocketRef.current.getSessionId(),
        });
      }

      setInput('');
    }
  };

  return (
    <div className="flex flex-col h-full bg-gray-800">
      <div className="flex-grow overflow-y-auto p-4 space-y-4">
        {messages.map((message, index) => (
          <Message key={index} message={message} />
        ))}
        <div ref={messagesEndRef} />
      </div>
      <div className="p-4 bg-gray-700">
        <div className="flex space-x-2">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
            className="flex-grow px-4 py-2 bg-gray-600 text-white rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Type your message..."
          />
          <button
            onClick={handleSend}
            className="px-4 py-2 bg-blue-500 text-white rounded-full hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
};

export default ChatInterface;