import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [authState, setAuthState] = useState(null)

  useEffect(() => {
    // Check if already authenticated
    const accessToken = localStorage.getItem('spotify_access_token')
    if (accessToken) {
      setIsAuthenticated(true)
    }
  }, [])

  const handleSpotifyLogin = async () => {
    setLoading(true)
    setError('')

    try {
      // Get auth URL from backend
      const [authUrl, state, codeVerifier] = await invoke('spotify_auth_url')

      // Store state and code verifier for later use
      localStorage.setItem('spotify_state', state)
      localStorage.setItem('spotify_code_verifier', codeVerifier)

      // Open auth URL in external browser
      await invoke('open_url', { url: authUrl })

      // For now, we'll simulate the callback process
      // In a real implementation, you'd set up a local server to handle the callback
      console.log('Auth URL opened:', authUrl)
      setError('Please complete authentication in your browser, then return here.')

    } catch (err) {
      console.error('Authentication error:', err)
      setError(`Authentication failed: ${err}`)
    } finally {
      setLoading(false)
    }
  }

  const handleAuthCallback = async () => {
    // This is a simplified callback handler
    // In a real implementation, you'd extract the code from the callback URL
    const code = prompt('Enter the authorization code from the callback URL:')
    if (!code) return

    const state = prompt('Enter the state parameter from the callback URL:')
    if (!state) return

    const storedState = localStorage.getItem('spotify_state')
    const codeVerifier = localStorage.getItem('spotify_code_verifier')

    if (state !== storedState) {
      setError('State mismatch - authentication failed')
      return
    }

    setLoading(true)
    try {
      const authStateJson = await invoke('spotify_exchange_code', {
        code,
        codeVerifier
      })

      const authState = JSON.parse(authStateJson)

      if (authState.access_token) {
        localStorage.setItem('spotify_access_token', authState.access_token)
        localStorage.setItem('spotify_refresh_token', authState.refresh_token)
        setIsAuthenticated(true)
        setAuthState(authState)
        setError('')
      }
    } catch (err) {
      console.error('Token exchange error:', err)
      setError(`Token exchange failed: ${err}`)
    } finally {
      setLoading(false)
    }
  }

  const handleLogout = () => {
    localStorage.removeItem('spotify_access_token')
    localStorage.removeItem('spotify_refresh_token')
    localStorage.removeItem('spotify_state')
    localStorage.removeItem('spotify_code_verifier')
    setIsAuthenticated(false)
    setAuthState(null)
  }

  return (
    <div className="app">
      <header className="header">
        <h1>🎵 Playlist Platformer</h1>
        <p>Bridge your Spotify playlists with Soulseek P2P file sharing</p>
      </header>

      <main>
        {!isAuthenticated ? (
          <div className="auth-section">
            <h2>Connect to Spotify</h2>
            <p>Sign in with your Spotify account to access your playlists</p>

            {error && <div className="error">{error}</div>}

            <div style={{ marginTop: '1rem' }}>
              <button
                className="auth-button"
                onClick={handleSpotifyLogin}
                disabled={loading}
              >
                {loading ? 'Connecting...' : 'Login with Spotify'}
              </button>

              <button
                className="auth-button"
                onClick={handleAuthCallback}
                disabled={loading}
                style={{ marginLeft: '1rem', background: '#666' }}
              >
                Complete Authentication
              </button>
            </div>

            <div style={{ marginTop: '1rem', fontSize: '0.9rem', color: '#666' }}>
              <p>Note: This requires a Spotify Premium account for full functionality</p>
            </div>
          </div>
        ) : (
          <div className="main-content">
            <div className="success">
              ✅ Successfully connected to Spotify!
            </div>

            <div style={{ marginTop: '2rem' }}>
              <h2>Your Playlists</h2>
              <p>Playlist loading functionality will be implemented in Phase 2</p>

              <button
                className="auth-button"
                onClick={handleLogout}
                style={{ background: '#666', marginTop: '1rem' }}
              >
                Logout
              </button>
            </div>
          </div>
        )}
      </main>
    </div>
  )
}

export default App
