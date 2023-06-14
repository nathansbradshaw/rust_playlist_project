CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    date_created TIMESTAMP NOT NULL DEFAULT current_timestamp,
    last_updated TIMESTAMP NOT NULL DEFAULT current_timestamp,
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    spotify_id TEXT NOT NULL UNIQUE,
    access_token TEXT,
    spotify_access_token TEXT,
    spotify_refresh_token TEXT,
    spotify_exp TEXT,
    meta JSONB
);

CREATE TABLE IF NOT EXISTS playlists (
    id SERIAL PRIMARY KEY,
    spotify_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    href TEXT NOT NULL,
    owner TEXT,
    owner_href TEXT,
    owner_spotify_id TEXT,
    public BOOL,
    tracks_href TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS artists (
    id SERIAL PRIMARY KEY,
    spotify_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    href TEXT NOT NULL,
    popularity INTEGER
);

CREATE TABLE IF NOT EXISTS albums (
    id SERIAL PRIMARY KEY,
    spotify_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    uri TEXT NOT NULL,
    popularity INTEGER
);

CREATE TABLE IF NOT EXISTS images (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    height INTEGER,
    width INTEGER
);

CREATE TABLE IF NOT EXISTS tracks (
    id SERIAL PRIMARY KEY,
    spotify_id TEXT NOT NULL UNIQUE,
    href TEXT NOT NULL,
    spotify_uri TEXT NOT NULL,
    duration_ms INTEGER,
    album_id TEXT NOT NULL,
    CONSTRAINT fk_album FOREIGN KEY (album_id) REFERENCES albums(spotify_id)
);

CREATE TABLE IF NOT EXISTS comment (
    id SERIAL PRIMARY KEY,
    user_id TEXT NOT NULL,
    playlist_id TEXT NOT NULL,
    track_id TEXT NOT NULL,
    rating INTEGER,
    comment_text TEXT,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(spotify_id),
    CONSTRAINT fk_track FOREIGN KEY(track_id) REFERENCES tracks(spotify_id),
    CONSTRAINT fk_playlist FOREIGN KEY(playlist_id) REFERENCES playlists(spotify_id)
);