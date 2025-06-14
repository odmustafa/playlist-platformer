# Spotify Web Playback SDK Integration Guide

## Overview
This guide covers integrating the Spotify Web Playback SDK into the Tauri desktop application for audio preview functionality.

## Prerequisites
- Spotify Premium account (required for Web Playback SDK)
- Valid Spotify access token with appropriate scopes
- Modern web browser engine (provided by Tauri's webview)

## Required Scopes
Add these scopes to your Spotify authentication:
```
streaming
user-read-email
user-read-private
user-read-playback-state
user-modify-playback-state
```

## Implementation Steps

### 1. SDK Installation and Setup

#### Frontend Dependencies
```bash
npm install @spotify/web-playback-sdk
```

#### HTML Setup
Add to your main HTML file (`src/index.html`):
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Playlist Platformer</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body>
    <div id="app"></div>
    
    <!-- Spotify Web Playback SDK -->
    <script src="https://sdk.scdn.co/spotify-player.js"></script>
    <script type="module" src="/src/main.js"></script>
</body>
</html>
```

### 2. Player Service Implementation

Create `src/services/spotifyPlayer.js`:
```javascript
class SpotifyPlayerService {
    constructor() {
        this.player = null;
        this.deviceId = null;
        this.isReady = false;
        this.currentTrack = null;
        this.isPlaying = false;
        this.position = 0;
        this.duration = 0;
    }

    async initialize(accessToken) {
        return new Promise((resolve, reject) => {
            // Wait for Spotify SDK to load
            window.onSpotifyWebPlaybackSDKReady = () => {
                this.player = new window.Spotify.Player({
                    name: 'Playlist Platformer',
                    getOAuthToken: cb => { cb(accessToken); },
                    volume: 0.5
                });

                // Error handling
                this.player.addListener('initialization_error', ({ message }) => {
                    console.error('Failed to initialize:', message);
                    reject(new Error(message));
                });

                this.player.addListener('authentication_error', ({ message }) => {
                    console.error('Failed to authenticate:', message);
                    reject(new Error(message));
                });

                this.player.addListener('account_error', ({ message }) => {
                    console.error('Failed to validate Spotify account:', message);
                    reject(new Error(message));
                });

                this.player.addListener('playback_error', ({ message }) => {
                    console.error('Failed to perform playback:', message);
                });

                // Playback status updates
                this.player.addListener('player_state_changed', (state) => {
                    if (!state) return;

                    this.currentTrack = state.track_window.current_track;
                    this.isPlaying = !state.paused;
                    this.position = state.position;
                    this.duration = state.duration;

                    // Emit events for UI updates
                    this.onStateChange && this.onStateChange(state);
                });

                // Ready
                this.player.addListener('ready', ({ device_id }) => {
                    console.log('Ready with Device ID', device_id);
                    this.deviceId = device_id;
                    this.isReady = true;
                    resolve(device_id);
                });

                // Not Ready
                this.player.addListener('not_ready', ({ device_id }) => {
                    console.log('Device ID has gone offline', device_id);
                    this.isReady = false;
                });

                // Connect to the player
                this.player.connect().then(success => {
                    if (!success) {
                        reject(new Error('Failed to connect to Spotify player'));
                    }
                });
            };

            // Trigger SDK loading if already available
            if (window.Spotify) {
                window.onSpotifyWebPlaybackSDKReady();
            }
        });
    }

    async playTrack(trackUri, accessToken) {
        if (!this.isReady || !this.deviceId) {
            throw new Error('Player not ready');
        }

        try {
            const response = await fetch(`https://api.spotify.com/v1/me/player/play?device_id=${this.deviceId}`, {
                method: 'PUT',
                body: JSON.stringify({
                    uris: [trackUri],
                    position_ms: 0
                }),
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${accessToken}`
                }
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            return true;
        } catch (error) {
            console.error('Error playing track:', error);
            throw error;
        }
    }

    async pause() {
        if (this.player) {
            await this.player.pause();
        }
    }

    async resume() {
        if (this.player) {
            await this.player.resume();
        }
    }

    async togglePlay() {
        if (this.player) {
            await this.player.togglePlay();
        }
    }

    async seek(positionMs) {
        if (this.player) {
            await this.player.seek(positionMs);
        }
    }

    async setVolume(volume) {
        if (this.player) {
            await this.player.setVolume(volume);
        }
    }

    getCurrentState() {
        return {
            track: this.currentTrack,
            isPlaying: this.isPlaying,
            position: this.position,
            duration: this.duration,
            deviceId: this.deviceId,
            isReady: this.isReady
        };
    }

    onStateChange(callback) {
        this.onStateChange = callback;
    }

    disconnect() {
        if (this.player) {
            this.player.disconnect();
        }
    }
}

export default new SpotifyPlayerService();
```

### 3. Player Component

Create `src/components/SpotifyPlayer.jsx`:
```jsx
import React, { useState, useEffect } from 'react';
import spotifyPlayer from '../services/spotifyPlayer';
import spotifyService from '../services/spotify';

export function SpotifyPlayer() {
    const [playerState, setPlayerState] = useState({
        track: null,
        isPlaying: false,
        position: 0,
        duration: 0,
        isReady: false
    });
    const [volume, setVolume] = useState(50);

    useEffect(() => {
        initializePlayer();
        
        return () => {
            spotifyPlayer.disconnect();
        };
    }, []);

    const initializePlayer = async () => {
        try {
            const accessToken = spotifyService.accessToken;
            if (!accessToken) {
                console.error('No access token available');
                return;
            }

            await spotifyPlayer.initialize(accessToken);
            
            // Set up state change listener
            spotifyPlayer.onStateChange((state) => {
                setPlayerState({
                    track: state.track_window.current_track,
                    isPlaying: !state.paused,
                    position: state.position,
                    duration: state.duration,
                    isReady: true
                });
            });

        } catch (error) {
            console.error('Failed to initialize player:', error);
        }
    };

    const handlePlayPause = async () => {
        try {
            await spotifyPlayer.togglePlay();
        } catch (error) {
            console.error('Failed to toggle playback:', error);
        }
    };

    const handleSeek = async (event) => {
        const rect = event.currentTarget.getBoundingClientRect();
        const percent = (event.clientX - rect.left) / rect.width;
        const newPosition = percent * playerState.duration;
        
        try {
            await spotifyPlayer.seek(newPosition);
        } catch (error) {
            console.error('Failed to seek:', error);
        }
    };

    const handleVolumeChange = async (event) => {
        const newVolume = parseInt(event.target.value);
        setVolume(newVolume);
        
        try {
            await spotifyPlayer.setVolume(newVolume / 100);
        } catch (error) {
            console.error('Failed to set volume:', error);
        }
    };

    const formatTime = (ms) => {
        const seconds = Math.floor(ms / 1000);
        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;
        return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
    };

    if (!playerState.isReady) {
        return (
            <div className="spotify-player loading">
                <p>Initializing Spotify Player...</p>
            </div>
        );
    }

    return (
        <div className="spotify-player">
            {playerState.track && (
                <div className="track-info">
                    <img 
                        src={playerState.track.album.images[0]?.url} 
                        alt={playerState.track.album.name}
                        className="album-art"
                    />
                    <div className="track-details">
                        <h4>{playerState.track.name}</h4>
                        <p>{playerState.track.artists.map(artist => artist.name).join(', ')}</p>
                    </div>
                </div>
            )}

            <div className="player-controls">
                <button 
                    onClick={handlePlayPause}
                    className={`play-pause-btn ${playerState.isPlaying ? 'playing' : 'paused'}`}
                >
                    {playerState.isPlaying ? '⏸️' : '▶️'}
                </button>
            </div>

            <div className="progress-section">
                <span className="time-current">
                    {formatTime(playerState.position)}
                </span>
                <div className="progress-bar" onClick={handleSeek}>
                    <div 
                        className="progress-fill"
                        style={{ 
                            width: `${(playerState.position / playerState.duration) * 100}%` 
                        }}
                    />
                </div>
                <span className="time-total">
                    {formatTime(playerState.duration)}
                </span>
            </div>

            <div className="volume-section">
                <span>🔊</span>
                <input
                    type="range"
                    min="0"
                    max="100"
                    value={volume}
                    onChange={handleVolumeChange}
                    className="volume-slider"
                />
            </div>
        </div>
    );
}
```

### 4. Track Preview Component

Create `src/components/TrackPreview.jsx`:
```jsx
import React from 'react';
import spotifyPlayer from '../services/spotifyPlayer';
import spotifyService from '../services/spotify';

export function TrackPreview({ track }) {
    const handlePreview = async () => {
        try {
            const accessToken = spotifyService.accessToken;
            if (!accessToken) {
                throw new Error('No access token available');
            }

            await spotifyPlayer.playTrack(track.uri, accessToken);
        } catch (error) {
            console.error('Failed to play preview:', error);
            
            // Fallback to preview_url if available
            if (track.preview_url) {
                const audio = new Audio(track.preview_url);
                audio.play();
            } else {
                alert('Preview not available for this track');
            }
        }
    };

    return (
        <button 
            onClick={handlePreview}
            className="preview-btn"
            title="Play 30-second preview"
        >
            🎵 Preview
        </button>
    );
}
```

### 5. CSS Styling

Add to your CSS file:
```css
.spotify-player {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: #1db954;
    color: white;
    border-radius: 8px;
    margin: 1rem 0;
}

.track-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
}

.album-art {
    width: 50px;
    height: 50px;
    border-radius: 4px;
}

.track-details h4 {
    margin: 0;
    font-size: 0.9rem;
}

.track-details p {
    margin: 0;
    font-size: 0.8rem;
    opacity: 0.8;
}

.player-controls {
    display: flex;
    gap: 0.5rem;
}

.play-pause-btn {
    background: none;
    border: 2px solid white;
    color: white;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    cursor: pointer;
    font-size: 1rem;
}

.play-pause-btn:hover {
    background: rgba(255, 255, 255, 0.1);
}

.progress-section {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 2;
}

.progress-bar {
    flex: 1;
    height: 4px;
    background: rgba(255, 255, 255, 0.3);
    border-radius: 2px;
    cursor: pointer;
}

.progress-fill {
    height: 100%;
    background: white;
    border-radius: 2px;
    transition: width 0.1s ease;
}

.time-current,
.time-total {
    font-size: 0.8rem;
    min-width: 40px;
}

.volume-section {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.volume-slider {
    width: 80px;
}

.preview-btn {
    background: #1db954;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.8rem;
}

.preview-btn:hover {
    background: #1ed760;
}

.spotify-player.loading {
    justify-content: center;
    background: #333;
}
```

### 6. Integration with Main App

Update your main app component to include the player:
```jsx
import React, { useState, useEffect } from 'react';
import { SpotifyPlayer } from './components/SpotifyPlayer';
import { TrackPreview } from './components/TrackPreview';
import spotifyService from './services/spotify';

function App() {
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [playlists, setPlaylists] = useState([]);
    const [selectedPlaylist, setSelectedPlaylist] = useState(null);
    const [tracks, setTracks] = useState([]);

    useEffect(() => {
        // Check if already authenticated
        if (spotifyService.accessToken) {
            setIsAuthenticated(true);
        }
    }, []);

    const loadPlaylistTracks = async (playlistId) => {
        try {
            const response = await spotifyService.getPlaylistTracks(playlistId);
            setTracks(response.items);
        } catch (error) {
            console.error('Failed to load tracks:', error);
        }
    };

    return (
        <div className="app">
            {isAuthenticated && <SpotifyPlayer />}
            
            {/* Your existing playlist and track components */}
            {tracks.map((item, index) => (
                <div key={index} className="track-item">
                    <span>{item.track.name}</span>
                    <span>{item.track.artists.map(a => a.name).join(', ')}</span>
                    <TrackPreview track={item.track} />
                    {/* Your Soulseek search button */}
                </div>
            ))}
        </div>
    );
}

export default App;
```

## Important Notes

### Limitations
- Requires Spotify Premium subscription
- 30-second preview limit for non-premium content
- Device must be active in Spotify Connect

### Error Handling
- Always check for Premium subscription
- Handle network connectivity issues
- Provide fallback to preview_url when available

### Performance
- Initialize player only once
- Clean up resources on component unmount
- Handle state updates efficiently

### Testing
- Test with various track types (premium/free)
- Verify cross-platform compatibility
- Test network interruption scenarios
