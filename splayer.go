package main

import (
	"bytes"
	"fmt"
	"image/color"
	"io"
	"os"
	"path/filepath"
	"strings"

	"gioui.org/app"
	"gioui.org/font/gofont"
	"gioui.org/io/system"
	"gioui.org/layout"
	"gioui.org/op"
	"gioui.org/widget"
	"gioui.org/widget/material"
	"github.com/dhowden/tag"
	"github.com/hajimehoshi/go-mp3"
	"github.com/hajimehoshi/oto"
)

type Track struct {
	Path     string
	Title    string
	Artist   string
	Album    string
	Duration string
}

var (
	playButton   widget.Clickable
	nextButton   widget.Clickable
	prevButton   widget.Clickable
	playlist     []Track
	currentTrack int
)

func main() {
	go func() {
		w := new(app.Window)
		if err := loop(w); err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}
		os.Exit(0)
	}()
	app.Main()
}

func loop(w *app.Window) error {
	th := material.NewTheme(gofont.Collection())
	var ops op.Ops

	loadPlaylist("music/")

	for {
		e := <-w.Events()
		switch e := e.(type) {
		case system.FrameEvent:
			gtx := layout.NewContext(&ops, e)
			layout.Flex{
				Axis: layout.Vertical,
			}.Layout(gtx,
				layout.Rigid(func(gtx layout.Context) layout.Dimensions {
					return material.Button(th, &playButton, "Play").Layout(gtx)
				}),
				layout.Rigid(func(gtx layout.Context) layout.Dimensions {
					return material.Button(th, &prevButton, "Previous").Layout(gtx)
				}),
				layout.Rigid(func(gtx layout.Context) layout.Dimensions {
					return material.Button(th, &nextButton, "Next").Layout(gtx)
				}),
			)

			if playButton.Clicked() {
				playAudio(playlist[currentTrack].Path)
			}
			if nextButton.Clicked() {
				currentTrack = (currentTrack + 1) % len(playlist)
				playAudio(playlist[currentTrack].Path)
			}
			if prevButton.Clicked() {
				currentTrack = (currentTrack - 1 + len(playlist)) % len(playlist)
				playAudio(playlist[currentTrack].Path)
			}

			e.Frame(gtx.Ops)
		case system.DestroyEvent:
			return e.Err
		}
	}
}

func loadPlaylist(folder string) {
	files, _ := filepath.Glob(folder + "*.mp3")
	for _, file := range files {
		f, _ := os.Open(file)
		meta, _ := tag.ReadFrom(f)
		playlist = append(playlist, Track{
			Path:     file,
			Title:    meta.Title(),
			Artist:   meta.Artist(),
			Album:    meta.Album(),
			Duration: fmt.Sprintf("%d sec", meta.Length()),
		})
		f.Close()
	}
}

func playAudio(filePath string) {
	f, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer f.Close()

	decoded, err := mp3.NewDecoder(f)
	if err != nil {
		fmt.Println("Error decoding MP3:", err)
		return
	}

	buf := new(bytes.Buffer)
	io.Copy(buf, decoded)

	context, readyChan, _ := oto.NewContext(decoded.SampleRate(), 2, 2, 8192)
	<-readyChan
	player := context.NewPlayer()
	player.Write(buf.Bytes())
}
