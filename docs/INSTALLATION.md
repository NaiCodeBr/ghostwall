# Installation Guide

## Prerequisites

### Required
- Rust 1.75 or higher
- Node.js 18 or higher
- Git

### Optional
- Docker and Docker Compose
- Flutter SDK (for mobile app)
- Tauri CLI (for desktop app)

## Installation Steps

### 1. Clone the Repository

```bash
git clone https://github.com/NaiCodeBr/ghostwall.git
cd ghostwall
```

### 2. Install Rust Dependencies

```bash
cargo build --release
```

### 3. Install Frontend Dependencies

```bash
cd dashboard
npm install
cd ..
```

### 4. Install Mobile Dependencies (Optional)

```bash
cd mobile
flutter pub get
cd ..
```

## Running the Application

### API Server Only

```bash
cd api
cargo run --bin ghostwall-api
```

The API will be available at `http://localhost:8080`

### Desktop Dashboard

```bash
cd dashboard
npm run tauri dev
```

### Mobile App

```bash
cd mobile
flutter run
```

### Docker Deployment

```bash
cd docker
docker-compose up -d
```

## Configuration

### Environment Variables

Create a `.env` file in the root directory:

```env
RUST_LOG=info
DATABASE_PATH=./ghostwall.db
API_PORT=8080
```

### Database

The SQLite database will be created automatically on first run.

## Platform-Specific Setup

### Linux

For WiFi scanning, you may need:
```bash
sudo apt-get install iw
```

### macOS

No additional setup required for basic functionality.

### Windows

For WiFi scanning, install Npcap:
https://nmap.org/npcap/

## Troubleshooting

### Permission Denied (Linux/macOS)

WiFi scanning requires elevated privileges:
```bash
sudo cargo run --bin ghostwall-api
```

### Port Already in Use

Change the port in the configuration or stop the conflicting service.

### Missing Dependencies

Run:
```bash
cargo build
npm install
```

## Verification

Test the installation:

```bash
curl http://localhost:8080/api/health
```

Expected response:
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

## Next Steps

- Read the [API Documentation](API.md)
- Check the [Architecture](ARCHITECTURE.md)
- Review the [Threat Model](THREAT_MODEL.md)
