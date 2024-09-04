import React from 'react';
import { ChatMessage } from '../types/ChatMessage';

interface MessageProps {
  message: ChatMessage
}

const Message: React.FC<MessageProps> = ({ message }) => {
  const { content, sender, isComplete } = message;
  const isUser = sender === 'user';

  return (
    <div className={`flex ${isUser ? 'justify-end' : 'justify-start'}`}>
    <div className={`max-w-xs md:max-w-md lg:max-w-lg xl:max-w-xl rounded-lg p-3 ${
      isUser ? 'bg-blue-500 text-white' : 'bg-gray-700 text-gray-200'
    }`}>
      <p>
        {content}
        {!isComplete && (
          <span className="inline-block w-2 h-4 ml-1 bg-gray-400 animate-pulse" />
        )}
      </p>
    </div>
  </div>
  );
};

export default Message;
