# Taken from https://cli.vuejs.org/guide/deployment.html#docker-nginx
FROM node:16.15 as build-stage
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY ./ .
RUN npm run build

FROM nginx:stable-alpine as production-stage
RUN mkdir /app
COPY --from=build-stage /app/dist /app
COPY nginx.conf /etc/nginx/nginx.conf

# Add permissions for nginx user
RUN chown -R nginx:nginx /app && \
    chmod -R 755 /app && \
    chown -R nginx:nginx /var/cache/nginx && \
    chown -R nginx:nginx /var/log/nginx && \
    chown -R nginx:nginx /etc/nginx/conf.d
RUN touch /var/run/nginx.pid && \
        chown -R nginx:nginx /var/run/nginx.pid
USER nginx

ENTRYPOINT ["nginx", "-g", "daemon off;"]