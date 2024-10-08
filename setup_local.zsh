#!/bin/zsh

# Function to check if a directory exists
dir_exists() {
    [ -d "$1" ]
}

# Function to check if a file exists
file_exists() {
    [ -f "$1" ]
}

# Install Node.js and npm (if not already installed)
if ! command -v node &> /dev/null; then
    echo "Installing Node.js and npm..."
    brew install node
fi

# Install Rust (if not already installed)
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Create project directory if it doesn't exist
if ! dir_exists "ai-chatbot"; then
    mkdir ai-chatbot
fi
cd ai-chatbot

# Create frontend if it doesn't exist
if ! dir_exists "frontend"; then
    npx create-react-app frontend --template typescript
    cd frontend
    npm install react-router-dom @types/react-router-dom

    # Install Tailwind CSS and its dependencies
    npm install -D tailwindcss@latest postcss@latest autoprefixer@latest

    # Create Tailwind configuration file
    npx tailwindcss init -p

    # Update the CSS file to include Tailwind directives
    echo "@tailwind base;
    @tailwind components;
    @tailwind utilities;" > src/index.css

    # Update the build script in package.json
    sed -i '' 's/"start": "react-scripts start"/"start": "TAILWIND_MODE=watch react-scripts start"/' package.json
    sed -i '' 's/"build": "react-scripts build"/"build": "TAILWIND_MODE=build react-scripts build"/' package.json

    cd ..
else
    echo "Frontend already exists, skipping creation."
fi

# Create backend if it doesn't exist
if ! dir_exists "backend"; then
    cargo new backend
    cd backend
    cargo add actix actix-web actix-cors actix-web-actors tokio serde serde_json uuid chrono dotenv env_logger log futures tokio-stream
    # Note: ollama-rs is not available on crates.io, so it needs to be added manually to Cargo.toml
    echo 'ollama-rs = { path = "../../ollama-rs", features = ["stream", "chat-history"] }' >> Cargo.toml
    cd ..
else
    echo "Backend already exists, skipping creation."
fi

# Create project structure if directories don't exist
for dir in frontend/src/{components,pages,services,types} backend/src/{routes,models,services}; do
    if ! dir_exists "$dir"; then
        mkdir -p "$dir"
    fi
done

# Create frontend files if they don't exist
for file in frontend/src/{components/{ChatInterface.tsx,Message.tsx},pages/ChatPage.tsx,services/websocket.ts,types/ChatMessage.ts}; do
    if ! file_exists "$file"; then
        touch "$file"
    fi
done

# Create backend files if they don't exist
for file in backend/src/{main.rs,routes/ws.rs,models/message.rs,services/{chat_server.rs,chat_session.rs,ai_model.rs}}; do
    if ! file_exists "$file"; then
        touch "$file"
    fi
done

# Create .env file if it doesn't exist
if ! file_exists "backend/.env"; then
    echo "RUST_LOG=debug" > backend/.env
    echo "RUST_BACKTRACE=1" >> backend/.env
fi

echo "Project structure updated successfully!"