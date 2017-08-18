# Conrod Android skeleton

This repository provides a simple example of using the Conrod Rust GUI library on Android.

## Quick start

Before starting, make sure you have an Android SDK at `$ANDROID_HOME`, the NDK at `$NDK_HOME`, a JDK and Gradle installed. Additionally, make sure you have the `platform-tools`, `platforms;android-18` and `build-tools;26.0.1` Android SDK packages installed (use `sdkmanager` to install them if you don't).

    rustup target add arm-linux-androideabi
    cargo install cargo-apk
    git clone https://github.com/jbg/conrod-android-skeleton
    cd conrod-android-skeleton
    cargo apk build
    adb install -r target/android-artifacts/app/build/outputs/apk/app-debug.apk
    adb logcat | grep Rust
    # launch the app on your device!
