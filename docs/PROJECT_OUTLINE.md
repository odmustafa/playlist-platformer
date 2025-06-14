# Playlist Platformer - Project Outline

## Project Overview
A desktop application that integrates Spotify playlists with Soulseek peer-to-peer file sharing, built using Tauri (Rust + Web Technologies).

## Core Features

### 1. Spotify Integration
- **Authentication**: OAuth 2.0 flow with PKCE for secure login
- **Playlist Management**: Display user's playlists with cover art and metadata
- **Track Listing**: Show detailed track information for selected playlists
- **Audio Preview**: 30-second track previews using Spotify Web Playback SDK

### 2. Soulseek Integration
- **Search Functionality**: Search for tracks using "Artist - Track Name" format
- **Results Display**: Show search results with file quality, size, and user info
- **Download Management**: Queue and manage file downloads
- **Connection Management**: Handle Soulseek network connections

### 3. User Interface
- **Modern Design**: Clean, responsive interface using web technologies
- **Playlist Browser**: Grid/list view of user playlists
- **Track View**: Detailed track listing with integrated search buttons
- **Download Queue**: Progress tracking and management
- **Settings Panel**: Configuration for both Spotify and Soulseek

## Technical Architecture

### Backend (Rust)
- **Tauri Framework**: Desktop app wrapper and native system access
- **Spotify API Client**: HTTP client for Web API calls
- **Soulseek Protocol**: Custom implementation or library integration
- **File Management**: Download handling and organization
- **Configuration**: Settings persistence and management

### Frontend (Web Technologies)
- **Framework**: React/Vue.js or vanilla JavaScript
- **Spotify Web Playback SDK**: Audio preview functionality
- **UI Components**: Modern component library (e.g., Tailwind CSS)
- **State Management**: Application state and API data handling

### Data Flow
1. User authenticates with Spotify
2. App fetches user playlists via Web API
3. User selects playlist → app fetches track details
4. User clicks "Search Soulseek" → app searches P2P network
5. User selects files → app initiates downloads
6. Audio previews play through Web Playback SDK

## Key Challenges & Solutions

### 1. Soulseek Protocol Implementation
- **Challenge**: Complex P2P protocol with no official Rust library
- **Solution**: Study Nicotine+ Python implementation and port key components

### 2. Cross-Platform Compatibility
- **Challenge**: Desktop app needs to work on Windows, macOS, Linux
- **Solution**: Tauri provides excellent cross-platform support

### 3. Audio Playback
- **Challenge**: Integrating Spotify's web-based playback in desktop app
- **Solution**: Embed web view with Web Playback SDK

### 4. Legal Considerations
- **Challenge**: P2P file sharing legal implications
- **Solution**: Clear disclaimers, user responsibility, educational purpose

## Development Phases

### Phase 1: Foundation (Weeks 1-2)
- Set up Tauri project structure
- Implement Spotify OAuth authentication
- Basic UI layout and navigation

### Phase 2: Spotify Integration (Weeks 3-4)
- Playlist fetching and display
- Track detail views
- Web Playback SDK integration

### Phase 3: Soulseek Integration (Weeks 5-8)
- Protocol implementation study
- Basic search functionality
- Results display and parsing

### Phase 4: Download Management (Weeks 9-10)
- File download implementation
- Progress tracking
- Queue management

### Phase 5: Polish & Testing (Weeks 11-12)
- UI/UX improvements
- Error handling
- Cross-platform testing
- Documentation

## File Structure
```
playlist-platformer/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── spotify/     # Spotify API integration
│   │   ├── soulseek/    # Soulseek protocol implementation
│   │   └── downloads/   # Download management
│   └── Cargo.toml
├── src/                 # Frontend
│   ├── components/      # UI components
│   ├── pages/          # Application pages
│   ├── services/       # API services
│   └── utils/          # Utility functions
├── docs/               # Documentation
├── spotify.settings.json # Spotify credentials
└── README.md
```

## Success Metrics
- Successful Spotify authentication and playlist loading
- Functional Soulseek search with relevant results
- Working download system with progress tracking
- Stable audio preview functionality
- Cross-platform compatibility
