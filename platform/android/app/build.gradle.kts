plugins {
    alias(libs.plugins.android.application)
}

android {
    namespace = "com.example.android_destroy_recreate_demo"

    compileSdk {
        version = release(37)
    }

    defaultConfig {
        applicationId = "com.example.android_destroy_recreate_demo"
        minSdk = 29
        targetSdk = 37
        versionCode = 1
        versionName = "1.0"
        ndk {
            abiFilters.addAll(listOf("arm64-v8a"))
        }
    }

    buildTypes {
        release {
            optimization {
                enable = true
                packageScope = setOf("androidx.**", "kotlin.**", "kotlinx.**")
            }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }

    //sourceSets {
    //    getByName("main") {
    //        assets {
    //            directories += "../../../assets"
    //        }
    //    }
    //}
}

dependencies {
    implementation(libs.games.activity)
    implementation(libs.material)
}
