# only for cmake --version >= 3.5.1
cmake_minimum_required(VERSION 3.5.1)

# project name
project(rcproject)

# set the C++17 standard
set(CMAKE_CXX_STANDARD 17)

# Include directories
include_directories(cproject/includes)

# Find all source files
file(GLOB SOURCES cproject/src/*.cpp)

# Add executable
add_executable(main ${SOURCES})

# Function to build the Rust library
add_custom_target(build_rust_lib ALL
    COMMAND cargo build --manifest-path ${CMAKE_SOURCE_DIR}/rc_lib/Cargo.toml --release --all
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/rc_lib
    COMMENT "Building Rust library"
)

# Ensure the Rust library is built before linking
add_dependencies(main build_rust_lib)

# Copy the Rust library files to the lib directory
add_custom_command(TARGET build_rust_lib POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/lib
    COMMAND ${CMAKE_COMMAND} -E copy_directory ${CMAKE_SOURCE_DIR}/rc_lib/target/release/lib ${CMAKE_SOURCE_DIR}/cproject/includes
    COMMAND ${CMAKE_COMMAND} -E copy_if_different 
        $<IF:$<PLATFORM_ID:Windows>,
            ${CMAKE_SOURCE_DIR}/rc_lib/target/release/*.lib,
            ${CMAKE_SOURCE_DIR}/rc_lib/target/release/*.a>
        ${CMAKE_SOURCE_DIR}/lib
    COMMENT "Copying Rust library files"
)

# Link the Rust library files to the executable
if (WIN32)
    file(GLOB FILES lib/*.lib)
else()
    file(GLOB FILES lib/*.a)
endif()

foreach(FILE ${FILES})
    message("Library file = ${FILE}")
    target_link_libraries(main ${FILE})
endforeach()

# Optionally run the executable after building
add_custom_target(run ALL
    DEPENDS main
    COMMAND ${CMAKE_COMMAND} -E remove -f ${CMAKE_BINARY_DIR}/main
    COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:main> ${CMAKE_BINARY_DIR}/main
    COMMAND ${CMAKE_BINARY_DIR}/main
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    COMMENT "Running the executable"
)
