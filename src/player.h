#ifndef PLAYER_H
#define PLAYER_H

#include <mpg123.h>
#include <alsa/asoundlib.h>
#include "xutils.h"

#define AUDIO_BUFFER_SIZE 4096

typedef struct {
    mpg123_handle *mpg;
    snd_pcm_t *pcm;
    int channels, encoding;
    long rate;
} MusicPlayer;

void player_init(MusicPlayer *player);
void player_play(MusicPlayer *player, const char *filename);
void player_cleanup(MusicPlayer *player);

#endif
