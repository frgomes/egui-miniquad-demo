#!/bin/bash -eu

# see: https://macroquad.rs/articles/android/

DOWNLOADS=${DOWNLOADS:=${HOME}/Downloads}
TOOLS_HOME=${TOOLS_HOME:=${HOME}/tools}

function suggest_bash_scripts {
    echo "========================================================================================"
    echo "| You can install Java 8 easily, literally a one liner, by employing Bash Scripts:     |"
    echo "|                                                                                      |"
    echo "|     $ install_java 8                                                                 |"
    echo "|                                                                                      |"
    echo "| Installing Bash Scripts is also a one liner.                                         |"
    echo "| If you hare interested, please have a look at http://github.com/frgomes/bash-scripts |"
    echo "|                                                                                      |"
    echo "========================================================================================"
    echo "ABORTING INSTALLATION..."
    exit 1
}


function check_java8 {
    [[ ! -z "${JAVA_HOME:=}" ]] && java -version 2>&1 | grep version | cut -d" " -f3 | grep 1.8.0 > /dev/null || suggest_bash_scripts
}


function rustup_add_targets {
    rustup target add armv7-linux-androideabi
    rustup target add aarch64-linux-android
    rustup target add i686-linux-android
    rustup target add x86_64-linux-android
}


function android_download_tools {
    [[ -d "${DOWNLOADS}" ]] || mkdir -p "${DOWNLOADS}"
    pushd "${DOWNLOADS}"
    [[ -f sdk-tools-linux-4333796.zip ]] || wget -q https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
    [[ -f android-ndk-r25b-linux.zip  ]] || wget -q https://dl.google.com/android/repository/android-ndk-r25b-linux.zip
    popd
}


function android_sdk_install {
    [[ -d "${TOOLS_HOME}/android" ]] || mkdir -p "${TOOLS_HOME}/android"
    pushd "${TOOLS_HOME}"/android
    # installs command line tools for Linux
    [[ -d tools ]] || unzip -q "${DOWNLOADS}"/sdk-tools-linux-4333796.zip
    popd
}


function android_ndk_install {
    [[ -d "${TOOLS_HOME}/android" ]] || mkdir -p "${TOOLS_HOME}/android"
    pushd "${TOOLS_HOME}"/android
    # installs NDK
    [[ -d android-ndk-r25b ]] || unzip -q "${DOWNLOADS}"/android-ndk-r25b-linux.zip
    popd
}


function android_sdkmanager_configure {
    [[ -d "${TOOLS_HOME}/android" ]] || mkdir -p "${TOOLS_HOME}/android"
    pushd "${TOOLS_HOME}"/android
    # reduce amount of files to be downloaded
    local filter="29"
    #-- local filter="[23][0-9]"
    # options
    local options=""
    # configure command line tools for Linux
    [[ -d tools ]] && \
        echo "y" | tools/bin/sdkmanager --licenses && \
        tools/bin/sdkmanager "emulator" "platform-tools" && \
        tools/bin/sdkmanager --list | grep -E "build-tools;${filter}.0.0" | grep -v -E '\-rc' | sed -E 's/[ ]+//g' | cut -d"|" -f1 | sort | while read package ;do
            echo tools/bin/sdkmanager "${package}" "${options}"
            tools/bin/sdkmanager "${package}" "${options}"
	done && \
        tools/bin/sdkmanager --list | grep -E "platforms;android-${filter}" | sed -E 's/[ ]+//g' | cut -d"|" -f1 | sort | while read package ;do
            echo tools/bin/sdkmanager "${package}" "${options}"
            tools/bin/sdkmanager "${package}" "${options}"
	done && \
        tools/bin/sdkmanager --list | grep x86_64 | grep -E "system-images;android-${filter};google_apis;" | sed -E 's/[ ]+//g' | cut -d"|" -f1 | sort | while read package ;do
            echo tools/bin/sdkmanager "${package}" "${options}"
            tools/bin/sdkmanager "${package}" "${options}"
	done && \
        tools/bin/sdkmanager --update
    popd
}


