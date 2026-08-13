#!/bin/bash

echo "Starting Vaultkey deployment..."
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Build and start
echo "Building and starting containers..."
docker-compose up -d --build

echo ""
echo "✅ Vaultkey is now running!"
echo ""
echo "Access:"
echo "  - Web UI: http://localhost:3000"
echo "  - API: http://localhost:8000"
echo ""
echo "To stop: docker-compose down"
echo "To view logs: docker-compose logs -f"