FROM rust:slim-bullseye

# Update new packages
RUN apt-get update

RUN apt-get install libssl-dev pkg-config -y

RUN apt-get upgrade -y

RUN mkdir /build

COPY ./ /build/

WORKDIR /build/

CMD ["cargo", "test"]