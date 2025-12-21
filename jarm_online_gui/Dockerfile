# Taken from https://cli.vuejs.org/guide/deployment.html#docker-nginx
FROM node:24 AS build-stage
ARG VUE_APP_SENTRY_DSN
WORKDIR /app
COPY package*.json ./
RUN npm install --force # See https://github.com/codecov/codecov-javascript-bundler-plugins/issues/264
COPY ./ .
RUN npm run build

FROM nginx:stable-alpine AS production-stage
ARG VUE_APP_SENTRY_DSN
ENV VUE_APP_SENTRY_DSN=${VUE_APP_SENTRY_DSN}
RUN echo "Sentry dsn set to $VUE_APP_SENTRY_DSN"
RUN mkdir /app
COPY --from=build-stage /app/dist /app
COPY nginx.conf /etc/nginx/nginx.conf

ARG nginx_uid=1001
ARG nginx_gid=1001
RUN apk add shadow && usermod -u $nginx_uid -o nginx && groupmod -g $nginx_gid -o nginx

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