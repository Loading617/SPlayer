#include "player.h"

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <audiofile.mp3>\n", argv[0]);
        return 1;
    }

    MusicPlayer player;
    player_init(&player);
    player_play(&player, argv[1]);
    player_cleanup(&player);

    return 0;
}
