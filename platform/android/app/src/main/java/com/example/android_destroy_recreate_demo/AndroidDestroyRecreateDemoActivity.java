package com.example.android_destroy_recreate_demo;

import android.os.Build;
import android.os.Bundle;
import android.view.View;
import android.view.Window;
import android.view.WindowManager;

import androidx.core.view.WindowCompat;
import androidx.core.view.WindowInsetsCompat;
import androidx.core.view.WindowInsetsControllerCompat;

import com.google.androidgamesdk.GameActivity;

public class AndroidDestroyRecreateDemoActivity extends GameActivity {
    static {
        System.loadLibrary("android_destroy_recreate_demo");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        hideSystemUI();
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);

        if (hasFocus) {
            hideSystemUI();
        }
    }

    private void hideSystemUI() {
        Window window = getWindow();

        // Enable edge-to-edge rendering.
        WindowCompat.setDecorFitsSystemWindows(window, false);

        // Allow drawing into display cutout areas.
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            WindowManager.LayoutParams lp = window.getAttributes();

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
                lp.layoutInDisplayCutoutMode =
                        WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS;
            } else {
                lp.layoutInDisplayCutoutMode =
                        WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
            }

            window.setAttributes(lp);
        }

        View decorView = window.getDecorView();

        WindowInsetsControllerCompat controller =
                WindowCompat.getInsetsController(window, decorView);

        // Hide status bar and navigation bar.
        controller.hide(WindowInsetsCompat.Type.systemBars());

        // Bars can temporarily appear with edge swipe.
        controller.setSystemBarsBehavior(
                WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
        );
    }
}