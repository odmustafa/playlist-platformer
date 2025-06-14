# Technical Architecture - Playlist Platformer

## System Overview

The Playlist Platformer is a desktop application built using the Tauri framework, combining a Rust backend with a modern web frontend. The application integrates with both Spotify's Web API/Playback SDK and the Soulseek peer-to-peer network.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (Web Technologies)              │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │    React    │  │   Spotify   │  │    UI Components    │  │
│  │ Components  │  │ Web Playback│  │   (Playlists,       │  │
│  │             │  │     SDK     │  │   Tracks, Player)   │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Tauri Bridge Layer                       │
├─────────────────────────────────────────────────────────────┤
│                    Backend (Rust)                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Spotify   │  │  Soulseek   │  │    Download         │  │
│  │ API Client  │  │  Protocol   │  │    Manager          │  │
│  │             │  │ Implementation│  │                     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    System Layer                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ File System │  │   Network   │  │    Configuration    │  │
│  │   Access    │  │ Connections │  │     Storage         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Component Architecture

### Frontend Layer

#### React Application Structure
```
src/
├── components/
│   ├── auth/
│   │   ├── LoginScreen.jsx
│   │   └── AuthCallback.jsx
│   ├── playlists/
│   │   ├── PlaylistGrid.jsx
│   │   ├── PlaylistCard.jsx
│   │   └── PlaylistDetail.jsx
│   ├── tracks/
│   │   ├── TrackList.jsx
│   │   ├── TrackItem.jsx
│   │   └── TrackPreview.jsx
│   ├── player/
│   │   ├── SpotifyPlayer.jsx
│   │   └── PlayerControls.jsx
│   ├── soulseek/
│   │   ├── SearchButton.jsx
│   │   ├── SearchResults.jsx
│   │   └── DownloadQueue.jsx
│   └── common/
│       ├── LoadingSpinner.jsx
│       ├── ErrorBoundary.jsx
│       └── Layout.jsx
├── services/
│   ├── spotify.js
│   ├── spotifyPlayer.js
│   ├── soulseek.js
│   └── api.js
├── hooks/
│   ├── useSpotify.js
│   ├── usePlayer.js
│   └── useDownloads.js
├── utils/
│   ├── formatters.js
│   ├── constants.js
│   └── helpers.js
└── styles/
    ├── components/
    ├── globals.css
    └── themes.css
```

#### State Management
- **Local State**: React hooks for component-specific state
- **Global State**: Context API or Zustand for app-wide state
- **Persistent State**: localStorage for user preferences and tokens

### Backend Layer (Rust)

#### Module Structure
```
src-tauri/src/
├── main.rs                 # Application entry point
├── lib.rs                  # Library exports
├── commands/               # Tauri command handlers
│   ├── mod.rs
│   ├── spotify.rs
│   ├── soulseek.rs
│   └── downloads.rs
├── spotify/                # Spotify integration
│   ├── mod.rs
│   ├── auth.rs            # OAuth implementation
│   ├── api.rs             # Web API client
│   └── types.rs           # Data structures
├── soulseek/               # Soulseek integration
│   ├── mod.rs
│   ├── protocol.rs        # Protocol implementation
│   ├── network.rs         # Network handling
│   ├── search.rs          # Search functionality
│   ├── downloads.rs       # Download management
│   └── types.rs           # Data structures
├── storage/                # Data persistence
│   ├── mod.rs
│   ├── config.rs          # Configuration management
│   ├── cache.rs           # API response caching
│   └── database.rs        # Local data storage
├── utils/                  # Utility functions
│   ├── mod.rs
│   ├── crypto.rs          # Cryptographic functions
│   ├── network.rs         # Network utilities
│   └── files.rs           # File system utilities
└── error.rs               # Error handling
```

## Data Flow Architecture

### Authentication Flow
```
1. User clicks "Login with Spotify"
2. Frontend calls `spotify_auth_url` command
3. Backend generates OAuth URL with PKCE
4. System opens browser with auth URL
5. User authorizes application
6. Browser redirects to callback URL
7. Frontend extracts authorization code
8. Frontend calls `spotify_exchange_code` command
9. Backend exchanges code for access token
10. Tokens stored securely in backend
11. Frontend receives authentication confirmation
```

