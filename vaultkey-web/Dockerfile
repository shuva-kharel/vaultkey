# Build stage
FROM node:20-alpine as builder

WORKDIR /app

# Copy package files
COPY package.json ./

# Remove package-lock if exists
RUN rm -f package-lock.json

# Install dependencies
RUN npm install --legacy-peer-deps --no-audit --no-fund

# Fix ajv dependency issue
RUN npm install ajv@8.12.0 --save-dev --legacy-peer-deps

# Copy source
COPY . .

# Build with CI=false
RUN CI=false GENERATE_SOURCEMAP=false npm run build

# Runtime stage
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/build /usr/share/nginx/html

# Copy nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 3000

CMD ["nginx", "-g", "daemon off;"]