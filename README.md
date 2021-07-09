# Ra Server
Simple markdown server

**TOC**
* [Install](#install)
  * [Linux](#linux)
  * [Windows (Administrator PowerShell)](#windows-administrator-powershell)
  * [macOS (Intel)](#macos-intel)
* [Command](#command)
* [Usage](#usage)

## Install
Download binary from release page.
### Linux
```console
$ sudo curl -L "https://github.com/fwhy/ra/releases/download/v0.1.0/ra-Linux-x86_64" -o /usr/local/bin/ra
$ sudo chmod +x /usr/local/bin/ra
```
### Windows (Administrator PowerShell)
```console
PS> New-Item $Env:ProgramFiles\ra
PS> Invoke-WebRequest "https://github.com/fwhy/ra/releases/download/v0.1.0/ra-Windows-x86_64.exe" -UseBasicParsing -OutFile $Env:ProgramFiles\ra\ra.exe
PS> $path = [Environment]::GetEnvironmentVariable("Path", "Machine")
PS> $path += ";$Env:ProgramFiles\ra"
PS> [Environment]::SetEnvironmentVariable("Path", $path, "Machine")
```
### macOS (Intel)
```console
$ sudo curl -L "https://github.com/fwhy/ra/releases/download/v0.1.0/ra-Darwin-x86_64" -o /usr/local/bin/ra
$ sudo curl +x /usr/local/bin/ra
```

## Command
If the directory looks like
```
/path
└─ to
   └─ md
      ├─ foo.md
      └─ bar.md
```
then, run it
```console
$ cd /path/to/md
$ ra
Document root: /path/to/md
Running at  http://localhost:8383/
[2021-07-08T15:39:24Z INFO  actix_server::builder] Starting 8 workers
[2021-07-08T15:39:24Z INFO  actix_server::builder] Starting "actix-web-service-127.0.0.1:8383" service on 127.0.0.1:8383
```
and now access to [http://localhost:8383/foo](http://localhost:8383/foo), [http://localhost:8383/bar](http://localhost:8383/bar).

## Usage

| Option | Description |
| :-- | :-- |
| -h, --help | Prints help information |
| -V, --version | Prints version information |
| -d, --dir &lt;directory&gt; | Document root directory. Default: current directory |
| -p, --port &lt;port&gt; | Use port number. Default: 8383 |
