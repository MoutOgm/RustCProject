# only for cmake --version >= 3.5.1
cmake_minimum_required(VERSION 3.5.1)

# project name
project(rcproject)

# set the C++17 standard
set(CMAKE_CXX_STANDARD 17)

set(LIB_PATTERN $<IF:$<PLATFORM_ID:Windows>,*.lib,*.a>)

# Include directories
include_directories(${CMAKE_SOURCE_DIR}/cproject/includes)

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

# Fonction pour trouver les bibliothèques Rust
function(find_rust_libs OUTPUT_LIBS)
    file(GLOB_RECURSE RUST_LIBS
        ${CMAKE_SOURCE_DIR}/rc_lib/target/release/*.lib
        ${CMAKE_SOURCE_DIR}/rc_lib/target/release/*.a
    )
    set(${OUTPUT_LIBS} ${RUST_LIBS} PARENT_SCOPE)
endfunction()

# Trouver toutes les bibliothèques Rust
find_rust_libs(RUST_LIBRARIES)


# Copier les fichiers
add_custom_command(TARGET build_rust_lib POST_BUILD
    # Créer les répertoires
    COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/lib
    COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/cproject/includes/lib

    # Copier tous les fichiers d'en-tête
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${CMAKE_SOURCE_DIR}/rc_lib/target/cxxbridge/*.hpp
        ${CMAKE_SOURCE_DIR}/cproject/includes/lib/



    # Copier toutes les bibliothèques
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${CMAKE_SOURCE_DIR}/rc_lib/target/release/${LIB_PATTERN}
        ${CMAKE_SOURCE_DIR}/lib/
    COMMENT "Copying Rust libraries and headers"
)


# Lier les bibliothèques
if (WIN32)
    file(GLOB LIBS ${CMAKE_SOURCE_DIR}/lib/*.lib)
else()
    file(GLOB LIBS ${CMAKE_SOURCE_DIR}/lib/*.a)
endif()

foreach(LIB ${LIBS})
    get_filename_component(LIB_NAME ${LIB} NAME)
    message("Linking library: ${LIB_NAME}")
    target_link_libraries(main PRIVATE ${LIB})
endforeach()

# Optionally run the executable after building
add_custom_target(run ALL
    DEPENDS main
    COMMAND ${CMAKE_BINARY_DIR}/main
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    COMMENT "Running the executable"
)
