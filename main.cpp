#include <CtrlLib/CtrlLib.h>
#include "miniaudio.h"

using namespace Upp;

class SPlayerWindow : public TopWindow {
    Button btnOpen, btnPlayPause;
    Label lblFile;
    String currentFile;
    bool isPlaying = false;
    ma_engine engine;
    ma_sound sound;

public:
    typedef SPlayerWindow CLASSNAME;

    SPlayerWindow() {
        Title("SPlayer");
        SetRect(0, 0, 400, 150);
        
        btnOpen.SetLabel("Open Audio").LeftPos(10, 120).TopPos(10, 30);
        btnPlayPause.SetLabel("Play").LeftPos(140, 80).TopPos(10, 30);
        lblFile.SetLabel("No file loaded").LeftPos(10, 380).TopPos(50, 20);
        
        Add(btnOpen);
        Add(btnPlayPause);
        Add(lblFile);
        
        btnOpen << [=] { OnOpen(); };
        btnPlayPause << [=] { OnPlayPause(); };
        
        ma_engine_init(NULL, &engine); 
    }

    ~SPlayerWindow() {
        ma_sound_uninit(&sound);
        ma_engine_uninit(&engine);
    }

    void OnOpen() {
        FileSelector fs;
        fs.Type("Audio Files", "*.mp3;*.wav;*.flac");
        if (fs.ExecuteOpen()) {
            currentFile = ~fs;
            lblFile.SetLabel(GetFileName(currentFile));
            ma_sound_uninit(&sound);
            if (ma_sound_init_from_file(&engine, currentFile.ToStd().c_str(), 0, NULL, NULL, &sound) != MA_SUCCESS) {
                PromptOK("Failed to load audio file.");
            }
            isPlaying = false;
            btnPlayPause.SetLabel("Play");
        }
    }

    void OnPlayPause() {
        if (currentFile.IsEmpty())
            return;
        
        if (!isPlaying) {
            ma_sound_start(&sound);
            btnPlayPause.SetLabel("Pause");
        } else {
            ma_sound_stop(&sound);
            btnPlayPause.SetLabel("Play");
        }
        isPlaying = !isPlaying;
    }
};

GUI_APP_MAIN
{
    SPlayerWindow().Run();
}
