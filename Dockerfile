FROM ubuntu:latest
LABEL authors="will-boyle"

ENTRYPOINT ["top", "-b"]