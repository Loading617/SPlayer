#include "player.h"

void player_init(MusicPlayer *player) {
    mpg123_init();
    player->mpg = mpg123_new(NULL, NULL);
    snd_pcm_open(&player->pcm, "default", SND_PCM_STREAM_PLAYBACK, 0);
}

void player_play(MusicPlayer *player, const char *filename) {
    unsigned char buffer[AUDIO_BUFFER_SIZE];
    size_t bytes_read;
    int err;

    if (mpg123_open(player->mpg, filename) != MPG123_OK) {
        x_log_error("Error opening file: %s", filename);
        return;
    }

    mpg123_getformat(player->mpg, &player->rate, &player->channels, &player->encoding);
    snd_pcm_set_params(player->pcm, SND_PCM_FORMAT_S16_LE, SND_PCM_ACCESS_RW_INTERLEAVED, player->channels, player->rate, 1, 500000);

    while (mpg123_read(player->mpg, buffer, AUDIO_BUFFER_SIZE, &bytes_read) == MPG123_OK) {
        snd_pcm_writei(player->pcm, buffer, bytes_read / 2);
    }

    mpg123_close(player->mpg);
}

void player_cleanup(MusicPlayer *player) {
    mpg123_delete(player->mpg);
    mpg123_exit();
    snd_pcm_close(player->pcm);
}
