#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
if [[ -z "$SCRIPT_DIR" ]]; then
    echo "Failed to determine script directory"
    exit 1
fi

cd "$SCRIPT_DIR"

: "${ANDROID_SDK_ROOT:?ANDROID_SDK_ROOT must be set}"

if [[ ! -d "$ANDROID_SDK_ROOT" ]]; then
    echo "ANDROID_SDK_ROOT does not exist: $ANDROID_SDK_ROOT"
    exit 1
fi

case "$(uname -s)" in
    Linux*)
        OS_IDENTIFIER="linux-x86_64"
        ;;
    Darwin*)
        OS_IDENTIFIER="darwin-x86_64"
        ;;
    *)
        echo "Unsupported operating system: $(uname -s)"
        exit 1
        ;;
esac

export ANDROID_NDK_ROOT="$ANDROID_SDK_ROOT/ndk/29.0.14206865"
export CMAKE_TOOLCHAIN_FILE_aarch64_linux_android="$ANDROID_NDK_ROOT/build/cmake/android.toolchain.cmake"
export CC_aarch64_linux_android="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$OS_IDENTIFIER/bin/aarch64-linux-android29-clang"
export CXX_aarch64_linux_android="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$OS_IDENTIFIER/bin/aarch64-linux-android29-clang++"
export AR_aarch64_linux_android="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$OS_IDENTIFIER/bin/llvm-ar"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$OS_IDENTIFIER/bin/aarch64-linux-android29-clang"

BUILD_TYPE="${1:-debug}"
CARGO_FEATURES="${2:-}"

CARGO_ARGS=(rustc --lib --target aarch64-linux-android)

case "$BUILD_TYPE" in
    release)
        CARGO_ARGS+=(--release)
        ;;
    debug)
        ;;
    *)
        echo "Unsupported build type: $BUILD_TYPE"
        echo "Usage: $0 [debug|release]"
        exit 1
        ;;
esac

if [[ -n "$CARGO_FEATURES" ]]; then
    CARGO_ARGS+=(--features "$CARGO_FEATURES")
fi

CARGO_ARGS+=(--crate-type=cdylib)

cargo "${CARGO_ARGS[@]}"

ANDROID_APP_SRC_MAIN="./platform/android/app/src/main"

JNI_LIBS_DIR="$ANDROID_APP_SRC_MAIN/jniLibs"
JNI_LIBS_DIR_ARM64="$JNI_LIBS_DIR/arm64-v8a"

if [[ -d "$JNI_LIBS_DIR" ]]; then
    rm -rf "$JNI_LIBS_DIR"
fi

mkdir -p "$JNI_LIBS_DIR_ARM64"

TARGET_DIR="target/aarch64-linux-android/$BUILD_TYPE"

cp "$TARGET_DIR/libandroid_destroy_recreate_demo.so" "$JNI_LIBS_DIR_ARM64/"

echo "Build completed successfully."
