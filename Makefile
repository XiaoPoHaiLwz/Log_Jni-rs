# 定义变量
CARGO = cargo

# Android 相关变量
ANDROID_ARCH ?= aarch64-linux-android
ANDROID_API_LEVEL ?= 29

# 默认目标
.PHONY: all
all: debug

# 构建 release 版本
.PHONY: release
release:
	$(CARGO) build --release

# 构建 debug 版本
.PHONY: debug
debug:
	$(CARGO) build

# 清理构建产物
.PHONY: clean
clean:
	$(CARGO) clean

# Android 构建任务
.PHONY: android-setup
android-setup:
	rustup target add $(ANDROID_ARCH)
	$(CARGO) install cargo-ndk

.PHONY: android-build
android-build: android-setup
	$(CARGO) ndk -t $(ANDROID_ARCH) --platform=$(ANDROID_API_LEVEL) --bindgen build --release

.PHONY: android-debug
android-debug: android-setup
	$(CARGO) ndk -t $(ANDROID_ARCH) --platform=$(ANDROID_API_LEVEL) --bindgen build