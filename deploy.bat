@echo off
echo ========================================
echo    Vaultkey - One Command Setup
echo ========================================
echo.

echo [1/3] Checking Docker...
docker --version >nul 2>&1
if errorlevel 1 (
    echo Docker is not installed. Please install Docker Desktop from https://docker.com
    pause
    exit /b 1
)

echo [2/3] Building and starting containers...
docker-compose up -d --build

echo [3/3] Checking status...
timeout /t 3 >nul
docker-compose ps

echo.
echo ========================================
echo    Vaultkey is now running!
echo ========================================
echo.
echo Access:
echo   - Web UI: http://localhost:3000
echo   - API: http://localhost:8000
echo   - Health: http://localhost:8000/health
echo.
echo To stop: docker-compose down
echo To view logs: docker-compose logs -f
echo.
pause