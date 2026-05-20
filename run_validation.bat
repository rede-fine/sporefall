@echo off
setlocal enabledelayedexpansion

echo Running cargo test...
cargo test
if %errorlevel% neq 0 (
    echo Cargo test failed with exit code %errorlevel%
    exit /b %errorlevel%
)

echo.
echo Running trunk build...
trunk build
if %errorlevel% neq 0 (
    echo Trunk build failed with exit code %errorlevel%
    exit /b %errorlevel%
)

echo.
echo All validation commands passed successfully.
