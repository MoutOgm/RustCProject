# only for cmake --version >= 3.5.1
cmake_minimum_required(VERSION 3.5.1)

# project name
project(rcproject)

# creates the variable EXEC and sets it to hello
set(EXEC main)
set (lib_dir ${CMAKE_SOURCE_DIR})
# set the C++17 standard
set(CMAKE_CXX_STANDARD 17)

include_directories(cproject/includes)

file (GLOB SOURCES cproject/src/*.cpp)

# compiles the files defined by SOURCES to generante the executable defined by EXEC
add_executable(${EXEC} ${SOURCES})
if (WIN32)
    file(GLOB FILES lib/*.lib)
else()
    file(GLOB FILES lib/*.a)
endif()

foreach(FILE ${FILES})
    message("Library file = ${FILE}")
    target_link_libraries(main ${FILE})
endforeach()
