#include <windows.h>
#include <shlobj.h>
#include <stdio.h>

void selectFolder(char *selectedPath, size_t maxPath) {
    BROWSEINFO bi = {0};
    bi.lpszTitle = "Select a folder containing music files";
    LPITEMIDLIST pidl = SHBrowseForFolder(&bi);

    if (pidl != NULL) {
        SHGetPathFromIDList(pidl, selectedPath);
    }
}

char folderPath[MAX_PATH];
selectFolder(folderPath, sizeof(folderPath));
printf("Selected folder: %s\n", folderPath);
