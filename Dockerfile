FROM rust:1.75-bullseye

WORKDIR /usr/src/myapp
COPY . .

RUN apt update && apt install libclang-dev -y

RUN cargo install --path .

CMD ["mctl"]
