FROM rust:1.69-bullseye
WORKDIR /usr/src/mushie_bot

# installs system dependencies
RUN apt-get update 
RUN apt-get install -y libopus-dev 
RUN apt-get install -y ffmpeg 
RUN apt-get install -y pip 
RUN pip install youtube_dl 

COPY ./code .

# installs dependencies and compiles code
# RUN cargo install --path .
RUN cargo build -r

#RUN cargo run
ENTRYPOINT ["./target/release/mushie_bot"]
