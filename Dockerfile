FROM node:14

RUN npm install embedme -g

# copy files. 
COPY . .

# run embedme 
RUN npx embedme $FILES

#


