#ifndef _SPlayer_SPlayer_h
#define _SPlayer_SPlayer_h

#include <CtrlLib/CtrlLib.h>

using namespace Upp;

#define LAYOUTFILE <SPlayer/SPlayer.lay>
#include <CtrlCore/lay.h>

class SPlayer : public WithSPlayerLayout<TopWindow> {
public:
	SPlayer();
};

#endif
