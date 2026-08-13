@echo off
echo Starting Vaultkey deployment...
echo.

docker-compose up -d --build

echo.
echo ✅ Vaultkey is now running!
echo.
echo Access:
echo   - Web UI: http://localhost:3000
echo   - API: http://localhost:8000
echo.
echo To stop: docker-compose down
echo To view logs: docker-compose logs -f
pause