# Conrod Android skeleton

This repository provides a simple example of using the Conrod Rust GUI library on Android.

It displays all the Conrod UI widgets using OpenGL. This repository contains no Java!

## Requirements

 * [rustup](https://rustup.rs/)
 * the [Android SDK](https://developer.android.com/studio/index.html#command-tools) at `$ANDROID_HOME`
 * a JDK (only tested with 8)
 * Gradle
 * the following Android SDK packages:
   * `ndk-bundle` (and set `$NDK_HOME` to point to it)
   * `platform-tools`
   * `platforms;android-18`
   * `build-tools;26.0.1`

## Quick start

    rustup target add arm-linux-androideabi
    cargo install cargo-apk
    git clone https://github.com/jbg/conrod-android-skeleton && cd conrod-android-skeleton
    cargo apk build
    adb install -r target/android-artifacts/app/build/outputs/apk/app-debug.apk
    # launch the app on your device!

## Project outline

The Conrod & Glium setup code, event loop, etc are in `main.rs`. The code to draw the GUI is in `app.rs`.
