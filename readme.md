#Mushie Bot

mushie bot is a multi purpose discord bot for my personal discord. don't @ me youtube.

infra and ci stuff should go in infra, code in code


Dependencies
Songbird needs a few system dependencies before you can use it.

Opus - Audio codec that Discord uses. If you are on Windows and you are using the MSVC toolchain, a prebuilt DLL is provided for you, you do not have to do anything. On other platforms, you will have to install it. You can install the library with apt install libopus-dev on Ubuntu or pacman -S opus on Arch Linux. If you do not have it installed it will be built for you. However, you will need a C compiler and the GNU autotools installed. Again, these can be installed with apt install build-essential autoconf automake libtool m4 on Ubuntu or pacman -S base-devel on Arch Linux.
This is a required dependency. Songbird cannot work without it.

FFmpeg - Audio/Video conversion tool. You can install the tool with apt install ffmpeg on Ubuntu or pacman -S ffmpeg on Arch Linux.
This is an optional, but recommended dependency. It allows Songbird to convert from, for instance, .mp4 files to the audio format Discord uses.

youtube-dl - Audio/Video download tool. You can install the tool with Python's package manager, pip, which we recommend for youtube-dl. You can do it with the command pip install youtube_dl. Alternatively, you can install it with your system's package manager, apt install youtube-dl on Ubuntu or pacman -S youtube-dl on Arch Linux.
This is an optional dependency. It allows Songbird to download an audio source from the Internet, which will be converted to the audio format Discord uses.

TODO: setup nix+direnv or docker (probably both) so we don't have to install these manually. 

docker build stuff:

`sudo docker build -f ./infra/Dockerfile. -tag mushie_bot:latest`
`sudo docker run mushie_bot`
