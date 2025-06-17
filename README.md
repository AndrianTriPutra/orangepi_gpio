# 🔧 Orange Pi Zero GPIO Control in Rust

This project provides a minimal Rust application to **read digital inputs** and **write digital outputs** using GPIO pins on an **Orange Pi Zero**.

---
![rust on opi](img/beras.png)

## 📦 Features

- ✅ Reads digital inputs from 8 GPIO pins (DI0–DI7)
- ✅ Writes alternating values to 2 GPIO output pins (DO0–DO1)
- ⏱️ Looping every 60 seconds
- 📂 Uses [`sysfs_gpio`](https://docs.rs/sysfs_gpio) to interact with GPIO via `/sys/class/gpio`

---

## 🧱 GPIO Mapping

### 📥 Digital Inputs (Read)
```bash
| DI  |  GPIO  |
|-----|--------|
| DI0 | GPIO18 |
| DI1 | GPIO19 |
| DI2 | GPIO13 |
| DI3 | GPIO14 |
| DI4 | GPIO15 |
| DI5 | GPIO16 |
| DI6 | GPIO6  |
| DI7 | GPIO7  |
```

### 📤 Digital Outputs (Write)
```bash
| DO  |  GPIO  |
|-----|--------|
| DO0 | GPIO2  |
| DO1 | GPIO3  |
```

---
## 🚀 Getting Started
- ssh to orangepi
- git clone
- chmod +x orangepi_gpio
- ./orangepi_gpio

---
## 🚀 Run Instructions

### 1. ✅ Prerequisites

Build this project on your development machine and run it on an **Orange Pi Zero (ARMv7)**.
Install [`cross`](https://github.com/cross-rs/cross) for cross-compilation:

### 2. 🔧 Build the Project for ARM

💡 Make sure Rust and Cargo are installed on your dev machine.
```bash
# 1. Install cross
cargo install cross

# 2. Check version
cross --version

# 3. Build for ARMv7 (Orange Pi Zero)
cross build --release --target=armv7-unknown-linux-gnueabihf

*if needed
sudo usermod -aG docker $USER
newgrp docker

# 4. copy binary
cp target/armv7-unknown-linux-gnueabihf/release/orangepi_gpio release

```

