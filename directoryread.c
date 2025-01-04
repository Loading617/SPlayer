#include <windows.h>
#include <mmsystem.h>
#include <stdio.h>
#include <string.h>


int hasExtension(const char *filename, const char *extension) {
    const char *dot = strrchr(filename, '.');
    return (dot && strcmp(dot, extension) == 0);
}

int main() {
    WIN32_FIND_DATA findFileData;
    HANDLE hFind;
    char folderPath[MAX_PATH] = "C:\\Music";
    char searchPath[MAX_PATH];
    snprintf(searchPath, sizeof(searchPath), "%s\\*.*", folderPath);

    hFind = FindFirstFile(searchPath, &findFileData);

    if (hFind == INVALID_HANDLE_VALUE) {
        printf("Error: Unable to open folder %s\n", folderPath);
        return 1;
    }

    printf("Files in folder: %s\n", folderPath);

    do {
        if (!(findFileData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY)) {
            
            if (hasExtension(findFileData.cFileName, ".wav")) {
                printf("Playing: %s\n", findFileData.cFileName);

                
                char filePath[MAX_PATH];
                snprintf(filePath, sizeof(filePath), "%s\\%s", folderPath, findFileData.cFileName);

                
                PlaySound(filePath, NULL, SND_FILENAME | SND_ASYNC);
                printf("Press Enter to play the next file...\n");
                getchar(); // Wait for user input to proceed
            }
        }
    } while (FindNextFile(hFind, &findFileData) != 0);

    FindClose(hFind);
    return 0;
}
