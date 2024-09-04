# AI Chat Bot Application

This project is an AI-powered chat bot application built with a Rust backend and a React TypeScript frontend. It uses WebSocket for real-time communication and integrates with the Ollama AI model for generating responses.

## Features

- Real-time chat interface
- AI-powered responses using Ollama
- WebSocket communication for instant messaging
- Rust backend for high performance
- React TypeScript frontend for a responsive UI
- Tailwind CSS for styling

## Project Structure

ai-chatbot/
├── backend/
│ ├── src/
│ │ ├── main.rs
│ │ ├── routes/
│ │ │ └── ws.rs
│ │ ├── models/
│ │ │ └── message.rs
│ │ └── services/
│ │ ├── chat_server.rs
│ │ ├── chat_session.rs
│ │ └── ai_model.rs
│ ├── Cargo.toml
│ └── .env
└── frontend/
├── src/
│ ├── components/
│ │ ├── ChatInterface.tsx
│ │ └── Message.tsx
│ ├── pages/
│ │ └── ChatPage.tsx
│ ├── services/
│ │ └── websocket.ts
│ └── types/
│ └── ChatMessage.ts
├── package.json
└── tsconfig.json

# Prerequisites

- Node.js and npm
- Rust and Cargo
- Ollama AI model (locally installed)

## Setup Instructions

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/ai-chatbot.git
   cd ai-chatbot
   ```

2. Run the setup script:

   ```
   chmod +x setup_local.zsh
   ./setup_local.zsh
   ```

3. Start the backend:

   ```
   cd backend
   cargo run
   ```

4. In a new terminal, start the frontend:

   ```
   cd frontend
   npm start
   ```

5. Open your browser and navigate to `http://localhost:3000` to use the chat bot.

## Development

- Backend: The Rust backend uses Actix for the web server and WebSocket handling. The AI model integration is done through the `ai_model.rs` service.
- Frontend: The React frontend uses TypeScript and Tailwind CSS. The WebSocket communication is handled in the `websocket.ts` service.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
