# Rust YouTube Content Cache Server

This project is a high-performance backend server built with Rust and the Axum web framework. It fetches YouTube video data, caches it in a local SQLite database to reduce API calls, and exposes a simple REST API.

This project is a rewrite and translation from an original TypeScript/Bun backend [youtube-api-proxy](https://github.com/TegarAditya/youtube-api-proxy), demonstrating a transition to Rust for improved performance and safety.

## Features

- **Caching Layer**: Uses SQLite (`rusqlite`) to cache responses from the YouTube API, with a configurable time-to-live (TTL).
- **External API Integration**: Fetches data from the YouTube Data API v3 using `reqwest`.
- **Asynchronous**: Built on Tokio and Axum for non-blocking, concurrent request handling.
- **Configuration Management**: Uses a `.env` file for secure management of API keys and settings.
- **Health Checks**: Includes a `/healthz` endpoint to monitor service and database status.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A YouTube Data API v3 key from the [Google Cloud Console](https://console.cloud.google.com/).

### Installation & Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/TegarAditya/youtube-api-proxy-rust
    cd youtube-api-proxy-rust
    ```

2.  **Create a `.env` file:**
    Create a file named `.env` in the root of the project and add your configuration variables.

    ```env
    # .env
    YOUTUBE_API_KEY="YOUR_API_KEY_HERE"
    SECRET_KEY="your-super-secret-key-for-clearing-cache"
    CACHE_TTL_SECONDS=86400 # Cache duration in seconds (default: 24 hours)
    PORT=3000 # Define port
    ```

3.  **Run the application:**
    Use `cargo run` to compile and start the server. The first build will take a moment to compile dependencies.

    ```bash
    cargo run
    ```

    The server will start on `http://127.0.0.1:3000`.

## API Endpoints

### 1. Get Content by ID

Fetches YouTube video data. It will first check the cache. If a valid cache entry is found, it's returned immediately. Otherwise, it fetches from the YouTube API and caches the new result.

- **Endpoint**: `GET /api/video/{id}`
- **`{id}`**: The YouTube video ID (e.g., `dQw4w9WgXcQ`).
- **Success Response (200 OK)**:
    ```json
    {
    "etag": "o4b5ZGzkKNVLEI60AlNbD7tvbPg",
    "items": [
        {
        "etag": "I6s3gc8P1aaOjn1nXKmxD-Z6Pn8",
        "id": "dQw4w9WgXcQ",
        "kind": "youtube#video",
        "snippet": {
            "categoryId": "10",
            "channelId": "UCuAXFkgsw1L7xaCfnd5JJOw",
            "channelTitle": "Rick Astley",
            "description": "The official video for “Never Gonna Give You Up” by Rick Astley...",
            "liveBroadcastContent": "none",
            "localized": {
            "description": "The official video for “Never Gonna Give You Up” by Rick Astley..."
            },
            "publishedAt": "2009-10-25T06:57:33Z",
            "thumbnails": {
            "default": {
                "height": 90,
                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/default.jpg",
                "width": 120
            },
            "high": {
                "height": 360,
                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg",
                "width": 480
            },
            "maxres": {
                "height": 720,
                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg",
                "width": 1280
            },
            "medium": {
                "height": 180,
                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/mqdefault.jpg",
                "width": 320
            },
            "standard": {
                "height": 480,
                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/sddefault.jpg",
                "width": 640
            }
            },
            "title": "Rick Astley - Never Gonna Give You Up (Official Video) (4K Remaster)"
        }
        }
    ],
    "kind": "youtube#videoListResponse",
    "pageInfo": {
        "resultsPerPage": 1,
        "totalResults": 1
    }
    }
    ```
Example using curl:
```bash
curl http://localhost:3000/api/video/dQw4w9WgXcQ
```

### 2. Clear Cache
Deletes all entries from the cache database. This is a protected endpoint that requires the SECRET_KEY from your .env file.

Endpoint: `DELETE /api/video/clear`

Query Parameter: `key=<your_secret_key>`

Success Response (200 OK):
```
Cache cleared successfully
```

Example using curl:
```bash
curl -X DELETE "http://localhost:3000/api/video/clear?key=your-super-secret-key-for-clearing-cache"
```

### 3. Health Check
Checks the status of the service, primarily its ability to connect to the database.

Endpoint: `GET /healthz`

Success Response (200 OK):

```
OK
```
Example using curl:

```bash
curl http://localhost:3000/healthz
```