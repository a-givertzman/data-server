# data-server

Server application written on rust for linux os.

Multithread application handles events coming from subordinated devices over industrial protocols (Profinet, Modbus, IEC 60870-5-104) over tcp connections.
    - [x] Profinet
    - [ ] Modbus TCP
    - [ ] IEC 60870-5-104

## installation

- download latest snap7 package from [here](https://sourceforge.net/projects/snap7/files/) (tested on [v1.4.2](https://sourceforge.net/projects/snap7/files/1.4.2/))
- unzip
- cd into uziped:

```bash
cd snap7-full-1.4.2/build/unix
```

- compile & install (debian linux)

```bash
sudo make -f x86_64_linux.mk all install LibInstall=/usr/lib
```

## run

```bash
git clone https://github.com/a-givertzman/data-server.git
cd data-server
cargo run --release
```
