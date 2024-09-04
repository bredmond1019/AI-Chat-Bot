import React from 'react';

interface MessageProps {
  message: {
    id: string;
    text: string;
    sender: 'user' | 'bot';
  };
}

const Message: React.FC<MessageProps> = ({ message }) => {
  const isUser = message.sender === 'user';
  
  return (
    <div className={`flex ${isUser ? 'justify-end' : 'justify-start'}`}>
      <div className={`max-w-xs md:max-w-md lg:max-w-lg xl:max-w-xl rounded-lg p-3 ${
        isUser ? 'bg-blue-500 text-white' : 'bg-gray-700 text-gray-200'
      }`}>
        <p>{message.text}</p>
      </div>
    </div>
  );
};

export default Message;