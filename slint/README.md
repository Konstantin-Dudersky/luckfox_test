SLINT_BACKEND=linuxkms-femtovg ./slint

cross build --target="armv7-unknown-linux-gnueabihf" --release; scp target/armv7-unknown-linux-gnueabihf/release/slint root@target:/root/slint_opengl
