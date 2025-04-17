# Use an official Nginx runtime as a parent image
FROM nginx:alpine

# Copy static website files from the 'static' directory to the default Nginx public HTML directory
COPY ./static /usr/share/nginx/html

# Expose port 80 to allow traffic to Nginx
EXPOSE 80

# Command to run Nginx when the container launches
CMD ["nginx", "-g", "daemon off;"]
