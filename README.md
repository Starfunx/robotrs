# ROBOT
Firmware for two stepper motor differential drive robot.


## Setup
install target
```
rustup target install thumbv7m-none-eabi
```
programs required for debug: 
* **openocd** make the translation between gdb and device 
* **arm-none-eabi-gdb** gdb server for arm
* **cortex-debug vscode extension** to use gdb with vscode interface (breakpoints, watchpoints, etc..)

## build
```
cargo build --release
```

## debug
### without vscode
run openocd to connect to stm32:
```
openocd -f interface/stlink.cfg -f target/cs32f1x.cfg
```

then in an other terminal, run: 
```
cargo run
```
then press c to continue
