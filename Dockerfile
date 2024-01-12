FROM node:latest

RUN npm install embedme -g

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT [ "/entrypoint.sh" ]