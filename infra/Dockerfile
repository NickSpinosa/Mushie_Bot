FROM rust:1.69-bullseye
WORKDIR /usr/src/mushie_bot
COPY ./code .

# installs system dependencies
RUN apt-get update 
RUN apt-get install -y libopus-dev 
RUN apt-get install -y ffmpeg 
RUN apt-get install -y pip 
RUN pip install youtube_dl 

# installs dependencies and compiles code
RUN cargo install --path .

#RUN cargo run
CMD ["mushie_bot"]
