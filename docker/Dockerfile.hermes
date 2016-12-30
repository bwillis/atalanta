FROM scorpil/rust

MAINTAINER Ben Willis

EXPOSE 3456

RUN apt-get update && \
  apt-get install -y git && \
  apt-get install -y  nodejs && \
  apt-get install -y npm

RUN mkdir /hermes
WORKDIR /hermes

RUN git clone -b master https://github.com/bwillis/hermes.git /hermes

# build the rust application
RUN cd rust && cargo build

# build the react application
RUN cd npm && npm install
