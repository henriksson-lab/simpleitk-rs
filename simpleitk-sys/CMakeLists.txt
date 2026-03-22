cmake_minimum_required(VERSION 3.20)
project(simpleitk_vendor C CXX)

include(ExternalProject)

# Version pinned — update here to upgrade SimpleITK
set(SITK_VERSION "2.4.0")
set(SITK_URL "https://github.com/SimpleITK/SimpleITK/releases/download/v${SITK_VERSION}/SimpleITK-${SITK_VERSION}.tar.gz")

ExternalProject_Add(SimpleITK
    URL                      ${SITK_URL}
    DOWNLOAD_EXTRACT_TIMESTAMP ON
    SOURCE_SUBDIR            SuperBuild

    CMAKE_ARGS
        # build.rs passes -DCMAKE_INSTALL_PREFIX on the configure command line,
        # which takes precedence — this <INSTALL_DIR> is the ExternalProject default
        -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
        -DCMAKE_BUILD_TYPE=Release
        # MacOSX 26.2 SDK moved cxxabi.h into c++/v1/; use the 15.4 SDK where
        # it is still at the traditional location expected by ITK's GoogleTest.
        -DCMAKE_OSX_SYSROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX15.4.sdk

        # Only build what we need for the C bindings
        -DBUILD_SHARED_LIBS=OFF
        -DWRAP_DEFAULT=OFF
        -DWRAP_PYTHON=OFF
        -DWRAP_JAVA=OFF
        -DWRAP_CSHARP=OFF
        -DWRAP_LUA=OFF
        -DWRAP_TCL=OFF
        -DWRAP_R=OFF

        # Reduce build time — disable testing in both SimpleITK and ITK
        -DBUILD_TESTING=OFF
        -DBUILD_EXAMPLES=OFF
        -DSimpleITK_BUILD_DISTRIBUTE=OFF
        # These flags propagate into the ITK sub-build inside the SuperBuild
        -DITK_BUILD_TESTING=OFF
        -DModule_ITKGoogleTest=OFF

        -DCMAKE_BUILD_PARALLEL_LEVEL=${CMAKE_BUILD_PARALLEL_LEVEL}

    INSTALL_DIR ${CMAKE_INSTALL_PREFIX}
)
