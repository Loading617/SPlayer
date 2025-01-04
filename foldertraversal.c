void processFolder(const char *folderPath) {
    WIN32_FIND_DATA findFileData;
    HANDLE hFind;
    char searchPath[MAX_PATH];
    snprintf(searchPath, sizeof(searchPath), "%s\\*.*", folderPath);

    hFind = FindFirstFile(searchPath, &findFileData);
    if (hFind == INVALID_HANDLE_VALUE) return;

    do {
        if (findFileData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
            // Skip "." and ".."
            if (strcmp(findFileData.cFileName, ".") != 0 && strcmp(findFileData.cFileName, "..") != 0) {
                // Create subdirectory path
                char subFolderPath[MAX_PATH];
                snprintf(subFolderPath, sizeof(subFolderPath), "%s\\%s", folderPath, findFileData.cFileName);
                processFolder(subFolderPath); // Recursive call
            }
        } else {
            // Handle file (e.g., play audio)
            printf("Found file: %s\\%s\n", folderPath, findFileData.cFileName);
        }
    } while (FindNextFile(hFind, &findFileData) != 0);

    FindClose(hFind);
}
