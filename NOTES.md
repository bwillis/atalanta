# Download Docker for Mac
https://docs.docker.com/docker-for-mac/

# Install a Rust image
Rust forum discussion: https://users.rust-lang.org/t/creating-official-docker-image-for-rust/4165
Hosted DockerFile:
https://hub.docker.com/r/scorpil/rust/
Command to Run:
`docker pull scorpil/rust`

# Created a new docker image to get my code


# Mount a local directory on the container
You can edit your code locally and use git pull/push etc with you local keys, but let the container build and run the server. This is better?
`docker run -t -i -P --name dev -v /Users/ben/Google\ Drive/Hermes/rust:/hermes hermes`

# View a mapped port
`EXPOSE` with `-P` makes those ports exposed, but they bind to different ports on the parent aka http://localhost:XXXX. To view what the mapping is and see what the localhost port is run: `docker port cb1c7ee0713b`

# How to work with Docker
https://docs.docker.com/engine/getstarted/

# Notes
4 graphql crates, one updated this month:
https://crates.io/crates/juniper