function android_avdmanager_configure {
    [[ -d "${TOOLS_HOME}/android" ]] || mkdir -p "${TOOLS_HOME}/android"
    pushd "${TOOLS_HOME}"/android
    # reduce amount of files to be downloaded
    local filter="29"
    #-- local filter="[23][0-9]"
    # options
    local options="--force"
    # configure command line tools for Linux
    [[ -d tools ]] && \
        echo "y" | tools/bin/sdkmanager --licenses && \
        tools/bin/sdkmanager --list | grep x86_64 | grep -E "system-images;android-${filter};google_apis;" | sed -E 's/[ ]+//g' | cut -d"|" -f1 | sort | while read package ;do
	    local emulator=$(echo ${package} | cut -d';' -f2)
            echo tools/bin/avdmanager create avd --name "Nexus6P_${emulator}" --device "Nexus 6P" --package "${package}" --tag "google_apis" --abi "x86_64" "${options}"
            echo "y" | tools/bin/avdmanager create avd --name "Nexus6P_${emulator}" --device "Nexus 6P" --package "${package}" --tag "google_apis" --abi "x86_64" "${options}"
	done && \
        tools/bin/sdkmanager --update && \
        tools/bin/avdmanager list avd
    popd
}


function android_exports {
    local name=android_studio
    local group=300
    local config="${VIRTUAL_ENV:-${HOME}/.local/share/bash-scripts}"/postactivate/postactivate.d/${group}-${name}.sh
    [[ ! -d $(dirname "${config}") ]] && mkdir -p $(dirname "${config}")
    cat <<EOD > "${config}"
#!/bin/bash
export ANDROID_SDK_HOME=\${TOOLS_HOME:=$HOME/tools}/android
export ANDROID_NDK_HOME=\${TOOLS_HOME:=$HOME/tools}/android/android-ndk-r25b
export ANDROID_API_LEVEL=29
export     ANDROID_HOME=\${ANDROID_SDK_HOME}
export         NDK_HOME=\${NDK_HOME}
export PATH=\${PATH}:\${ANDROID_SDK_HOME}/tools/bin:\${ANDROID_SDK_HOME}/platform-tools::\${ANDROID_SDK_HOME}/emulator

export ANDROID_SDK_ROOT=\${ANDROID_SDK_HOME}
export ANDROID_NDK_ROOT=\${ANDROID_NDK_HOME}

#[target.aarch64-linux-android]
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=\${ANDROID_NDK_HOME}/25.2.8784281/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android\${ANDROID_API_LEVEL}-clang
export CARGO_TARGET_AARCH64_LINUX_ANDROID_AR=\${ANDROID_NDK_HOME}/25.2.8784281/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar

#[target.armv7-linux-android]
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=\${ANDROID_NDK_HOME}/25.2.8784281/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi\${ANDROID_API_LEVEL}-clang
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_AR=\${ANDROID_NDK_HOME}/25.2.8784281/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar

#[target.i686-linux-android]
export CARGO_TARGET_I686_LINUX_ANDROID_LINKER=\${ANDROID_NDK_HOME}/25.2.8784281/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android\${ANDROID_API_LEVEL}-clang
export CARGO_TARGET_I686_LINUX_ANDROID_AR=\${ANDROID_NDK_HOME}/25.2.8784281/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar

#[target.x86_64-linux-android]
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=\${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android\${ANDROID_API_LEVEL}-clang
export CARGO_TARGET_X86_64_LINUX_ANDROID_AR=\${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-ar
EOD
  chmod ugo+x "${config}"
  echo "${config}"
}


check_java8 && \
  rustup_add_targets && \
    android_download_tools && android_sdk_install && android_ndk_install && \
      android_sdkmanager_configure && android_avdmanager_configure && \
        android_exports
