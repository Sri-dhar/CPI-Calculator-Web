# Use the official lightweight nginx image based on Alpine Linux
FROM nginx:alpine

# Set the working directory inside the container
WORKDIR /usr/share/nginx/html

# Remove default nginx static assets
RUN rm -rf ./*

# Copy the built static files from the host to the nginx html directory
# Copy index.html and style.css first
COPY static/index.html .
COPY static/style.css .
# Copy the wasm/js package contents into a 'pkg' subdirectory within the nginx root
COPY pkg/ ./pkg/

# Expose port 80 (the default nginx port)
EXPOSE 80

# Start nginx when the container launches
# '-g daemon off;' ensures nginx stays in the foreground so Docker keeps the container running
CMD ["nginx", "-g", "daemon off;"]
