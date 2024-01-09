FROM node:latest


RUN npm install embedme -g

WORKDIR /app

COPY entrypoint.sh entrypoint.sh

ENTRYPOINT [ "/app/entrypoint.sh" ]
