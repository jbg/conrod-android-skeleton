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

The Conrod & Glium setup code, event loop, render thread, etc are in `main.rs`. The code to lay out the GUI is in `app.rs`. Assets are loaded from the APK in `assets.rs`.

## Issues

This project currently depends on my own fork of Conrod (containing [just one patch](https://github.com/jbg/conrod/commit/cfab936b3b5b1bc6c419de5ae939f5a83b89baec)), due to [an issue](https://github.com/PistonDevelopers/conrod/issues/1045) with rendering on OpenGL ES.

Everything is much smaller than it should be, possibly due to a DPI bug (see the end of the conversation in [this issue](https://github.com/PistonDevelopers/conrod/issues/884)).
