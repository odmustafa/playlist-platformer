# Playlist Platformer

A desktop application that bridges Spotify playlists with Soulseek peer-to-peer file sharing, built with Tauri (Rust + Web Technologies).

## 🎵 Features

### Spotify Integration
- **OAuth Authentication**: Secure login with Spotify accounts
- **Playlist Management**: Browse and view your Spotify playlists
- **Track Details**: Display comprehensive track information and metadata
- **Audio Previews**: 30-second track previews using Spotify Web Playback SDK

### Soulseek Integration
- **P2P Search**: Search the Soulseek network for tracks from your playlists
- **Download Management**: Queue and manage file downloads with progress tracking
- **Quality Filtering**: View file quality, bitrate, and size information
- **Direct Downloads**: Download files directly from other users

### User Experience
- **Modern Interface**: Clean, responsive design built with web technologies
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Real-time Updates**: Live progress tracking and status updates
- **Keyboard Shortcuts**: Efficient navigation and control

## 🚀 Quick Start

### Prerequisites
- **Spotify Premium Account** (required for Web Playback SDK)
- **Rust** (latest stable version)
- **Node.js** (version 18 or higher)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/odmustafa/playlist-platformer.git
   cd playlist-platformer
   ```

2. **Install Tauri CLI**
   ```bash
   cargo install tauri-cli
   ```

3. **Install dependencies**
   ```bash
   npm install
   ```

4. **Configure Spotify credentials**
   - The `spotify.settings.json` file already contains the necessary credentials
   - Create a Spotify app at https://developer.spotify.com/dashboard if needed

5. **Run in development mode**
   ```bash
   cargo tauri dev
   ```

## 📁 Project Structure

```
playlist-platformer/
├── docs/                          # Comprehensive documentation
│   ├── PROJECT_OUTLINE.md         # High-level project overview
│   ├── IMPLEMENTATION_GUIDE.md    # Step-by-step implementation
│   ├── SOULSEEK_INTEGRATION.md    # Soulseek protocol details
│   ├── SPOTIFY_WEB_PLAYBACK_SDK.md # Audio playback integration
│   ├── DEVELOPMENT_CHECKLIST.md   # Development progress tracking
│   ├── TECHNICAL_ARCHITECTURE.md  # System architecture details
│   └── RESOURCES_AND_LINKS.md     # Useful links and references
├── src-tauri/                     # Rust backend
│   ├── src/
│   │   ├── main.rs               # Application entry point
│   │   ├── spotify/              # Spotify API integration
│   │   ├── soulseek/             # Soulseek protocol implementation
│   │   └── downloads/            # Download management
│   └── Cargo.toml
├── src/                          # Frontend (React/Web)
│   ├── components/               # UI components
│   ├── services/                 # API services
│   └── utils/                    # Utility functions
├── spotify.settings.json         # Spotify API credentials
└── README.md
```

## 🛠️ Development

### Architecture Overview

The application uses a hybrid architecture:
- **Backend (Rust)**: Handles Spotify API calls, Soulseek protocol, and file management
- **Frontend (Web)**: Modern web UI with React components and Spotify Web Playback SDK
- **Tauri Bridge**: Secure communication between frontend and backend

### Key Technologies
- **Tauri**: Desktop app framework
- **Rust**: Backend systems programming
- **React**: Frontend UI framework
- **Spotify Web API**: Playlist and track data
- **Spotify Web Playback SDK**: Audio preview functionality
- **Soulseek Protocol**: P2P file sharing network

### Development Workflow

1. **Start with Spotify Integration**
   - Implement OAuth authentication
   - Build playlist browsing functionality
   - Add Web Playback SDK for previews

2. **Add Soulseek Integration**
   - Study Nicotine+ implementation
   - Implement basic protocol communication
   - Add search and download functionality

3. **Polish and Optimize**
   - Improve UI/UX
   - Add error handling
   - Optimize performance

## 📚 Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[Project Outline](docs/PROJECT_OUTLINE.md)**: Overview of features and architecture
- **[Implementation Guide](docs/IMPLEMENTATION_GUIDE.md)**: Step-by-step development instructions
- **[Soulseek Integration](docs/SOULSEEK_INTEGRATION.md)**: P2P protocol implementation details
- **[Spotify Web Playback SDK](docs/SPOTIFY_WEB_PLAYBACK_SDK.md)**: Audio preview integration
- **[Development Checklist](docs/DEVELOPMENT_CHECKLIST.md)**: Progress tracking and milestones
- **[Technical Architecture](docs/TECHNICAL_ARCHITECTURE.md)**: System design and data flow
- **[Resources and Links](docs/RESOURCES_AND_LINKS.md)**: Useful references and documentation

## ⚖️ Legal and Ethical Considerations

### Important Disclaimers
- This application is for **educational and research purposes**
- Users are responsible for complying with copyright laws
- Respect artists' rights and support them through official channels
- P2P file sharing may have legal implications in your jurisdiction

### Terms of Service
- Spotify's Developer Terms of Service apply
- Soulseek network rules and terms apply
- Users must acknowledge responsibility for their actions

## 🔒 Security and Privacy

### Data Protection
- Spotify tokens stored securely using system keychain
- No user data collected or transmitted to third parties
- Local-only operation with no external analytics

### Network Security
- All Spotify communication over HTTPS
- Input validation and sanitization
- Secure file handling and path validation

## 🤝 Contributing

### Getting Started
1. Read the documentation in the `docs/` folder
2. Check the [Development Checklist](docs/DEVELOPMENT_CHECKLIST.md)
3. Follow the [Implementation Guide](docs/IMPLEMENTATION_GUIDE.md)

### Development Guidelines
- Follow Rust and JavaScript best practices
- Write comprehensive tests
- Document new features
- Respect legal and ethical boundaries

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Spotify** for their comprehensive Web API and Playback SDK
- **Nicotine+** project for Soulseek protocol reference implementation
- **Tauri** team for the excellent desktop app framework
- **Rust** and **React** communities for amazing tools and libraries

## 📞 Support

- **Issues**: Report bugs and request features via GitHub Issues
- **Documentation**: Comprehensive guides available in the `docs/` folder
- **Community**: Join discussions in the project repository

## 🚧 Project Status

This project is currently in **planning and early development** phase. The documentation provides a comprehensive roadmap for implementation.

### Current Progress
- ✅ Project planning and architecture design
- ✅ Comprehensive documentation created
- ⏳ Spotify integration implementation
- ⏳ Basic UI development
- ⏳ Soulseek protocol research and implementation
- ⏳ Download management system
- ⏳ Testing and optimization

### Next Steps
1. Set up Tauri project structure
2. Implement Spotify OAuth authentication
3. Build playlist browsing interface
4. Research and implement Soulseek protocol
5. Add download management functionality
6. Polish UI and add error handling

---

**Note**: This application is designed for educational purposes and personal use. Always respect copyright laws and support artists through official channels.