### Playlist Loading Flow
```
1. Frontend calls `get_playlists` command
2. Backend checks token validity
3. Backend makes API request to Spotify
4. Response cached locally
5. Playlist data returned to frontend
6. Frontend renders playlist grid
7. User selects playlist
8. Frontend calls `get_playlist_tracks` command
9. Backend fetches track details
10. Track list rendered with search buttons
```

### Soulseek Search Flow
```
1. User clicks "Search Soulseek" on track
2. Frontend calls `soulseek_search` command
3. Backend formats search query (Artist - Track)
4. Backend sends search request to Soulseek network
5. Backend collects search responses
6. Results parsed and filtered
7. Search results returned to frontend
8. Frontend displays results with download options
```

### Download Flow
```
1. User selects file to download
2. Frontend calls `add_download` command
3. Backend adds to download queue
4. Backend initiates peer connection
5. File transfer begins
6. Progress updates sent to frontend
7. Frontend displays download progress
8. File saved to local directory
9. Download completion notification
```

## Network Architecture

### Spotify Integration
- **Protocol**: HTTPS REST API
- **Authentication**: OAuth 2.0 with PKCE
- **Rate Limiting**: Built-in retry logic with exponential backoff
- **Caching**: API responses cached locally to reduce requests

### Soulseek Integration
- **Server Connection**: TCP connection to server.slsknet.org:2242
- **Peer Connections**: Direct TCP connections to other users
- **Protocol**: Custom binary protocol
- **Message Types**: Login, search, download, browse, etc.

### Connection Management
```rust
pub struct ConnectionManager {
    server_connection: Option<TcpStream>,
    peer_connections: HashMap<String, PeerConnection>,
    connection_pool: ConnectionPool,
    retry_policy: RetryPolicy,
}
```

## Security Architecture

### Token Management
- Access tokens stored in secure system keychain
- Refresh tokens encrypted at rest
- Automatic token refresh before expiration
- Secure token transmission over HTTPS

### Network Security
- All Spotify communication over HTTPS
- Soulseek connections validated
- Input sanitization for all user data
- File path validation for downloads

### File System Security
- Downloads restricted to designated directory
- File name sanitization
- Size limits on downloads
- Virus scanning integration (optional)

## Performance Considerations

### Frontend Optimization
- Virtual scrolling for large playlists
- Image lazy loading and caching
- Component memoization
- Bundle splitting and code splitting

### Backend Optimization
- Async/await for all I/O operations
- Connection pooling for network requests
- Efficient memory management
- Background task processing

### Caching Strategy
- API response caching with TTL
- Image caching for album artwork
- Search result caching
- Configuration caching

## Error Handling Strategy

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Spotify API error: {0}")]
    SpotifyApi(#[from] SpotifyError),
    
    #[error("Soulseek protocol error: {0}")]
    SoulseekProtocol(String),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
}
```

### Error Recovery
- Automatic retry for transient network errors
- Graceful degradation when services unavailable
- User-friendly error messages
- Detailed logging for debugging

## Scalability Considerations

### Concurrent Operations
- Multiple simultaneous downloads
- Parallel API requests
- Background search processing
- Non-blocking UI operations

### Resource Management
- Memory usage monitoring
- Connection limits
- Download bandwidth throttling
- CPU usage optimization

## Testing Architecture

### Unit Testing
- Individual component testing
- Mock external dependencies
- Protocol message parsing tests
- Authentication flow tests

### Integration Testing
- End-to-end workflow testing
- Cross-platform compatibility
- Network interruption handling
- Large dataset performance

### Performance Testing
- Load testing with large playlists
- Memory leak detection
- Network latency simulation
- Concurrent operation testing

## Deployment Architecture

### Build Process
- Cross-platform compilation
- Asset optimization
- Code signing
- Package generation

### Distribution
- GitHub releases
- Auto-updater integration
- Platform-specific installers
- Dependency management

This architecture provides a solid foundation for building a robust, scalable, and maintainable desktop application that integrates multiple complex systems while maintaining good performance and user experience.
