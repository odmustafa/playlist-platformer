# Spotify API Setup Guide

## Overview
This guide will help you set up Spotify API credentials for the Playlist Platformer application.

## Prerequisites
- Spotify account (Premium recommended for full functionality)
- Access to Spotify Developer Dashboard

## Step 1: Create a Spotify App

1. **Go to Spotify Developer Dashboard**
   - Visit: https://developer.spotify.com/dashboard
   - Log in with your Spotify account

2. **Create a New App**
   - Click "Create app"
   - Fill in the details:
     - **App name**: `Playlist Platformer` (or your preferred name)
     - **App description**: `Desktop app for bridging Spotify playlists with P2P file sharing`
     - **Website**: Leave blank or add your GitHub repo URL
     - **Redirect URI**: `http://localhost:8080/callback`
   - Check the boxes for Terms of Service and Branding Guidelines
   - Click "Save"

3. **Configure App Settings**
   - In your app dashboard, click "Settings"
   - Add these Redirect URIs:
     - `http://localhost:8080/callback`
     - `http://localhost:3000/callback` (for development)
   - Save the settings

## Step 2: Get Your Credentials

1. **Copy Client ID and Secret**
   - In your app dashboard, you'll see:
     - **Client ID**: A long string of characters
     - **Client Secret**: Click "View client secret" to reveal it

2. **Create Local Configuration File**
   - Copy the template file:
     ```bash
     cp spotify.settings.json.template spotify.settings.json
     ```
   
   - Edit `spotify.settings.json` with your credentials:
     ```json
     {
       "SPOTIFY_CLIENT_ID": "your_actual_client_id_here",
       "SPOTIFY_CLIENT_SECRET": "your_actual_client_secret_here"
     }
     ```

## Step 3: Security Notes

### Important Security Considerations
- **NEVER commit `spotify.settings.json` to version control**
- The file is already added to `.gitignore` for protection
- Keep your Client Secret private and secure
- Regenerate credentials if they're ever compromised

### Environment Variables (Alternative)
Instead of using a JSON file, you can set environment variables:
```bash
export SPOTIFY_CLIENT_ID="your_client_id"
export SPOTIFY_CLIENT_SECRET="your_client_secret"
```

Then update the Rust code to read from environment variables as a fallback.

## Step 4: Required Scopes

The application requests these Spotify scopes:
- `playlist-read-private`: Access your private playlists
- `user-read-private`: Access your profile information
- `streaming`: Control playback in the app
- `user-read-email`: Access your email address
- `user-read-playback-state`: Read your current playback state
- `user-modify-playback-state`: Control your playback

## Step 5: Testing the Setup

1. **Run the application**:
   ```bash
   cargo tauri dev
   ```

2. **Test authentication**:
   - Click "Login with Spotify"
   - You should be redirected to Spotify's authorization page
   - After granting permissions, you'll be redirected back

3. **Troubleshooting**:
   - If you get "Invalid client" error: Check your Client ID
   - If you get "Invalid redirect URI" error: Verify the redirect URI in your app settings
   - If you get "Invalid client secret" error: Check your Client Secret

## Development vs Production

### Development Setup
- Use `http://localhost:8080/callback` as redirect URI
- Client credentials stored in local file
- Debug logging enabled

### Production Setup (Future)
- Use proper domain for redirect URI
- Store credentials securely (environment variables, key vault, etc.)
- Implement proper error handling and user feedback

## Rate Limits and Best Practices

### Spotify API Limits
- **Rate Limiting**: Spotify has rate limits on API calls
- **Caching**: Cache API responses when possible
- **Retry Logic**: Implement exponential backoff for failed requests

### Best Practices
- Only request necessary scopes
- Handle token expiration gracefully
- Provide clear error messages to users
- Respect user privacy and data

## Troubleshooting Common Issues

### "INVALID_CLIENT: Invalid client"
- Double-check your Client ID in `spotify.settings.json`
- Ensure there are no extra spaces or characters

### "INVALID_CLIENT: Invalid client secret"
- Verify your Client Secret is correct
- Make sure you're using the secret, not the ID

### "Invalid redirect URI"
- Check that `http://localhost:8080/callback` is added to your app settings
- Ensure the URI matches exactly (including protocol and port)

### "Access token expired"
- The app should handle token refresh automatically
- If issues persist, try logging out and back in

## Next Steps

Once your Spotify integration is working:
1. Test playlist loading functionality
2. Verify audio preview capabilities
3. Proceed to implement Soulseek integration

## Support

If you encounter issues:
- Check the [Spotify Web API documentation](https://developer.spotify.com/documentation/web-api)
- Review the [OAuth 2.0 guide](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow)
- Check the project's GitHub issues for similar problems
