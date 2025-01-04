#include <windows.h>
#include <mmsystem.h>
#include <stdio.h>

int main() {
    // Path to the audio file
    const char *filePath = "sample.wav";

    // Play the audio file
    printf("Playing %s...\n", filePath);
    if (PlaySound(filePath, NULL, SND_FILENAME | SND_ASYNC)) {
        printf("Press any key to stop the music...\n");
        getchar(); // Wait for user input
        PlaySound(NULL, NULL, 0); // Stop playback
    } else {
        printf("Failed to play the audio file.\n");
    }

    return 0;
}
