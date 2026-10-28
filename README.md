This demo requires a patch of winit, see `Cargo.toml`

Build PC:
```sh
cargo r
```

Build Android:
```sh
export ANDROID_SDK_ROOT="$HOME/Android/Sdk"
./build_android.sh # Debug
# ./build_android.sh release feature0,feature1
```

# General issues on Android:
https://github.com/bevyengine/bevy/issues/26282
