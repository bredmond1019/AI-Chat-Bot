import React from 'react';
import ChatInterface from '../components/ChatInterface';

const ChatPage: React.FC = () => {
  return (
    <div className="flex flex-col h-screen bg-gray-900">
      <header className="bg-gray-800 p-4 text-white">
        <h1 className="text-2xl font-bold text-green-400">AI Chat Bot</h1>
      </header>
      <main className="flex-grow overflow-hidden">
        <ChatInterface />
      </main>
      <footer className="bg-gray-800 p-2 text-center text-gray-400 text-sm">
        © 2023 Health Tech Startup
      </footer>
    </div>
  );
};

export default ChatPage;
