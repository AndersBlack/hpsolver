FROM ubuntu:latest
COPY . /hp_solver
WORKDIR /hp_solver
RUN apt-get update -y && \
  apt-get upgrade -y && \
  apt-get install -y && \
  apt-get install -y cargo && \
  cargo build