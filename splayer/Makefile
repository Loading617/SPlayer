CC = gcc
CFLAGS = -Wall -O2
LIBS = -lxutils -lmpg123 -lasound

all: splayer

splayer: main.o player.o
	$(CC) $(CFLAGS) -o splayer main.o player.o $(LIBS)

main.o: src/main.c player.h
	$(CC) $(CFLAGS) -c src/main.c

player.o: src/player.c player.h
	$(CC) $(CFLAGS) -c src/player.c

clean:
	rm -f *.o splayer
