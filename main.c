#include <windows.h>
#include <commdlg.h>
#include <mmsystem.h>
#pragma comment(lib, "winmm.lib")

char filename[MAX_PATH];
HWND hwndPlay, hwndPause, hwndStop;

void OpenAudioFile(HWND hwnd);
void PlayAudio();
void PauseAudio();
void StopAudio();

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    WNDCLASS wc = {0};
    HWND hwnd;
    MSG msg;

    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = "CPlayer";

    RegisterClass(&wc);

    hwnd = CreateWindowEx(0, "CPlayer", "CPlayer", WS_OVERLAPPEDWINDOW,
                          CW_USEDEFAULT, CW_USEDEFAULT, 400, 300, NULL, NULL, hInstance, NULL);

    ShowWindow(hwnd, nCmdShow);

    while (GetMessage(&msg, NULL, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return (int)msg.wParam;
}
