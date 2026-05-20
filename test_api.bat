@echo off
REM Test 1: Basic fungi observations
echo === Test 1: Basic fungi query ===
curl -s "https://api.inaturalist.org/v1/observations?taxon_id=47124&per_page=2&quality_grade=research"

echo.
echo === Test 2: Fungi with user_login parameter ===
curl -s "https://api.inaturalist.org/v1/observations?taxon_id=47124&user_login=username&per_page=2" 2>&1 | findstr "422\|error\|status"

echo.
echo === Test 3: Check response headers for CORS ===
curl -i -s "https://api.inaturalist.org/v1/observations?taxon_id=47124&per_page=1" 2>&1 | findstr "Access-Control\|allow"
