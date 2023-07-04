CREATE TABLE IF NOT EXISTS user_playlists (
    playlist_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    over_all_rating INTEGER,
    over_all_review TEXT,
    PRIMARY KEY (user_id, playlist_id),
    CONSTRAINT fk_playlist FOREIGN KEY(playlist_id) REFERENCES playlists(spotify_id),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(spotify_id)
);


CREATE TABLE IF NOT EXISTS artist_images (
    artist_id TEXT NOT NULL,
    image_id uuid NOT NULL,
    PRIMARY KEY (artist_id, image_id),
    CONSTRAINT fk_artist FOREIGN KEY(artist_id) REFERENCES artists(spotify_id),
    CONSTRAINT fk_image FOREIGN KEY(image_id) REFERENCES images(id)
);


CREATE TABLE IF NOT EXISTS album_images (
    album_id TEXT NOT NULL,
    image_id uuid NOT NULL,
    PRIMARY KEY (album_id, image_id),
    CONSTRAINT fk_album FOREIGN KEY(album_id) REFERENCES albums(spotify_id),
    CONSTRAINT fk_image FOREIGN KEY(image_id) REFERENCES images(id)
);

CREATE TABLE IF NOT EXISTS playlist_images (
    playlist_id TEXT NOT NULL,
    image_id uuid NOT NULL,
    PRIMARY KEY (playlist_id, image_id),
    CONSTRAINT fk_playlist FOREIGN KEY(playlist_id) REFERENCES playlists(spotify_id),
    CONSTRAINT fk_image FOREIGN KEY(image_id) REFERENCES images(id)
);

CREATE TABLE IF NOT EXISTS playlist_tracks (
    playlist_id TEXT NOT NULL,
    track_id TEXT NOT NULL,
    PRIMARY KEY (playlist_id, track_id),
    CONSTRAINT fk_playlist FOREIGN KEY(playlist_id) REFERENCES playlists(spotify_id),
    CONSTRAINT fk_track FOREIGN KEY(track_id) REFERENCES tracks(spotify_id)
);

CREATE TABLE IF NOT EXISTS artist_tracks (
    artist_id TEXT NOT NULL,
    track_id TEXT NOT NULL,
    PRIMARY KEY (artist_id, track_id),
    CONSTRAINT fk_artist FOREIGN KEY(artist_id) REFERENCES artists(spotify_id),
    CONSTRAINT fk_track FOREIGN KEY(track_id) REFERENCES tracks(spotify_id)
);

CREATE TABLE IF NOT EXISTS artist_albums (
    artist_id TEXT NOT NULL,
    album_id TEXT NOT NULL,
    PRIMARY KEY (artist_id, album_id),
    CONSTRAINT fk_album FOREIGN KEY(album_id) REFERENCES albums(spotify_id),
    CONSTRAINT fk_artist FOREIGN KEY(artist_id) REFERENCES artists(spotify_id)
);