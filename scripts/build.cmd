@set ERRORS=0
@pushd "%~dp0.."
@call :cargo test --all-features
@call :cargo test
@call :cargo doc
@popd
@exit /b %ERRORS%

:cargo
cargo %*
@if ERRORLEVEL 1 set /A ERRORS=%ERRORS%+1
@exit /b %ERRORLEVEL%
